use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Thing {
    pub id: uuid::Uuid,
    pub text: String,
}

impl Thing {
    pub fn new(text: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            text,
        }
    }
    /// How this thing is displayed in the browser
    pub fn browser_view(&self) -> String {
        format!("Thing({}): {}", self.id, self.text)
    }
}

#[server(ReadThings, "/api", "Url", "read-things")]
pub async fn read_things() -> Result<Vec<Thing>, leptos::ServerFnError> {
    panic!("server fn called on frontend")
}
