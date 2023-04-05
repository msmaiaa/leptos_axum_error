use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <Title text="Welcome to Leptos"/>
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

#[server(ApiEndpoint, "/api")]
pub async fn api_endpoint() -> Result<(), ServerFnError> {
    Err(ServerFnError::ServerError(
        "hello from api_endpoint".to_string(),
    ))
}

#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    leptos::spawn_local(async move {
        let res = api_endpoint().await;
        match res {
            Ok(_) => {}
            Err(e) => match e {
                ServerFnError::ServerError(msg) => {
                    leptos::log!("this is the error message: {}", msg);
                }
                _ => {}
            },
        }
    });
    view! { cx,
        <h1></h1>
    }
}
