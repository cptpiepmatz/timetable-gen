use crate::config::Config;
use genpdf::elements::TableLayout;
use genpdf::error::Error;
use genpdf::render::Area;
use genpdf::style::Style;
use genpdf::{Context, Element, RenderResult};

#[derive(Debug, Clone)]
pub struct TimetableHead {
    left: LeftCell,
    middle: MiddleCell,
    right: RightCell,
}

impl TimetableHead {
    pub fn new(config: &Config) -> Self {
        TimetableHead {
            left: LeftCell {
                class_name: config.class_name.clone(),
                class_room: config.class_room.clone(),
                class_teacher: config.class_teacher.clone(),
            },
            middle: MiddleCell {
                title: config.title.clone(),
                subtitle: config.subtitle.clone(),
            },
            right: RightCell {
                school_name: config.school_name.clone(),
                school_address: config.school_address.clone(),
            },
        }
    }
}

impl Element for TimetableHead {
    fn render(
        &mut self,
        context: &Context,
        area: Area<'_>,
        style: Style,
    ) -> Result<RenderResult, Error> {
        let mut table = TableLayout::new(vec![1, 1, 1]);
        let mut row = table.row();
        row.push_element(self.left.clone());
        row.push_element(self.middle.clone());
        row.push_element(self.right.clone());
        row.push()?;

        let size = table.render(context, area, style)?.size;
        Ok(RenderResult {
            size: (size.width, size.height * 1.2).into(),
            has_more: false,
        })
    }
}

#[derive(Debug, Clone)]
struct LeftCell {
    class_name: String,
    class_room: String,
    class_teacher: String,
}

impl Element for LeftCell {
    fn render(
        &mut self,
        context: &Context,
        area: Area<'_>,
        style: Style,
    ) -> Result<RenderResult, Error> {
        let font_cache = &context.font_cache;
        let font_size: u8 = 17;

        let top_style = Style::new().with_font_size(font_size).bold().and(style);
        let bot_style = Style::new().with_font_size(font_size).and(style);

        let class_teacher_width = bot_style.str_width(font_cache, &self.class_teacher);
        let class_room_width = top_style.str_width(font_cache, &self.class_room);
        let class_room_start = class_teacher_width - class_room_width;
        let line_height = top_style.line_height(font_cache);

        area.print_str(font_cache, (0, 0).into(), top_style, &self.class_name)?;
        area.print_str(
            font_cache,
            (class_room_start, 0).into(),
            top_style,
            &self.class_room,
        )?;
        area.print_str(
            font_cache,
            (0, line_height).into(),
            bot_style,
            &self.class_teacher,
        )?;

        Ok(RenderResult {
            size: (area.size().width, line_height * 2.0).into(),
            has_more: false,
        })
    }
}

#[derive(Debug, Clone)]
struct MiddleCell {
    title: String,
    subtitle: String,
}

impl Element for MiddleCell {
    fn render(
        &mut self,
        context: &Context,
        area: Area<'_>,
        style: Style,
    ) -> Result<RenderResult, Error> {
        let font_cache = &context.font_cache;
        let font_size: u8 = 17;

        let top_style = Style::new().with_font_size(font_size).and(style);
        let bot_style = Style::new().with_font_size(font_size).bold().and(style);

        let title_width = top_style.str_width(font_cache, &self.title);
        let subtitle_width = bot_style.str_width(font_cache, &self.subtitle);
        let area_width = area.size().width;

        let title_start = area_width / 2.0 - title_width / 2.0;
        let subtitle_start = area_width / 2.0 - subtitle_width / 2.0;

        let line_height = top_style.line_height(font_cache);
        area.print_str(font_cache, (title_start, 0).into(), top_style, &self.title)?;
        area.print_str(
            font_cache,
            (subtitle_start, line_height).into(),
            bot_style,
            &self.subtitle,
        )?;

        Ok(RenderResult {
            size: (area.size().width, line_height * 2.0).into(),
            has_more: false,
        })
    }
}

#[derive(Debug, Clone)]
struct RightCell {
    school_name: String,
    school_address: String,
}

impl Element for RightCell {
    fn render(
        &mut self,
        context: &Context,
        area: Area<'_>,
        style: Style,
    ) -> Result<RenderResult, Error> {
        let font_cache = &context.font_cache;
        let font_size: u8 = 15;

        let top_style = Style::new().with_font_size(font_size).bold().and(style);
        let bot_style = Style::new().with_font_size(font_size).and(style);

        let school_name_width = top_style.str_width(font_cache, &self.school_name);
        let school_address_width = bot_style.str_width(font_cache, &self.school_address);
        let area_width = area.size().width;

        let school_name_start = area_width - school_name_width;
        let school_address_start = area_width - school_address_width;

        let line_height = top_style.line_height(font_cache);
        area.print_str(
            font_cache,
            (school_name_start, 0).into(),
            top_style,
            &self.school_name,
        )?;
        area.print_str(
            font_cache,
            (school_address_start, line_height).into(),
            bot_style,
            &self.school_address,
        )?;

        Ok(RenderResult {
            size: (area.size().width, line_height * 2.0).into(),
            has_more: false,
        })
    }
}
