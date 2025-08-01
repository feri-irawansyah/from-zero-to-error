use gloo_net::http::Request;
use leptos::{prelude::*, task::spawn_local};
use serde::Deserialize;

use crate::{app::BACKEND_URL, contexts::index::format_cell_value};

#[derive(Clone, Debug, Deserialize)]
pub struct ColumnData {
    pub data: Vec<Columns>,
}


#[derive(Clone, Debug, Deserialize)]
pub struct Columns {
    pub field: String,
    pub formatter: Option<String>,
    pub title: String,
    pub sortable: bool,
    #[serde(rename = "filterControl")]
    pub filter_control: String,
}

#[allow(non_snake_case)]
#[component]
pub fn Table(table: String, data: RwSignal<Vec<serde_json::Value>>, loading_data: RwSignal<bool>, page: RwSignal<i32>, limit: i32, total: RwSignal<usize>, refresh: impl Fn(i32) + 'static) -> impl IntoView {
    let columns = RwSignal::new(vec![]);
    let (loading, set_loading) = signal(false);

    let get_table_object = move |tablename: String| {
        let url = format!("{}/data/header?tablename={}", BACKEND_URL, tablename);

        spawn_local(async move {
            set_loading(true);
            if let Ok(response) = Request::get(&url).send().await {
                if let Ok(header) = response.json::<ColumnData>().await {
                    columns.set(header.data);
                }
            }
            set_loading(false);
        });
    };

    Effect::new(move |_| {
        get_table_object(table.clone());
    });

    view! {
        <div class="table-container">

            <div class="toolbar">
                <div class="left-toolbar">
                    <button class="btn btn-success btn-sm">
                        <i class="bi bi-plus me-1 text-white"></i>
                        <span class="d-none d-md-inline text-white">New Record</span>
                    </button>
                </div>
                <div class="right-toolbar">
                    <button class="btn btn-primary btn-sm" on:click=move |_| refresh(page.get())>
                        <i class="bi bi-arrow-clockwise"></i>
                    </button>
                    <button class="btn btn-primary btn-sm ms-1">
                        <i class="bi bi-clock"></i>
                    </button>
                    <button class="btn btn-primary btn-sm ms-1">
                        <i class="bi bi-search"></i>
                    </button>
                </div>
            </div>
            <div class="table-responsive">
                <table class="table table-striped responsive-table">
                    <thead>
                        <tr>
                            <Show
                                when=move || !loading.get()
                                fallback=|| {
                                    view! {
                                        <a
                                            class="btn btn-primary disabled placeholder col-12"
                                            aria-disabled="true"
                                        ></a>
                                    }
                                }
                            >
                                {move || {
                                    columns
                                        .get()
                                        .iter()
                                        .filter(|col| match col.field.as_str() {
                                            "notes_id" => false,
                                            "tsv" => false,
                                            "portfolio_id" => false,
                                            _ => true,
                                        })
                                        .map(|col| {
                                            // sembunyikan
                                            // sembunyikan
                                            // sembunyikan
                                            view! { 
                                                <th>{col.title.clone()}</th>
                                             }
                                        })
                                        .collect_view()
                                }}
                            </Show>
                        </tr>
                    </thead>
                    <tbody>
                        <Show
                            when=move || !loading_data.get()
                            fallback=|| {
                                view! {
                                    <a
                                        class="btn btn-primary disabled placeholder col-12"
                                        aria-disabled="true"
                                    ></a>
                                }
                            }
                        >
                            {move || {
                                data.get()
                                    .iter()
                                    .map(|item| {
                                        view! {
                                            <tr>
                                                {columns
                                                    .get()
                                                    .iter()
                                                    .map(|col| { format_cell_value(item, col) })
                                                    .collect_view()}
                                            </tr>
                                        }
                                    })
                                    .collect_view()
                            }}
                        </Show>
                    </tbody>
                </table>
            </div>
            <Show
                when=move || total.get() as i32 <= limit
                fallback=move || {
                    view! {
                        <nav class="table-pagination">
                            <ul class="pagination justify-content-end pagination-sm">
                                <li class=format!(
                                    "page-item {}",
                                    if page.get() == 1 { "disabled" } else { "" },
                                )>
                                    <button
                                        class="page-link"
                                        on:click=move |_| page.set(page.get() - 1)
                                    >
                                        <i class="bi bi-caret-left-fill"></i>
                                    </button>
                                </li>
                                {
                                    let total_pages = (total.get() as f64 / limit as f64).ceil()
                                        as i32;
                                    (1..=total_pages)
                                        .map(|i| {
                                            view! {
                                                <li class=format!(
                                                    "page-item {}",
                                                    if page.get() == i { "active" } else { "" },
                                                )>
                                                    <button class="page-link" on:click=move |_| page.set(i)>
                                                        {i}
                                                    </button>
                                                </li>
                                            }
                                        })
                                        .collect_view()
                                }
                                <li class=format!(
                                    "page-item {}",
                                    if page.get() * limit >= total.get().try_into().unwrap() {
                                        "disabled"
                                    } else {
                                        ""
                                    },
                                )>
                                    <button
                                        class="page-link"
                                        on:click=move |_| page.set(page.get() + 1)
                                    >
                                        <i class="bi bi-caret-right-fill"></i>
                                    </button>
                                </li>
                            </ul>
                        </nav>
                    }
                }
            >
                <span></span>
            </Show>
        </div>
    }
}

