use std::env;
use std::fs::{exists, OpenOptions, File};
use std::io::{self, Write};

use super::types::TimeLog;

use crossterm::{
    event::{
        self, Event, KeyCode, read,
    },
    style::Stylize,
    terminal::{
        self, LeaveAlternateScreen
    },
    execute,
};

/*****************************************************
 * Program IO functions
 *****************************************************/

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

pub fn handle_yes_no(callback: fn()) {
    
    terminal::enable_raw_mode().expect("Failed to enable raw mode");

    //wait for yes/no keypress
    let result = loop {
        if let Event::Key(event) = read().expect("Failed to read event") {
            match event.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    break String::from("y");
                }
                KeyCode::Char('n') | KeyCode::Char('N') => {
                    break String::from("n");
                }
                _ => {
                    continue;
                }
            }
        }
    };
    
    //handle results based on user input
    if result == "y" {
        println!("");
        println!("\rStarting timer...\r");
        callback();
    } else {
        exit_message();
        terminal::disable_raw_mode().expect("Failed to disable raw mode");
    }
}

//non-blocking wait for termination keypress
pub fn await_terminate() {
    unimplemented!();
}

/*****************************************************
 * OUTPUT FUNCTIONS
 *****************************************************/

//welcome message, displays on program start
pub fn welcome_message() {
    println!("--------------------------");
    println!("| {}  |", " Terminal Study Timer".blue().bold());
    println!("--------------------------");
    print!("Would you like to start a study timer? [y/n]...\n");
}

//exit msg, displays on program close
pub fn exit_message() {
    println!("\rExiting the program. Goodbye!\r");
}

//updates time_log.txt with new session details
pub fn update_time_log(session_details: &TimeLog) { //abstract file existence check into its own function
 
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
                .expect("Failed to open time_log.txt for appending");

            let json_entry = serde_json::to_string(&session_details)
                .expect("Failed to serialize session details to JSON");

            writeln!(&file, "{}", json_entry)
                .expect("Failed to write session details to time_log.txt");
        },
        Ok(false) => {
            println!("\rTime log file does not exist. Creating new file...");
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

    //add silent, blocking, event read that waits for any keypress to continue
    println!("\rPress any key to exit...\r");
    let _ = read();

    terminal::disable_raw_mode().expect("Failed to disable raw mode");

    execute!(
        io::stdout(),
        LeaveAlternateScreen,
    ).expect("Failed to leave alternate screen");

}
