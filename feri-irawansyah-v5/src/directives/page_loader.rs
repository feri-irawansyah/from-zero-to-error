use leptos::prelude::*;

pub fn page_loader(content: impl IntoView + 'static) -> impl IntoView {
    let (loading, set_loading) = signal(true);

    // pas mount, tampilkan loader dulu
    Effect::new(move |_| {
        set_loading(true);
        set_timeout(move || {
            set_loading.set(false);
        }, std::time::Duration::from_millis(800));
    });

    view! {
       <Show when=move || loading.get() fallback=|| view! {<div class="d-none"></div>}>
           <div class="lazy-loader">
               <div class="loader"></div>
           </div>
       </Show>
       
       <Transition
            // the fallback will show initially
            // on subsequent reloads, the current child will
            // continue showing
            fallback=move || view! { 
                <div class="lazy-loader">
                    <div class="loader"></div>
                </div>
             }
            // this will be set to `true` whenever the transition is ongoing
            
        >
            {content}
        </Transition>
    }
}