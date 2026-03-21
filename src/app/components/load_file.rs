use leptos::{ev::Event, prelude::*, wasm_bindgen::JsCast};
use web_sys::{Blob, HtmlInputElement};
use web_sys::js_sys::{ArrayBuffer, Uint8Array};

#[component]
pub fn LoadFile() -> impl IntoView {
    let (file, set_file) = signal::<Option<Blob>>(None);
    let text = LocalResource::new(
        move || {
            let file = file.get();
            async move {
                let buffer = file?.array_buffer().await
                    .and_then(|b| b.dyn_into::<ArrayBuffer>())
                    .expect("Expected an ArrayBuffer");
                let buffer = Uint8Array::new(&buffer);
                let mut file = vec![0; buffer.length() as usize];
                buffer.copy_to(&mut file);
                let file = String::from_utf8(file)
                    .expect("Invalid utf8");
                Some(file)
            }
        },
    );
    let handle_upload = move |ev: Event| {
        let input: HtmlInputElement = ev.target().unwrap().unchecked_into();
        let Some(files) = input.files() else { return };
        let Some(file) = files.get(0) else { return };
        let Ok(blob) = file.slice() else { return };
        set_file.set(Some(blob));
    };

    view! {
        <div>
            <p>"Choose an annotation file to analyze:"</p>
            <input type="file" accept="text/csv" on:input=handle_upload />
            {move || text.get()}
        </div>
    }
}
