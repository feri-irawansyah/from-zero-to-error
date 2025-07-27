use leptos::{leptos_dom::logging::console_log, prelude::*, task::spawn_local};
use leptos_router::{components::Outlet, hooks::use_location, location::Location};

use crate::{contexts::models::{AppState, ModalState}, middleware::session::check_session};

#[allow(non_snake_case)]
#[component]
pub fn MenuList() -> impl IntoView {
    
    let location: Location = use_location();
    let state = expect_context::<AppState>();
    let show = RwSignal::new(false);

    Effect::new(move |_| {
        spawn_local(async move { 
            // state.loading.set(true);
            let response = check_session().await;
            match response {
                Ok(session) => {
                    state.session.set(session);
                }
                Err(error) => {
                    console_log(format!("Error: {:#?}", error).as_str());
                }
            }
            // state.loading.set(false);
        });
    });

    let toggle_show = move |_| {
        show.update(|show| *show = !*show);
    };

    view! {
        <div class=move || format!("menu-list col-2 p-0 scroll-custom {}", if show.get() { "show" } else { "" })>
            <img src="https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/feri.webp"
                alt="feri"
                class="rounded-circle img-fluid about-img mb-1"
                loading="lazy"
            />
            <h5 class="fw-bold mb-0">
                {move || state.name.get()}
                <img class="real-image" src="https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/real.png" alt="feri" loading="lazy" />
            </h5>
            <p class="mt-0">Software Engineer From Indonesia</p>
            <ul class="list-unstyled">
                <li class:active=move || (location.pathname)() == "/">
                    <a href="/" on:click=move |_| show.set(false)>
                        <i class="bi bi-house"></i>
                        <span>Home</span>
                    </a>
                </li>
                <li class:active=move || (location.pathname)() == "/portfolio">
                    <a href="/portfolio" on:click=move |_| show.set(false)>
                        <i class="bi bi-journal-code"></i>
                        <span>Portfolio</span>
                    </a>
                </li>
                <li class:active=move || (location.pathname)().contains("catatan")>
                    <a href="/catatan" on:click=move |_| show.set(false)>
                        <i class="bi bi-journal-text"></i>
                        <span>Catatan</span>
                    </a>
                </li>
                <li class:active=move || (location.pathname)() == "/about">
                    <a href="/about" on:click=move |_| show.set(false)>
                        <i class="bi bi-person"></i>
                        <span>About</span>
                    </a>
                </li>
                <li class:active=move || (location.pathname)() == "/services">
                    <a href="/services" on:click=move |_| show.set(false)>
                        <i class="bi bi-briefcase"></i>
                        <span>Services</span>
                    </a>
                </li>
                <li class:active=move || (location.pathname)() == "/contact">
                    <a href="/contact" on:click=move |_| show.set(false)>
                        <i class="bi bi-envelope"></i>
                        <span>Contact</span>
                    </a>
                </li>
            </ul>
            <div class="copyright">
                @ <strong>
                    <a href="https://github.com/feri-irawansyah">{move || state.name.get()}</a>
                </strong>. <p>All Rights Reserved</p>
                <div class="credits">
                    Powered by <a target="_blank" href="https://leptos.dev">
                        Rust Leptos
                    </a> <i>"❤️"</i>
                </div>
            </div>
        </div>

        <div class="col-10 p-0 content scroll-custom">
            <Show when=move || state.session.get().usernid != 0 fallback=|| view! { <span></span> }>
                <a href="/admin" class="btn btn-outline-primary btn-sm to-dashboard">
                    <i class="bi bi-house-door-fill me-2"></i>
                    <span>Dashboard</span>
                </a>
            </Show>
            <div class="btn-group version">
                <button type="button" class="btn btn-danger btn-sm dropdown-toggle" data-bs-toggle="dropdown" aria-expanded="false">
                    V5.0.0
                </button>
                <ul class="dropdown-menu px-1">
                    <li><a class="dropdown-item bg-danger" href="https://feri-irawansyah.my.id">Latest(5.0.0) <i class="bi bi-check"></i></a></li>
                    <li><hr class="dropdown-divider"/></li>
                    <li><a class="dropdown-item" href="#">V4.0.0</a></li>
                    <li><a class="dropdown-item" href="#">V3.0.0</a></li>
                    <li><a class="dropdown-item" href="#">V2.0.0</a></li>
                    <li><a class="dropdown-item" href="#">V1.0.0</a></li>
                </ul>
            </div>
            <button class="btn btn-primary btn-sm menu-toggle-mobile" type="button" on:click=toggle_show>
                <i class="bi bi-list"></i>
            </button>
            <Outlet />
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn AppSettings() -> impl IntoView {

    let state = expect_context::<ModalState>();

    let toggle = move || {
        state.title.set("Application Settings".to_string());
        state.note_url.set(None);
    };

    view! {
        <button
            class="btn btn-primary btn-sm app-settings"
            type="button"
            data-bs-toggle="offcanvas"
            data-bs-target="#offcanvasScrolling"
            aria-controls="offcanvasScrolling"
        >
            <i class="bi bi-gear"></i>
        </button>

        <div
            class="offcanvas offcanvas-end"
            data-bs-scroll="true"
            data-bs-backdrop="false"
            tabindex="-1"
            id="offcanvasScrolling"
            aria-labelledby="offcanvasScrollingLabel"
        >
            <div class="offcanvas-header">
                <h5 class="offcanvas-title" id="offcanvasScrollingLabel">
                    Application Settings
                </h5>
                <button
                    type="button"
                    class="btn-close"
                    data-bs-dismiss="offcanvas"
                    aria-label="Close"
                ></button>
            </div>
            <div class="offcanvas-body">
                <div class="row">
                    <div class="col-12">
                        <div class="card">
                            <div class="card-header">
                                <h5 class="card-title">Version</h5>
                            </div>
                            <div class="card-body d-flex flex-row gap-3">
                                <div class="dropdown">
                                    <button
                                        class="btn btn-outline-primary btn-sm dropdown-toggle"
                                        type="button"
                                        data-bs-toggle="dropdown"
                                        aria-expanded="false"
                                    >
                                        V 5.0.0
                                    </button>
                                    <ul class="dropdown-menu dropdown-menu-dark">
                                        <li>
                                            <a class="dropdown-item disabled text-center">V5 Release</a>
                                        </li>
                                        <li>
                                            <a class="dropdown-item active">
                                                V 5.0.0 <i class="bi bi-check"></i>
                                            </a>
                                        </li>
                                        <li>
                                            <hr class="dropdown-divider" />
                                        </li>
                                        <li>
                                            <a class="dropdown-item">V 4.0.0</a>
                                        </li>
                                        <li>
                                            <a class="dropdown-item">V 3.0.0</a>
                                        </li>
                                        <li>
                                            <a class="dropdown-item">V 2.0.0</a>
                                        </li>
                                        <li>
                                            <a class="dropdown-item">V 1.0.0</a>
                                        </li>
                                    </ul>
                                </div>
                                <button
                                    type="button"
                                    class="btn btn-info btn-sm"
                                    onclick=toggle
                                    data-bs-toggle="modal"
                                    data-bs-target="#aboutApp"
                                >
                                    About App
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
                <hr />
            </div>
        </div>
    }
}