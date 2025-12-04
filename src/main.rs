use rust_study_timer::timer;
use crossterm::{event, event::Event, event::KeyCode, terminal};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("--------------------------\r");
    println!("Terminal Study Timer...\r");
    println!("--------------------------\r");
    println!("\r");
    println!("Would you like to start a study timer? (y/n)\r");

    //match input to handle yes/no response for starting the timer
    loop {

        terminal::enable_raw_mode().expect("Failed to enable raw mode");

        if let Event::Key(key_event) = event::read().expect("Event read failed: line 26 main.rs") {

            match key_event.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    println!("Starting study timer...");
                    timer();
                    break;
                }

                KeyCode::Char('n') | KeyCode::Char('N') => {
                    println!("Exiting the program. Goodbye!");
                    std::process::exit(0);
                }
                _ => {
                    println!("Invalid input. Please enter 'y' or 'n'.");
                    std::process::exit(1);
                }
            }

        }
    }

    Ok(())
}
