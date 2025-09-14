use leptos::{leptos_dom::logging::console_log, prelude::*};
use leptos_sweetalert::{Swal, SwalIcon, SwalOptions};

use crate::{components::clock::Clock, contexts::models::EmailRequest, middleware::email::send_email};

#[allow(non_snake_case)]
#[component]
pub fn Contact() -> impl IntoView {

    let form = RwSignal::new(EmailRequest::default());

    let on_submit = Action::new(move |_: &()| {
        
        async move {
            
            match send_email(EmailRequest { name: form.get().name, recipient: form.get().recipient, subject: form.get().subject, message: form.get().message }).await {

                Ok(res) => {
                    console_log(format!("Response: {:#?}", res).as_str());
                    Swal::fire(SwalOptions {
                        title: format!("{}", res.message),
                        text: "This is some text".to_owned(),
                        icon: SwalIcon::SUCCESS,
                        confirm_button_text: "LETS GO".to_owned(),
                        show_cancel_button: true,
                        show_deny_button: false,
                        ..SwalOptions::default()
                    });
                },
                Err(err) => {
                    let err_str = format!("{:?}", err);

                    if let Some(json_str) = err_str.strip_prefix("ServerError(\"").and_then(|s| s.strip_suffix("\")")) {
                        console_log(json_str);
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(json_str) {
                            console_log(format!("JSON: {:?}", json["message"].as_str().unwrap_or("Unknown error")).as_str());
                            let message_err = json["message"].as_str().unwrap_or("Unknown error");
                            let error_detail = json["error"].as_str().unwrap_or("No details");

                            console_log(message_err);
                            console_log(error_detail);

                        } else {
                            console_log("Failed to parse JSON");
                        }
                    } else {
                        console_log("Failed to extract JSON string");
                    }
                    Swal::fire(SwalOptions {
                        title: "This is a title",
                        text: "This is some text",
                        icon: SwalIcon::ERROR,
                        confirm_button_text: "LETS GO",
                        show_cancel_button: true,
                        show_deny_button: false,
                        ..SwalOptions::default()
                    });
                }
            }
        }
    });
    
    view! {
        // Metadata
        <leptos_meta::Title text="Contact | Feri Irawansyah"/>
        <leptos_meta::Meta name="description" content="Kamu bisa menghubungin dan mengenal lebih dalam tentang Feri Irawansyah lewat contact, instagram dan linkedin"/>

        <section id="contact" class="contact section" data-aos="fade-right" >
            <div class="container section-title" data-aos="fade-right">
                <h2>Contact Me</h2>
            </div>
            <div class="container contact-info" data-aos="fade-right">
                <p>
                    I am a Software Engineer dedicated to building efficient, scalable, and user-friendly digital solutions. With a strong background in web development, I am used to working with various modern technologies such as JavaScript/TypeScript, Svelte, Rust, and various other frameworks.
                </p>
                <div class="row">
                    <div class="col-12 mb-3 d-flex gap-2 justify-content-start">
                        <a
                            class="btn btn-success"
                            href="https://wa.me/6282323443535"
                            target="_blank"
                        >
                            <i class="bi bi-whatsapp"></i>
                            <span>WhatsApp</span>
                        </a>
                        <a
                            class="btn btn-primary"
                            href="https://wa.me/6282323443535"
                            target="_blank"
                        >
                            <i class="bi bi-linkedin"></i>
                            <span>Linkedin</span>
                        </a>
                        <a class="btn btn-info" href="https://wa.me/6282323443535" target="_blank">
                            <i class="bi bi-twitter"></i>
                            <span>Twitter</span>
                        </a>
                        <a class="btn btn-dark" href="https://wa.me/6282323443535" target="_blank">
                            <i class="bi bi-github"></i>
                            <span>GitHub</span>
                        </a>
                        <a
                            class="btn btn-danger"
                            href="https://wa.me/6282323443535"
                            target="_blank"
                        >
                            <i class="bi bi-instagram"></i>
                            <span>Instagram</span>
                        </a>
                    </div>
                    <div class="col-lg-6 mb-3">
                        <h4>
                            <i class="bi bi-alarm me-2 text-primary"></i>
                            Availability
                        </h4>
                        <div class="row">
                            <div class="col-12 mb-3">
                                <div class="card p-3">
                                    <p class="card-text">
                                        <i class="bi bi-clock"></i>
                                        Jakarta Indonesia
                                    </p>
                                    <Clock />
                                </div>
                            </div>
                            <div class="col-12 mb-3">
                                <div class="card p-3">
                                    <div
                                        class="alert alert-primary d-flex align-items-center"
                                        role="alert"
                                    >
                                        <i class="bi bi-info-circle me-2"></i>
                                        <div>
                                            Response time: I typically respond within 24-48 hours.
                                        </div>
                                    </div>

                                    <div class="d-flex justify-content-between">
                                        <span>Monday - Friday</span>
                                        <div
                                            class="alert alert-primary p-1 d-flex align-items-center"
                                            role="alert"
                                        >
                                            <div>09:00 - 17:00</div>
                                        </div>
                                    </div>
                                    <div class="d-flex justify-content-between">
                                        <span>Saturday</span>
                                        <div
                                            class="alert alert-primary p-1 d-flex align-items-center"
                                            role="alert"
                                        >
                                            <div>By Appointment</div>
                                        </div>
                                    </div>
                                    <div class="d-flex justify-content-between">
                                        <span>Sunday</span>
                                        <div
                                            class="alert alert-primary p-1 d-flex align-items-center"
                                            role="alert"
                                        >
                                            <div>Rest Day</div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="col-lg-6 mb-3">
                        <h4>
                            <i class="bi bi-envelope me-2 text-primary"></i>
                            Contact Me
                        </h4>
                        <form
                            class="card p-3"
                            on:submit=move |e| {
                                e.prevent_default();
                                let fullname = form().name.clone();
                                let email = form().recipient.clone();
                                let subject = form().subject.clone();
                                let message = form().message.clone();
                                if fullname.is_empty() || email.is_empty() || message.is_empty()
                                    || subject.is_empty()
                                {
                                    Swal::fire(SwalOptions {
                                        title: "Error",
                                        text: "Please fill in all fields",
                                        icon: SwalIcon::ERROR,
                                        confirm_button_text: "OK",
                                        show_cancel_button: false,
                                        show_deny_button: false,
                                        ..SwalOptions::default()
                                    });
                                    return;
                                }
                                on_submit.dispatch(());
                            }
                        >
                            <div class="mb-3">
                                <label for="fullname" class="form-label">
                                    "Full Name"
                                </label>
                                <input
                                    type="text"
                                    class="form-control"
                                    id="fullname"
                                    prop:value=move || form().name.clone()
                                    on:input=move |ev| {
                                        form.update(|f| f.name = event_target_value(&ev));
                                    }
                                />
                            </div>

                            <div class="row mb-3">
                                <div class="col-lg-6">
                                    <label for="email" class="form-label">
                                        "Email address"
                                    </label>
                                    <input
                                        type="email"
                                        class="form-control"
                                        id="email"
                                        prop:value=move || form().recipient.clone()
                                        on:input=move |ev| {
                                            form.update(|f| f.recipient = event_target_value(&ev));
                                        }
                                    />
                                </div>

                                <div class="col-lg-6">
                                    <label for="subject" class="form-label">
                                        "Subject"
                                    </label>
                                    <select
                                        class="form-select"
                                        id="subject"
                                        prop:value=move || form().subject.clone()
                                        on:input=move |ev| {
                                            form.update(|f| f.subject = event_target_value(&ev));
                                        }
                                    >
                                        <option value="">"Choose..."</option>
                                        <option value="Feedback">"Feedback"</option>
                                        <option value="Support">"Support"</option>
                                        <option value="Other">"Other"</option>
                                    </select>
                                </div>
                            </div>

                            <div class="mb-3">
                                <label for="message" class="form-label">
                                    "Message"
                                </label>
                                <textarea
                                    class="form-control"
                                    id="message"
                                    rows="3"
                                    prop:value=move || form().message.clone()
                                    on:input=move |ev| {
                                        form.update(|f| f.message = event_target_value(&ev));
                                    }
                                ></textarea>
                            </div>

                            <button type="submit" class="btn btn-primary">
                                "Submit"
                            </button>
                        </form>
                    </div>
                </div>
            </div>
        </section>
    }
}