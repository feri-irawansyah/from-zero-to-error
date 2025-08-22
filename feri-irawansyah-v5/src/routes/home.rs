use gloo_net::http::Request;
use leptos::{prelude::*, task::spawn_local};
use wasm_bindgen::JsCast;
use leptos::web_sys::HtmlImageElement;
use crate::{app::{initTypeit, BACKEND_URL}, components::{card_loading::CardLoading, skill_slider::{SkillMarquee, SkillSignals}}, contexts::{index::format_wib_date, models::{AppState, Notes, NotesData, Skill, SkillsData}}};

#[allow(non_snake_case)]
#[component]
pub fn Home() -> impl IntoView {

    let state: AppState = expect_context::<AppState>();
    let notes: RwSignal<Vec<Notes>> = RwSignal::new(vec![]);
    let (loading, set_loading) = signal(false);
    let (loading_skill, set_loading_skill) = signal(false);
    let current_page: RwSignal<i32> = RwSignal::new(1);
    let limit = 3;
    let signal_skills = SkillSignals::new();

    let fetch_notes = move |page: i32| {
        let offset = (page - 1) * limit;
        let url = format!(
            "{}/data/table?tablename=notes&offset={}&limit={}&nidkey=notes_id&sort=last_update&order=desc",
            BACKEND_URL,
            offset,
            limit
        );

        spawn_local(async move {
            set_loading(true);
            if let Ok(response) = Request::get(&url).send().await {
                if let Ok(data) = response.json::<NotesData>().await {
                    notes.set(data.rows);
                }
            }
            set_loading(false);
        });
    };

    let fetch_skills = move || {
        let url = format!(
            "{}/data/table?tablename=skills&offset=0&limit=50&nidkey=skill_id&sort=skill_id&order=asc",
            BACKEND_URL
        );

        spawn_local(async move {
            set_loading_skill(true);
            if let Ok(response) = Request::get(&url).send().await {

                let data = response.json::<SkillsData>().await;
                
                if let Ok(data) = data {
                    let skills = data.rows.clone();
                    signal_skills.techstack.set(skills.clone().into_iter().filter(|skill| skill.tech_category.to_lowercase().as_str() == "techstack").collect::<Vec<Skill>>());
                    signal_skills.programming.set(skills.clone().into_iter().filter(|skill| skill.tech_category.to_lowercase().as_str() == "programming").collect::<Vec<Skill>>());
                    signal_skills.devops.set(skills.clone().into_iter().filter(|skill| skill.tech_category.to_lowercase().as_str() == "devops").collect::<Vec<Skill>>());
                }
            }
            set_loading_skill(false);
        });
    };

    Effect::new(move |_| {
        fetch_notes(current_page.get());
        fetch_skills();
        || ()
    });

    view! {
        <section id="hero" class="hero section" data-aos="zoom-in">
            <img src="https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/hero-bg.webp" alt="" loading="lazy" />
            <div class="container" data-aos="fade-up">
                <div class="row justify-content-center">
                    <div class="col-lg-10 mb-3 headline" data-aos="zoom-in" data-aos-delay="100">
                        <h2>
                            <span class="me-3 text-primary">Hi, "✨" "I'm"</span>
                            Feri
                        </h2>
                        <p>
                            <span class="text-info">Software Engineer</span>
                            yang doyan ngoprek tentang teknologi. Gue paling suka pake
                            <span class="text-info">Rust</span>
                            bikin AI, Desktop, dan Web.
                        </p>
                        <p>
                            Di waktu senggang, gue suka nulis artikel tentang teknologi, biasanya berkaitan sama hal yang gue ekplor untuk berbagi dan biar gue engga lupa juga.
                            <a class="text-primary text-decoration-none fw-bold" href="/about" rel="noopener noreferrer">
                                Selengkapnya
                                <i class="bi bi-arrow-right"></i>
                            </a>
                        </p>
                        <div class="d-flex justify-content-start gap-3 pt-3 social">
                            <a href=move || {
                                let nomor = "6282323443535";
                                let pesan = "Halo Bro, Gue pingin diskusi lebih lanjut nih";
                                let wa_url = format!(
                                    "https://wa.me/{}?text={}",
                                    nomor,
                                    urlencoding::encode(pesan)
                                );
                                wa_url
                            } rel="noopener noreferrer" class="btn btn-success" target="_blank">
                                <i class="bi bi-whatsapp me-2"></i>
                                <span>Whatsapp</span>
                            </a>
                            <a href=move || {
                                let email = "feryirawansyah09@gmail.com";
                                let subject = "Halo Feri";
                                let body = "Saya ingin kerja sama dengan Anda.";
                                let mailto_url = format!(
                                    "mailto:{}?subject={}&body={}",
                                    email,
                                    urlencoding::encode(subject),
                                    urlencoding::encode(body)
                                );
                                mailto_url
                            } class="btn btn-warning" target="_blank">
                                <i class="bi bi-envelope me-2"></i>
                                <span>Email</span>
                            </a>
                            <a href="https://github.com/feri-irawansyah" class="btn btn-dark" target="_blank">
                                <i class="bi bi-github me-2"></i>
                                <span>Github</span>
                            </a>
                            <a href="https://www.instagram.com/fery_ir.1" class="btn btn-danger" target="_blank">
                                <i class="bi bi-instagram me-2"></i>
                                <span>Instagram</span>
                            </a>
                        </div>
                        <hr />
                    </div>
                    <div class="col-lg-12">
                        <div class="d-flex flex-row justify-content-between" data-aos="fade-in" data-aos-delay="300">
                            <h4 class="fw-bold">My <span class="text-primary">Tech Stack</span></h4>
                        </div>
                        <div class="row mb-3" data-aos="zoom-in" data-aos-delay="200">
                            <Show
                                when=move || !loading_skill.get()
                                fallback=|| {
                                    view! {
                                        <div class="card card-marquee">
                                            <a class="btn btn-primary disabled placeholder fw bold fs-4" aria-disabled="true">
                                                Please wait...
                                            </a>
                                        </div>
                                    }
                                }
                            >
                                {move || {
                                    view! {
                                        <div class="card card-marquee">
                                            <SkillMarquee
                                                skills=signal_skills.programming
                                                position=Some("left".to_string())
                                            />
                                            <SkillMarquee
                                                skills=signal_skills.techstack
                                                position=Some("right".to_string())
                                            />
                                            <SkillMarquee
                                                skills=signal_skills.devops
                                                position=Some("left".to_string())
                                            />
                                        </div>
                                    }
                                }}
                            </Show>
                        </div>
                        <div class="d-flex flex-row justify-content-between" data-aos="fade-in" data-aos-delay="300" >
                            <div class="col-lg-10">
                                <hr />
                            </div>
                        </div>
                        <div class="d-flex flex-row justify-content-between" data-aos="fade-in" data-aos-delay="300" >
                            <h4 class="fw-bold">Latest <span class="text-primary">Notes</span></h4>
                            <a class="btn see-all" href="/catatan">
                                See All
                                <i class="bi bi-arrow-right"></i>
                            </a>
                        </div>
                        <div class="row mb-3 latest-notes" data-aos="fade-in" data-aos-delay="300">
                            <Show
                                when=move || !loading.get()
                                fallback=|| view! { <CardLoading delay=Some(300) count=Some(3) /> }
                            >
                                {move || {
                                    notes.get().iter().enumerate().map(|(i, note)| {
                                        view! {
                                            <div class="col-12 col-lg-4 col-md-6 d-flex align-items-stretch" data-aos="fade-up" data-aos-delay=format!("{}", i * 200) data-aos-duration="1000">
                                                <a class="card text-center" href=format!("/catatan/{}/{}", note.category.clone(), note.slug.clone(),)>
                                                    <img src=format!("https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/notes/{}.webp", note.slug.clone()) alt=note.title.clone()
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
                                                            {note.title.clone()}
                                                        </h5>
                                                        <p class="card-text text-start">
                                                            {note.description.clone()}
                                                        </p>
                                                    </div>
                                                    <div class="card-footer text-body-secondary">
                                                        <div class="d-flex justify-content-between">
                                                            <div class="d-flex gap-1 author">
                                                                <img
                                                                    class="rounded-circle"
                                                                    src="https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/logo-ss.webp"
                                                                    style="width: 1.5rem; height: 1.5rem;"
                                                                    loading="lazy"
                                                                />
                                                                <span>{move || state.name.get()}</span>
                                                            </div>
                                                            <small class="text-white date">
                                                                {format_wib_date(&note.last_update)}
                                                            </small>
                                                            <small class="text-white read">
                                                                Read more <i class="bi bi-arrow-right"></i>
                                                            </small>
                                                        </div>
                                                    </div>
                                                </a>
                                            </div>
                                        }
                                    })
                                    .collect_view()
                                }}
                            </Show>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn Typewriter() -> impl IntoView {

    // Jalankan JS saat mount
    Effect::new(move |_| {
        initTypeit();
    });

    view! {
        <h2>
            <span id="typewriter" class="typewriter"></span>
            <span class="cursor">|</span>
        </h2>
    }
}