use leptos::prelude::*;
use leptos_sweetalert::*;
use leptos_meta::{provide_meta_context, Stylesheet};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes}, StaticSegment, WildcardSegment
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{
    components:: {
        admin_layout::AdminLayout, catatan_layout::CatatanLayout, loading::LoadingScreen, menu_list::MenuList
    }, contexts::models::{AppState, ModalState}, directives::modal_container::ModalContainer, middleware::session::SessionData, routes::{
        about::About, admin::{dashboard::Dashboard, notes_management::NotesManagement, user_management::UserManagement}, contact::Contact, home::Home, login::Login, notes::{
            category::Category, list_catatan::ListCatatan, slug::Slug
        }, notfound::NotFound, portfolio::{Portfolio, PortfolioDetail}, services::Services
    }
};

// pub const BACKEND_URL: &str = "https://snakesystem-api.shuttle.app/api/v1"; // production shuttle
// pub const BACKEND_URL: &str = "https://snakesystem-api.apn.leapcell.app/api/v1"; // production leapcell
pub const BACKEND_URL: &str = "http://localhost:8080/api/v1"; // local

#[wasm_bindgen(module = "/index.js")]
extern "C" {
    fn initAOS();
    pub fn refreshAOS();
    pub fn openModal(modal_id: String);
    pub fn closeModal(modal_id: String);

    #[wasm_bindgen(js_name = saveSearch)]
    pub fn save_search(item: JsValue);

    #[wasm_bindgen(js_name = getSearch)]
    pub fn get_search() -> JsValue;

    #[wasm_bindgen(js_name = deleteSearch)]
    pub fn delete_search(term: &str, url: &str) -> JsValue;
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
        search: RwSignal::new(Vec::new()),
    };

    let modal_state = ModalState {
        title: RwSignal::new("".to_string()),
        note_url: RwSignal::new(None),
        data: RwSignal::new(serde_json::Value::Null),
    };

    // Register biar bisa dipakai semua komponen
    provide_context(global_state.clone());
    provide_context(modal_state);

    Effect::new(move |_| {
        initAOS(); // ini panggil JS function
        let js_value = get_search();
        if let Ok(search) = serde_wasm_bindgen::from_value(js_value) {
            global_state.search.set(search);
        } else {
            global_state.search.set(Vec::new());
        }
        || ()
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

        // content for this welcome page
        <Router>
            <main data-bs-theme="dark">
                <div class="container-fluid">
                    <LoadingScreen visible=state.loading />
                    <div class="row main-content">
                        <Routes fallback=move || view! { <div id="preloader"></div> }>
                            <ParentRoute path=leptos_router::path!("/") view=MenuList>
                                <Route path=StaticSegment("") view=Home />
                                <Route path=StaticSegment("about") view=About />
                                <Route path=StaticSegment("services") view=Services />
                                <Route path=StaticSegment("portfolio") view=Portfolio />
                                <Route path=leptos_router::path!("portfolio/:id") view=PortfolioDetail />
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
                </div>
                <ModalContainer
                    title=modal_state.title
                    size=Some("md".to_string())
                    modal_id="about-app".to_string()
                    control=false
                    event=Callback::new(move |_| {})
                >
                    <Show
                        when=move || modal_state.title.get() != ""
                        fallback=move || view! { <h1 class="text-center">Loading...</h1> }
                    >
                        {move || {
                            view! {
                                <AboutApp />
                            }
                        }}
                    </Show>
                </ModalContainer>
                <ModalContainer
                    title=state.title
                    size=Some("fullscreen".to_string())
                    modal_id="zoom-note".to_string()
                    control=true
                    event=Callback::new(move |_| {}) 
                >
                    <Show
                        when=move || state.note_url.get() != ""
                        fallback=move || view! { <h1 class="text-center">Loading...</h1> }
                    >
                        {move || view! { 
                            <div class="markdown-content prose max-w-none">
                                <div class="markdown-body">
                                    <div inner_html=state.note_url></div>
                                </div>
                            </div>
                         }}
                    </Show>
                </ModalContainer>
            </main>
        </Router>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn AboutApp() -> impl IntoView {
    let state = expect_context::<AppState>();
    view! {
        <div class="row">
            <div class="col-12 mb-5">
                {state.name.get_untracked()}
                <div class="btn-group version">
                    <button type="button" class="btn btn-danger btn-sm dropdown-toggle" data-bs-toggle="dropdown" aria-expanded="false">
                        V5.0.0
                    </button>
                    <ul class="dropdown-menu px-1">
                        <li><a class="dropdown-item bg-danger" href="https://feri-irawansyah.my.id">Latest(5.0.0) <i class="bi bi-check"></i></a></li>
                        <li><hr class="dropdown-divider"/></li>
                        <li><a class="dropdown-item" href="https://feri-irawansyah.github.io">V4.0.0</a></li>
                        <li><a class="dropdown-item" href="#">V3.0.0</a></li>
                        <li><a class="dropdown-item" href="#">V2.0.0</a></li>
                        <li><a class="dropdown-item" href="#">V1.0.0</a></li>
                    </ul>
                </div>
            </div>
            <div class="col-12 mb-5">
                Theme
                <div class="btn-group theme">
                    <button type="button" class="btn btn-success btn-sm dropdown-toggle">
                        Coming Soon
                    </button>
                </div>
            </div>
            <div class="col-12 mb-5">
                Language
                <div class="btn-group language">
                    <button type="button" class="btn btn-warning btn-sm dropdown-toggle">
                        Coming Soon
                    </button>
                </div>
            </div>
        </div>
    }
}