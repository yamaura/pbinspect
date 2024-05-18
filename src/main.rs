use leptos::*;

use web_sys::{window, Clipboard, ClipboardItem, Blob};
use web_sys::js_sys;
use wasm_bindgen::JsCast;

#[derive(Clone, Debug)]
pub struct Item {
    pub mime: String,
    pub js_typeof: String,
    pub data: String,
}

#[component]
fn DataTable(datas: ReadSignal<Vec<Item>>) -> impl IntoView {
    view! {
        <table>
            <thead>
                <tr>
                    <th>{"MIME"}</th>
                    <th>{"typeof"}</th>
                    <th>{"DATA"}</th>
                </tr>
            </thead>
            <tbody>
                {move || {
                    datas
                        .get()
                        .iter()
                        .map(|data| {
                            view! {
                                <tr>
                                    <td style="vertical-align: top;">{&data.mime}</td>
                                    <td>{&data.js_typeof}</td>
                                    <td>
                                        <pre>{&data.data}</pre>
                                    </td>
                                </tr>
                            }
                        })
                        .collect_view()
                }}
            </tbody>
        </table>
    }
}

#[component]
fn ClipboardInspector() -> impl IntoView {
    let (clipboard_data, set_clipboard_data) = create_signal(Vec::new());

    let inspect_clipboard = move |_| {
        set_clipboard_data.set(Vec::new());

        let window = window().unwrap();
        let clipboard: Clipboard = window.navigator().clipboard().unwrap();

        spawn_local(async move {
            let items = clipboard.read();
            let items = wasm_bindgen_futures::JsFuture::from(items).await.unwrap();
            let items: js_sys::Array = items.unchecked_into();

            for item in items.iter() {
                let clipboard_item: ClipboardItem = item.dyn_into().unwrap();
                let types = clipboard_item.types();

                for type_ in types.iter() {
                if let Some(mime) = type_.as_string() {
                    let blob = clipboard_item.get_type(&mime);
                    let blob = wasm_bindgen_futures::JsFuture::from(blob)
                        .await
                        .unwrap();
                    let js_typeof = blob.js_typeof().as_string().unwrap();
                    let blob = blob.unchecked_into::<Blob>();
                    let text = wasm_bindgen_futures::JsFuture::from(blob.text())
                        .await
                        .unwrap();

                    set_clipboard_data.update(|data| {
                        data.push(
                            Item{
                                mime: mime.clone(),
                                js_typeof,
                                data: text.as_string().unwrap_or("N/A".to_string()),
                            }
                        )
                    });
                }
                }
            }
        });
    };

    view! {
        <div>
            <input type="text" on:paste=inspect_clipboard placeholder="paste here!"/>

            <div>
                <h3>{"Clipboard Data: "}</h3>
                <div>
                    <DataTable datas=clipboard_data/>
                </div>
            </div>
            <div>
                <h3>{"Reference"}</h3>
                <p>
                    <a
                        href="https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/renderer/modules/clipboard/clipboard_reader.cc"
                        target="_blank"
                    >
                        {"Chromium Clipboard Reader Source"}
                    </a>
                </p>
            </div>
        </div>
    }
}

fn main() {
    leptos::mount_to_body(ClipboardInspector);
}
