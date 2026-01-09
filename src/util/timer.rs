/*******************************************
 * Timer logic
 *******************************************/

use std::time;

// ../util/
use super::io::update_time_log;
use super::secs_to_time_log::secs_to_time_log;
use super::types::TimeLog;

use crossterm::{
    event::{
        self, Event, KeyCode
    }
};

pub fn timer() {
    //println!("Debug: timer funtion start...");
    println!("\rType 'q' to stop the timer.\r");

    let mut elapsed_seconds: u64 = 0;
    let start_time = std::time::Instant::now();

    loop {

        if event::poll(time::Duration::from_millis(1000)).expect("Event poll failed: line 23 lib.rs") {
            if let Event::Key(key_event) = event::read().expect("Event read failed: line 26 lib.rs") {
                if key_event.code == KeyCode::Char('q') {
                    println!("\rTimer stopped.");
                    println!("");
                    break;
                }
            }
        }
        
        elapsed_seconds = start_time.elapsed().as_secs();

    }

    let formatted_time: TimeLog = secs_to_time_log(elapsed_seconds);
    update_time_log(&formatted_time);
    
    //println!("Session info: {:?}", &formatted_time.date);
    //println!("Debug: timer function end...");
}
