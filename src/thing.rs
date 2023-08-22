use leptos::*;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Thing {
    pub id: u16,
    pub text: String,
}

impl Thing {
    pub fn new(id: u16, text: String) -> Self {
        Self { id, text }
    }
}

impl IntoView for Thing {
    fn into_view(self, cx: Scope) -> View {
        view! { cx, <span class="font-mono">{format!("Thing({}): {}", self.id, self.text)}</span> }
            .into_view(cx)
    }
}

#[server(ReadThings, "/api")]
pub async fn read_things() -> Result<Vec<Thing>, leptos::ServerFnError> {
    Ok(vec![
        Thing::new(1, "Hello 1 from backend".to_string()),
        Thing::new(2, "Hello 2 from backend".to_string()),
        Thing::new(3, "Hello 3 from backend".to_string()),
        // NOTE: env::current_dir() will not work on wasm backend
        // Thus, acting as proof that the server macro discards body when
        // compiling on frontend.
        Thing::new(4, format!("CWD: {}", env::current_dir().unwrap().display())),
    ])
}
