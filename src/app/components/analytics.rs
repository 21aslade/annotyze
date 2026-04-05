use super::Summary;
use super::TimeChart;
use annotyze::analytics::progress::BoMProgress;
use annotyze::{analytics::Study, Annotation};
use chrono::Local;
use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct AnnotSet {
    pub annotations: Vec<Annotation>,
    pub study: Study,
}

#[component]
pub fn Analytics<'a>(annotations: &'a AnnotSet) -> impl IntoView + use<> {
    let bom_timestamps = annotations
        .annotations
        .iter()
        .filter_map(|a| {
            Some((
                a.created.with_timezone(&Local),
                BoMProgress::from_url(a.url.as_ref()?)?,
            ))
        })
        .collect();

    view! {
        <div class="border p-5 min-w-2/3 flex gap-x-2">
            <div class="w-1/2 flex flex-col">
                <TimeChart annotations=bom_timestamps />
            </div>
            <div class="w-1/2">
                <Summary item=&annotations.study />
            </div>
        </div>
    }
}
