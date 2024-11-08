use std::env;

#[derive(Debug)]
pub struct EmailConfig {
    pub from_email: String,
}

impl EmailConfig {
    pub fn from_env() -> Self {
        Self {
            from_email: env::var("EMAIL_FROM")
                .unwrap_or_else(|_| env::var("EMAIL_USER").unwrap_or_default()),
        }
    }
}
