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
pub fn SkillMarquee(skills: Vec<Skill>, position: Option<String>) -> impl IntoView {
    view! {
        <div class="marquee-wrapper">
            <div class=format!(
                "marquee-content marquee-{}",
                position.clone().unwrap_or("".to_string()),
            )>
                {
                    let doubled_skills = skills.iter().chain(skills.iter());

                    doubled_skills
                        .map(|skill| {
                            view! {
                                <div class="marquee-item">
                                    <img
                                        class="skill-icon"
                                        src=skill.image_src.clone()
                                        alt=skill.title.clone()
                                        loading="lazy"
                                    />
                                    <span>{skill.title.clone()}</span>
                                </div>
                            }
                        })
                        .collect::<Vec<_>>()
                }
            </div>
        </div>
    }
}


