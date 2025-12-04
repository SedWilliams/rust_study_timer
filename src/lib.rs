use rand::prelude::*;
use std::{io, time, thread};
use crossterm::{event, event::Event, event::KeyCode, terminal};

pub struct base_time {
    pub id: u32,
    pub time_spent: [i8;3],
    pub date: [i16;3],
}

fn generate_id() -> u32 {
    let mut rng = rand::rng();
    rng.random_range(1000..9999)
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
    }
    
    terminal::disable_raw_mode().expect("Failed to disable raw mode");
    println!("Total elapsed time: {} seconds", elapsed_seconds);
    //println!("Debug: timer function end...");
    
    
}

fn secs_to_base_time(seconds_from_timer: u64) -> base_time {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secs_to_base_time() {
        let time_in_secs: u64 = 3605; //1 hour, 0 minutes, 5 seconds
        
        let expected_formatted_time: base_time = base_time {
            id: 1234, //id is arbitrary for this test
            time_spent: [1, 0, 5],
            date: [0, 0, 0], //date is arbitrary for this test
        };

        let res: base_time = secs_to_base_time(time_in_secs);
        
        //given a fixed input time in seconds, the output time_spent should match
        //expected_formatted_time's time_spent, which represents the correct converstion done
        //manually
        assert_eq!(res.time_spent, expected_formatted_time.time_spent);
    }
}

