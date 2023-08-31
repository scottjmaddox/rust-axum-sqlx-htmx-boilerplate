use axum::{response::Html, routing::get, Router};
use color_eyre::{eyre, eyre::WrapErr};
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqlitePool;
use std::net::SocketAddr;
use std::path::Path;

#[tokio::main]
async fn main() {
    color_eyre::install().unwrap();
    run().await.unwrap();
}

async fn run() -> eyre::Result<()> {
    let pool = create_db().await?;
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;
    assert_eq!(row.0, 150);

    let app = Router::new().route("/", get(handler));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn create_db() -> eyre::Result<SqlitePool> {
    // Create the database
    let db_url = std::env::var("DATABASE_URL")
        .wrap_err("DATABASE_URL env var not set")?;

    if !sqlx::Sqlite::database_exists(&db_url).await? {
        sqlx::Sqlite::create_database(&db_url).await?;
    }

    // Connect to the database
    let pool = SqlitePool::connect(&db_url).await?;

    // Migrate the database
    let migrations =
        if std::env::var("RUST_ENV") == Ok("production".to_string()) {
            std::env::current_exe()?.join("./migrations")
        } else {
            let crate_dir = std::env::var("CARGO_MANIFEST_DIR")?;
            Path::new(&crate_dir).join("./migrations")
        };

    sqlx::migrate::Migrator::new(migrations)
        .await?
        .run(&pool)
        .await?;

    Ok(pool)
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
