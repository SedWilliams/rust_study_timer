use rust_study_timer::*;
use crossterm::{event::{self, read, Event, KeyCode}, terminal};
use std::{io};
use chrono::Local;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    terminal::SetTitle("Terminal Study Timer");

    append_to_file();

    println!("--------------------------");
    println!("Terminal Study Timer...");
    println!("--------------------------");
    println!("");
    println!("Would you like to start a study timer? (y/n)...");

    //println!("{}", Local::now().format("%Y-%m-%d"));

    //match input to handle yes/no response for starting the timer
    loop {

        terminal::enable_raw_mode().expect("Failed to enable raw mode");

        if let Event::Key(event) = read()? {
            match event.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    println!("Starting study timer...");
                    timer();
                    break;
                }
                KeyCode::Char('n') | KeyCode::Char('N') => {
                    println!("Exiting the program. Goodbye!");
                    break;
                }
                _ => {
                    continue;
                }
            }
        }
    } 


    terminal::disable_raw_mode().expect("Failed to disable raw mode");

    Ok(())
}
