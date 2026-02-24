mod compact_formatter;
mod json_formatter;
mod table_formatter;

pub use compact_formatter::CompactFormatter;
pub use json_formatter::JsonFormatter;
pub use table_formatter::TableFormatter;

use crate::domain::Project;

/// Trait for formatting project output
pub trait ProjectFormatter {
    fn format(&self, projects: &[Project]) -> String;
    fn print(&self, projects: &[Project]) {
        println!("{}", self.format(projects));
    }
}

/// Factory for creating formatters
pub struct FormatterFactory;

impl FormatterFactory {
    pub fn create(format: &str) -> Box<dyn ProjectFormatter> {
        match format.to_lowercase().as_str() {
            "json" => Box::new(JsonFormatter),
            "compact" => Box::new(CompactFormatter),
            _ => Box::new(TableFormatter),
        }
    }
}
