use gloo_timers::callback::Timeout;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::Row;

#[cfg(feature = "ssr")]
use crate::middleware::connection::ssr::get_connection;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NoteRow {
    pub title: String,
    pub slug: String,
    pub category: Option<String>,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SearchHistory {
    pub term: String,             // kata yang dicari
    pub url: String,              // link tujuan
    pub category: Option<String>, // kategori (Portfolio / Catatan)
    pub timestamp: chrono::DateTime<chrono::Utc>,           // kapan disimpan (UNIX time)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiResponse {
    pub rows: Vec<NoteRow>,
    pub total: i64,
    #[serde(rename = "totalNotFiltered")]
    pub total_not_filtered: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub category: Option<String>,
    pub description: Option<String>,
}

impl From<NoteRow> for SearchResult {
    fn from(row: NoteRow) -> Self {
        Self {
            title: row.title,
            url: format!("/catatan/{}", row.slug),
            category: row.category,
            description: row.description,
        }
    }
}

#[server(SearchPalette, "/api")]
pub async fn search_palette(query: String) -> Result<Vec<SearchResult>, ServerFnError> {
    let connection = get_connection().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    let q = format!("{}:*", query); // support prefix search

    let rows = sqlx::query(r#"
        WITH notes_q AS (
            SELECT
                title,
                '/catatan/' || category || '/' || slug AS url,
                category,
                description
            FROM notes
            WHERE tsv @@ plainto_tsquery('indonesian', $1)
            ),

            portfolio_q AS (
            SELECT
                p.title,
                '/portfolio/' || p.portfolio_id::text AS url,
                string_agg(s.title, ', ') AS category,
                p.description
            FROM portfolio p
            LEFT JOIN LATERAL unnest(p.tech) AS t(skill_id) ON TRUE
            LEFT JOIN skills s ON s.skill_id = t.skill_id
            WHERE p.tsv @@ plainto_tsquery('indonesian', $1)
            GROUP BY p.portfolio_id, p.title, p.description
        )

        SELECT * FROM (
        SELECT * FROM notes_q
        UNION ALL
        SELECT * FROM portfolio_q
        ) all_results
        LIMIT 20;
    "#)
        .bind(&q) // atau .bind(&q) untuk prefix
        .fetch_all(&connection)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let results: Vec<SearchResult> = rows
        .into_iter()
        .map(|row| SearchResult {
            title: row.get::<String, _>(0),
            url: row.get::<String, _>(1),
            category: row.get::<Option<String>, _>(2),
            description: row.get::<Option<String>, _>(3),
        })
        .collect();

    Ok(results)

}

#[allow(non_snake_case)]
#[component]
pub fn SearchModal() -> impl IntoView {
    let (debounced, set_debounced) = signal(String::new());
    let state = expect_context::<crate::contexts::models::AppState>();
    // simpan handler timeout biar bisa cancel
    let timeout = std::rc::Rc::new(std::cell::RefCell::new(None::<Timeout>));

    let resource = Resource::new(move || debounced.get(), |q| async move {
        if q.is_empty() {
            Ok(vec![])
        } else  {
            search_palette(q).await 
        }
    });

    let navigate = use_navigate();

    view! {
        <div class="modal fade search-modal" id="searchModal" tabindex="-1" aria-hidden="true">
            <div class="modal-dialog modal-dialog-centered modal-dialog-scrollable">
                <div class="modal-content p-3">
                    <div class="input-search">
                        <i class="bi bi-search"></i>
                        <input
                            type="text"
                            class="form-control form-control-lg"
                            placeholder="Search..."
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                // clear timeout lama
                                if let Some(handle) = timeout.borrow_mut().take() {
                                    handle.cancel();
                                }

                                // bikin timeout baru
                                let handle = {
                                    let set_debounced = set_debounced.clone();
                                    Timeout::new(400, move || {
                                        set_debounced.set(value.clone());
                                    })
                                };

                                *timeout.borrow_mut() = Some(handle);
                            }
                        />
                    </div>
                    <div class="modal-body">
                        <ul class="list-group list-search mt-3">
                            <Transition fallback=move || view! { <li class="list-group-item">"Loading..."</li> }>
                                {move || {
                                    resource.get().map(|results| {
                                        match results {
                                            Ok(rows) => {
                                                // --- Kelompokkan berdasarkan kategori ---
                                                use std::collections::HashMap;
                                                let mut grouped: HashMap<&str, Vec<_>> = HashMap::new();

                                                for res in rows {
                                                    let url = res.url.clone();
                                                    let category = if url.contains("portfolio") {
                                                        "Portfolio"
                                                    } else if url.contains("catatan") {
                                                        "Catatan"
                                                    } else {
                                                        "Other"
                                                    };

                                                    grouped.entry(category).or_default().push(res);
                                                }

                                                // --- Render per kategori ---
                                                grouped.into_iter().map(|(category, items)| {
                                                    view! {
                                                        <>
                                                            <li class="list-group-item active">{category}</li>
                                                            {items.into_iter().map(|res| {
                                                                let title = res.title.clone();
                                                                let desc = res.description.clone().unwrap_or_default();
                                                                let url = res.url.clone();
                                                                let navigate = navigate.clone();

                                                                view! {
                                                                    <li
                                                                        class="list-group-item"
                                                                        data-bs-dismiss="modal"
                                                                        on:click=move |_| {
                                                                            // --- Navigate ke halaman tujuan ---
                                                                            navigate(&url, Default::default());

                                                                            // --- Simpan ke search history ---
                                                                            let item = SearchHistory {
                                                                                term: debounced.get().clone(),
                                                                                url: url.clone(),
                                                                                category: None,
                                                                                timestamp: chrono::Utc::now(),
                                                                            };

                                                                            if let Ok(js_value) = serde_wasm_bindgen::to_value(&item) {
                                                                                crate::app::save_search(js_value);

                                                                                let js_value = crate::app::get_search();
                                                                                if let Ok(search) = serde_wasm_bindgen::from_value(js_value) {
                                                                                    state.search.set(search);
                                                                                } else {
                                                                                    state.search.set(Vec::new());
                                                                                }
                                                                            }
                                                                        }
                                                                    >
                                                                        <div>
                                                                            <strong>{title.clone()}</strong>
                                                                            <p class="small text-muted">{desc.clone()}</p>
                                                                        </div>
                                                                    </li>
                                                                }
                                                            }).collect_view()}
                                                        </>
                                                    }
                                                }).collect_view().into_any()
                                            }
                                            Err(e) => view! {
                                                <li class="list-group-item text-danger">{e.to_string()}</li>
                                            }.into_any()
                                        }
                                    })
                                }}

                            </Transition>
                        </ul>
                        {move || {
                            let navigate = use_navigate();

                            if state.search.get().len() > 0 {
                                view! {
                                    <ul class="list-group list-history">
                                        {state.search.get().into_iter().map(|item| {
                                            let navigate = navigate.clone();
                                            let term = item.term.clone();
                                            let url = item.url.clone();
                                            view! {
                                                <li class="list-group-item" on:click=move |_| navigate(&item.url, Default::default())>
                                                    <span data-bs-dismiss="modal">History: {item.term}</span>
                                                    <small>{move || crate::contexts::index::format_wib_datetime(&item.timestamp.to_string())}</small>
                                                    <button class="btn" on:click=move |_| { 
                                                        crate::app::delete_search(&term, &url);

                                                        let js_value = crate::app::get_search();
                                                        if let Ok(search) = serde_wasm_bindgen::from_value(js_value) {
                                                            state.search.set(search);
                                                        } else {
                                                            state.search.set(Vec::new());
                                                        }
                                                        }>
                                                        <i class="bi bi-x-circle cursor-pointer"></i>
                                                    </button>
                                                </li>
                                            }
                                        }).collect_view()}
                                    </ul>
                                }.into_any()
                            } else {
                                view! { <></> }.into_any()
                            }
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}