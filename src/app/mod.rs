use annotyze::{
    analytics::{LibraryItem, Study},
    Annotation,
};
use leptos::prelude::*;

mod components;

use components::{LoadFile, Summary};

#[derive(Clone, Debug)]
struct AnnotSet {
    annotations: Vec<Annotation>,
    study: Study,
}

#[component]
pub fn App() -> impl IntoView {
    let (annotations, set_annotations) = signal::<Option<AnnotSet>>(None);
    let on_load = move |file: String| {
        let mut annotations = vec![];
        let mut study = Study::default();

        let mut reader = csv::Reader::from_reader(file.as_bytes());
        for record in reader.deserialize() {
            let annotation: Annotation = record.expect("csv parsing failed");
            if let Some(url) = &annotation.url {
                let (_, url) = url
                    .split_once("churchofjesuschrist.org/study/")
                    .unwrap_or(("", url));
                let url = url.split('/').flat_map(|u| u.split('?'));
                study.insert(url).expect("Failed to insert study item");
            }
            annotations.push(annotation);
        }

        set_annotations.set(Some(AnnotSet { annotations, study }))
    };
    view! {
        <LoadFile on_load=on_load />
        {move || annotations.with(|a| a.as_ref().map(|a| view! { <Summary item=&a.study /> }))}
    }
}
