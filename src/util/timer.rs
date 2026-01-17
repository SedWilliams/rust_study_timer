/*******************************************
 * Timer logic
 *      TODO...
 *      * refactor into struct with impl
 *      * abstract spinner
 *******************************************/

use std::time;

// ../util/
use super::io::update_time_log;
use super::types::{TimeLog, UnitResult};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
};

pub fn timer() -> UnitResult {
    //println!("Debug: timer funtion start...");
    println!("\rType 'q' to stop the timer.\r");

    let start_time = std::time::Instant::now();

    //non-mut, stack-allocated array of spinner frames
    let spinner_frames: [&str; 4] = ["|", "/", "-", "\\"];
    let mut current_frame: usize = 0;

    loop {
        //update spinner
        if current_frame >= spinner_frames.len() {
            current_frame = 0;
        }
        execute!(
            std::io::stdout(),
            crossterm::cursor::MoveToColumn(0),
            crossterm::style::Print(format!(
                "\rElapsed time: {} seconds {} ",
                start_time.elapsed().as_secs(),
                &spinner_frames[current_frame]
            ))
        )
        .expect("Crossterm error, run 'cargo fetch'");
        current_frame += 1;

        //check for 'q' keypress to quit
        //poll for event every 100ms
        if event::poll(time::Duration::from_millis(100))
            .expect("Crossterm error, run 'cargo fetch'")
        {
            //if event detected, read it
            //      error? triggers a blocking read when any key is pressed, not just 'q'
            if let Event::Key(key) = event::read().expect("Crossterm error, run 'cargo fetch'") {
                //if event was 'q' keypress, break loop
                //    else, continue loop
                if key.code == KeyCode::Char('q') {
                    println!("\n\rTimer stopped.");
                    println!("");
                    break;
                } else {
                    continue;
                }
            }
        }
    }

    let formatted_time: TimeLog = TimeLog::from(start_time.elapsed());
    update_time_log(&formatted_time);

    Ok(())
}
