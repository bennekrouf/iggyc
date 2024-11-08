use anyhow::Result;
use lettre::{Message, Transport, message::header::ContentType};
use tracing::info;
use crate::config::{smtp::SmtpConfig, email::EmailConfig};

pub struct EmailSender {
    config: EmailConfig,
    mailer: lettre::SmtpTransport,
}

impl EmailSender {
    pub fn new(smtp_config: &SmtpConfig, email_config: EmailConfig) -> Result<Self> {
        Ok(Self {
            config: email_config,
            mailer: smtp_config.build_mailer()?,
        })
    }

    pub fn send_to(&self, to_email: &str, subject: &str, body: String) -> Result<()> {
        let email = Message::builder()
            .from(self.config.from_email.parse()?)
            .to(to_email.parse()?)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body)?;

        match self.mailer.send(&email) {
            Ok(_) => {
                info!("Email notification sent successfully to {}", to_email);
                Ok(())
            }
            Err(e) => Err(anyhow::anyhow!("Failed to send email: {}", e)),
        }
    }
}
