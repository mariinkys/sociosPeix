use std::time::Duration;

use anyhow::Context;
use sqlx::{
    Connection, PgConnection, PgPool,
    migrate::Migrator,
    postgres::{PgConnectOptions, PgPoolOptions},
};
use tracing::{info, warn};

use crate::infra::config::{AppConfig, DatabaseConfig};

pub static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

fn connect_options(db: &DatabaseConfig) -> anyhow::Result<PgConnectOptions> {
    let opts = match db {
        DatabaseConfig::Url(url) => url
            .parse::<PgConnectOptions>()
            .context("invalid DATABASE_URL")?,
        DatabaseConfig::Parts {
            host,
            port,
            name,
            username,
            password,
        } => PgConnectOptions::new()
            .host(host)
            .port(*port)
            .database(name)
            .username(username)
            .password(password),
    };
    // Timestamps are stored as naive UTC; make `now()` agree.
    Ok(opts
        .application_name("sociospeix")
        .options([("timezone", "UTC")]))
}

/// Connects (creating the database if needed), migrates, and returns the pool.
pub async fn init_db(config: &AppConfig) -> anyhow::Result<PgPool> {
    let options = connect_options(&config.database)?;
    create_database_if_missing(&options).await;

    let pool = PgPoolOptions::new()
        .max_connections(config.db_max_connections)
        .min_connections(0)
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Some(Duration::from_secs(5 * 60)))
        .connect_with(options)
        .await
        .context("connecting to PostgreSQL")?;
    info!("connected to database");

    adopt_flyway_history(&pool).await?;
    MIGRATOR.run(&pool).await.context("running migrations")?;
    info!("database migrations are up to date");
    Ok(pool)
}

/// Mirrors the old backend's behaviour of creating its database on first
/// start. Failures (e.g. no CREATEDB privilege) are only logged: the real
/// connection attempt afterwards reports the actual problem.
async fn create_database_if_missing(options: &PgConnectOptions) {
    let Some(name) = options.get_database().map(str::to_owned) else {
        return;
    };
    let admin = options.clone().database("postgres");
    let result: anyhow::Result<()> = async {
        let mut conn = PgConnection::connect_with(&admin).await?;
        let exists: bool =
            sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM pg_database WHERE datname = $1)")
                .bind(&name)
                .fetch_one(&mut conn)
                .await?;
        if !exists {
            let ident = name.replace('"', "\"\"");
            sqlx::query(sqlx::AssertSqlSafe(format!("CREATE DATABASE \"{ident}\"")))
                .execute(&mut conn)
                .await?;
            info!(database = %name, "database created");
        }
        conn.close().await?;
        Ok(())
    }
    .await;
    if let Err(e) = result {
        warn!(error = %e, "could not check/create the database; assuming it exists");
    }
}

/// Databases created by the previous (Spring/Flyway) backend already have the
/// schema. Record the Flyway-applied versions as applied sqlx migrations — the
/// SQL files are the same, numbered identically — so they aren't re-run.
async fn adopt_flyway_history(pool: &PgPool) -> anyhow::Result<()> {
    let (has_flyway, has_sqlx): (bool, bool) = sqlx::query_as(
        "SELECT to_regclass('flyway_schema_history') IS NOT NULL,
                to_regclass('_sqlx_migrations') IS NOT NULL",
    )
    .fetch_one(pool)
    .await?;
    if !has_flyway || has_sqlx {
        return Ok(());
    }

    let applied: Vec<String> = sqlx::query_scalar(
        "SELECT version FROM flyway_schema_history WHERE success AND version IS NOT NULL",
    )
    .fetch_all(pool)
    .await?;
    let applied: Vec<i64> = applied.iter().filter_map(|v| v.parse().ok()).collect();
    if applied.is_empty() {
        return Ok(());
    }

    let mut tx = pool.begin().await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS _sqlx_migrations (
            version BIGINT PRIMARY KEY,
            description TEXT NOT NULL,
            installed_on TIMESTAMPTZ NOT NULL DEFAULT now(),
            success BOOLEAN NOT NULL,
            checksum BYTEA NOT NULL,
            execution_time BIGINT NOT NULL
        )",
    )
    .execute(&mut *tx)
    .await?;
    let mut adopted = 0;
    for m in MIGRATOR.iter().filter(|m| applied.contains(&m.version)) {
        sqlx::query(
            "INSERT INTO _sqlx_migrations (version, description, success, checksum, execution_time)
             VALUES ($1, $2, true, $3, 0)",
        )
        .bind(m.version)
        .bind(m.description.as_ref())
        .bind(m.checksum.as_ref())
        .execute(&mut *tx)
        .await?;
        adopted += 1;
    }
    tx.commit().await?;
    info!(adopted, "adopted existing Flyway migration history");
    Ok(())
}
