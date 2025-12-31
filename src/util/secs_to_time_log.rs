use super::types::TimeLog;
use super::generate_id;

use chrono::Local;

pub fn secs_to_time_log(seconds_from_timer: u64) -> TimeLog {
    let hours: i8 = (seconds_from_timer / 3600) as i8;
    let minutes: i8 = ((seconds_from_timer % 3600) / 60) as i8;
    let seconds: i8 = (seconds_from_timer % 60) as i8;

    TimeLog {
        id: generate_id(),
        time_spent: [hours, minutes, seconds],
        date: Local::now().format("%Y-%m-%d").to_string(), 
    }
}


