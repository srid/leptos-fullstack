use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::thing::{read_things, ReadThings, Thing};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/leptos-fullstack.css"/>
        <Router fallback=|cx| {
            view! { cx, <NotFound/> }
        }>

            <div class="flex flex-col items-center justify-center min-h-screen bg-blue-300">
                <div class="flex flex-col items-center justify-start px-4 py-8 mx-auto bg-white border-4 rounded-lg">
                    <Routes>
                        <Route path="" view=Home/>
                        <Route path="/things" view=Things>
                            <Route path="" view=ThingsList/>
                            <Route path=":id" view=ThingView/>
                        </Route>
                        <Route path="/about" view=About/>
                    </Routes>
                </div>
            </div>
        </Router>
    }
}

#[component]
fn About(cx: Scope) -> impl IntoView {
    view! { cx,
        <p>
            This is about page.
        </p>
        <p>
            Go back to
            <Link link="/" text="home"/>
            .
        </p>
    }
}
#[component]
fn Home(cx: Scope) -> impl IntoView {
    let thing = Thing::new(0, "Hello from frontend".to_string());
    let things = create_resource(cx, move || (), move |_| read_things());
    view! { cx,
        <Header1 text="Welcome to leptos-fullstack template"/>
        <div class="items-left">
            <Header2 text="Frontend"/>
            <p class="my-1">"This value ⤵️ is generated in-browser:"</p>
            <pre>{thing}</pre>
            <Header2 text="Backend"/>
            <p class="my-1">
                "These values ⤵️ are generated in-server (via server functions):"
            </p>
            <div>
                <Link link="/things" text="Things page"/>
            </div>

            <Link link="/hello" text="request backend /hello API" rel="external"/>
            <div>
                <Link link="/sdf" text="broken link"/>
            </div>
            <div>
                <Link link="/about" text="About page"/>
            </div>
            <Counter/>
        </div>
    }
}

#[component]
fn Things(cx: Scope) -> impl IntoView {
    view! { cx,
        <Header1 text="Things"/>
        <div class="items-left">
            <Header2 text="create_resource on Thing"/>
            <Outlet/>
        </div>
    }
}

#[component]
fn ThingsList(cx: Scope) -> impl IntoView {
    let things = create_resource(cx, move || (), move |_| read_things());
    view! { cx,
        <h2>"Thing List"</h2>
        <SuspenseWithErrorHandling>
            {move || {
                things
                    .read(cx)
                    .map(move |v| {
                        v.map(|things| {
                            log!("things: {:?}", things);
                            things
                                .into_iter()
                                .map(move |thing| {

                                    view! { cx,
                                        <li>
                                            <a href=format!("/things/{}", thing.id)>{thing}</a>
                                        </li>
                                    }
                                })
                                .collect_view(cx)
                        })
                    })
            }}

        </SuspenseWithErrorHandling>

        <div>
            <Link link="/" text="Main page"/>
        </div>
        <div>
            <Link link="/things/other" text="Bug Other"/>
        </div>
    }
}

#[component]
fn ThingView(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    let things = create_resource(cx, move || (), move |_| read_things());
    view! { cx,
        <h2>"Thing: " {id}</h2>
        <SuspenseWithErrorHandling>
            {move || {
                let id: u16 = id().parse::<u16>().unwrap();
                things
                    .read(cx)
                    .map(move |v| {
                        v.map(|things| {
                            log!("things: {:?}", things);
                            things
                                .into_iter()
                                .find(|thing| {
                                    log!("thing.id: {:?} == id: {:?}", thing.id, id);
                                    thing.id == id
                                })
                                .map(move |thing| {

                                    view! { cx, <li>{thing}</li> }
                                })
                                .collect_view(cx)
                        })
                    })
            }}

        </SuspenseWithErrorHandling>

        <div>
            <Link link="/" text="Main page"/>
        </div>
        <div>
            <Link link="/things" text="Things Home"/>
        </div>
    }
}

#[component]
fn Link(
    cx: Scope,
    link: &'static str,
    text: &'static str,
    #[prop(optional)] rel: Option<&'static str>,
) -> impl IntoView {
    view! { cx,
        <a href=link class="text-red-500 underline hover:no-underline" rel=rel>
            {text}
        </a>
    }
}

#[component]
fn Header1(cx: Scope, text: &'static str) -> impl IntoView {
    view! { cx, <h1 class="my-3 text-3xl font-bold">{text}</h1> }
}
#[component]
fn Header2(cx: Scope, text: &'static str) -> impl IntoView {
    view! { cx, <h2 class="my-2 text-2xl font-bold text-gray-600">{text}</h2> }
}

#[component]
fn NotFound(cx: Scope) -> impl IntoView {
    cfg_if! { if #[cfg(feature="ssr")] {
        use http::status::StatusCode;
        use leptos_axum::ResponseOptions;
        if let Some(response) = use_context::<ResponseOptions>(cx) {
            response.set_status(StatusCode::NOT_FOUND);
        }
    }}
    view! { cx,
        <div class="flex flex-row justify-center text-3xl text-red-500">"404: Page not found"</div>
    }
}

/// Renders the home page of your application.
#[component]
fn Counter(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <div class="mx-auto my-8 text-center md:container">
            <Header2 text="Leptops Counter"/>
            <button
                class="p-4 border-2 rounded-full shadow-lg active:shadow-none bg-blue-50 hover:bg-blue-200 active:bg-blue-500"
                on:click=on_click
            >
                "Click Me: "
                {count}
            </button>
        </div>
    }
}

/// Like [Suspense] but also handles errors using [ErrorBoundary]
#[component(transparent)]
pub fn SuspenseWithErrorHandling(cx: Scope, children: ChildrenFn) -> impl IntoView {
    let children = store_value(cx, children);
    view! { cx,
        <Suspense fallback=move || view! { cx, <Spinner/> }>
            <ErrorBoundary fallback=|cx, errors| {
                view! { cx, <Errors errors=errors.get()/> }
            }>{children.with_value(|c| c(cx))}</ErrorBoundary>
        </Suspense>
    }
}

/// Display errors to the user
#[component]
pub fn Errors(cx: Scope, errors: Errors) -> impl IntoView {
    tracing::error!("Errors: {:?}", errors);
    view! { cx,
        <div class="flex flex-row justify-center overflow-auto text-xl text-white bg-error-500">
            <div class="font-mono whitespace-pre-wrap">
                <ul>
                    {errors
                        .into_iter()
                        .map(|(_, e)| view! { cx, <li>{e.to_string()}</li> })
                        .collect_view(cx)}
                </ul>
            </div>
        </div>
    }
}

// A loading spinner
#[component]
pub fn Spinner(cx: Scope) -> impl IntoView {
    view! { cx,
        <div
            class="animate-spin inline-block w-6 h-6 border-[3px] border-current border-t-transparent text-blue-600 rounded-full"
            role="status"
            aria-label="loading"
        >
            <span class="sr-only">"Loading..."</span>
        </div>
    }
}
