/*********************************
 * Type Defs
 *********************************/

use std::error::Error;

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
