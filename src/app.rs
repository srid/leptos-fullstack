use crate::thing::{read_things, ReadThings, Thing};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/leptos-fullstack.css"/>
        <Router>
            <Routes>
                <Route path="" view=  move |cx| view! { cx, <Home/> }/>
            </Routes>
        </Router>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    let thing = Thing::new("Hello from frontend".to_string());
    let things = create_local_resource(cx, move || (), move |_| read_things());
    view! { cx,
        <div class="flex flex-col items-center justify-center min-h-screen bg-red-600">
            <div class="flex flex-col items-center justify-start px-4 py-8 mx-auto bg-white border-4 rounded-lg ">
                <Header1 text="Welcome to leptos-fullstack template" />
                <div class="items-left">
                    <Header2 text="Frontend" />
                    <p class="my-1">"This value ⤵️ is generated in-browser:"</p>
                    <pre>{thing.browser_view()}</pre>
                    <Header2 text="Backend" />
                    <pre>fn_url: {ReadThings::url()}</pre>
                    {move || {
                        things.read(cx)
                            .map(move |things| match things {
                                Err(e) => {
                                view! { cx, <pre class="p-2 my-2 font-bold bg-red-200 shadow-lg">"Server Error: " {e.to_string()}</pre>}.into_view(cx)
                                }
                                Ok(things) => {
                                    things.into_iter().map(move |thing| {
                                        view! {
                                            cx,
                                            <li>{thing.browser_view()}</li>
                                        }
                                    }).collect_view(cx)
                                }
                            })
                    }}
                    <Link link="/hello" text="request backend /hello API" />
                    <Counter />
                </div>
            </div>
        </div>
    }
}

#[component]
fn Link(cx: Scope, link: &'static str, text: &'static str) -> impl IntoView {
    view! {cx,
        <a href=link target="_blank" class="text-red-500 underline hover:no-underline">{text}</a>
    }
}

#[component]
fn Header1(cx: Scope, text: &'static str) -> impl IntoView {
    view! {cx,
        <h1 class="my-3 text-3xl font-bold">{text}</h1>
    }
}
#[component]
fn Header2(cx: Scope, text: &'static str) -> impl IntoView {
    view! {cx,
        <h2 class="my-2 text-2xl font-bold text-gray-600">{text}</h2>
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
            <h1 class="mb-4 text-3xl font-bold text-pink-500">"Leptops Counter"</h1>
            <button
                class="p-4 border-2 rounded-full shadow-lg active:shadow-none bg-blue-50 hover:bg-blue-200 active:bg-blue-500"
                on:click=on_click>"Click Me: " {count}</button>
        </div>
    }
}
