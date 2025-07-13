use leptos::prelude::*;

use crate::{contexts::models::ModalState, directives::modal_container::ModalContainer};

#[allow(non_snake_case)]
#[component]
pub fn ModalDetail() -> impl IntoView {
    let modal_state = expect_context::<ModalState>();

    view! {
        <ModalContainer title=modal_state.title size=Some("xl".to_string()) modal_id="detail".to_string()>
            <Show
                when=move || !modal_state.data.get().is_null() // ✅ cek Value::Null dulu
                fallback=move || view! { <h1 class="text-center">Loading...</h1> }
            >
                {move || {
                    let data_opt = modal_state.data.get().as_object().unwrap().clone(); // bisa None
                    let value = data_opt.clone();
                    view! {
                        <Show when=move || !value.is_empty() fallback=move || view! { <h1 class="text-center">Not Found</h1> }>
                            <form>
                                {data_opt.iter().filter(|(key, _)| {
                                    *key != "tsv" && *key != "ip_address"
                                }).map(|(key, val)| {
                                    let value_str = match val {
                                        serde_json::Value::String(s) => s.clone(),
                                        serde_json::Value::Number(n) => n.to_string(),
                                        serde_json::Value::Bool(b) => b.to_string(),
                                        _ => serde_json::to_string(val).unwrap_or_default(),
                                    };

                                    view! {
                                        <div class="mb-3">
                                            <label for={key.clone()} class="form-label">{key.clone()}</label>
                                            <input
                                                type="text"
                                                disabled={key == "slug" || key.contains("id")}
                                                id={key.clone()}
                                                class="form-control"
                                                readonly=true
                                                value={value_str}
                                            />
                                        </div>
                                    }
                                }).collect_view()}
                            </form>
                    </Show>
                    }
                }}
            </Show>
        </ModalContainer>

    }
}