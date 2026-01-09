use super::types::TimeLog;
use super::generate_id;

use chrono::Local;


//MOVE THIS LOGIC TO IMPL BLOCK FOR TimeLog
pub fn secs_to_time_log(seconds_from_timer: u64) -> TimeLog {
    let hours = seconds_from_timer / 3600;
    let minutes = (seconds_from_timer % 3600) / 60;
    let seconds = seconds_from_timer % 60;

    TimeLog {
        id: generate_id(),
        time_spent: [hours, minutes, seconds],
        date: Local::now().format("%m-%d-%Y").to_string(), 
    }
}


