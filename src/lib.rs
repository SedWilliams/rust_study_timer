use std::{io, time, thread};
use std::fs::{self, OpenOptions, File};
use std::path::Path;

use rand::prelude::*;
use chrono::Local;
use crossterm::{event, event::Event, event::KeyCode, terminal};
use serde::{Serialize, Deserialize};
use serde_json::{from_str};

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseTime {
    pub id: u32,
    pub time_spent: [i8;3],
    pub date: String,
    //pub date: chrono::format::DelayedFormat<chrono::format::StrftimeItems<'static>>, //type from
                                                                                     //chrono crate
}

fn generate_id() -> u32 {
    let mut rng = rand::rng();
    rng.random_range(1000..9999)
}

pub fn append_to_file() {
    let cwd = std::env::current_dir().unwrap().display().to_string();
    match fs::exists(format!("{cwd}/time_log.txt")) {
        Ok(file) => println!("File opened successfully: {:?}", file),
        Err(e) => println!("Failed to open file: {}", e),
    }

}

pub fn timer() {
    //println!("Debug: timer funtion start...");
    println!("Type 'q' to stop the timer.");

    terminal::enable_raw_mode().expect("Failed to enable raw mode");

    let mut elapsed_seconds: u64 = 0;

    loop {

        if event::poll(time::Duration::from_millis(1000)).expect("Event poll failed: line 23 lib.rs") {
            if let Event::Key(key_event) = event::read().expect("Event read failed: line 26 lib.rs") {
                if key_event.code == KeyCode::Char('q') {
                    println!("\nTimer stopped.");
                    break;
                }
            }
        }

        elapsed_seconds += 1;

        //run secs_to_base_time function to convert elapsed_seconds into base_time struct

    }

    let formatted_time: BaseTime = secs_to_base_time(elapsed_seconds);
    
    //println!("Session info: {:?}", &formatted_time.date);
    //println!("Debug: timer function end...");
    
}

fn secs_to_base_time(seconds_from_timer: u64) -> BaseTime{
    let hours: i8 = (seconds_from_timer / 3600) as i8;
    let minutes: i8 = ((seconds_from_timer % 3600) / 60) as i8;
    let seconds: i8 = (seconds_from_timer % 60) as i8;

    BaseTime {
        id: generate_id(),
        time_spent: [hours, minutes, seconds],
        date: Local::now().format("%Y-%m-%d").to_string(), 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secs_to_base_time() {
        let time_in_secs: u64 = 3605; //1 hour, 0 minutes, 5 seconds
        
        let expected_formatted_time: BaseTime = BaseTime {
            id: 1234, //id is arbitrary for this test
            time_spent: [1, 0, 5],
            date: Local::now().format("%Y-%m-%d").to_string(), //date is arbitrary for this test
        };

        let res: BaseTime = secs_to_base_time(time_in_secs);
        
        //given a fixed input time in seconds, the output time_spent should match
        //expected_formatted_time's time_spent, which represents the correct converstion done
        //manually
        assert_eq!(res.time_spent, expected_formatted_time.time_spent);
    }
}

