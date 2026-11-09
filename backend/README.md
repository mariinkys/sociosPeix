# Socios Peix — backend

Rust + [axum](https://github.com/tokio-rs/axum) + PostgreSQL ([sqlx](https://github.com/launchbadge/sqlx)).
It serves the JSON API used by the Vue frontend, sends emails,
exports Excel files, runs the daily jobs (birthday emails, cleanup) and reads
scanned registration forms with OCR.

## Layout

The code follows Clean Architecture, in the style of
[Thodin/axum-clean-architecture](https://github.com/Thodin/axum-clean-architecture):
dependencies only point inwards, and the business logic never touches axum,
sqlx or any other framework type.

```
src/
├── domain/                 Entities and value types. No I/O.
│   ├── entities/           user, member, interest, email, ...
│   └── pagination.rs       Page / PageRequest / SortDirection
├── application/
│   ├── app_error.rs        AppError — every error a use case can return
│   ├── clock.rs            "now" in UTC, "today" in the hotel's time zone
│   └── use_cases/          One file per feature. Each defines the use cases
│                           AND the ports (traits) it needs, e.g. member.rs
│                           declares `MemberPersistence`.
├── adapters/               Implementations of the ports, plus the HTTP layer
│   ├── persistence/        PostgreSQL (one file per port)
│   ├── crypto/             bcrypt password hashing, JWT access tokens
│   ├── email/              Resend and Mailtrap HTTP clients
│   ├── excel/              .xlsx export
│   ├── ocr/                Registration-form OCR (uses the pdf-analyzer crate)
│   └── http/               axum: routes, extractors, validation, cookies, rate limiting
└── infra/                  config (env vars), db (pool + migrations),
                            scheduler (daily jobs), setup (wires it all), app (router + middleware)

pdf-analyzer/               Library (+ dev CLI) that renders PDFs and runs OCR on
                            the fixed boxes of the registration form.
migrations/                 SQL migrations, applied automatically at startup.
```

**Data flow of a request:** `routes/members.rs` (parse + validate input) →
`MemberUseCases` (business rules) → `dyn MemberPersistence` (port) →
`adapters/persistence/member.rs` (SQL) → back out as a domain `Member` →
mapped to a JSON DTO in the route module.

### Adding a feature

Say you want to add *rooms*:

1. **Domain** — `domain/entities/room.rs` with a plain `Room` struct.
2. **Use cases + port** — `application/use_cases/room.rs`: a
   `RoomPersistence` trait (what you need from storage) and `RoomUseCases`
   holding an `Arc<dyn RoomPersistence>`. Business rules go here; unit-test them
   with an in-memory fake of the trait (see `use_cases/user.rs` for an example).
3. **Adapter** — `adapters/persistence/room.rs`:
   `impl RoomPersistence for PostgresPersistence`.
4. **HTTP** — `adapters/http/routes/rooms.rs` with the handlers and the
   request/response DTOs; add it to `routes/mod.rs`, add the use case to
   `AppState` (and its `from_ref!` list).
5. **Wiring** — construct `RoomUseCases` in `infra/setup.rs`.
6. **Schema** — a new `migrations/00NN_create_rooms.sql`.

Handlers get the signed-in user by taking an `AuthUser` (any role) or
`AdminUser` (admins only) argument; request bodies use `ValidJson<T>` with a
`Validate` impl so errors reach the frontend as `{status, message, timestamp}`.

## Running locally

```bash
cp .env.example .env          # point it at a local PostgreSQL
cargo run                     # http://localhost:8080 — migrations run on start
cargo test --workspace
```

The frontend dev server (`pnpm dev` in `../frontend`) talks to
`http://localhost:8080` and is allowed by `CORS_ALLOWED_ORIGIN`.

The OCR feature compiles ONNX Runtime, which makes the first build slow. For
day-to-day work you can leave it out:

```bash
cargo run --no-default-features
```

The first OCR request downloads the models (~13 MB) into `~/.oar`. The Docker
image already contains them.

## Configuration

All settings are environment variables (a `.env` file is read in development).

| Variable | Default | |
|---|---|---|
| `DB_HOST`, `DB_PORT`, `DB_NAME`, `DB_USERNAME`, `DB_PASSWORD` | `localhost`, `5432`, `sociosPeix`, —, — | Or a single `DATABASE_URL`. The database is created if missing. |
| `DB_MAX_CONNECTIONS` | `5` | |
| `JWT_SECRET` | — (required) | ≥ 32 characters. |
| `ACCESS_TOKEN_TTL_SECS` / `REFRESH_TOKEN_TTL_DAYS` | `10800` / `7` | |
| `COOKIE_SECURE` | `false` | `true` when served over HTTPS (also enables HSTS). |
| `CORS_ALLOWED_ORIGIN` | `http://localhost:5173` | Comma-separated. |
| `TRUST_PROXY` | `false` | Use `X-Real-IP` for rate limiting. Only when behind your own proxy. |
| `LOGIN_RATE_LIMIT_PER_MINUTE` | `5` | Per client, on login/register. |
| `MAX_LOGIN_ATTEMPTS` / `LOCKOUT_DURATION_MINUTES` | `5` / `15` | |
| `MAX_UPLOAD_MB` | `25` | Email attachments and scanned forms. |
| `APP_TIMEZONE` | `Europe/Madrid` | Defines "today" (birthdays, daily email quota) and job times. |
| `EMAIL_FROM` | `onboarding@resend.dev` | `Name <address>` is supported. |
| `RESEND_API_KEY` / `RESEND_DAILY_LIMIT` | — / `100` | Provider enabled when the key is set. |
| `MAILTRAP_API_KEY` / `MAILTRAP_DAILY_LIMIT` | — / `150` | |
| `BIRTHDAY_EMAILS_ENABLED` / `BIRTHDAY_EMAIL_TIME` | `true` / `08:00` | |
| `OCR_ENABLED` | `true` | |
| `OCR_IDLE_UNLOAD_MINUTES` | `15` | Free the OCR models' memory after this long unused (`0` = keep). |
| `OCR_PRELOAD` | `false` | Load the models at startup instead of on first use. |
| `OCR_TEMPLATE_PATH` | bundled | Use a different form template (`pdf-analyzer/template.toml`). |
| `BIND_ADDR` | `0.0.0.0:8080` | |
| `RUST_LOG` / `LOG_FORMAT` | `info,sqlx=warn` / text | `LOG_FORMAT=json` for structured logs. |
| `TOKIO_WORKER_THREADS` | min(CPUs, 4) | |

## Notes on behaviour

- Timestamps are stored as UTC and returned as ISO-8601 with a `Z`, so the
  browser shows them in local time.
- Emails to several members are sent as BCC batches of 49, so members never
  see each other's addresses.
- Genders, countries and the interest list are cached in memory (interests are
  invalidated on every change).
- Existing databases created by the previous Spring/Flyway backend are picked
  up automatically: the Flyway history is adopted and only new migrations run.
