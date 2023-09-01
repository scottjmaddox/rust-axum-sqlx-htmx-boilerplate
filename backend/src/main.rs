use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Form, Router};
use color_eyre::{eyre, eyre::WrapErr};
use minijinja_autoreload::AutoReloader;
use serde::{Deserialize, Serialize};
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqlitePool;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    color_eyre::install().unwrap();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    run().await.unwrap();
}

async fn run() -> eyre::Result<()> {
    // Create the database
    let db_url = std::env::var("DATABASE_URL")
        .wrap_err("DATABASE_URL env var not set")?;
    if !sqlx::Sqlite::database_exists(&db_url).await? {
        sqlx::Sqlite::create_database(&db_url).await?;
    }
    let db = SqlitePool::connect(&db_url).await?;

    // Migrate the database
    let migrations = std::path::Path::new("./migrations");
    sqlx::migrate::Migrator::new(migrations)
        .await?
        .run(&db)
        .await?;

    // prepare template engine
    let reloader = AutoReloader::new(|notifier| {
        let template_path = "./templates";
        let mut mj = minijinja::Environment::new();
        mj.set_loader(minijinja::path_loader(template_path));
        notifier.watch_path(template_path, true);
        Ok(mj)
    });

    // prepare handler
    let handler = Handler {
        db: db,
        template_reloader: Arc::new(reloader),
    };

    // prepare router
    let router = Router::new()
        .route("/", get(index))
        .route("/tasks", post(post_task))
        .route("/tasks", get(get_task_list))
        .route("/tasks/:id", get(get_task_details))
        .with_state(handler)
        .layer(TraceLayer::new_for_http());

    // serve
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;
    Ok(())
}

struct EyreResponse(eyre::Error);

impl IntoResponse for EyreResponse {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for EyreResponse
where
    E: Into<eyre::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

#[derive(Clone)]
struct Handler {
    db: SqlitePool,
    template_reloader: Arc<AutoReloader>,
}

impl Handler {
    fn render_template<S: Serialize>(
        &self,
        name: &str,
        ctx: S,
    ) -> Result<String, minijinja::Error> {
        let env = self.template_reloader.acquire_env()?;
        let template = env.get_template(name)?;
        let rendered = template.render(ctx)?;
        Ok(rendered)
    }
}

async fn index(
    State(handler): State<Handler>,
) -> Result<Html<String>, EyreResponse> {
    let rendered =
        handler.render_template("index.html.jinja", minijinja::context!())?;
    Ok(Html(rendered))
}

#[derive(Deserialize)]
struct NewTask {
    title: String,
    description: Option<String>,
}

#[derive(FromRow, Serialize)]
struct Task {
    id: i64,
    title: String,
    description: Option<String>,
    status: String,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

// TODO: impl IntoResponse for Task and Vec<Task>

async fn post_task(
    State(handler): State<Handler>,
    Form(new_task): Form<NewTask>,
) -> Result<Html<String>, EyreResponse> {
    // TODO: compile-time flag to delay all responses?
    // tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    let tasks: Task =
        sqlx::query_as!(Task,
        "INSERT INTO tasks (title, description) VALUES ($1, $2) RETURNING *",
        new_task.title,
        new_task.description
    )
        .fetch_one(&handler.db)
        .await?;
    let rendered = handler.render_template(
        "task-details.html.jinja",
        minijinja::context!(tasks),
    )?;
    Ok(Html(rendered))
}

async fn get_task_list(
    State(handler): State<Handler>,
) -> Result<Html<String>, EyreResponse> {
    let tasks: Vec<Task> = sqlx::query_as!(Task, "SELECT * FROM tasks")
        .fetch_all(&handler.db)
        .await?;
    let rendered = handler
        .render_template("task-list.html.jinja", minijinja::context!(tasks))?;
    Ok(Html(rendered))
}

async fn get_task_details(
    State(handler): State<Handler>,
    Path(id): Path<String>,
) -> Result<Html<String>, EyreResponse> {
    let task: Task =
        sqlx::query_as!(Task, "SELECT * FROM tasks WHERE id = $1", id)
            .fetch_one(&handler.db)
            .await?;
    let rendered = handler.render_template(
        "task-details.html.jinja",
        minijinja::context!(task),
    )?;
    Ok(Html(rendered))
}
