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
    },
    style::Stylize,
    terminal,
    execute
};

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
        
        //this loop is slept by 1 second by the poll function above
        //increment elapsed seconds
        elapsed_seconds += 1;

        //run secs_to_base_time function to convert elapsed_seconds into base_time struct

    }

    let formatted_time: TimeLog = secs_to_time_log(elapsed_seconds);
    update_time_log(&formatted_time);
    
    //println!("Session info: {:?}", &formatted_time.date);
    //println!("Debug: timer function end...");
}
