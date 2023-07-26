use leptos::*;
use leptos_fullstack_common::Thing;
use leptos_meta::*;
use leptos_router::*;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    // TODO: ssr
    log!("csr mode - mounting to body");
    mount_to_body(|cx| {
        view! { cx, <App /> }
    });
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
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
    view! { cx,
        <h1>"Welcome to leptos-fullstack template"</h1>
        <p>
            <b>Frontend</b> value: <pre>{thing.browser_view()}</pre>
        </p>
        <p>
            <b>Backend</b> value: <Link link="/hello" text="fetch backend /hello API" />
        </p>
    }
}

#[component]
fn Link(cx: Scope, link: &'static str, text: &'static str) -> impl IntoView {
    view! {cx,
        <a href=link target="_blank" >{text}</a>
    }
}
