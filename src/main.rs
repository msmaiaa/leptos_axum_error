pub mod fallback;
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{
        extract::Extension,
        http::Uri,
        routing::{get, post},
        Router,
    };
    use fallback::*;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use leptos_start::app::*;
    use std::sync::Arc;

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

    ApiEndpoint::register();

    let app = Router::new()
        .route("/favicon.ico", get(file_and_error_handler))
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(
            conf.leptos_options.clone(),
            routes,
            |cx| view! { cx, <App/> },
        )
        .fallback(file_and_error_handler)
        .layer(Extension(Arc::new(conf.leptos_options)));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
