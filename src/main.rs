use crate::config::Config;
use crate::head::TimetableHead;
use crate::size::A4Landscape;
use crate::table::TimetableTable;

use clap::Parser;
use genpdf::{Document, SimplePageDecorator};
use std::fs;
use std::path::PathBuf;

mod config;
mod font;
mod head;
mod size;
mod table;

/// A command-line tool designed to simplify the creation of timetables.
///
/// It takes a configuration file in TOML format as input, processes the given data, and outputs a
/// neatly organized timetable in PDF format.
/// The output PDF will be in A4 landscape orientation, with the classes matrix fitted as needed
/// according to your configuration file.
#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    // Path to config file
    #[arg(short, long, default_value = "timetable.toml")]
    config: PathBuf,

    // Path where the output should be generated
    #[arg(short, long, default_value = "timetable.pdf")]
    out: PathBuf,
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
