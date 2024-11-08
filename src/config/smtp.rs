use anyhow::Result;
use lettre::{
    transport::smtp::{authentication::Credentials, client::{Tls, TlsParameters}},
    SmtpTransport,
};
use std::env;

#[derive(Debug)]
pub struct SmtpConfig {
    host: String,
    port: u16,
    use_tls: bool,
    username: String,
    password: String,
}

impl SmtpConfig {
    pub fn from_env() -> Result<Self> {
        dotenv::dotenv().ok();

        Ok(Self {
            host: env::var("SMTP_HOST").unwrap_or_else(|_| "smtp.gmail.com".to_string()),
            port: env::var("SMTP_PORT")
                .unwrap_or_else(|_| "587".to_string())
                .parse()
                .unwrap_or(587),
            use_tls: env::var("SMTP_TLS")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            username: env::var("EMAIL_USER").expect("EMAIL_USER must be set"),
            password: env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD must be set"),
        })
    }

    pub fn build_mailer(&self) -> Result<SmtpTransport> {
        let creds = Credentials::new(self.username.clone(), self.password.clone());

        let mut transport_builder = SmtpTransport::relay(&self.host)?;

        transport_builder = transport_builder
            .credentials(creds)
            .port(self.port);

        if self.use_tls {
            let tls_parameters = TlsParameters::new(self.host.clone())?;
            transport_builder = transport_builder.tls(Tls::Opportunistic(tls_parameters));
        }

        Ok(transport_builder.build())
    }
}
