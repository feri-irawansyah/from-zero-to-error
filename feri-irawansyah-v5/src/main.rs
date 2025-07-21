#![recursion_limit = "256"]

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use leptos::prelude::*;
    use leptos::config::get_configuration;
    use leptos_meta::MetaTags;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use feri_irawansyah::app::*;

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;

    HttpServer::new(move || {
        // Generate the list of routes in your Leptos App
        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = leptos_options.site_root.clone().to_string();

        println!("listening on http://{}", &addr);

        App::new()
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // .service(Files::new("/js", format!("{site_root}/vendor/js")))
            // .service(Files::new("/css", format!("{site_root}/vendor/css")))
            // // serve other assets from the `assets` directory
            // .service(Files::new("/assets", &site_root))
            // serve the favicon from /favicon.ico
            .service(favicon)
            .service(static_from_supabase)
            .service(sitemap)
            .service(robots)
            .leptos_routes(routes, {
                let leptos_options = leptos_options.clone();
                move || {
                    view! {
                        <!DOCTYPE html>
                        <html lang="en">
                            <head>
                                <meta charset="utf-8" />
                                <meta name="robots" content="index, follow" />
                                <meta
                                    name="viewport"
                                    content="width=device-width, initial-scale=1"
                                />
                                <AutoReload options=leptos_options.clone() />
                                <HydrationScripts options=leptos_options.clone() />
                                <MetaTags />
                                <script src="https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/js/bootstrap.bundle.min.js"></script>
                                <script src="https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/js/aos.min.js"></script>
                                <script src="https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/js/marquee.js"></script>
                                <script src="https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/js/typeit.js"></script>
                            </head>
                            <body class="dark-background">
                                <App />
                            </body>
                        </html>
                    }
                }
            })
            .app_data(web::Data::new(leptos_options.to_owned()))
        //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("sitemap.xml")]
async fn sitemap(
    leptos_options: actix_web::web::Data<leptos::config::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/sitemap.xml"
    ))?)
}

#[cfg(feature = "ssr")]
#[actix_web::get("robots.txt")]
async fn robots(
    leptos_options: actix_web::web::Data<leptos::config::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/robots.txt"
    ))?)
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

// #[cfg(feature = "ssr")]
// #[actix_web::get("/favicon.ico")]
// async fn favicon() -> actix_web::Result<actix_web::HttpResponse> {
//     use actix_web::http::header::{self, HeaderValue};
//     use reqwest::StatusCode;

//     let url = "https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/favicon.ico";

//     let response = reqwest::get(url).await.map_err(|_| {
//         actix_web::error::ErrorBadGateway("Failed to fetch favicon from Supabase")
//     })?;

//     if response.status() != StatusCode::OK {
//         return Err(actix_web::error::ErrorNotFound("Favicon not found"));
//     }

//     let content_type = response
//         .headers()
//         .get("content-type")
//         .and_then(|val| val.to_str().ok())
//         .map(|s| s.to_owned())
//         .unwrap_or_else(|| "image/x-icon".to_string());

//     let bytes = response
//         .bytes()
//         .await
//         .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to read favicon"))?;

//     Ok(actix_web::HttpResponse::Ok()
//         .insert_header((
//             header::CONTENT_TYPE,
//             HeaderValue::from_str(&content_type)
//                 .unwrap_or_else(|_| HeaderValue::from_static("image/x-icon")),
//         ))
//         .insert_header((header::CACHE_CONTROL, "public, max-age=86400"))
//         .body(bytes))
// }

#[cfg(feature = "ssr")]
#[actix_web::get("/{folder:assets}/{tail:.*}")]
async fn static_from_supabase(
    path: actix_web::web::Path<(String, String)>,
) -> actix_web::Result<actix_web::HttpResponse> {
    use actix_web::http::header::{self, HeaderValue};
    use reqwest::StatusCode;

    let (folder, tail) = path.into_inner();

    let supabase_url = format!(
        "https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/{}/{}",
        folder, tail
    );

    // Fetch file from Supabase
    let response = match reqwest::get(&supabase_url).await {
        Ok(resp) => resp,
        Err(_) => return Err(actix_web::error::ErrorBadGateway("Failed to fetch file")),
    };

    // Check if file exists
    if response.status() != StatusCode::OK {
        return Err(actix_web::error::ErrorNotFound("File not found"));
    }

    // Ambil content-type dulu
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|val| val.to_str().ok())
        .map(|s| s.to_owned()) // convert ke String supaya gak borrow
        .unwrap_or_else(|| "application/octet-stream".to_string());

    // Baru ambil bytes-nya setelahnya
    let bytes = response
        .bytes()
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to read file"))?;

    Ok(actix_web::HttpResponse::Ok()
        .insert_header((
            header::CONTENT_TYPE,
            HeaderValue::from_str(&content_type)
                .unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream")),
        ))
        .insert_header((header::CACHE_CONTROL, "public, max-age=86400"))
        .body(bytes))
}


#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use feri_irawansyah::app::*;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}
