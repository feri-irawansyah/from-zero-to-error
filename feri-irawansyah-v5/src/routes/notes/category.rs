use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use crate::{app::BACKEND_URL, components::card_loading::CardLoading, contexts::{index::format_wib_date, models::{AppState, NotesData}}};
use wasm_bindgen::JsCast;
use leptos::web_sys::HtmlImageElement;

#[server(GetNotes, "/api")]
pub async fn get_notes(page: i32, limit: i32, filter: serde_json::Value) -> Result<NotesData, ServerFnError> {
    let offset = (page - 1) * limit;
    let url = format!(
        "{}/data/table?tablename=notes&offset={}&limit={}&nidkey=notes_id&filter={}",
        BACKEND_URL, offset, limit, urlencoding::encode(&filter.to_string())
    );

    // Fetch ke backend
    let resp = reqwest::get(url)
        .await?;

    // Parse JSON ke struct NotesData
    let data: NotesData = resp
        .json()
        .await?;

    Ok(data)
}

#[allow(non_snake_case)]
#[component]
pub fn Category() -> impl IntoView {
    let state: AppState = expect_context::<AppState>();
    let params = use_params_map();
    let category = Memo::new(move |_| {
        params.with(|p| p.get("category"))
    });
    let limit = 9;
    let (current_page, set_current_page) = signal(1);

    let filter = Memo::new(move |_| {
        serde_json::json!({
            "category": category.get().unwrap_or_else(|| "".to_string())
        })
    });

    let (total, set_total) = signal(0);

    let notes = Resource::new(
        move || (current_page.get(), filter.get().clone()),
        move |(page, filter)| get_notes(page, limit, filter),
    );

    view! {
        <div class="d-flex category" data-aos="fade-in">
            <Transition fallback=|| view! { <CardLoading delay=Some(0) count=Some(3) /> }>
                <div class="row list-notes">
                    {move || {
                        notes.get().map(|res| match res {
                            Ok(data) => {
                                set_total(data.total); // update total pages

                                data.rows.into_iter().enumerate().map(|(i, note)| {
                                    view! {
                                        <div class="col-12 col-lg-4 col-md-6 d-flex align-items-stretch"
                                            data-aos="fade-up"
                                            data-aos-delay=format!("{}", i * 200)
                                            data-aos-duration="1000">
                                            <a class="card text-center" href=format!("/catatan/{}/{}", note.category.clone(), note.slug.clone())>
                                                <img
                                                    src=format!("https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/notes/{}.webp", note.slug.clone())
                                                    alt=note.title.clone()
                                                    on:error=move |e: leptos::ev::ErrorEvent| {
                                                        if let Some(target) = e.target() {
                                                            if let Ok(img) = target.dyn_into::<HtmlImageElement>() {
                                                                img.set_src("https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/notes/default.webp");
                                                            }
                                                        }
                                                    }
                                                    class="card-img rounded"
                                                    loading="lazy"
                                                />
                                                <div class="card-img-overlay">
                                                    <div class="hashtag">
                                                        {
                                                            let list_hashtag = note
                                                                .hashtag
                                                                .clone()
                                                                .unwrap_or(vec!["".to_string()]);
                                                            list_hashtag
                                                                .iter()
                                                                .map(|hashtag| view! { <span>#{hashtag.clone()}</span> })
                                                                .collect_view()
                                                        }
                                                    </div>
                                                    <h5 class="card-title text-start text-uppercase">
                                                        {move || note.title.clone()}
                                                    </h5>
                                                    <p class="card-text text-start">
                                                        {move || note.description.clone()}
                                                    </p>
                                                </div>
                                                <div class="card-footer text-body-secondary">
                                                    <div class="d-flex justify-content-between">
                                                        <div class="d-flex gap-2">
                                                            <img
                                                                class="rounded-circle"
                                                                src="https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/logo-ss.webp"
                                                                style="width: 1.5rem; height: 1.5rem;"
                                                                loading="lazy"
                                                            />
                                                            <span>{move || state.name.get()}</span>
                                                        </div>
                                                        <small class="text-white date">
                                                            {move ||format_wib_date(&note.last_update)}
                                                        </small>
                                                        <small class="text-white read">
                                                            Read more <i class="bi bi-arrow-right"></i>
                                                        </small>
                                                    </div>
                                                </div>
                                            </a>
                                        </div>
                                    }
                                }).collect_view().into_any()
                            }
                            Err(e) => view! { <p>{format!("Error: {e}")}</p> }.into_any(),
                        })
                    }}
                </div>
                <nav class=move || if total.get_untracked() as i32 <= limit { "d-none" } else { "pagination-container" }>
                    <ul class="pagination justify-content-end">
                        <li class=format!(
                            "page-item {}",
                            if current_page.get_untracked() == 1 { "disabled" } else { "" },
                        )>
                            <button
                                class="page-link"
                                on:click=move |_| set_current_page(current_page.get_untracked() - 1)
                            >
                                <i class="bi bi-caret-left-fill"></i>
                            </button>
                        </li>
                        {
                            let total_pages = (total.get_untracked() as f64 / limit as f64).ceil() as i32;
                            (1..=total_pages)
                                .map(|i| {
                                    view! {
                                        <li class=format!(
                                            "page-item {}",
                                            if current_page.get_untracked() == i { "active" } else { "" },
                                        )>
                                            <button
                                                class="page-link"
                                                on:click=move |_| set_current_page(i)
                                            >
                                                {move || i}
                                            </button>
                                        </li>
                                    }
                                })
                                .collect_view()
                        }
                        <li class=format!(
                            "page-item {}",
                            if current_page.get_untracked() * limit >= total.get_untracked().try_into().unwrap() {
                                "disabled"
                            } else {
                                ""
                            },
                        )>
                            <button
                                class="page-link"
                                on:click=move |_| set_current_page(current_page.get_untracked() + 1)
                            >
                                <i class="bi bi-caret-right-fill"></i>
                            </button>
                        </li>
                    </ul>
                </nav>
            </Transition>
        </div>
    }
}