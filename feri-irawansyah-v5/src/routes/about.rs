use gloo_net::http::Request;
use leptos::{leptos_dom::logging::console_log, prelude::*, task::spawn_local};

use crate::{app::BACKEND_URL, components::about_tab::AboutTab, contexts::{index::hitung_usia, models::{Skill, SkillsData}}};

#[allow(non_snake_case)]
#[component]
pub fn About() -> impl IntoView {

    let menu_item = RwSignal::new("Intro");
    
    view! {
        <section id="about" class="about section" data-aos="fade-right">

            <div class="container section-title" data-aos="slide-right" data-aos-delay="100">
                <h2>About Me</h2>
            </div>
            <div class="container" data-aos="slide-right" data-aos-delay="200">

                <div class="row justify-content-start">
                    <AboutTab menu_item=menu_item />
                </div>
                <div class="row justify-content-start">
                    <div class="col-12 content">
                        {move || {
                            match menu_item.get() {
                                "Intro" => view! { <Intro /> }.into_any(),
                                "Experience" => view! { <Experience /> }.into_any(),
                                "Skills" => view! { <Skills /> }.into_any(),
                                "Journey" => {
                                    view! {
                                        <h2>
                                            <Journey />
                                        </h2>
                                    }
                                        .into_any()
                                }
                                "Certifications" => view! { <h2>Certifications</h2> }.into_any(),
                                _ => view! { <h2>Intro</h2> }.into_any(),
                            }
                        }}
                    </div>
                </div>
            </div>
        </section>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn Intro() -> impl IntoView {

    let usia = hitung_usia("2000-06-09").unwrap_or(0);

    view! {
        <div data-aos="slide-right">
            <h2>Software Engineer & Developer.</h2>
            <p>
                "Hi, I'm Feri, commonly known as snakesystem —a Rust & C# Programmer, AI Engineer and Software Engineer. Based in West Java, Indonesia. I create Web-based applications, RESTfull API, Desktop using Rust and C# focusing on performance, clean code and scalable."
            </p>
            <p>
                "Personally, I am a child of odd-job laborers. I have worked since I was in elementary school to pay for school, helped by my parents' endless prayers. This has shaped me to have determination, focus, and discipline, the same energy that I pour into every line of code."
            </p>
            <p>
                "What's next? I am fully involved in the world of technology to overcome challenges and needs in the world using technology by developing Tech Snake System as an application development organization."
            </p>
            <p>
                "Have a crazy idea or just curious about technology? Contact me, let's build something great together!"
            </p>
            <hr />
            <div class="row mt-3">
                <div class="col-lg-6">
                    <ul>
                        <li>
                            <i class="bi bi-chevron-right"></i>
                            <strong>Birth Date:</strong>
                            <span>09 Juni 2000</span>
                        </li>
                        <li>
                            <i class="bi bi-chevron-right"></i>
                            <strong>Website:</strong>
                            <span>www.feri-irawansyah.github.io</span>
                        </li>
                        <li>
                            <i class="bi bi-chevron-right"></i>
                            <strong>Phone:</strong>
                            <span>+62 82323443535</span>
                        </li>
                        <li>
                            <i class="bi bi-chevron-right"></i>
                            <strong>City:</strong>
                            <span>DKI Jakarta, Indonesia</span>
                        </li>
                    </ul>
                </div>
                <div class="col-lg-6">
                    <ul>
                        <li>
                            <i class="bi bi-chevron-right"></i>
                            <strong>Age:</strong>
                            <span>{usia}</span>
                        </li>
                        <li>
                            <i class="bi bi-chevron-right"></i>
                            <strong>Degree:</strong>
                            <span>Sarjana Teknik Informatika</span>
                        </li>
                        <li>
                            <i class="bi bi-chevron-right"></i>
                            <strong>Email:</strong>
                            <span>feryirawansyah09@gmail.com</span>
                        </li>
                        <li>
                            <i class="bi bi-chevron-right"></i>
                            <strong>Freelance:</strong>
                            <span>Available</span>
                        </li>
                    </ul>
                </div>
            </div>
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn Experience() -> impl IntoView {

    let experience = hitung_usia("2021-02-04").unwrap_or(0);

    view! {
        <div data-aos="slide-right" class="experience">
            <div class="row">
                <div class="col-12">
                    <div class="card">
                        <div class="d-flex justify-content-between">
                            <div class="flex-column experience-left">
                                <div class="d-flex align-items-center">
                                    <i class="bi bi-briefcase"></i>
                                    <div class="flex-column">
                                        <h5>Curriculum Vitae</h5>
                                        <p>"Access my updated curriculum vitae"</p>
                                    </div>
                                </div>
                                <div
                                    class="alert alert-success d-flex align-items-center"
                                    role="alert"
                                >
                                    <i class="bi bi-bookmark-check"></i>
                                    <div>{format!("{}+ Years", experience)}Experience</div>
                                </div>
                            </div>
                            <div class="flex-column experience-right justify-content-end me-3">
                                <button class="btn" type="button">
                                    <i class="bi bi-eye me-2"></i>
                                    <span>Preview</span>
                                </button>
                                <button class="btn" type="button">
                                    <i class="bi bi-download me-2"></i>
                                    <span>Download</span>
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn Journey() -> impl IntoView {

    view! {
        <div data-aos="slide-right" class="journey">
            <div class="row">
                <div class="col-12">
                    <div class="card">
                        <div class="d-flex justify-content-between">
                            <div class="flex-column experience-left">
                                <div class="d-flex align-items-center">
                                    <i class="bi bi-briefcase"></i>
                                    <div class="flex-column">
                                        <h5>Curriculum Vitae</h5>
                                        <p>"Access my updated curriculum vitae"</p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn Skills() -> impl IntoView {
    
    let skills: RwSignal<Vec<Skill>> = RwSignal::new(vec![]);
    let (loading, set_loading) = signal(false);
    let (total, set_total) = signal(0);
    let (current_page, set_current_page) = signal(1);
    let limit = 20;

    let fetch_notes = move |page: i32| {
        let offset = (page - 1) * limit;
        let url = format!(
            "{}/data/table?tablename=skills&offset={}&limit={}&nidkey=skill_id&sort=skill_id&order=asc",
            BACKEND_URL,
            offset,
            limit
        );

        spawn_local(async move {
            set_loading(true);
            if let Ok(response) = Request::get(&url).send().await {
                if response.status() == 200 {
                    if let Ok(data) = response.json::<SkillsData>().await {
                        skills.set(data.rows);
                        set_total(data.total);
                    }
                } else {
                    console_log(format!("Error: {}", response.status()).as_str());
                }
            }
            set_loading(false);
        });
    };

    Effect::new(move |_| {
        fetch_notes(current_page.get());
    });

    view! {
        <div data-aos="slide-right" class="skills">
            <div class="row">
                <Show
                    when=move || !loading.get()
                    fallback=move || {
                        view! {
                            <div class="col-12" data-aos="fade-bottom">
                                <div class="card border-0 w-100 h-100 bg-primary">
                                    <div class="card-body">
                                        <div class="d-flex justify-content-center">
                                            <div class="spinner-border" role="status">
                                                <span class="visually-hidden">Loading...</span>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        }
                    }
                >
                    <Show
                        when=move || !skills.get().is_empty()
                        fallback=move || {
                            view! {
                                <div class="col-12">
                                    <div class="card">
                                        <div class="card-body">
                                            <h5 class="card-title">No Data</h5>
                                        </div>
                                    </div>
                                </div>
                            }
                        }
                    >
                        {move || {
                            let list_skill = skills.get().clone();
                            list_skill
                                .iter()
                                .enumerate()
                                .map(|(i, skill)| {
                                    let max = 5;
                                    let rating = skill.star;
                                    view! {
                                        <div
                                            class="col-4 col-lg-3 col-md-2 list-skill"
                                            data-aos="fade-up"
                                            data-aos-delay=format!("{}", i * 100)
                                        >
                                            <div class="card">
                                                <div class="card-header d-flex justify-content-between">
                                                    <h5 class="card-title">{skill.title.clone()}</h5>
                                                    <div class="ms-auto d-flex justify-content-between">
                                                        {move || {
                                                            (0..max)
                                                                .map(|i| {
                                                                    view! {
                                                                        <i class=format!(
                                                                            "bi bi-star-fill{}",
                                                                            if i < rating { " text-warning" } else { "" },
                                                                        )></i>
                                                                    }
                                                                })
                                                                .collect_view()
                                                        }}
                                                    </div>
                                                </div>
                                                <div class="card-body">
                                                    <div class="row">
                                                        <div class="col-md-4">
                                                            <img
                                                                class="img-fluid"
                                                                src=format!("{}", skill.image_src.clone())
                                                                alt=skill.title.clone()
                                                                loading="lazy"
                                                            />
                                                        </div>
                                                        <div class="col-md-8">
                                                            <p class="card-text">{skill.description.clone()}</p>
                                                            <div
                                                                class="progress"
                                                                role="progressbar"
                                                                aria-label="Example with label"
                                                                aria-valuenow="25"
                                                                aria-valuemin="0"
                                                                aria-valuemax="100"
                                                            >
                                                                <div
                                                                    class="progress-bar"
                                                                    style=format!("width: {}%", skill.progress)
                                                                >
                                                                    {skill.progress}
                                                                    %
                                                                </div>
                                                            </div>
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    }
                                })
                                .collect_view()
                        }}
                    </Show>
                    <nav class="pagination-container">
                        <ul class="pagination justify-content-end">
                            <li class=format!(
                                "page-item {}",
                                if current_page.get() == 1 { "disabled" } else { "" },
                            )>
                                <button
                                    class="page-link"
                                    on:click=move |_| set_current_page(current_page.get() - 1)
                                >
                                    <i class="bi bi-caret-left-fill"></i>
                                </button>
                            </li>
                            {
                                let total_pages = (total.get() as f64 / limit as f64).ceil() as i32;
                                (1..=total_pages)
                                    .map(|i| {
                                        view! {
                                            <li class=format!(
                                                "page-item {}",
                                                if current_page.get() == i { "active" } else { "" },
                                            )>
                                                <button
                                                    class="page-link"
                                                    on:click=move |_| set_current_page(i)
                                                >
                                                    {i}
                                                </button>
                                            </li>
                                        }
                                    })
                                    .collect_view()
                            }
                            <li class=format!(
                                "page-item {}",
                                if current_page.get() * limit >= total.get().try_into().unwrap() {
                                    "disabled"
                                } else {
                                    ""
                                },
                            )>
                                <button
                                    class="page-link"
                                    on:click=move |_| set_current_page(current_page.get() + 1)
                                >
                                    <i class="bi bi-caret-right-fill"></i>
                                </button>
                            </li>
                        </ul>
                    </nav>
                </Show>
            </div>
        </div>
    }
}