use std::net::SocketAddr;

use anyhow::Context;
use tracing::info;

use sociospeix::infra::{
    app::create_app,
    config::AppConfig,
    setup::{init_app_state, init_tracing, spawn_jobs},
};

fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    init_tracing();

    match std::env::args().nth(1).as_deref() {
        None | Some("serve") => {}
        Some("healthcheck") => return healthcheck(),
        Some("fetch-ocr-models") => return fetch_ocr_models(),
        Some(other) => anyhow::bail!(
            "unknown command `{other}` (expected: serve, healthcheck, fetch-ocr-models)"
        ),
    }

    // A small app: a couple of worker threads is plenty and keeps the
    // per-thread memory overhead low. Override with TOKIO_WORKER_THREADS.
    let workers = std::env::var("TOKIO_WORKER_THREADS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or_else(|| std::thread::available_parallelism().map_or(2, |n| n.get().min(4)));

    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(workers)
        .max_blocking_threads(16)
        .enable_all()
        .build()?
        .block_on(run())
}

async fn run() -> anyhow::Result<()> {
    let config = AppConfig::from_env()?;
    let (state, jobs) = init_app_state(&config).await?;
    spawn_jobs(&config, jobs);

    let app = create_app(state, &config);
    let listener = tokio::net::TcpListener::bind(config.bind_addr)
        .await
        .with_context(|| format!("binding {}", config.bind_addr))?;
    info!(addr = %listener.local_addr()?, version = env!("CARGO_PKG_VERSION"), "backend listening");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;
    info!("shut down cleanly");
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.ok();
    };
    #[cfg(unix)]
    let terminate = async {
        if let Ok(mut s) = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        {
            s.recv().await;
        }
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! { _ = ctrl_c => {}, _ = terminate => {} }
}

/// `sociospeix healthcheck` — exit 0 if the local server answers. Lets the
/// container image stay shell-less (no curl/wget needed).
fn healthcheck() -> anyhow::Result<()> {
    use std::io::{Read, Write};
    use std::time::Duration;

    let port = std::env::var("BIND_ADDR")
        .ok()
        .and_then(|a| a.parse::<SocketAddr>().ok())
        .map_or(8080, |a| a.port());
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let mut stream = std::net::TcpStream::connect_timeout(&addr, Duration::from_secs(3))?;
    stream.set_read_timeout(Some(Duration::from_secs(3)))?;
    stream.write_all(b"GET /api/system/health HTTP/1.0\r\nHost: localhost\r\n\r\n")?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    if response.starts_with("HTTP/1.0 200") || response.starts_with("HTTP/1.1 200") {
        Ok(())
    } else {
        anyhow::bail!("unhealthy: {}", response.lines().next().unwrap_or_default())
    }
}

/// `sociospeix fetch-ocr-models` — download the OCR models into `$OAR_HOME`
/// (used at image build time so containers never download at runtime).
#[cfg(feature = "ocr")]
fn fetch_ocr_models() -> anyhow::Result<()> {
    let template = pdf_analyzer::config::FormTemplate::parse(pdf_analyzer::BUNDLED_TEMPLATE)?;
    pdf_analyzer::ocr::Engine::new(&template.ocr)?;
    info!("OCR models are available");
    Ok(())
}

#[cfg(not(feature = "ocr"))]
fn fetch_ocr_models() -> anyhow::Result<()> {
    anyhow::bail!("this binary was built without the `ocr` feature")
}
