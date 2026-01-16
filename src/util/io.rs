/*****************************************************
 * Program IO functions
 *****************************************************/
use std::env;
use std::fs::{File, OpenOptions, exists};
use std::io::{self, Write};

use super::types::{StringResult, TimeLog, TimerCallback, UnitResult};

use crossterm::{
    cursor::*,
    event::{Event, KeyCode, read},
    execute,
    style::Stylize,
    terminal::*,
};

/*****************************************************
 * INPUT (handling) FUNCTIONS
 *****************************************************/

//handle yes_no()
//
//Params:
//      callback function
//      yes or no string (pass ownership)
//
//if yes -> callback()
//if no -> terminate prgm
pub fn await_yes_no() -> StringResult {
    //wait for yes/no keypress and store result in 'result': String
    let result: String = loop {
        if let Event::Key(key) = read()? {
            match key.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    //break String::from("y");
                    break "y".into();
                }
                KeyCode::Char('n') | KeyCode::Char('N') => {
                    break "n".into();
                }
                _ => {
                    continue;
                }
            }
        }
    };

    //pass ownership of result String back to caller
    Ok(result)
}

pub fn handle_yes_no(result: String, callback: TimerCallback) -> UnitResult {
    if result == "y" {
        println!("");
        println!("\rStarting timer...\r");
        callback()?;
    } else {
        exit_message();
        clear_terminal();
    }
    Ok(())
}

/*****************************************************
 * OUTPUT FUNCTIONS
 *****************************************************/

//welcome message, displays on program start
pub fn welcome_message() {
    println!("--------------------");
    println!("|   {}    |", " STUI Timer".blue().bold());
    println!("--------------------");
    print!("Would you like to start a study timer? [y/n]...\n");
}

//exit msg, displays on program close
pub fn exit_message() {
    println!("\rExiting the program. Goodbye!\r");
}

//sets up terminal for program use
pub fn set_terminal() {
    //if fails -> propogate error up

    execute!(
        io::stdout(),
        SetTitle("Study Timer"),
        //enter alternate terminal buffer; MAX: 2 buffers
        EnterAlternateScreen,
        //SetBackgroundColor(Color::DarkGrey),
        //SetForegroundColor(Color::Blue),
        MoveTo(0, 0),
    ).unwrap_or_else(|error| {
        panic!("\n\rCrossterm error while setting terminal: {}. Please restart terminal and run 'cargo fetch' to install Crossterm.", error);
    });

    welcome_message();

    enable_raw_mode().unwrap_or_else(|error| {
        panic!("\n\rCrossterm error while enabling raw mode: {}. Please restart terminal and run 'cargo fetch' to install Crossterm.", error);
    });
}

//clears terminal and resets to normal screen
pub fn clear_terminal() {
    execute!(io::stdout(), LeaveAlternateScreen).unwrap_or_else(|error| {
        panic!("\n\rCrossterm error while clearing terminal: {}. Please restart terminal and run 'cargo fetch' to install Crossterm.", error);
    });

    disable_raw_mode().unwrap_or_else(|error| {
        panic!("\n\rCrossterm error while disabling raw mode: {}. Please restart terminal and run 'cargo fetch' to install Crossterm.", error);
    });
}

//static spinner animation
pub fn spinner_animation() {
    unimplemented!();
}

//updates time_log.txt with new session details
pub fn update_time_log(session_details: &TimeLog) {
    //abstract file existence check into its own function

    let time_log_txt_path = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("time_log.txt");
    //println!("Time log file path: {:?}", &time_log_txt_path);

    //Handles time_log.txt file appending and existence checking
    match exists(&time_log_txt_path) {
        Ok(true) => {
            println!("\rTime log file exists. Appending new entry...");
            let file = OpenOptions::new()
                .append(true)
                .open(&time_log_txt_path)
                .unwrap_or_else(|error| {
                    clear_terminal();
                    panic!("\n\rFailed to open time_log.txt for appending: {}", error);
                });

            let json_entry = serde_json::to_string(&session_details).unwrap_or_else(|error| {
                clear_terminal();
                panic!("\n\rFailed to serialize session details to JSON: {}", error);
            });

            writeln!(&file, "{}", json_entry)
                .expect("Failed to write session details to time_log.txt, and session was not stored. Please file an issue.");
        }

        Ok(false) => {
            println!("\rTime log file does not exist. Creating new file...");

            let file = File::create(&time_log_txt_path).unwrap_or_else(|error| {
                clear_terminal();
                panic!("\n\rFailed to create time_log.txt: {}", error);
            });

            let json_entry = serde_json::to_string(&session_details).unwrap_or_else(|error| {
                clear_terminal();
                panic!("\n\rFailed to serialize session details to JSON: {}", error);
            });

            writeln!(&file, "{}", json_entry).unwrap_or_else(|error| {
                clear_terminal();
                panic!("\n\rFailed to write session details to time_log.txt, and session was not stored. Please file an issue: {}", error);
            });
        }

        Err(error) => {
            clear_terminal();
            panic!("Error checking file existence: {}", error);
        }
    }

    //show the user the session details that were logged
    println!("\r Logged: {}\n\r", &session_details);

    //add silent, blocking, event read that waits for any keypress to continue
    println!("\rPress any key to exit...\r");
    let _ = read();
}
