use std::sync::Arc;

use bytes::Bytes;
use tracing::{debug, error, info, warn};

use crate::{
    app_error::{AppError, AppResult},
    clock::Clock,
    entities::{email::EmailAttachment, member::Member},
    use_cases::{
        email::{EmailContent, EmailUseCases},
        member::MemberPersistence,
    },
};

pub const BIRTHDAY_SUBJECT: &str = "¡Feliz Cumpleaños! 🎂";

/// Sends the daily birthday greeting to members.
pub struct BirthdayUseCases {
    members: Arc<dyn MemberPersistence>,
    emails: Arc<EmailUseCases>,
    clock: Clock,
    logo: Bytes,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct BirthdayRun {
    pub sent: usize,
    pub skipped: usize,
    pub failed: usize,
}

impl BirthdayUseCases {
    pub fn new(
        members: Arc<dyn MemberPersistence>,
        emails: Arc<EmailUseCases>,
        clock: Clock,
        logo: Bytes,
    ) -> Self {
        Self {
            members,
            emails,
            clock,
            logo,
        }
    }

    pub async fn send_birthday_emails(&self) -> AppResult<BirthdayRun> {
        let (day, month) = self.clock.today_day_month();
        let members = self.members.members_with_birthday(day, month).await?;
        info!(count = members.len(), "birthday job started");

        let mut run = BirthdayRun::default();
        for member in &members {
            if member.email.trim().is_empty()
                || self
                    .emails
                    .was_sent_today(&member.email, BIRTHDAY_SUBJECT)
                    .await?
                || member.emails_disabled
            {
                debug!(member_id = %member.id, "skipping birthday email");
                run.skipped += 1;
                continue;
            }
            match self
                .emails
                .send_to_member(member.id, self.content(member))
                .await
            {
                Ok(_) => run.sent += 1,
                Err(AppError::TooManyRequests(_)) => {
                    warn!(
                        missed = members.len() - run.sent - run.skipped,
                        "daily email limit reached during birthday job"
                    );
                    break;
                }
                Err(e) => {
                    error!(member_id = %member.id, error = %e, "birthday email failed");
                    run.failed += 1;
                }
            }
        }
        info!(?run, "birthday job finished");
        Ok(run)
    }

    fn content(&self, member: &Member) -> EmailContent {
        let html = birthday_html(&member.name);
        EmailContent {
            subject: BIRTHDAY_SUBJECT.to_owned(),
            preview_body: html.clone(),
            html_body: html,
            attachments: vec![EmailAttachment {
                filename: "logo.png".into(),
                content_type: Some("image/png".into()),
                content: self.logo.clone(),
                content_id: Some("logo".into()),
            }],
        }
    }
}

fn escape_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            c => out.push(c),
        }
    }
    out
}

fn birthday_html(name: &str) -> String {
    let name = escape_html(name);
    format!(
        r#"<!DOCTYPE html>
        <html>
        <body style="font-family: Arial, sans-serif; background-color: #fff7e6; color: #333;">
            <div style="max-width: 600px; margin: auto; background: #fff; border-radius: 8px;
                        padding: 30px; box-shadow: 0 2px 6px rgba(0,0,0,0.1); text-align: center;">
            <img src="cid:logo" alt="Hotel Casa Peix (Logo)" style="max-width: 200px; margin-bottom: 20px;"/>
            <h1 style="color: #d35400;">¡Feliz Cumpleaños, {name}!</h1>
            <p style="font-size: 16px; line-height: 1.6;">
                Esperamos que este día esté lleno de alegría, momentos especiales y felicidad.
            </p>
            <p style="font-size: 16px; line-height: 1.6;">
                Como muestra de nuestro cariño, queremos recordarle que siempre será
                bienvenido(a) en nuestro hotel.
            </p>
            <p style="font-size: 14px; color: #5b5b5b; margin-top: 30px;">
                Con nuestros mejores deseos,<br/>El equipo del Hotel Casa Peix.
            </p>
            <p style="font-size: 12px; color: #777; text-align: center;">
                (Puede darse de baja en cualquier momento escribiendo un correo a
                infohotelcasapeix@gmail.com)
            </p>
            </div>
        </body>
        </html>
        "#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn member_name_is_escaped() {
        let html = birthday_html("<script>x</script>");
        assert!(html.contains("&lt;script&gt;"));
        assert!(!html.contains("<script>"));
    }
}
