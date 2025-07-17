use leptos::prelude::*;

use crate::contexts::models::{ActionResult, EmailRequest};

#[allow(non_snake_case)]
#[component]
pub fn EmailTemplate(name: String, message: String) -> impl IntoView {
    view! {
        <html>
            <body style="font-family: Arial, sans-serif; padding: 20px;">
                <h2 style="color: #333;">Hello, {name}!</h2>
                <p>Youve received a new message:</p>
                <blockquote style="background-color: #f9f9f9; padding: 10px; border-left: 3px solid #ccc;">
                    {message}
                </blockquote>
                <p>Best regards,<br/>Your App Team</p>
            </body>
        </html>
    }
}

#[server(SendEmail, "/api")]
pub async fn send_email(request: EmailRequest) -> Result<ActionResult<String, String>, ServerFnError<serde_json::Value>> {
    use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};

    let mut result = ActionResult::default();

    // Konfigurasi pengirim dan SMTP
    let smtp_server = "smtp-relay.brevo.com";
    let smtp_username = "8cf4d6002@smtp-brevo.com";
    let smtp_password = "m0bfcwQOYXkvr6qp"; // Gunakan app password, bukan password biasa

    // Buat message
    let email = Message::builder()
        .from("techsnakesystem@gmail.com".parse().unwrap())
        .to(request.recipient.parse().unwrap())
        .subject(request.subject)
        .header(lettre::message::header::ContentType::TEXT_HTML) // Bisa TEXT_PLAIN
        .body(request.message)
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    // Buat kredensial dan transport
    let creds = Credentials::new(smtp_username.to_string(), smtp_password.to_string());
    let mailer = SmtpTransport::relay(&smtp_server)
            .unwrap()
            .credentials(creds)
            .build();

    // Kirim email
    mailer.send(&email)
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    Ok(result)
}