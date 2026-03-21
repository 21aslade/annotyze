use leptos::prelude::*;

mod components;

use components::LoadFile;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <LoadFile />
    }
}
