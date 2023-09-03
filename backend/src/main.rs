use color_eyre::{eyre, eyre::WrapErr};
use tower_http::trace::TraceLayer;

mod database;
mod endpoints;
mod models;
mod templates;

#[tokio::main]
async fn main() {
    color_eyre::install().unwrap();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    run().await.unwrap();
}

async fn run() -> eyre::Result<()> {
    let db_url = std::env::var("DATABASE_URL")
        .wrap_err("DATABASE_URL env var not set")?;
    let db = database::Database::new(&db_url).await?;
    db.migrate("./migrations").await?;
    let templates = templates::Templates::new("./templates");
    let router =
        endpoints::router(db, templates).layer(TraceLayer::new_for_http());

    // Start the server
    let addr: std::net::SocketAddr = "127.0.0.1:8000".parse()?;
    println!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;
    Ok(())
}
