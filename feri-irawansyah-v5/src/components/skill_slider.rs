use leptos::prelude::*;

use crate::contexts::models::Skill;

pub struct SkillSignals {
    pub techstack: RwSignal<Vec<Skill>>,
    pub programming: RwSignal<Vec<Skill>>,
    pub devops: RwSignal<Vec<Skill>>,
}

impl SkillSignals {
    pub fn new() -> Self {
        Self {
            techstack: RwSignal::new(vec![]),
            programming: RwSignal::new(vec![]),
            devops: RwSignal::new(vec![]),
        }
    }
}

#[allow(non_snake_case)]
#[component]
pub fn SkillMarquee(skills: RwSignal<Vec<Skill>>, position: Option<String>) -> impl IntoView {
    let list_skills = skills.get();
    let doubled_skills = list_skills.iter().chain(list_skills.iter()); // duplikat biar seamless scroll

    view! {
        <div class="marquee-wrapper">
            <div class=format!("marquee-content marquee-{}", position.unwrap_or("".to_string()))>
                {doubled_skills
                    .map(|skill| {
                        view! {
                            <img class="skill-icon" src=format!("/assets/{}", skill.image_src.clone()) alt={skill.title.clone()} />
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>
        </div>
    }
}
