use web_sys::wasm_bindgen::{JsValue, JsCast};
use web_sys::{js_sys, Blob, ClipboardItem};

#[derive(Clone, Debug)]
pub struct Item {
    pub mime: String,
    pub js_typeof: String,
    pub blob: Blob,
}

pub async fn read_clipboard_items() -> Result<Vec<Item>, JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
    let clipboard = window.navigator().clipboard();
    let items_promise = clipboard.read();
    let items_js = wasm_bindgen_futures::JsFuture::from(items_promise).await?;
    let items: js_sys::Array = items_js.unchecked_into();

    let mut result = Vec::with_capacity(items.length() as usize);

    for item in items.iter() {
        let clipboard_item: ClipboardItem = item.dyn_into()?;
        for t in clipboard_item.types().iter() {
            if let Some(mime) = t.as_string() {
                let blob_js = wasm_bindgen_futures::JsFuture::from(clipboard_item.get_type(&mime)).await?;
                let js_typeof = blob_js.js_typeof().as_string().unwrap_or_default();
                let blob: Blob = blob_js.unchecked_into();
                result.push(Item {
                    mime,
                    js_typeof,
                    blob,
                });
            }
        }
    }
    Ok(result)
}

