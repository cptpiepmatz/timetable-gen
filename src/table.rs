use crate::config::{ClassEntry, Config, DayIdentifiers, GridSize};
use genpdf::elements::{FramedElement, TableLayout, Text};
use genpdf::error::Error;
use genpdf::render::Area;
use genpdf::style::Style;
use genpdf::{Context, Element, Mm, RenderResult};

#[derive(Debug, Clone)]
pub struct TimetableTable {
    class_durations: Vec<String>,
    day_identifiers: DayIdentifiers,

    monday: Vec<ClassEntry>,
    tuesday: Vec<ClassEntry>,
    wednesday: Vec<ClassEntry>,
    thursday: Vec<ClassEntry>,
    friday: Vec<ClassEntry>,
    saturday: Vec<ClassEntry>,
    sunday: Vec<ClassEntry>,

    grid_size: GridSize,
}

impl TimetableTable {
    pub fn new(config: &Config) -> Self {
        TimetableTable {
            class_durations: config.class_durations.clone(),
            day_identifiers: config.day_identifiers.clone(),

            monday: config.monday.clone(),
            tuesday: config.tuesday.clone(),
            wednesday: config.wednesday.clone(),
            thursday: config.thursday.clone(),
            friday: config.friday.clone(),
            saturday: config.saturday.clone(),
            sunday: config.sunday.clone(),

            grid_size: config.grid_size(),
        }
    }
}

impl Element for TimetableTable {
    fn render(
        &mut self,
        context: &Context,
        mut area: Area<'_>,
        style: Style,
    ) -> Result<RenderResult, Error> {
        let mut weights = Vec::with_capacity(8);
        weights.push(1);
        weights.resize(self.grid_size.columns + 1, 2);

        let mut table = TableLayout::new(weights);
        let mut day_row = table.row();
        day_row.push_element(Text::default());
        let mut days = Vec::new();
        for (day_col, day_identifier) in [
            (&self.monday, &self.day_identifiers.monday),
            (&self.tuesday, &self.day_identifiers.tuesday),
            (&self.wednesday, &self.day_identifiers.wednesday),
            (&self.thursday, &self.day_identifiers.thursday),
            (&self.friday, &self.day_identifiers.friday),
            (&self.saturday, &self.day_identifiers.saturday),
            (&self.sunday, &self.day_identifiers.sunday),
        ] {
            if !day_col.is_empty() {
                day_row.push_element(FramedElement::new(DayCell(day_identifier.clone())));
                days.push(day_col);
            }
        }
        day_row.push()?;

        let rendered_size = table.render(context, area.clone(), style)?.size;
        let rest_size = area.size() - rendered_size;
        let row_height = rest_size.height / self.grid_size.rows as f64;

        for i in 0..self.grid_size.rows {
            let mut row = table.row();
            row.push_element(FramedElement::new(IndexCell {
                index: i + 1,
                text: self
                    .class_durations
                    .get(i)
                    .map(ToString::to_string)
                    .unwrap_or(String::new()),
                height: row_height,
            }));

            for j in 0..self.grid_size.columns {
                let entry = days.get(j).and_then(|v| v.get(i));
                let name = entry.and_then(|e| e.class.clone());
                let teacher = entry.and_then(|e| e.teacher.clone());
                let room = entry.and_then(|e| e.room.clone());

                row.push_element(FramedElement::new(ClassCell {
                    name,
                    teacher,
                    room,

                    height: row_height,
                }));
            }

            row.push()?;
        }

        area.add_offset((0, rendered_size.height));
        table.render(context, area.clone(), style)
    }
}

#[derive(Debug, Clone)]
struct DayCell(String);

