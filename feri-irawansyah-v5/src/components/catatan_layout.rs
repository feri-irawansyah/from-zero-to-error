use leptos::prelude::*;
use leptos_router::{components::Outlet, hooks::use_params_map};
use wasm_bindgen::JsCast;
use leptos::web_sys::HtmlImageElement;
use crate::{contexts::models::{AppState, ModalState}, directives::page_loader::page_loader};

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
    let modal_state = expect_context::<ModalState>();

    Effect::new(move |_| {
        if slug.get().is_none() {
            state.title.set("".to_string());
        }
    });

    page_loader(
        view! {
            // Metadata
            <leptos_meta::Title text="Catatan | Feri Irawansyah"/>
            <leptos_meta::Meta name="description" content="Semua pengetahuan teknologi yang dibuat oleh Feri Irawansyah"/>

            <section id="catatan" class="catatan section" data-aos="fade-left">

                <Show when=move || { slug.get().is_some() } fallback=|| view! { <div class="d-none"></div> }>
                    <div class="image-content d-flex justify-content-center">
                        <img
                            class="img-fluid rounded"
                            src=format!("https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/notes/{}.webp", slug.get().clone().unwrap())
                            alt=move || state.title.get().clone()
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
                    <Show when=move || { category.get().is_some() } fallback=|| view! { <span></span> }>
                        <Show
                            when=move || { !slug.get().is_some() }
                            fallback=move || {
                                let cat = category.get().clone().unwrap_or("".to_string());
                                view! {
                                    <div class="d-flex justify-content-between">
                                        <a class="btn text-start back" href=format!("/catatan/{}", cat)>
                                            <i class="bi bi-arrow-left-circle me-2"></i>
                                            Kembali
                                        </a>
                                        <a class="btn btn-zoom" data-bs-toggle="modal" on:click=move |_| {
                                                modal_state.note_url.set(Some(state.note_url.get().clone()));
                                                modal_state.title.set(state.title.get().clone());
                                            } data-bs-target="#zoom-note"><i class="bi bi-arrows-fullscreen"></i>
                                        </a>
                                    </div>
                                }
                            }
                        >
                            <a class="btn text-start back" href="/catatan">
                                <i class="bi bi-arrow-left-circle me-2"></i>
                                Kembali
                            </a>
                        </Show>
                    </Show>
                </div>

                <div class="container content-wrapper" data-aos="fade-up" data-aos-delay="100">
                    <Show
                        when=move || !state.title.get().is_empty()
                        fallback=|| {
                            view! {
                                <p>
                                    "Catatan gue tutorial, wawasan teknologi, opini gajelas, kadang membingungkan, dan ide - ide tentang teknologi yang disusun untuk memicu ide dan terkadang memecahkan masalah kadang juga engga."
                                </p>
                            }
                        }
                    >
                        <h5 class="text-uppercase fw-bold">{move || state.title.get()}</h5>
                        <hr />
                    </Show>

                    <Outlet />

                </div>
            </section>
        }
    )
}
