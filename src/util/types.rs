/*********************************
 * Type Defs
 *********************************/

use std::error::Error;

use chrono::Local;

//function return types
pub type UnitResult = Result<(), Box<dyn Error>>;
pub type StringResult = Result<String, Box<dyn Error>>;
pub type EventResult = Result<crossterm::event::Event, Box<dyn Error>>;
pub type TimerCallback = fn() -> Result<(), Box<dyn Error>>;

//stored time log type
//
// todo: impl a From trait for converting from seconds and to displayable format
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TimeLog {
    pub id: u32,
    pub time_spent: [u64; 3],
    pub date: String,
}

impl std::fmt::Display for TimeLog {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Session ID: {}\n\rTime Spent: {} hours, {} minutes, {} seconds\n\rDate: {}",
            self.id, self.time_spent[0], self.time_spent[1], self.time_spent[2], self.date
        )
    }
}

impl From<std::time::Duration> for TimeLog {
    fn from(time: std::time::Duration) -> TimeLog {
        let seconds = time.as_secs();
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        let seconds = seconds % 60;

        TimeLog {
            id: super::generate_id(),
            time_spent: [hours, minutes, seconds],
            date: Local::now().format("%m-%d-%Y").to_string(),
        }
    }
}

pub trait EventReader {
    fn read_event(&mut self) -> EventResult;
}

pub struct TerminalEventReader;

impl TerminalEventReader {
    pub fn new() -> Self {
        TerminalEventReader
    }
}
impl EventReader for TerminalEventReader {
    fn read_event(&mut self) -> EventResult {
        Ok(crossterm::event::read()?)
    }
}
