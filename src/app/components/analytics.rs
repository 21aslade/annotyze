use super::Summary;
use annotyze::{analytics::Study, Annotation};
use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct AnnotSet {
    pub annotations: Vec<Annotation>,
    pub study: Study,
}

#[component]
pub fn Analytics<'a>(annotations: &'a AnnotSet) -> impl IntoView + use<> {
    view! {
        <div class="border p-5 min-w-2/3 flex flex-col">
            <div class="w-1/2">
                <Summary item=&annotations.study />
            </div>
        </div>
    }
}
