use stui_timer::util::types::UnitResult;
use stui_timer::util::{io, timer};

//note from crossterm docs:
//      New line character will not be processed therefore println! can’t be used, use write! instead

// program entry point
fn main() -> UnitResult {
    //display welcome message and set terminal
    io::set_terminal();

    //handle user input for starting timer -> start timer if yes
    let result: String = io::await_yes_no().unwrap_or_else(|error| {
        io::clear_terminal();
        panic!("\n\rError while awaiting yes/no input: {}.", error);
    });

    io::handle_yes_no(result, timer::timer).unwrap_or_else(|error| {
        io::clear_terminal();
        panic!("\n\rError handling yes/no input: {}", error);
    });

    io::clear_terminal();

    Ok(())
}
