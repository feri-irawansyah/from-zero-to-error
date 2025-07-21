use leptos::prelude::*;
use leptos_sweetalert::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes}, StaticSegment, WildcardSegment
};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    components:: {
        admin_layout::AdminLayout, catatan_layout::CatatanLayout, loading::LoadingScreen, menu_list::MenuList
    }, contexts::models::{AppState, ModalState}, directives::modal_container::ModalContainer, middleware::session::SessionData, routes::{
        about::About, admin::{dashboard::Dashboard, notes_management::NotesManagement, user_management::UserManagement}, contact::Contact, home::Home, login::Login, notes::{
            category::Category, list_catatan::ListCatatan, slug::Slug
        }, notfound::NotFound, portfolio::Portfolio, services::Services
    }
};

pub const BACKEND_URL: &str = "https://snakesystem-api.shuttle.app/api/v1";

#[wasm_bindgen(inline_js = "
    export function initAOS() {
        AOS.init({
            disable: false,
            startEvent: 'DOMContentLoaded', 
            initClassName: 'aos-init',
            animatedClassName: 'aos-animate',
            useClassNames: false,
            disableMutationObserver: false, 
            debounceDelay: 50,
            throttleDelay: 99, 
            offset: -9999, 
            delay: 0, 
            duration: 600, 
            easing: 'ease',
            once: false, 
            mirror: false, 
            anchorPlacement: 'top-center',
        });
    }

    export function initTypeit() {
        window.startTypeIt = function () {
        new TypeIt(`#typewriter`, {
            strings: ['Programmer', 'Software Engineer', 'Freelancer'],
            speed: 100,
            breakLines: false,
            loop: true,
            deleteSpeed: 50,
            nextStringDelay: 1000,
            waitUntilVisible: true,
        }).go();
        };

    }

    export function openModal(modal_id) {
        const modal = new bootstrap.Modal(document.getElementById(modal_id));
        modal.show();

    }
")]
extern "C" {
    fn initAOS();
    pub fn refreshAOS();
    pub fn initTypeit();
    pub fn openModal(modal_id: String);
}

#[allow(non_snake_case)]
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    Swal::init_key_handlers();
    provide_meta_context();
    let global_state = AppState {
        count: RwSignal::new(0),
        name: RwSignal::new("Feri Irawansyah".to_string()),
        title: RwSignal::new("".to_string()),
        loading: RwSignal::new(false),
        session: RwSignal::new(SessionData::new()),
        note_url: RwSignal::new("".to_string()),
    };

    let modal_state = ModalState {
        title: RwSignal::new("".to_string()),
        note_url: RwSignal::new(None),
        data: RwSignal::new(serde_json::Value::Null),
    };

    // Register biar bisa dipakai semua komponen
    provide_context(global_state);
    provide_context(modal_state);

    Effect::new(move |_| {
        initAOS(); // ini panggil JS function
    });

    let state = expect_context::<AppState>();
    let modal_state = expect_context::<ModalState>();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/feri-irawansyah.css" />
        <Stylesheet
            id="aos"
            href="https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/css/aos.min.css"
        />
        <Stylesheet
            id="icons"
            href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.13.1/font/bootstrap-icons.min.css"
        />

        // sets the document title
        <Title text="Feri Irawansyah" />

        // content for this welcome page
        <Router>
            <main data-bs-theme="dark">
                <div class="container-fluid">
                    <LoadingScreen visible=state.loading />
                    <div class="row main-content">
                        <Routes fallback=move || "Not found.">
                            <ParentRoute path=leptos_router::path!("/") view=MenuList>
                                <Route path=StaticSegment("") view=Home />
                                <Route path=StaticSegment("about") view=About />
                                <Route path=StaticSegment("services") view=Services />
                                <Route path=StaticSegment("portfolio") view=Portfolio />
                                <ParentRoute
                                    path=leptos_router::path!("/catatan")
                                    view=CatatanLayout
                                >
                                    <Route path=leptos_router::path!("") view=ListCatatan />
                                    <Route path=leptos_router::path!(":category") view=Category />
                                    <Route path=leptos_router::path!(":category/:slug") view=Slug />
                                </ParentRoute>
                                <Route path=StaticSegment("contact") view=Contact />
                            </ParentRoute>
                            <ParentRoute path=leptos_router::path!("/admin") view=AdminLayout>
                                <Route path=leptos_router::path!("") view=Dashboard />
                                <Route path=leptos_router::path!("user") view=UserManagement />
                                <Route
                                    path=leptos_router::path!("notes-management")
                                    view=NotesManagement
                                />
                            </ParentRoute>
                            <Route path=StaticSegment("login") view=Login />
                            <Route path=WildcardSegment("any") view=NotFound />
                        </Routes>
                    </div>
                    <ModalContainer
                        title=modal_state.title
                        size=Some("lg".to_string())
                        modal_id="aboutApp".to_string()
                    >
                        <Show
                            when=move || modal_state.title.get() != ""
                            fallback=move || view! { <h1 class="text-center">Loading...</h1> }
                        >
                            {move || {
                                view! {
                                    <div class="row">
                                        <div class="col-12">{state.name.get()}</div>
                                    </div>
                                }
                            }}
                        </Show>
                    </ModalContainer>
                </div>
            </main>
        </Router>
    }
}
