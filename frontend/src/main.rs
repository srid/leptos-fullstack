use console_error_panic_hook::set_once as set_panic_hook;
use leptos_fullstack_common::Thing;
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

    let thing = Thing::new("Hello from frontend".to_string());
    let thing_p = document
        .create_element("p")
        .expect("Failed to create <p> element");
    thing_p
        .append_child(document.create_text_node(&thing.browser_view()).as_ref())
        .expect("Failed to append text");
    body.append_child(&thing_p).expect("Failed to append <p>");

    let text_node = document.create_text_node("\nTo visit backend page, click: ");

    let alink = document
        .create_element("a")
        .expect("Failed to create <a> element");
    alink
        .set_attribute("href", "/hello")
        .expect("Failed to set href");
    alink.set_text_content(Some("/hello"));
    body.append_child(text_node.as_ref())
        .expect("Failed to append text");
    body.append_child(alink.as_ref())
        .expect("Failed to append link");
}
