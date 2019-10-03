
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element};
use chrono::prelude::*;
use std::panic;

const SIZE: &'static str = "700";
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
        container.set_attribute("width", SIZE)?;
        container.set_attribute("height", SIZE)?;

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
    svg.set_attribute_ns(None, "width", SIZE)?;
    svg.set_attribute_ns(None, "height", SIZE)?;
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
    for month in 1..12+1 {
        let (x,y) = calpoint(Local.ymd(now.year(), month, 1).and_hms(0,0,0));
        let stroke_width = match month {
            3 | 6 | 9 | 12 => "3",
            _ => "1",
        };
        let line = svg_line(document, x, y, 500.0, 500.0, "black", stroke_width)?;
        svg.append_child(&line)?;
    }

    let jan = svg_text(document, 490.0, 20.0, "Jan")?;
    svg.append_child(&jan)?;
    let feb = svg_text(document, 729.0, 93.0, "Feb")?;
    svg.append_child(&feb)?;
    let mar = svg_text(document, 895.0, 270.0, "Mar")?;
    svg.append_child(&mar)?;
    let apr = svg_text(document, 960.0, 500.0, "Apr")?;
    svg.append_child(&apr)?;
    let may = svg_text(document, 895.0, 730.0, "May")?;
    svg.append_child(&may)?;
    let jun = svg_text(document, 729.0, 907.0, "Jun")?;
    svg.append_child(&jun)?;
    let jul = svg_text(document, 490.0, 980.0, "Jul")?;
    svg.append_child(&jul)?;
    let aug = svg_text(document, 241.0, 907.0, "Aug")?;
    svg.append_child(&aug)?;
    let sep = svg_text(document, 75.0, 730.0, "Sep")?;
    svg.append_child(&sep)?;
    let oct = svg_text(document, 10.0, 500.0, "Oct")?;
    svg.append_child(&oct)?;
    let nov = svg_text(document, 75.0, 270.0, "Nov")?;
    svg.append_child(&nov)?;
    let dec = svg_text(document, 241.0, 93.0, "Dec")?;
    svg.append_child(&dec)?;

    let summer = svg_text(document, 395.0, 270.0, "Summer")?;
    summer.set_attribute_ns(None, "font-size", "48")?;
    summer.set_attribute_ns(None, "stroke", "black")?;
    summer.set_attribute_ns(None, "fill", "gold")?;
    svg.append_child(&summer)?;

    let winter = svg_text(document, 420.0, 760.0, "Winter")?;
    winter.set_attribute_ns(None, "font-size", "48")?;
    winter.set_attribute_ns(None, "stroke", "black")?;
    winter.set_attribute_ns(None, "fill", "white")?;
    svg.append_child(&winter)?;

    let spring = svg_text(document, 180.0, 515.0, "Spring")?;
    spring.set_attribute_ns(None, "font-size", "48")?;
    spring.set_attribute_ns(None, "stroke", "black")?;
    spring.set_attribute_ns(None, "fill", "green")?;
    svg.append_child(&spring)?;

    let autumn = svg_text(document, 645.0, 515.0, "Autumn")?;
    autumn.set_attribute_ns(None, "font-size", "48")?;
    autumn.set_attribute_ns(None, "stroke", "black")?;
    autumn.set_attribute_ns(None, "fill", "brown")?;
    svg.append_child(&autumn)?;

    let (nowx, nowy) = calpoint(now);
    let nowline = svg_line(document, nowx, nowy, 500.0, 500.0,
                           "blue", "3")?;
    svg.append_child(&nowline)?;

    let (sx, sy) = calpoint(summer_solstice(now.year()));
    let ssline = svg_line(document, sx, sy, 500.0, 500.0, "brown", "1")?;
    svg.append_child(&ssline)?;
    let (wx, wy) = calpoint(winter_solstice(now.year()));
    let wsline = svg_line(document, wx, wy, 500.0, 500.0, "brown", "1")?;
    svg.append_child(&wsline)?;
    let (sx, sy) = calpoint(spring_equinox(now.year()));
    let seline = svg_line(document, sx, sy, 500.0, 500.0, "brown", "1")?;
    svg.append_child(&seline)?;
    let (fx, fy) = calpoint(autumn_equinox(now.year()));
    let feline = svg_line(document, fx, fy, 500.0, 500.0, "brown", "1")?;
    svg.append_child(&feline)?;


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

fn svg_text(document: &Document, x: f32, y: f32, text: &str)
            -> Result<Element, JsValue>
{
    let elem = document.create_element_ns(Some(XMLNS), "text")?;
    elem.set_attribute_ns(None, "x", &*format!("{}", x))?;
    elem.set_attribute_ns(None, "y", &*format!("{}", y))?;
    elem.set_inner_html(text);
    Ok(elem)
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

fn spring_equinox(year: i32) -> DateTime<Local> {
    match year {
        2019 => Local.ymd(year,9,23).and_hms(7,50,0),
        2020 => Local.ymd(year,9,22).and_hms(13,31,0),
        2021 => Local.ymd(year,9,22).and_hms(19,21,0),
        2022 => Local.ymd(year,9,23).and_hms(1,4,0),
        2023 => Local.ymd(year,9,23).and_hms(6,50,0),
        2024 => Local.ymd(year,9,22).and_hms(12,44,0),
        _ => Local.ymd(year,9,23).and_hms(0,0,0), // approximation
    }
}

fn autumn_equinox(year: i32) -> DateTime<Local> {
    match year {
        2019 => Local.ymd(year,3,20).and_hms(21,58,0),
        2020 => Local.ymd(year,3,20).and_hms(3,50,0),
        2021 => Local.ymd(year,3,20).and_hms(9,37,0),
        2022 => Local.ymd(year,3,20).and_hms(15,33,0),
        2023 => Local.ymd(year,3,20).and_hms(21,24,0),
        2024 => Local.ymd(year,3,20).and_hms(3,7,0),
        _ => Local.ymd(year,3,21).and_hms(0,0,0), // approximation
    }
}

fn summer_solstice(year: i32) -> DateTime<Local> {
    match year {
        2019 => Local.ymd(year,12,22).and_hms(4,19,0),
        2020 => Local.ymd(year,12,21).and_hms(10,02,0),
        2021 => Local.ymd(year,12,21).and_hms(15,59,0),
        2022 => Local.ymd(year,12,21).and_hms(21,48,0),
        2023 => Local.ymd(year,12,22).and_hms(3,27,0),
        2024 => Local.ymd(year,12,21).and_hms(9,20,0),
        _ => Local.ymd(year,12,21).and_hms(0,0,0),
    }
}

fn winter_solstice(year: i32) -> DateTime<Local> {
    match year {
        2019 => Local.ymd(year,6,21).and_hms(15,54,0),
        2020 => Local.ymd(year,6,20).and_hms(21,44,0),
        2021 => Local.ymd(year,6,21).and_hms(3,32,0),
        2022 => Local.ymd(year,6,21).and_hms(9,14,0),
        2023 => Local.ymd(year,6,21).and_hms(14,58,0),
        2024 => Local.ymd(year,6,20).and_hms(20,51,0),
        _ => Local.ymd(year,6,21).and_hms(0,0,0),
    }
}
