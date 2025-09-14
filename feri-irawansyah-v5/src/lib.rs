#![recursion_limit = "256"]
#![type_length_limit = "2097152"]

pub mod app;
pub mod contexts {
    pub mod index;
    pub mod models;
}
pub mod routes {
    pub mod home;
    pub mod notfound;
    pub mod about;
    pub mod services;
    pub mod portfolio;
    pub mod contact;
    pub mod login;
    pub mod notes {
        pub mod category;
        pub mod slug;
        pub mod list_catatan;
    }
    pub mod admin {
        pub mod dashboard;
        pub mod user_management;
        pub mod notes_management;
    }
}
pub mod components {
    pub mod catatan_layout;
    pub mod menu_list;
    pub mod admin_layout;
    pub mod clock;
    pub mod skill_slider;
    pub mod about_tab;
    pub mod loading;
    pub mod card_loading;
    pub mod modal_detail;
}
pub mod directives {
    pub mod markdown;
    pub mod table;
    pub mod modal_container;
}
pub mod middleware {
    pub mod session;
    pub mod email;
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}

#[macro_export]
macro_rules! leptos_ignore {
    ($item:item) => {
        $item
    };
}
