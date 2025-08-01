use leptos::{leptos_dom::logging::console_log, prelude::*, task::spawn_local};
use leptos_router::{components::Outlet, hooks::{use_location, use_navigate}};

use crate::{components::modal_detail::ModalDetail, contexts::models::{AppState, ModalState}, directives::{markdown::MarkdownFromUrl, modal_container::ModalContainer}, middleware::session::check_session};

#[allow(non_snake_case)]
#[component]
pub fn AdminLayout() -> impl IntoView {
    let state = expect_context::<AppState>(); 
    let modal_state = expect_context::<ModalState>(); 
    let is_open = RwSignal::new(true);
    let location = use_location();

    let segment1 = Memo::new(move |_| {
        location.pathname.get()
            .split('/')
            .filter(|s| !s.is_empty())
            .nth(0)
            .unwrap_or("")
            .to_string()
    });

    let segment2 = Memo::new(move |_| {
        location.pathname.get()
            .split('/')
            .filter(|s| !s.is_empty())
            .nth(1)
            .unwrap_or("")
            .to_string()
    });


    Effect::new(move |_| {
        let navigate = use_navigate();
        spawn_local(async move { 
            state.loading.set(true);
            let response = check_session().await;
            match response {
                Ok(session) => {
                    state.session.set(session);
                }
                Err(error) => {
                    console_log(format!("Error: {:#?}", error).as_str());
                    navigate("/login", Default::default());
                }
            }
            state.loading.set(false);
        });
    });

    view! {
        <Show
            when=move || state.session.get().usernid != 0
            fallback=move || view! { <span></span> }
        >
            <div class="container-fluid admin-layout scroll-custom" data-aos="fade-left">
                <div class="d-flex">
                    <div class=move || {
                        if is_open.get() { "sidebar" } else { "sidebar collapsed" }
                    }>
                        <ul>
                            <li class="logo">
                                <a href="/">
                                    <img
                                        src="https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/logo-ss.webp"
                                        alt="feri"
                                        class="rounded-circle img-fluid about-img mb-1"
                                        loading="lazy"
                                    />
                                    <h5>
                                        Feri Irawansyah
                                        <img
                                            class="real-image"
                                            src="https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/real.png"
                                            alt="feri"
                                            loading="lazy"
                                        />
                                    </h5>
                                    <p>Admin Dashboard Area</p>
                                </a>
                                <hr/>
                            </li>
                            <li class:active=move || (location.pathname)() == "/admin">
                                <a href="/admin">
                                    <i class="bi bi-grid"></i>
                                    <span>Dashboard</span>
                                </a>
                            </li>
                            <li class:active=move || (location.pathname)() == "/admin/user">
                                <a href="/admin/user">
                                    <i class="bi bi-person"></i>
                                    <span>User Management</span>
                                </a>
                            </li>
                            <li class:active=move || (location.pathname)() == "/admin/notes-management">
                                <a href="/admin/notes-management">
                                    <i class="bi bi-journal-code"></i>
                                    <span>Notes Management</span>
                                </a>
                            </li>
                            <li>
                                <a href="/">
                                    <i class="bi bi-box-arrow-left"></i>
                                    <span>Kembali</span>
                                </a>
                            </li>
                        </ul>
                    </div>
                    <div class=move || {
                        if is_open.get() { "main-area" } else { "main-area expanded" }
                    }>
                        <nav class="navbar">
                            <div class="container-fluid">
                                <div class="navbar-brand">
                                    <div class="d-flex align-items-start">
                                        <button
                                            class="menu-toggle"
                                            on:click=move |_| is_open.set(!is_open.get())
                                        >
                                            <i class="bi bi-list"></i>
                                        </button>
                                        <h5 class="fw-bold mb-0">Snakesystem Admin Area</h5>
                                    </div>
                                    <div class="d-flex align-items-start navigation">
                                        <a href=move || {
                                            format!("/{}", segment1.get())
                                        }>{move || format!("/{}", segment1.get())}</a>
                                        <a href=move || {
                                            format!("/{}/{}", segment1.get(), segment2.get())
                                        }>{move || format!("/{}", segment2.get())}</a>
                                    </div>
                                </div>
                                <div class="navbar-nav">
                                    <a class="nav-link">
                                        <i class="bi bi-bell"></i>
                                    </a>
                                    <a class="nav-link">
                                        <i class="bi bi-person"></i>
                                    </a>
                                    <a class="nav-link">
                                        <i class="bi bi-box-arrow-right"></i>
                                    </a>
                                </div>
                            </div>
                        </nav>
                        <div class="content">
                            <Outlet />
                        </div>
                    </div>
                </div>
            </div>
        </Show>
        <ModalContainer
            title=modal_state.title
            size=Some("xl".to_string())
            modal_id="note-content".to_string()
        >
            <Show
                when=move || modal_state.note_url.get().is_some()
                fallback=move || view! { <h1 class="text-center">Loading...</h1> }
            >
                {move || view! { <MarkdownFromUrl url=modal_state.note_url /> }}
            </Show>
        </ModalContainer>
        <ModalDetail />
    }
}