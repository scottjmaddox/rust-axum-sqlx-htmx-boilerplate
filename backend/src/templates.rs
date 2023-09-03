use crate::models;
use axum::response::Html;
use minijinja_autoreload::AutoReloader;
use serde::Serialize;
use std::path::Path;
use std::sync::Arc;

#[derive(Clone)]
pub struct Templates(Arc<AutoReloader>);

impl Templates {
    pub fn new(dir: impl AsRef<Path> + Sync + Send + 'static) -> Self {
        let template_reloader = AutoReloader::new(move |notifier| {
            let dir = dir.as_ref();
            let mut env = minijinja::Environment::new();
            env.set_loader(minijinja::path_loader(dir));
            notifier.watch_path(dir, true);
            Ok(env)
        });
        Templates(Arc::new(template_reloader))
    }

    fn render(
        &self,
        name: &str,
        context: impl Serialize,
    ) -> Result<String, Error> {
        let env = self.0.acquire_env().map_err(Error)?;
        let template = env.get_template(name).map_err(Error)?;
        let rendered = template.render(context).map_err(Error)?;
        Ok(rendered)
    }

    pub fn render_contacts_html(
        &self,
        search: &models::SearchQuery,
        contacts: &[models::Contact],
    ) -> Result<Html<String>, Error> {
        self.render(
            "contacts.html.jinja",
            minijinja::context!(contacts, search),
        )
        .map(Html)
    }

    pub fn render_new_contact_html(
        &self,
        new_contact: models::NewContact,
        errors: models::NewContactErrors,
    ) -> Result<Html<String>, Error> {
        self.render(
            "new_contact.html.jinja",
            minijinja::context!(new_contact, errors),
        )
        .map(Html)
    }

    pub fn render_not_found_html(&self) -> Result<Html<String>, Error> {
        self.render("not_found.html.jinja", minijinja::context!())
            .map(Html)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("minijinja error: {0}")]
pub struct Error(minijinja::Error);

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        tracing::event!(tracing::Level::ERROR, "templates error: {:?}", self.0);
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong...",
        )
            .into_response()
    }
}
