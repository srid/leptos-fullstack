use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::thing::{read_things, Thing};

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
    view! { cx,
        <Header1>"Welcome to leptos-fullstack template"</Header1>
        <div class="items-left">
            <Header2>"Frontend"</Header2>
            <p class="my-1">"This value ⤵️ is generated in-browser:"</p>
            <pre>{thing}</pre>
            <Header2>"Backend"</Header2>
            <p class="my-1">
                "These values ⤵️ are generated in-server (via server functions):"
            </p>
            <div>
                <Link link="/things" text="Things page"/>
            </div>

            <Header2>"Links"</Header2>
            <Link link="/hello" text="request backend /hello API" rel="external"/>
            <div>
                <Link link="/sdf" text="broken link"/>
            </div>
            <div>
                <Link link="/about" text="About page"/>
            </div>
        </div>
    }
}

#[component]
fn Things(cx: Scope) -> impl IntoView {
    view! { cx,
        <Header1>"Things"</Header1>
        <div class="items-left">
            <Outlet/>
        </div>
    }
}

#[component]
fn ThingsNav(cx: Scope) -> impl IntoView {
    let things = create_resource(cx, move || (), move |_| read_things());
    view! { cx,
        <ul>
            <SuspenseWithErrorHandling>
                {move || {
                    things
                        .read(cx)
                        .map(move |v| {
                            v.map(|things| {
                                things
                                    .into_iter()
                                    .map(move |thing| {

                                        view! { cx,
                                            <li>
                                                <a
                                                    class="p-2 m-2 underline hover:bg-blue-200"
                                                    href=format!("/things/{}", thing.id)
                                                >
                                                    "Thing "
                                                    {thing.id}
                                                </a>
                                            </li>
                                        }
                                    })
                                    .collect_view(cx)
                            })
                        })
                }}

            </SuspenseWithErrorHandling>
        </ul>
    }
}

#[component]
fn ThingsList(cx: Scope) -> impl IntoView {
    view! { cx,
        <Header2>"List of things"</Header2>

        <ThingsNav/>
        <div>
            <Link link="/" text="Main page"/>
        </div>
    }
}

#[component]
fn ThingView(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    let things = create_resource(cx, move || (), move |_| read_things());
    view! { cx,
        <ThingsNav/>
        <Header2>"Thing: " {id}</Header2>
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
                                .find(|thing| thing.id == id)
                                .map(move |thing| {

                                    view! { cx, <li>{thing}</li> }
                                })
                                .collect_view(cx)
                        })
                    })
            }}

        </SuspenseWithErrorHandling>

        <div>
            <Link link="/things" text="Things Home"/>
        </div>
        <div>
            <Link link="/" text="Main page"/>
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
fn Header1(cx: Scope, children: ChildrenFn) -> impl IntoView {
    view! { cx, <h1 class="my-3 text-3xl font-bold">{children(cx)}</h1> }
}
#[component]
fn Header2(cx: Scope, children: ChildrenFn) -> impl IntoView {
    view! { cx, <h2 class="my-2 text-2xl font-bold text-gray-600">{children(cx)}</h2> }
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
    error!("Errors: {:?}", errors);
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
