use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub class_name: String,
    pub class_room: String,
    pub class_teacher: String,

    pub title: String,
    pub subtitle: String,

    pub school_name: String,
    pub school_address: String,

    pub class_durations: Vec<String>,

    pub day_identifiers: DayIdentifiers,

    #[serde(default)]
    pub monday: Vec<ClassEntry>,
    #[serde(default)]
    pub tuesday: Vec<ClassEntry>,
    #[serde(default)]
    pub wednesday: Vec<ClassEntry>,
    #[serde(default)]
    pub thursday: Vec<ClassEntry>,
    #[serde(default)]
    pub friday: Vec<ClassEntry>,
    #[serde(default)]
    pub saturday: Vec<ClassEntry>,
    #[serde(default)]
    pub sunday: Vec<ClassEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayIdentifiers {
    pub monday: String,
    pub tuesday: String,
    pub wednesday: String,
    pub thursday: String,
    pub friday: String,
    pub saturday: String,
    pub sunday: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassEntry {
    pub class: Option<String>,
    pub teacher: Option<String>,
    pub room: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub struct GridSize {
    pub rows: usize,
    pub columns: usize,
}

impl Config {
    pub fn grid_size(&self) -> GridSize {
        let has_monday = !self.monday.is_empty() as usize;
        let has_tuesday = !self.tuesday.is_empty() as usize;
        let has_wednesday = !self.wednesday.is_empty() as usize;
        let has_thursday = !self.thursday.is_empty() as usize;
        let has_friday = !self.friday.is_empty() as usize;
        let has_saturday = !self.saturday.is_empty() as usize;
        let has_sunday = !self.sunday.is_empty() as usize;

        let columns = has_monday
            + has_tuesday
            + has_wednesday
            + has_thursday
            + has_friday
            + has_saturday
            + has_sunday;

        let rows = *[
            self.monday.len(),
            self.tuesday.len(),
            self.wednesday.len(),
            self.thursday.len(),
            self.friday.len(),
            self.saturday.len(),
            self.sunday.len(),
        ]
        .iter()
        .max()
        .unwrap_or(&0);

        GridSize { columns, rows }
    }
}
