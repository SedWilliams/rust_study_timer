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
    execute,
};

pub fn timer() {
    //println!("Debug: timer funtion start...");
    println!("\rType 'q' to stop the timer.\r");

    let mut elapsed_seconds: u64 = 0;
    let start_time = std::time::Instant::now();

    let spinner_frames = vec!["|", "/", "-", "\\"];
    let mut current_frame = 0;

    loop {

        if current_frame >= spinner_frames.len() {
            current_frame = 0;
        }
        execute!(
            std::io::stdout(),
            crossterm::cursor::MoveToColumn(0),
            crossterm::style::Print(format!(
                "\rElapsed time: {} seconds {} ",
                start_time.elapsed().as_secs(),
                spinner_frames[current_frame]
            ))
        ).expect("Failed to write to stdout");
        current_frame+=1;

        if event::poll(time::Duration::from_millis(100)).expect("Event poll failed: line 23 lib.rs") {
            if let Event::Key(key_event) = event::read().expect("Event read failed: line 26 lib.rs") {
                if key_event.code == KeyCode::Char('q') {
                    println!("\n\rTimer stopped.");
                    println!("");
                    break;
                } else {
                    continue;
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
