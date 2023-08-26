use crate::config::Config;
use crate::head::TimetableHead;
use crate::size::A4Landscape;
use crate::table::TimetableTable;
use genpdf::elements::{Paragraph, TableLayout, Text};
use genpdf::{Document, SimplePageDecorator};
use std::fs;

mod config;
mod font;
mod head;
mod size;
mod table;

fn main() {
    let config = fs::read_to_string("timetable.toml").expect("to read file");
    let config: Config = toml::from_str(config.as_str()).expect("to parse config");

    let font_family = font::init_font_family();
    let mut doc = Document::new(font_family);
    doc.set_paper_size(A4Landscape);
    doc.set_title(config.title.to_string());

    let mut decorator = SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    doc.push(TimetableHead::new(&config));
    doc.push(TimetableTable::new(&config));

    doc.render_to_file("out.pdf").expect("to write pdf file");
}
