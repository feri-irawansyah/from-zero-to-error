use leptos::prelude::*;
use leptos_router::{components::Outlet, hooks::use_params_map};
use wasm_bindgen::JsCast;
use leptos::web_sys::HtmlImageElement;
use crate::contexts::models::AppState;

#[allow(non_snake_case)]
#[component]
pub fn CatatanLayout() -> impl IntoView {
    let params = use_params_map();
    let category = Memo::new(move |_| {
        params.with(|p| p.get("category"))
    });
    let slug = Memo::new(move |_| {
        params.with(|p| p.get("slug"))
    });

    let state = expect_context::<AppState>();

    Effect::new(move |_| {
        if slug.get_untracked().is_none() {
            state.title.set("".to_string());
        }
    });

    view! {
        // Metadata
        <leptos_meta::Title text="Catatan | Feri Irawansyah"/>
        <leptos_meta::Meta name="description" content="Semua pengetahuan teknologi yang dibuat oleh Feri Irawansyah"/>

        <section id="catatan" class="catatan section" data-aos="fade-left">

            <Show when=move || { slug.get().is_some() } fallback=|| view! { <div class="d-none"></div> }>
                <div class="image-content d-flex justify-content-center">
                    <img
                        class="img-fluid rounded"
                        src=format!("https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/notes/{}.webp", slug.get_untracked().clone().unwrap())
                        alt=move || state.title.get_untracked().clone()
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
            </Show>

            <div class="container section-title" data-aos="fade-left">
                <h2 class="text-capitalize">Catatan {move || category.get()}</h2>
            </div>

            <div class="container content-wrapper" data-aos="fade-up" data-aos-delay="100">
                <Outlet />
            </div>
        </section>
    }
}
