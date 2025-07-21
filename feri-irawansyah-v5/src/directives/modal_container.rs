use leptos::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn ModalContainer(children: Children, title: RwSignal<String>, size: Option<String>, modal_id: String) -> impl IntoView {

    view! {
        <div
            class="modal fade"
            id=modal_id.clone()
            data-bs-backdrop="static"
            data-bs-keyboard="false"
            tabindex="-1"
            aria-labelledby=format!("{}Label", modal_id.clone())
            aria-hidden="true"
        >
            <div class=format!(
                "modal-dialog modal-dialog-scrollable modal-dialog modal-{}",
                size.unwrap_or("md".to_string()),
            )>
                <div class="modal-content">
                    <div class="modal-header">
                        <Show
                            when=move || title.get().len() != 0
                            fallback=move || view! { <h5 class="modal-title">Loading....</h5> }
                        >
                            {move || view! { <h5 class="modal-title">{title.get()}</h5> }}
                        </Show>
                        <button
                            type="button"
                            class="btn-close"
                            data-bs-dismiss="modal"
                            aria-label="Close"
                        ></button>
                    </div>
                    <div class="modal-body">{children()}</div>
                    <div class="modal-footer">
                        <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">
                            Close
                        </button>
                        <button type="button" class="btn btn-primary">
                            Understood
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}