use std::env;
use std::fs::{exists, OpenOptions, File};
use std::io::{self, Write};
use std::error::Error;

use super::types::TimeLog;

use crossterm::{
    event::{
        Event, KeyCode, read,
    },
    style::Stylize,
    terminal::*,
    cursor::*,
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
pub fn await_yes_no() -> Result<String, Box<dyn Error>> {

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

pub fn handle_yes_no(result: String, callback:fn()->Result<(),Box<dyn Error>>) -> Result<(), Box<dyn Error>> {
    if result == "y" {
        println!("");
        println!("\rStarting timer...\r");
        callback()?;
    } else {
        exit_message();
        clear_terminal()?;
    }
    Ok(())
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

//sets up terminal for program use
pub fn set_terminal() -> Result<(), Box<dyn Error>> {

    execute!(
        io::stdout(),
        SetTitle("Study Timer"),
        //enter alternate terminal buffer; MAX: 2 buffers
        EnterAlternateScreen,
        //SetBackgroundColor(Color::DarkGrey),
        //SetForegroundColor(Color::Blue),
        MoveTo(0, 0),
    )?;
    
    welcome_message();

    enable_raw_mode().expect("Failed to enable raw mode");

    Ok(())
}

//clears terminal and resets to normal screen
pub fn clear_terminal() -> Result<(), Box<dyn Error>> {

    execute!(
        io::stdout(),
        LeaveAlternateScreen,
    ).expect("Failed to leave alternate screen");

    disable_raw_mode().expect("Failed to disable raw mode");

    Ok(())
}

//static spinner animation
pub fn spinner_animation() {
    unimplemented!();
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

    println!("\r{}\n\r", &session_details);

    //add silent, blocking, event read that waits for any keypress to continue
    println!("\rPress any key to exit...\r");
    let _ = read();
}
