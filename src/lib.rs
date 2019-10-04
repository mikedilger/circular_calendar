
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element};
use chrono::prelude::*;
use std::panic;

const SIZE: &'static str = "800";

/* Summer Solstice
const TOP_MONTH: u32 = 12;
const TOP_DAY: u32 = 22;
const TOP_HOUR: u32 = 4;
const TOP_MINUTE: u32 = 19;
const TOP_SECOND: u32 = 0;
*/

const TOP_MONTH: u32 = 1;
const TOP_DAY: u32 = 1;
const TOP_HOUR: u32 = 0;
const TOP_MINUTE: u32 = 0;
const TOP_SECOND: u32 = 0;

const XMLNS: &'static str = "http://www.w3.org/2000/svg";

#[wasm_bindgen(start)]
pub fn go() -> Result<(), JsValue> {

    // Panic to the console
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    // Set Top date

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

    // Main circle
    let circle = document.create_element_ns(Some(XMLNS), "circle")?;
    circle.set_attribute_ns(None, "cx", "500")?;
    circle.set_attribute_ns(None, "cy", "500")?;
    circle.set_attribute_ns(None, "r", "499")?;
    circle.set_attribute_ns(None, "stroke", "black")?;
    circle.set_attribute_ns(None, "stroke-width", "3")?;
    circle.set_attribute_ns(None, "fill", "lightyellow")?;
    circle.set_attribute_ns(None, "id", "maincircle")?;
    svg.append_child(&circle)?;

    // Month separator lines
    let now = Local::now();
    for month in 1..12+1 {
        let (x,y) = calpoint(Local.ymd(now.year(), month, 1).and_hms(0,0,0));
        let stroke_width = match month {
            3 | 6 | 9 | 12 => "3",
            _ => "1",
        };
        let line = svg_line(document, x, y, 500.0, 500.0, "#808080", stroke_width)?;
        line.set_attribute_ns(None, "stroke-dasharray", "12,6")?;
        svg.append_child(&line)?;
    }

    // Mark Solstices and Equinoxes
    for (x,y) in &[calpoint(summer_solstice(now.year())),
                   calpoint(winter_solstice(now.year())),
                   calpoint(spring_equinox(now.year())),
                   calpoint(autumn_equinox(now.year()))]
    {
        let line = svg_line(document, *x, *y, 500.0, 500.0, "brown", "1")?;
        //line.set_attribute_ns(None, "stroke-dasharray", "6,12")?;
        svg.append_child(&line)?;
    }

    // Label months
    for (txt,mon,day,hour) in &[("Jan", 1, 16, 12),
                                ("Feb", 2, 15, 12),
                                ("Mar", 3, 16, 12),
                                ("Apr", 4, 16, 0),
                                ("May", 5, 16, 12),
                                ("Jun", 6, 16, 0),
                                ("Jul", 7, 16, 12),
                                ("Aug", 8, 16, 12),
                                ("Sep", 9, 16, 0),
                                ("Oct", 10, 16, 12),
                                ("Nov", 11, 16, 0),
                                ("Dec", 12, 16, 12)]
    {
        let mpoint = calpoint(Local.ymd(now.year(), *mon, *day).and_hms(*hour,0,0));
        let (x,y) = txtpoint(mpoint, 0.08, txt, 18.0);
        let m = svg_text(document, x, y, txt)?;
        svg.append_child(&m)?;
    }

    // Label seasons
    for (txt,mon,day,hour,color) in &[("Summer", 1, 16, 12, "gold"),
                                      ("Autumn", 4, 16, 0, "brown"),
                                      ("Winter", 7, 16, 12, "white"),
                                      ("Spring", 10, 16, 12, "green")]
    {
        let mpoint = calpoint(Local.ymd(now.year(), *mon, *day).and_hms(*hour,0,0));
        let (mut x,y) = txtpoint(mpoint, 0.5, txt, 48.0);
        if *mon==1 { x = x - 18.0; } // "Summer" m's are wide, special case adjustment.
        let m = svg_text(document, x, y, txt)?;
        m.set_attribute_ns(None, "font-size", "48")?;
        m.set_attribute_ns(None, "stroke", "#404040")?;
        m.set_attribute_ns(None, "fill", color)?;
        svg.append_child(&m)?;
    }

    // Dial pointing to now
    let (nowx, nowy) = calpoint(now);
    let nowline = svg_line(document, nowx, nowy, 500.0, 500.0,
                           "blue", "3")?;
    svg.append_child(&nowline)?;

    // Cover the center
    let cover = document.create_element_ns(Some(XMLNS), "circle")?;
    cover.set_attribute_ns(None, "cx", "500")?;
    cover.set_attribute_ns(None, "cy", "500")?;
    cover.set_attribute_ns(None, "r", "60")?;
    cover.set_attribute_ns(None, "stroke", "black")?;
    cover.set_attribute_ns(None, "stroke-width", "3")?;
    cover.set_attribute_ns(None, "fill", "#505050")?;
    cover.set_attribute_ns(None, "id", "covercircle")?;
    svg.append_child(&cover)?;

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

fn get_ratio(from: DateTime<Local>, to: DateTime<Local>) -> f32 {
    let year = to.year();
    let seconds_so_far: i64 = to.signed_duration_since(from).num_seconds();
    let total_seconds = {
        let days_this_year: i64 = Local.ymd(year+1,1,1).and_hms(0,0,0)
            .signed_duration_since( Local.ymd(year,1,1).and_hms(0,0,0) )
            .num_days();
        60*60*24*days_this_year
    };
    (seconds_so_far as f32) / (total_seconds as f32)
}

fn calpoint(now: DateTime<Local>) -> (f32, f32) {
    let start = Local.ymd(now.year(),1,1).and_hms(0,0,0);
    let ratio = get_ratio(start, now);
    let rotate_fraction = {
        let top = Local.ymd(now.year(),TOP_MONTH,TOP_DAY).and_hms(TOP_HOUR,TOP_MINUTE,TOP_SECOND);
        get_ratio(start, top)
    };
    let angle = (ratio - rotate_fraction) * 2.0 * std::f32::consts::PI;
    (500.0 + angle.sin()*500.0, 500.0 - angle.cos()*500.0)
}

fn txtpoint(calpoint: (f32, f32), percent_inward: f32, text: &str, fontsize: f32) -> (f32, f32) {
    let ctrx = calpoint.0 - ((calpoint.0 - 500.0) * percent_inward);
    let ctry = calpoint.1 - ((calpoint.1 - 500.0) * percent_inward);
    let halfchars = text.len() as f32 / 2.0;
    let fontpx = fontsize * 0.5; // guess
    let x = ctrx - halfchars*fontpx;
    let y = ctry + fontpx/2.0;
    (x,y)
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
