use annotyze::analytics::LibraryItem;
use leptos::prelude::*;

#[component]
pub fn Summary<'a>(item: &'a dyn LibraryItem) -> impl IntoView + use<> {
    view! {
        <div>
            <p>{item.name()}" - "{item.count()}</p>
            {item.children().map(|c| view! { <Summary item=c /> }).collect::<Vec<_>>()}
        </div>
    }
}
