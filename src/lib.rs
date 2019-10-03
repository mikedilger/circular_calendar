
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element};
use chrono::prelude::*;
use std::panic;

const XMLNS: &'static str = "http://www.w3.org/2000/svg";

#[wasm_bindgen(start)]
pub fn go() -> Result<(), JsValue> {

    // Panic to the console
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let window = web_sys::window().expect("no global window");
    let document = window.document().expect("window has no document");
    let body = document.body().expect("document has no body");

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

    let now = Local::now();
    let (x,y) = calpoint(Local.ymd(now.year(), 1, 1).and_hms(0,0,0));
    let line = svg_line(document, x, y, 500.0, 500.0, "black", "1")?;
    svg.append_child(&line)?;

    Ok(svg)
}

fn svg_line(document: &Document, x: f32, y: f32, x2: f32, y2: f32,
            stroke: &str, stroke_width: &str)
            -> Result<Element, JsValue>
{
    let line = document.create_element_ns(Some(XMLNS), "line")?;
    line.set_attribute_ns(None, "x1", &*format!("{}", x))?;
    line.set_attribute_ns(None, "y1", &*format!("{}", y))?;
    line.set_attribute_ns(None, "x2", &*format!("{}", x2))?;
    line.set_attribute_ns(None, "y2", &*format!("{}", y2))?;
    line.set_attribute_ns(None, "stroke", stroke)?;
    line.set_attribute_ns(None, "stroke-width", stroke_width)?;
    Ok(line)
}

fn calpoint(now: DateTime<Local>) -> (f32, f32) {
    let year = now.year();
    let start = Local.ymd(year,1,1).and_hms(0,0,0);
    let seconds_so_far: i64 = now.signed_duration_since(start).num_seconds();
    let total_seconds = {
        let days_this_year: i64 = Local.ymd(year+1,1,1).and_hms(0,0,0)
            .signed_duration_since( Local.ymd(year,1,1).and_hms(0,0,0) )
            .num_days();
        60*60*24*days_this_year
    };
    let ratio = (seconds_so_far as f32) / (total_seconds as f32);
    let angle = (ratio - (1.0/24.0)) * 2.0 * std::f32::consts::PI;
    (500.0 + angle.sin()*500.0, 500.0 - angle.cos()*500.0)
}
