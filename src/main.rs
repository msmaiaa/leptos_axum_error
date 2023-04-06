pub mod fallback;

cfg_if::cfg_if! {
if #[cfg(feature="ssr")] {
    use axum::{
        extract::Extension,
        response::{Response, IntoResponse},
        body::Body as AxumBody,
        http::{Request},
        routing::{get, post},
        Router,
    };
    use fallback::*;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use leptos_start::app::*;
    use std::sync::Arc;

    async fn frontend_routes_handler(Extension(options): Extension<Arc<LeptosOptions>>, req: Request<AxumBody>) -> Response{
        let handler = leptos_axum::render_app_to_stream_with_context((*options).clone(),
        move |cx: Scope| {
            let data= vec![String::from("foo"), String::from("bar")];
            provide_context(cx, data);
        },
            |cx| view! { cx, <App/> }
        );
        handler(req).await.into_response()
    }

    #[tokio::main]
    async fn main() {
        let conf = get_configuration(None).await.unwrap();
        let addr = conf.leptos_options.site_addr;
        let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

        let app = Router::new()
        .leptos_routes_with_handler(routes, get(frontend_routes_handler) )
            .route("/favicon.ico", get(file_and_error_handler))
            .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
            .fallback(file_and_error_handler)
            .layer(Extension(Arc::new(conf.leptos_options)));

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
} else {
    pub fn main() {}
}
}
