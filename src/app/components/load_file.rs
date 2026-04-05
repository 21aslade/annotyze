use std::rc::Rc;

use leptos::task;
use leptos::{ev::Event, prelude::*, wasm_bindgen::JsCast};
use web_sys::{HtmlInputElement};
use web_sys::js_sys::{ArrayBuffer, Uint8Array};

#[component]
pub fn LoadFile(on_load: impl Fn(&'_ [u8]) + 'static) -> impl IntoView {
    let on_load = Rc::new(on_load);
    let handle_upload = move |ev: Event| {
        let input: HtmlInputElement = ev.target().unwrap().unchecked_into();
        let Some(files) = input.files() else { return };
        let Some(file) = files.get(0) else { return };
        let Ok(blob) = file.slice() else { return };
        let on_load = on_load.clone();
        task::spawn_local(async move {
            let buffer = blob.array_buffer().await
                .and_then(|b| b.dyn_into::<ArrayBuffer>())
                .expect("Expected an ArrayBuffer");
            on_load(&Uint8Array::new(&buffer).to_vec());
        });
    };

    view! {
        <div class="p-5 border-gb-fg border w-max flex flex-col items-center">
            <p>"Choose an annotation file to analyze:"</p>
            <input
                class="mt-2 file:bg-gb-blue file:rounded-xl file:p-1 file:mr-2"
                type="file"
                accept="text/csv"
                on:input=handle_upload
            />
        </div>
    }
}
