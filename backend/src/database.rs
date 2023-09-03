use crate::models;
use sqlx::migrate::MigrateDatabase;
use std::path::Path;

#[derive(Clone)]
pub struct Database(sqlx::sqlite::SqlitePool);

impl Database {
    pub async fn new(db_url: &str) -> Result<Self, Error> {
        if !sqlx::Sqlite::database_exists(&db_url).await? {
            sqlx::Sqlite::create_database(&db_url).await?;
        }
        let db_pool = sqlx::SqlitePool::connect(&db_url).await?;
        Ok(Database(db_pool))
    }

    pub async fn migrate(&self, dir: impl AsRef<Path>) -> Result<(), Error> {
        sqlx::migrate::Migrator::new(dir.as_ref())
            .await
            .map_err(|e| sqlx::Error::Migrate(Box::new(e)))?
            .run(&self.0)
            .await
            .map_err(|e| sqlx::Error::Migrate(Box::new(e)))?;
        Ok(())
    }

    pub async fn search_contacts(
        &self,
        query: &str,
    ) -> Result<Vec<models::Contact>, Error> {
        let arg = format!("%{}%", query);
        sqlx::query_as!(
            models::Contact,
            "SELECT * FROM contacts
            WHERE full_name LIKE $1
            OR phone LIKE $1
            OR email LIKE $1",
            arg
        )
        .fetch_all(&self.0)
        .await
        .map_err(Error)
    }

    pub async fn get_contacts(&self) -> Result<Vec<models::Contact>, Error> {
        sqlx::query_as!(models::Contact, "SELECT * FROM contacts")
            .fetch_all(&self.0)
            .await
            .map_err(Error)
    }

    pub async fn insert_new_contact(
        &self,
        contact: &models::NewContact,
    ) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO contacts (full_name, phone, email)
            VALUES ($1, $2, $3)",
            contact.full_name,
            contact.phone,
            contact.email
        )
        .execute(&self.0)
        .await
        .map_err(Error)?;
        Ok(())
    }

    pub async fn get_contact(
        &self,
        contact_id: i64,
    ) -> Result<Option<models::Contact>, Error> {
        sqlx::query_as!(
            models::Contact,
            "SELECT * FROM contacts WHERE id = $1",
            contact_id
        )
        .fetch_optional(&self.0)
        .await
        .map_err(Error)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("sqlx error: {0}")]
pub struct Error(#[from] sqlx::Error);

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        tracing::event!(tracing::Level::ERROR, "database error: {}", self);
        // TODO: if it's a temporary error, return a 503 instead
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong...",
        )
            .into_response()
    }
}
