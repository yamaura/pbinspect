use leptos::prelude::*;
use leptos::task::spawn_local;

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
                        .into_iter()
                        .map(|data| {
                            view! {
                                <tr>
                                    <td style="vertical-align: top;">{data.mime}</td>
                                    <td>{data.js_typeof}</td>
                                    <td>
                                        <pre>{data.data}</pre>
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

        spawn_local(async move {
            let items = pbinspect::read_clipboard_items().await.unwrap();

            for item in items.into_iter() {
                let blob = item.blob.into();
                let data = gloo_file::futures::read_as_bytes(&blob).await.unwrap();
                let text = String::from_utf8(data.to_vec()).unwrap_or_else(|_| "Binary Data".to_string());

                set_clipboard_data.update(|data| {
                    data.push(Item {
                        mime: item.mime,
                        js_typeof: item.js_typeof,
                        data: text,
                    })
                });
            }
        });
    };

    view! {
        <div>
            <input type="text" on:paste=inspect_clipboard placeholder="paste here!" />

            <div>
                <h3>{"Clipboard Data: "}</h3>
                <div>
                    <DataTable datas=clipboard_data />
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
    mount_to_body(ClipboardInspector);
}
