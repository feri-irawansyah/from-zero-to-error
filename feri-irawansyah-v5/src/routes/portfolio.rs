use gloo_net::http::Request;
use leptos::{leptos_dom::logging::console_log, prelude::*, task::spawn_local};

use crate::{app::BACKEND_URL, components::card_loading::CardLoading, contexts::models::{AppState, Portfolio, PortfolioData, Skill, SkillsData}};

#[allow(non_snake_case)]
#[component]
pub fn Portfolio() -> impl IntoView {

    let navigate = leptos_router::hooks::use_navigate();
    let portfolio: RwSignal<Vec<Portfolio>> = RwSignal::new(vec![]);
    let (total, set_total) = signal(0);
    let (current_page, set_current_page) = signal(1);
    let (loading, set_loading) = signal(false);

    let skills: RwSignal<Vec<Skill>> = RwSignal::new(vec![]);
    let (loading_skill, set_loading_skill) = signal(false);
    let state = expect_context::<AppState>();

    let limit = 9;

    let fetch_portfolio = move |page: i32| {
        let offset = (page - 1) * limit;
        let url = format!(
            "{}/data/table?tablename=portfolio&offset={}&limit={}&nidkey=portfolio_id",
            BACKEND_URL,
            offset,
            limit
        );

        spawn_local(async move {
            set_loading(true);
            if let Ok(response) = Request::get(&url).send().await {
                if let Ok(data) = response.json::<PortfolioData>().await {
                    console_log(format!("{:?}", data).as_str());
                    portfolio.set(data.rows);
                    set_total(data.total);
                }
            }
            set_loading(false);
        });
    };

    let fetch_skill = move |_| {
        let url = format!(
            "{}/data/table?tablename=skills&offset={}&limit={}&nidkey=skill_id",
            BACKEND_URL,
            0,
            50
        );

        spawn_local(async move {
            set_loading_skill(true);
            if let Ok(response) = Request::get(&url).send().await {
                if response.status() == 200 {
                    if let Ok(data) = response.json::<SkillsData>().await {
                        skills.set(data.rows);
                    }
                } else {
                    console_log(format!("Error: {}", response.status()).as_str());
                }
            }
            set_loading_skill(false);
        });
    };

    Effect::new(move |_| {
        fetch_portfolio(current_page.get());
        fetch_skill(());
        move || {}
    });

    fn find_skill_by_id(skills: &[Skill], id: i32) -> Option<Skill> {
        skills.iter().find(|s| s.skill_id == id).cloned()
    }

    
    view! {
        <section id="portfolio" class="portfolio section" data-aos="fade-right">     
            <div class="container section-title" data-aos="slide-right" data-aos-delay="100">
                <h2>Porfolio</h2>
                <p>I am a Software Engineer dedicated to building efficient, scalable, and user-friendly digital solutions. With a strong background in web development, I am used to working with various modern technologies such as JavaScript/TypeScript, Svelte, Rust, and various other frameworks.</p>
            </div>   
             <div class="container" data-aos="slide-right" data-aos-delay="200">
                <div class="row justify-content-start">
                    <Show 
                        when=move || !loading.get()
                        fallback=|| view! { <CardLoading delay={Some(0)} count={Some(2)} /> }
                    > 
                    {move || portfolio.get().iter().map(|p| {
                        let tech = p.tech.clone();
                        view! {
                            <div class="col-lg-6">
                                <a class="card text-bg-dark" href={p.url_docs.clone()} target="_blank">
                                    <img src={format!("/assets/{}", p.image_src.clone())} class="card-img" alt={p.title.clone()} loading="lazy"/>
                                    <div class="card-img-overlay">
                                        <h4 class="card-title view-project"><span>View Project</span> <i class="bi bi-eye"></i></h4>
                                        <h5 class="card-title">{p.title.clone()}</h5>
                                        <p class="card-text">{p.description.clone()}</p>
                                        <div class="card-tech">
                                            <Show 
                                                when=move || !loading_skill.get()
                                                fallback=|| view! { Loading... }
                                            >
                                                {tech.iter().filter_map(|id| {
                                                    let skill_id = *id;
                                                    let skill = find_skill_by_id(&skills.get(), skill_id);
                                                    skill.map(|s| view! {
                                                        <a class="tech" title={s.title.clone()} href={format!("{}", s.url_docs.clone())} target="_blank">
                                                            <img src={format!("/assets/{}", s.image_src.clone())} alt={s.title.clone()} loading="lazy" />
                                                        </a>
                                                    })
                                                    }).collect::<Vec<_>>()
                                                }
                                            </Show>
                                        </div>
                                    </div>
                                    <div class="overlay-card"></div>
                                </a>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                    </Show>
                </div>
             </div>     
        </section>
    }
}