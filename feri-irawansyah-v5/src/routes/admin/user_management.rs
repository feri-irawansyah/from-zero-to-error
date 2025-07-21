use gloo_net::http::Request;
use leptos::{prelude::*, task::spawn_local};

use crate::{app::BACKEND_URL, directives::table::Table};

#[allow(non_snake_case)]
#[component]
pub fn UserManagement() -> impl IntoView {

    // portfolio state
    let portfolio: RwSignal<Vec<serde_json::Value>> = RwSignal::new(vec![serde_json::Value::Array(vec![])]);
    let portfolio_page: RwSignal<i32> = RwSignal::new(1);
    let portfolio_total: RwSignal<usize> = RwSignal::new(0);
    let portfolio_loading: RwSignal<bool> = RwSignal::new(false);
    let portfolio_limit = 50;

    // skills state
    let skills: RwSignal<Vec<serde_json::Value>> = RwSignal::new(vec![serde_json::Value::Array(vec![])]);
    let skills_page: RwSignal<i32> = RwSignal::new(1);
    let skills_total: RwSignal<usize> = RwSignal::new(0);
    let skills_loading: RwSignal<bool> = RwSignal::new(false);
    let skills_limit = 50;

    // experience state
    let experience: RwSignal<Vec<serde_json::Value>> = RwSignal::new(vec![serde_json::Value::Array(vec![])]);
    let experience_page: RwSignal<i32> = RwSignal::new(1);
    let experience_total: RwSignal<usize> = RwSignal::new(0);
    let experience_loading: RwSignal<bool> = RwSignal::new(false);
    let experience_limit = 50;

    let fetch_portfolio = move |page: i32| {
        let offset = (page - 1) * portfolio_limit;
        let url = format!(
            "{}/data/table?tablename=portfolio&offset={}&limit={}&nidkey=portfolio_id",
            BACKEND_URL,
            offset,
            portfolio_limit
        );

        spawn_local(async move {
            portfolio_loading.set(true);
            if let Ok(response) = Request::get(&url).send().await {
                if let Ok(data) = response.json::<serde_json::Value>().await {
                    portfolio.set(data["rows"].as_array().unwrap_or(&vec![]).to_vec());
                    portfolio_total.set(data["total"].as_i64().unwrap_or(0) as usize);
                }
            }
            portfolio_loading.set(false);
        });
    };

    let fetch_skills = move |page: i32| {
        let offset = (page - 1) * skills_limit;
        let url = format!(
            "{}/data/table?tablename=skills&offset={}&limit={}&nidkey=skill_id",
            BACKEND_URL,
            offset,
            skills_limit
        );

        spawn_local(async move {
            skills_loading.set(true);
            if let Ok(response) = Request::get(&url).send().await {
                if let Ok(data) = response.json::<serde_json::Value>().await {
                    skills.set(data["rows"].as_array().unwrap_or(&vec![]).to_vec());
                    skills_total.set(data["total"].as_i64().unwrap_or(0) as usize);
                }
            }
            skills_loading.set(false);
        });
    };

    let fetch_experience = move |page: i32| {
        let offset = (page - 1) * experience_limit;
        let url = format!(
            "{}/data/table?tablename=experience&offset={}&limit={}&nidkey=experience_id",
            BACKEND_URL,
            offset,
            experience_limit
        );

        spawn_local(async move {
            experience_loading.set(true);
            if let Ok(response) = Request::get(&url).send().await {
                if let Ok(data) = response.json::<serde_json::Value>().await {
                    experience.set(data["rows"].as_array().unwrap_or(&vec![]).to_vec());
                    experience_total.set(data["total"].as_i64().unwrap_or(0) as usize);
                }
            }
            experience_loading.set(false);
        });
    };

    Effect::new(move |_| {
        fetch_portfolio(portfolio_page.get());
        fetch_skills(skills_page.get());
        fetch_experience(experience_page.get());
    });

    view! {
        <div class="user-management">
            <div class="row">
                <div class="col-12 portfolio">
                    <div class="card">
                        <div class="card-header">Portfolio</div>
                        <div class="card-body">
                            <Table
                                table="portfolio".to_string()
                                data=portfolio.clone()
                                page=portfolio_page
                                total=portfolio_total
                                limit=portfolio_limit
                                refresh=fetch_portfolio
                                loading_data=portfolio_loading
                            />
                        </div>
                    </div>
                </div>
                <div class="col-md-6 experience">
                    <div class="card">
                        <div class="card-header">Experience</div>
                        <div class="card-body">
                            <Table
                                table="experience".to_string()
                                data=experience.clone()
                                page=experience_page
                                total=experience_total
                                limit=experience_limit
                                refresh=fetch_experience
                                loading_data=experience_loading
                            />
                        </div>
                    </div>
                </div>
                <div class="col-md-6 skills">
                    <div class="card">
                        <div class="card-header">Skills</div>
                        <div class="card-body">
                            <Table
                                table="skills".to_string()
                                data=skills.clone()
                                page=skills_page
                                total=skills_total
                                limit=skills_limit
                                refresh=fetch_skills
                                loading_data=skills_loading
                            />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}