use rust_study_timer::util;

use crossterm::{
    execute,
    terminal::*,
    cursor::*,
    style::{Color, Attribute, SetForegroundColor, SetBackgroundColor, Stylize},
};

use std::io;

//note from crossterm docs:
//      New line character will not be processed therefore println! can’t be used, use write! instead

// program entry point
fn main() -> Result<(), Box<dyn std::error::Error>> {

    //let window = window_size()?;

    // Resize terminal and scroll up.
    execute!(
        io::stdout(),
        SetTitle("Study Timer"),
        //enter alternate terminal buffer; MAX: 2 buffers
        EnterAlternateScreen,
        //SetBackgroundColor(Color::DarkGrey),
        //SetForegroundColor(Color::Blue),
        MoveTo(0, 0),
    )?;

    util::io::program_welcome();

    util::io::handle_yes_no(util::timer::timer);

    Ok(())
}
