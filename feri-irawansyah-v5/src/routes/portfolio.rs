use leptos::prelude::*;

use crate::{app::BACKEND_URL, components::card_loading::CardLoading, contexts::models::{PortfolioData, Skill, SkillsData}};

#[server(GetPortfolio, "/api")]
pub async fn get_portfolio(page: i32, limit: i32) -> Result<PortfolioData, ServerFnError> {
    let offset = (page - 1) * limit;
    let url = format!(
        "{}/data/table?tablename=portfolio&offset={}&limit={}&nidkey=portfolio_id&sort=pined&order=desc",
        BACKEND_URL, offset, limit
    );

    // Fetch ke backend
    let resp = reqwest::get(url)
        .await?;

    // Parse JSON ke struct PortfolioData
    let data: PortfolioData = resp
        .json()
        .await?;

    Ok(data)
}

#[server(GetSkills, "/api")]
pub async fn get_skills() -> Result<SkillsData, ServerFnError> {
    let url = format!(
        "{}/data/table?tablename=skills&offset=0&limit=100&nidkey=skill_id&sort=skill_id&order=asc",
        BACKEND_URL
    );

    // Fetch ke backend
    let resp = reqwest::get(url)
        .await?;

    // Parse JSON ke struct SkillsData
    let data: SkillsData = resp 
        .json()
        .await?;

    Ok(data)
}

#[allow(non_snake_case)]
#[component]
pub fn Portfolio() -> impl IntoView {
    // pagination signals
    let (current_page, set_current_page) = signal(1);
    let limit = 9;

    // ambil portfolio sesuai page
    let portfolio = Resource::new(
        move || current_page.get(),
        move |page| get_portfolio(page, limit),
    );

    // ambil skills sekali saja
    let skills = Resource::new(|| (), move |_| get_skills());

    // total pages (nanti di-update dari hasil portfolio)
    let (total, set_total) = signal(0);

    // helper buat cari skill
    fn find_skill_by_id(skills: &[Skill], id: i32) -> Option<Skill> {
        skills.iter().find(|s| s.skill_id == id).cloned()
    }

    view! {
        // Metadata
        <leptos_meta::Title text="Portfolio | Feri Irawansyah"/>
        <leptos_meta::Meta name="description" content="Kumpulan portfolio dan karya-karya dari Feri Irawansyah"/>

        <section id="portfolio" class="portfolio section" data-aos="fade-in">
            <div class="container section-title" data-aos="fade-up">
                <h2>Portfolio</h2>
            </div>

            <div class="container portfolio-container" data-aos="fade-up">
                <div class="row justify-content-start">
                    <p>
                        I am a Software Engineer dedicated to building efficient, scalable, and user-friendly digital solutions. With a strong background in web development, I am used to working with various modern technologies such as JavaScript/TypeScript, Svelte, Rust, and various other frameworks.
                    </p>

                    // Portfolio list
                    <Transition fallback=|| view! { <CardLoading delay=Some(0) count=Some(2) /> }>
                        {move || {
                            portfolio.get().map(|res| match res {
                                Ok(data) => {
                                    set_total(data.total); // update total pages

                                    data.rows.into_iter().enumerate().map(|(i, p)| {
                                        let tech = p.tech.clone();
                                        view! {
                                            <div class="col-lg-6 card-container"
                                                data-aos="fade-up"
                                                data-aos-delay=format!("{}", 100 + (i * 100))
                                                data-aos-duration="500"
                                            >
                                                <a class="card text-bg-dark" href=p.url_docs.clone() target="_blank">
                                                    <img src=p.image_src.clone()
                                                        class="card-img"
                                                        alt=p.title.clone()
                                                        loading="lazy"
                                                    />
                                                    <div class="card-img-overlay">
                                                        <h4 class="card-title view-project">
                                                            <span>"View Project"</span>
                                                            <i class="bi bi-eye"></i>
                                                        </h4>
                                                        <h5 class="card-title">{p.title.clone()}</h5>
                                                        <p class="card-text">{p.description.clone()}</p>
                                                        <div class=format!("pined {}", if p.pined { "" } else { "d-none" })>
                                                            <span>"Special"</span>
                                                            <i class="bi bi-pin-angle-fill"></i>
                                                        </div>

                                                        // Tech list
                                                        <div class="card-tech">
                                                            <Transition fallback=|| view! { <p>"Loading skills..."</p> }>
                                                                {move || {
                                                                    skills.get().map(|res| match res {
                                                                        Ok(sdata) => {
                                                                            tech.iter().filter_map(|id| {
                                                                                find_skill_by_id(&sdata.rows, *id).map(|s| view! {
                                                                                    <a class="tech" title=s.title.clone()
                                                                                        href=s.url_docs.clone()
                                                                                        target="_blank"
                                                                                    >
                                                                                        <span>{s.title.clone()}</span>
                                                                                        <img src=s.image_src.clone()
                                                                                            alt=s.title.clone()
                                                                                            loading="lazy"
                                                                                        />
                                                                                    </a>
                                                                                })
                                                                            }).collect_view().into_any()
                                                                        }
                                                                        Err(_) => view! { <p>"Failed to load skills"</p> }.into_any(),
                                                                    })
                                                                }}
                                                            </Transition>
                                                        </div>
                                                    </div>
                                                    <div class="overlay-card"></div>
                                                </a>
                                            </div>
                                        }
                                    }).collect_view().into_any()
                                }
                                Err(e) => view! { <p>{format!("Error: {e}")}</p> }.into_any(),
                            })
                        }}
                    </Transition>

                    // Pagination
                    <nav class=move || {
                        if total.get() as i32 <= limit { "d-none" } else { "pagination-container" }
                    }>
                        <ul class="pagination justify-content-end">
                            <li class=format!(
                                "page-item {}",
                                if current_page.get() == 1 { "disabled" } else { "" },
                            )>
                                <button class="page-link" on:click=move |_| set_current_page(current_page.get() - 1)>
                                    <i class="bi bi-caret-left-fill"></i>
                                </button>
                            </li>
                            {
                                let total_pages = (total.get() as f64 / limit as f64).ceil() as i32;
                                (1..=total_pages).map(|i| {
                                    view! {
                                        <li class=format!(
                                            "page-item {}",
                                            if current_page.get() == i { "active" } else { "" },
                                        )>
                                            <button class="page-link" on:click=move |_| set_current_page(i)>
                                                {i}
                                            </button>
                                        </li>
                                    }
                                }).collect_view()
                            }
                            <li class=format!(
                                "page-item {}",
                                if current_page.get() * limit >= total.get() as i32 { "disabled" } else { "" },
                            )>
                                <button class="page-link" on:click=move |_| set_current_page(current_page.get() + 1)>
                                    <i class="bi bi-caret-right-fill"></i>
                                </button>
                            </li>
                        </ul>
                    </nav>
                </div>
            </div>
        </section>
    }
}
