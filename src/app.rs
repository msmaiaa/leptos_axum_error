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

#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let data_from_route_handler = use_context::<Vec<String>>(cx).unwrap_or(vec![]);

    let data_from_handler_view = data_from_route_handler
        .iter()
        .map(|_| {
            view! {cx,
                <div on:click=move |_| {
                    log!("data_from_handler_view on:click");
                }>"data_from_handler_view"</div>
            }
        })
        .collect::<Vec<_>>();

    view! { cx,
        //  it renders but it doesn't log when clicked
        {data_from_handler_view}

        //  same as above
        <For
            each=move || data_from_route_handler.clone()
            key = |n| n.clone()
            view=|cx, item| {
                view! {cx,
                    <div on:click=move |_| {
                        log!("data_from_route_handler on:click");
                    }>
                        "data_from_route_handler"
                        <ChildComponent/>
                    </div>
                }
            }
            />
    }
}

#[component]
pub fn ChildComponent(cx: Scope) -> impl IntoView {
    view! {cx,
        <div on:click=move |_| {
            log!("ChildComponent on:click");
        }>"ChildComponent"</div>
    }
}
