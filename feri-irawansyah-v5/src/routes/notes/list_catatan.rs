use leptos::prelude::*;

use crate::contexts::models::Category;

#[allow(non_snake_case)]
#[component]
pub fn ListCatatan() -> impl IntoView {

    let categories = vec![
        Category {
            name: "backend",
            image: "https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/backend.webp",
            title: "Backend",
            desc: "Intip catatan tentang aplikasi backend dengan performa tinggi dan modern sekalian nyobain.",
            list_tech: &[
                "rest", "graphql", "rust", "actix-web", "bun", "hono", "nodejs", "nestjs", "laravel-api", ".net",
            ],
        },
        Category {
            name: "frontend",
            image: "https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/frontend.webp",
            title: "Frontend",
            desc: "Lu mungkin bakal suka liat catatan tentang tampilan aplikasi yang oke dan interaktif.",
            list_tech: &[
                "svelte/sveltekit", "react/nextjs", "sass", "tailwind", "bootstrap", "tauri", "handlebars", "flutter",
            ],
        },
        Category {
            name: "fullstack",
            image: "https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/fullstack.webp",
            title: "Fullstack",
            desc: "Kombinasi backend dan frontend dalam satu catatan mungkin bisa jadi inspirasi baru bang.",
            list_tech: &[
                "graphql", "prisma", "session", "handlebars", "jwt", "architecture", "blade", "laravel", "web-socket",
            ],
        },
        Category {
            name: "devops",
            image: "https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/devops.webp",
            title: "Devops",
            desc: "Catatan tentang bagaimana kita bisa menjalankan aplikasi dengan cepat dan mudah meski ngga punya server.",
            list_tech: &[
                "linux", "docker", "nginx", "kubernetes", "internet-information-service", "laragon", "load-balancer",
            ],
        },
        Category {
            name: "random",
            image: "https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/random.webp",
            title: "Random",
            desc: "Jangan ngoding mulu bang ntar pingsan kita, kita juga manusia bukan robot. Robot aja bisa error.",
            list_tech: &[
                "cerita-lucu", "pengalaman", "fullstuck-developer", "gajelas", "gabut", "bisa", "error", "olahraga", "makan",
            ],
        },
        Category {
            name: "series",
            image: "https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/series.webp",
            title: "Series",
            desc: "Kita bacotin teknologi yang ada dimuka bumi dalam konteks pebelajaran.",
            list_tech: &[
                "python-lemot", "rust-dewa-compiler", "php-realita-hidup", "c#-anak-manja", "node-module-isi-dosa",
            ],
        },
    ];
     
    view! {
        <div class="row" data-aos="slide-up" data-aos-duration="800">
            {categories
                .into_iter()
                .enumerate()
                .map(|(i, cat)| {
                    view! {
                        <div
                            class="col-md-4 my-3"
                            data-aos="fade-up"
                            data-aos-delay=format!("{}", 100 + (i * 100))
                        >
                            <a
                                class="card p-3 text-decoration-none h-100"
                                href=format!("/catatan/{}", cat.name)
                            >
                                <img
                                    src=cat.image
                                    class="card-img-top"
                                    alt=cat.title
                                    loading="lazy"
                                />
                                <div class="card-body">
                                    <h4 class="card-title">{cat.title}</h4>
                                    <p class="card-text">{cat.desc}</p>
                                </div>
                                <div class="card-footer">
                                    <div class="d-flex flex-row gap-2 flex-wrap">
                                        {cat
                                            .list_tech
                                            .iter()
                                            .map(|tech| {
                                                view! { <span class="badge bg-primary">#{*tech}</span> }
                                            })
                                            .collect_view()}
                                    </div>
                                </div>
                            </a>
                        </div>
                    }
                })
                .collect_view()}
        </div>
    }
}