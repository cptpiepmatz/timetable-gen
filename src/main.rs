use crate::config::Config;
use crate::head::TimetableHead;
use crate::size::A4Landscape;
use crate::table::TimetableTable;
use genpdf::elements::{Paragraph, TableLayout, Text};
use genpdf::{Document, SimplePageDecorator};
use std::fs;
use std::path::PathBuf;
use clap::Parser;

mod config;
mod font;
mod head;
mod size;
mod table;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    // Path to config file
    #[arg(short, long, default_value = "timetable.toml")]
    config: PathBuf,

    // Path where the output should be generated
    #[arg(short, long, default_value = "timetable.pdf")]
    out: PathBuf
}

fn main() {
    let args = Args::parse();

    let config = fs::read_to_string(args.config).expect("to read file");
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

    if let Some(dirname) = args.out.parent() {
        fs::create_dir_all(dirname).expect("to create needed directories");
    }
    doc.render_to_file(args.out).expect("to write pdf file");
}
