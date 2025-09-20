use leptos::prelude::*;

use crate::contexts::models::{ActionResult, EmailRequest};

#[allow(non_snake_case)]
#[component]
pub fn EmailTemplate(username: String, message: String) -> impl IntoView {
    view! {
        <html>
            <head>
                <meta charset="UTF-8" />
                <style>
                    "
                    body {
                        font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
                        background-color: #f4f4f4;
                        padding: 20px;
                        color: #333;
                    }
                    .container {
                        background-color: #fff;
                        padding: 20px;
                        max-width: 600px;
                        margin: auto;
                        border-radius: 8px;
                        box-shadow: 0 2px 8px rgba(0,0,0,0.1);
                    }
                    h1 {
                        color: #007bff;
                        font-size: 24px;
                    }
                    p {
                        font-size: 16px;
                        line-height: 1.5;
                    }
                    "
                </style>
            </head>
            <body>
                <div class="container">
                    <h1>"Hello, " {username.clone()} "!"</h1>
                    <p inner_html=message></p>
                    <hr />
                    <p>Salam,<br />Tech Snake System</p>
                </div>
            </body>
        </html>
    }
}

#[server(SendEmail, "/api")]
pub async fn send_email(request: EmailRequest) -> Result<ActionResult<String, String>, ServerFnError<serde_json::Value>> {
    use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};

    let mut result = ActionResult::default();

    // Konfigurasi pengirim dan SMTP
    let smtp_server = std::env::var("SMTP_SERVER").unwrap();
    let smtp_username = std::env::var("SMTP_USERNAME").unwrap();
    let smtp_password = std::env::var("SMTP_PASSWORD").unwrap(); // Gunakan app password, bukan password biasa

    // Buat message
    let email = Message::builder()
        .from("techsnakesystem@gmail.com".parse().unwrap())
        .to(request.recipient.parse().unwrap())
        .subject(request.subject)
        .header(lettre::message::header::ContentType::TEXT_HTML) // Bisa TEXT_PLAIN
        .body(request.message)
        .map_err(|e| {
            result.error = Some(e.to_string());
            ServerFnError::ServerError(serde_json::to_string(&result).unwrap())
        })?;

    // Buat kredensial dan transport
    let creds = Credentials::new(smtp_username.to_string(), smtp_password.to_string());
    let mailer = SmtpTransport::relay(&smtp_server)
            .unwrap()
            .credentials(creds)
            .build();

    if let Err(e) = mailer.send(&email) {
        result.message = "Failed to send email".to_string();
        result.error = Some(e.to_string());

        return Err(ServerFnError::ServerError(serde_json::to_string(&result).unwrap()));
    }

    result.message = "Email sent successfully".to_string();

    Ok(result)
}