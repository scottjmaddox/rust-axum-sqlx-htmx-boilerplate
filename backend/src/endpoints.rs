use crate::database;
use crate::database::Database;
use crate::models;
use crate::templates;
use crate::templates::Templates;
use axum::extract::{Form, Query, State};
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::{get, Router};
use axum_macros::{debug_handler, FromRef};

pub fn router(db: Database, templates: Templates) -> axum::Router {
    Router::new()
        .route("/", get(Redirect::to("/contacts")))
        .route("/contacts", get(get_contacts))
        .route("/contacts/new", get(get_new_contact).post(post_new_contact))
        .fallback(not_found)
        .with_state(SharedState { db, templates })
}

#[derive(Clone, FromRef)]
struct SharedState {
    db: Database,
    templates: Templates,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("database error: {0}")]
    Database(#[from] database::Error),
    #[error("template error: {0}")]
    Templates(#[from] templates::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::Database(e) => e.into_response(),
            Self::Templates(e) => e.into_response(),
        }
    }
}

#[debug_handler]
async fn get_contacts(
    State(state): State<SharedState>,
    Query(search): Query<models::SearchQuery>,
) -> Result<Html<String>, Error> {
    let contacts = if let Some(q) = &search.q {
        state.db.search_contacts(&q).await?
    } else {
        state.db.get_contacts().await?
    };
    Ok(state.templates.render_contacts_html(&search, &contacts)?)
}

#[debug_handler]
async fn get_new_contact(
    State(templates): State<Templates>,
) -> Result<Html<String>, Error> {
    Ok(templates.render_new_contact_html(
        models::NewContact::default(),
        models::NewContactErrors::default(),
    )?)
}

#[debug_handler]
async fn post_new_contact(
    State(state): State<SharedState>,
    Form(new_contact): Form<models::NewContact>,
) -> Result<Response, Error> {
    match new_contact.validate() {
        Ok(()) => {
            state.db.insert_new_contact(&new_contact).await?;
            // TODO: flash "Created new contact!"?
            Ok(Redirect::to("/contacts").into_response())
        }
        Err(errors) => Ok(state
            .templates
            .render_new_contact_html(new_contact, errors)?
            .into_response()),
    }
}

#[debug_handler]
async fn not_found(
    State(templates): State<Templates>,
) -> Result<Html<String>, Error> {
    Ok(templates.render_not_found_html()?)
}
