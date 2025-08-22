use gloo_net::http::Request;
use leptos::{prelude::*, task::spawn_local};
use leptos_router::hooks::use_params_map;
use wasm_bindgen::JsCast;
use leptos::web_sys::HtmlImageElement;
use crate::{app::BACKEND_URL, components::card_loading::CardLoading, contexts::{index::format_wib_date, models::{AppState, Note, NoteData}}, directives::markdown::MarkdownFromUrl};

#[allow(non_snake_case)]
#[component]
pub fn Slug() -> impl IntoView {
    let params = use_params_map();
    let slug = Memo::new(move |_| {
        params.with(|p| p.get("slug").unwrap_or_default())
    });
    let content: RwSignal<Option<String>> = RwSignal::new(None);
    let note: RwSignal<Note> = RwSignal::new(Note::new());
    let state = expect_context::<AppState>();
    let (loading, set_loading) = signal(false);

    Effect::new(move |_| {
        let slug_name = slug.get(); // ✅ reactive
        let url = format!("{}/library/get-library/{}", BACKEND_URL, slug_name);

        spawn_local(async move {
            set_loading(true);
            if let Ok(response) = Request::get(&url).send().await {
                if let Ok(data) = response.json::<NoteData>().await {
                    note.set(data.data);

                    note.with_untracked(|n| {
                        state.title.set(n.title.clone());
                        content.set(Some(n.content.clone()));
                    });

                }
            }
            set_loading(false);
        });
    });

    view! {
        <Show
            when=move || { !loading.get() }
            fallback=|| view! { <CardLoading count=Some(1) delay=Some(0) /> }
        >
            <Show
                when=move || { note.get().content.is_empty() }
                fallback=move || {
                    view! {
                        <div class="author d-flex flex-row align-items-start justify-content-start w-100">
                            <img
                                src="https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/logo-ss.webp"
                                class="mb-3 rounded-circle"
                                width="50px"
                                alt=""
                                loading="lazy"
                            />
                            <div class="flex-column">
                                <a
                                    class="text-decoration-none text-muted"
                                    href="https://github.com/feri-irawansyah"
                                    target="_blank"
                                >
                                    {move || state.name.get().to_string()}
                                    <img
                                        src="https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/real.png"
                                        width="20px"
                                        alt=""
                                        loading="lazy"
                                    />
                                </a>
                                <p class="text-muted">{move || format_wib_date(&note.get().last_update)}</p>
                            </div>
                        </div>
                        <div class="w-100 slug-content" data-aos="fade-up" data-aos-duration="1000">
                            <div class="image-content d-flex justify-content-center">
                                <img
                                    class="img-fluid rounded"
                                    src=format!("https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/notes/{}.webp", note.get().slug)
                                    alt=move || note.get().title
                                    on:error=move |e: leptos::ev::ErrorEvent| {
                                        if let Some(target) = e.target() {
                                            if let Ok(img) = target.dyn_into::<HtmlImageElement>() {
                                                img.set_src("https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/notes/default.webp");
                                            }
                                        }
                                    }
                                    loading="lazy"
                                />
                            </div>
                            <div class="markdown-body">
                                <MarkdownFromUrl url=content />
                            </div>
                        </div>
                    }
                }
            >
                <h1>"Slug not found"</h1>
            </Show>
        </Show>
    }
}
