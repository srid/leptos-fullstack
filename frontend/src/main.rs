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
    let text_node = document.create_text_node("Hello! To visit backend page, click: ");
    let alink = document
        .create_element("a")
        .expect("Failed to create <a> element");
    alink
        .set_attribute("href", "/api")
        .expect("Failed to set href");
    alink.set_inner_html("/api");
    body.append_child(text_node.as_ref())
        .expect("Failed to append text");
    body.append_child(alink.as_ref())
        .expect("Failed to append link");
}