impl Element for DayCell {
    fn render(
        &mut self,
        context: &Context,
        area: Area<'_>,
        style: Style,
    ) -> Result<RenderResult, Error> {
        let font_cache = &context.font_cache;
        let font_size: u8 = 20;

        let style = Style::new().with_font_size(font_size).bold().and(style);
        let width = style.str_width(font_cache, &self.0);

        let start = area.size().width / 2.0 - width / 2.0;
        area.print_str(font_cache, (start, 0).into(), style, &self.0)?;

        let line_height = style.line_height(font_cache);
        Ok(RenderResult {
            size: (area.size().width, line_height * 1.3).into(),
            has_more: false,
        })
    }
}

#[derive(Debug, Clone)]
struct IndexCell {
    index: usize,
    text: String,

    height: Mm,
}

impl Element for IndexCell {
    fn render(
        &mut self,
        context: &Context,
        area: Area<'_>,
        style: Style,
    ) -> Result<RenderResult, Error> {
        let font_cache = &context.font_cache;
        let index_font_size: u8 = 30;
        let text_font_size: u8 = 10;

        let index_style = Style::new()
            .with_font_size(index_font_size)
            .bold()
            .and(style);
        let text_style = Style::new().with_font_size(text_font_size).and(style);

        let index_width = index_style.str_width(font_cache, &self.index.to_string());
        let text_width = text_style.str_width(font_cache, &self.text);

        let index_start = area.size().width / 2.0 - index_width / 2.0;
        let text_start = area.size().width / 2.0 - text_width / 2.0;

        let index_line_height = index_style.line_height(font_cache);
        let text_line_height = text_style.line_height(font_cache);
        area.print_str(
            font_cache,
            (index_start, self.height / 2.0 - index_line_height * 0.75).into(),
            index_style,
            self.index.to_string(),
        )?;
        area.print_str(
            font_cache,
            (text_start, self.height - (text_line_height * 1.5)).into(),
            text_style,
            &self.text,
        )?;

        Ok(RenderResult {
            size: (area.size().width, self.height).into(),
            has_more: false,
        })
    }
}

#[derive(Debug, Clone)]
struct ClassCell {
    name: Option<String>,
    teacher: Option<String>,
    room: Option<String>,

    height: Mm,
}

impl Element for ClassCell {
    fn render(
        &mut self,
        context: &Context,
        area: Area<'_>,
        style: Style,
    ) -> Result<RenderResult, Error> {
        let name = self.name.as_ref().map(AsRef::as_ref).unwrap_or("");
        let teacher = self.teacher.as_ref().map(AsRef::as_ref).unwrap_or("");
        let room = self.room.as_ref().map(AsRef::as_ref).unwrap_or("");

        let font_cache = &context.font_cache;

        let bot_font_size: u8 = 10;
        let bot_style = Style::new().with_font_size(bot_font_size).and(style);
        let room_width = bot_style.str_width(font_cache, room);

        let mut name_font_size: u8 = 25;
        let mut name_style;
        let mut name_width;
        loop {
            name_style = Style::new()
                .with_font_size(name_font_size)
                .bold()
                .and(style);
            name_width = name_style.str_width(font_cache, name);
            match name_width > area.size().width {
                true => name_font_size -= 1,
                false => break,
            }
        }

        let name_start = area.size().width / 2.0 - name_width / 2.0;
        let room_start = area.size().width - room_width;

        let name_line_height = name_style.line_height(font_cache);
        let bot_line_height = bot_style.line_height(font_cache);
        area.print_str(
            font_cache,
            (name_start, self.height / 2.0 - name_line_height * 0.75).into(),
            name_style,
            name,
        )?;
        area.print_str(
            font_cache,
            (2, self.height - (bot_line_height * 1.5)).into(),
            bot_style,
            teacher,
        )?;
        area.print_str(
            font_cache,
            (
                room_start - Mm::from(2.0),
                self.height - (bot_line_height * 1.5),
            )
                .into(),
            bot_style,
            room,
        )?;

        Ok(RenderResult {
            size: (area.size().width, self.height).into(),
            has_more: false,
        })
    }
}
