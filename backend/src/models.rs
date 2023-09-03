use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;

#[derive(Serialize, Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
}

#[derive(FromRow, Serialize)]
pub struct Contact {
    pub id: i64,
    pub full_name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Default, Serialize, Deserialize)]
pub struct NewContact {
    pub full_name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
}

impl NewContact {
    pub fn validate(&self) -> Result<(), NewContactErrors> {
        let mut errors = NewContactErrors::default();
        // Validate full name is not empty
        if self.full_name.is_empty() {
            errors.full_name = Some("Full Name is required".to_string());
        }
        // Validate phone number against regex
        if let Some(phone) = &self.phone {
            if !phone.is_empty() {
                let re = regex::Regex::new(r"^\+?[0-9 ]+$").unwrap();
                if !re.is_match(phone) {
                    errors.phone = Some("Invalid phone number".to_string());
                }
            }
        }
        // Validate email against regex
        if let Some(email) = &self.email {
            if !email.is_empty() {
                let re = regex::Regex::new(r"^[^@]+@[^@]+$").unwrap();
                if !re.is_match(email) {
                    errors.email = Some("Invalid email address".to_string());
                }
            }
        }
        if errors == NewContactErrors::default() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[derive(Default, PartialEq, Eq, Serialize)]
pub struct NewContactErrors {
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}
