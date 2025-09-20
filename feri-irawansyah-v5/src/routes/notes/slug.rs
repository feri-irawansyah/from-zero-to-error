use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::{app::BACKEND_URL, components::card_loading::CardLoading, contexts::{index::format_wib_date, models::{AppState, Note}}, directives::markdown::render_markdown};

#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct Response {
    data: Note,
}

#[server(Slugify, "/api")]
pub async fn get_slug(slug: String) -> Result<Note, ServerFnError> {
    let url = format!("{}/library/get-library/{}", BACKEND_URL, slug);

    // Fetch JSON dari backend
    let resp = reqwest::get(url).await?;
    let mut data: Response = resp.json().await?;

    // Markdown content di-convert jadi HTML di server
    data.data.content = render_markdown(data.data.content).await?;

    Ok(data.data)
}

#[allow(non_snake_case)]
#[component]
pub fn Slug() -> impl IntoView {
    let params = use_params_map();
    let slug = Memo::new(move |_| params.with(|p| p.get("slug").unwrap_or_default()));
    let state = expect_context::<AppState>();

    let value = state.clone();
    let note = Resource::new(
        move || slug.get(),
        move |slug_name| {
            let state = value.clone(); // clone biar bisa dipakai di async block
            async move {
                match get_slug(slug_name).await {
                    Ok(n) => {
                        state.title.set(n.title.clone());
                        state.note_url.set(n.content.clone());
                        Ok(n)
                    }
                    Err(e) => Err(e),
                }
            }
        },
    );

    view! {
        <div class="markdown-content prose max-w-none">
            <Transition fallback=move || view! { <CardLoading delay=Some(0) count=Some(1) /> }>
                {move || {
                    note.get().map(|result| match result {
                        Ok(data) => view! { 
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
                                    <p class="text-muted">{move || format_wib_date(&data.last_update)}</p>
                                </div>
                            </div>
                            <div class="w-100 slug-content" data-aos="fade-up" data-aos-duration="1000">
                                <div class="markdown-body">
                                    // langsung pake inner_html, karena udah HTML di SSR
                                    <div inner_html=data.content.clone()></div>
                                </div>
                            </div>
                        }.into_any(),
                        Err(err) => view! { <div>{move || err.to_string()}</div> }.into_any(),
                    })
                }}
            </Transition>
        </div>
    }
}
