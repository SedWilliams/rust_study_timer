pub mod io;

pub mod types;
use types::TimeLog;

use std::time;
use std::fs::{exists, OpenOptions, File};
use std::io::Write;
use std::env;

use rand::prelude::*;
use chrono::Local;
use crossterm::{event, event::Event, event::KeyCode, terminal};

fn generate_id() -> u32 {
    let mut rng = rand::rng();
    rng.random_range(1000..9999)
}

//abstract file existence check into its own function
pub fn update_time_log(session_details: &TimeLog) {

    let time_log_txt_path = env::current_exe()
        .unwrap()
        .join("/time_log.txt");

    //Handles time_log.txt file appending and existence checking
    match exists(&time_log_txt_path) {
        Ok(true) => {
            println!("Time log file exists. Appending new entry...");
            let file = OpenOptions::new()
                .append(true)
                .open(&time_log_txt_path)
                .expect("Failed to open time_log.txt for appending");

            let json_entry = serde_json::to_string(&session_details)
                .expect("Failed to serialize session details to JSON");

            writeln!(&file, "{}", json_entry)
                .expect("Failed to write session details to time_log.txt");
        },
        Ok(false) => {
            println!("Time log file does not exist. Creating new file...");
            let file = File::create(&time_log_txt_path)
                .expect("Failed to create time_log.txt");
            let json_entry = serde_json::to_string(&session_details)
                .expect("Failed to serialize session details to JSON");
            writeln!(&file, "{}", json_entry)
                .expect("Failed to write session details to time_log.txt");

        },
        Err(e) => {
            println!("Error checking file existence: {}", e);
        },
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

    terminal::disable_raw_mode().expect("Failed to disable raw mode");

    let formatted_time: TimeLog = secs_to_base_time(elapsed_seconds);
    update_time_log(&formatted_time);
    
    //println!("Session info: {:?}", &formatted_time.date);
    //println!("Debug: timer function end...");
    
}

fn secs_to_base_time(seconds_from_timer: u64) -> TimeLog {
    let hours: i8 = (seconds_from_timer / 3600) as i8;
    let minutes: i8 = ((seconds_from_timer % 3600) / 60) as i8;
    let seconds: i8 = (seconds_from_timer % 60) as i8;

    TimeLog {
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
        
        let expected_formatted_time = TimeLog {
            id: 1234, //id is arbitrary for this test
            time_spent: [1, 0, 5],
            date: Local::now().format("%Y-%m-%d").to_string(), //date is arbitrary for this test
        };

        let res: TimeLog = secs_to_base_time(time_in_secs);
        
        //given a fixed input time in seconds, the output time_spent should match
        //expected_formatted_time's time_spent, which represents the correct converstion done
        //manually
        assert_eq!(res.time_spent, expected_formatted_time.time_spent);
    }
}

