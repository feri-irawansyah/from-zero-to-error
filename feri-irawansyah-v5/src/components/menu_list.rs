use leptos::{ev::keydown, leptos_dom::logging::console_log, prelude::*, task::spawn_local};
use leptos_router::{components::Outlet, hooks::use_location, location::Location};
use wasm_bindgen::JsCast;

use crate::{app::openModal, contexts::models::{AppState, ModalState}, directives::search_palette::SearchModal, middleware::session::check_session};

#[allow(non_snake_case)]
#[component]
pub fn MenuList() -> impl IntoView {
    
    let location: Location = use_location();
    let state_modal = expect_context::<ModalState>();
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

    let open_modal = move || {
        if let Some(modal) = leptos::web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("searchModal")
        {
            let _ = modal.dyn_ref::<leptos::web_sys::HtmlElement>().unwrap().click();
        }
    };

    window_event_listener(keydown, move |ev: leptos::web_sys::KeyboardEvent| {
        if ev.ctrl_key() && ev.key() == "k" {
            ev.prevent_default();
            open_modal();
        }
    });

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
            <div class="search-button">
                <button class="btn btn-outline-primary btn-sm w-100" data-bs-toggle="modal" data-bs-target="#searchModal" on:click=move |_| show.set(false)>
                    <i class="bi bi-search"></i> Search 
                </button>
            </div>
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
                        Rust Leptos <img src="https://leptos-dev.translate.goog/favicon.ico" alt="leptos logo"/>
                    </a> 
                </div>
            </div>
        </div>
        <SearchModal />

        <div class="col-10 p-0 content scroll-custom">
            <Show when=move || state.session.get().usernid != 0 fallback=|| view! { <span></span> }>
                <a href="/admin" class="btn btn-outline-primary btn-sm to-dashboard">
                    <i class="bi bi-house-door-fill me-2"></i>
                    <span>Dashboard</span>
                </a>
            </Show>
            <button class="btn btn-primary rounded-circle setings" type="button" on:click=move |_| {
                state_modal.title.set("Pengaturan".to_string());
                openModal("about-app".to_string());
            }>
                <i class="bi bi-gear"></i>
            </button>
            <button class="btn btn-primary btn-sm menu-toggle-mobile" type="button" on:click=toggle_show>
                <i class="bi bi-list"></i>
            </button>
            <Outlet />
        </div>
    }
}