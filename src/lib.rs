
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element};

#[wasm_bindgen(start)]
pub fn go() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global window");
    let document = window.document().expect("window has no document");
    let body = document.body().expect("document has no body");

    let title = {
        let title = document.create_element("h1")?;
        title.set_inner_html("Calendar");
        title
    };
    body.append_child(&title)?;

    let container = {
        let container = document.create_element("div")?;
        container.set_attribute("width", "1000")?;
        container.set_attribute("height", "1000")?;

        let svg = svg(&document)?;
        container.append_child(&svg)?;

        container
    };
    body.append_child(&container)?;

    Ok(())
}

fn svg(document: &Document) -> Result<Element, JsValue>
{
    const XMLNS: &'static str = "http://www.w3.org/2000/svg";

    let svg = document.create_element_ns(Some(XMLNS), "svg")?;
    svg.set_attribute_ns(None, "viewBox", "0 0 1000 1000")?;
    svg.set_attribute_ns(None, "width", "1000")?;
    svg.set_attribute_ns(None, "height", "1000")?;
    //svg.style.display = "block";
    svg.set_attribute_ns(None, "id", "calendar")?;
    svg.set_attribute_ns(None, "version", "1.1")?;

    let circle = document.create_element_ns(Some(XMLNS), "circle")?;
    circle.set_attribute_ns(None, "cx", "500")?;
    circle.set_attribute_ns(None, "cy", "500")?;
    circle.set_attribute_ns(None, "r", "499")?;
    circle.set_attribute_ns(None, "stroke", "black")?;
    circle.set_attribute_ns(None, "stroke-width", "1")?;
    circle.set_attribute_ns(None, "fill", "lightyellow")?;
    circle.set_attribute_ns(None, "id", "maincircle")?;
    svg.append_child(&circle)?;

    Ok(svg)
}
