use console_error_panic_hook::set_once as set_panic_hook;
use web_sys::window;

pub fn main() {
    set_panic_hook();
    start_app();
}

fn start_app() {
    let document = window()
        .and_then(|win| win.document())
        .expect("Could not access document");
    let body = document.body().expect("Could not access document.body");
    let text_node = document.create_text_node("Hello, world from leptos-fullstack (vanilla Rust)!");
    body.append_child(text_node.as_ref())
        .expect("Failed to append text");
}
