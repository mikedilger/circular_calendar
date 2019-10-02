
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn go() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global window");
    let document = window.document().expect("window has no document");
    let body = document.body().expect("document has no body");

    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");
    body.append_child(&val)?;
    Ok(())
}
