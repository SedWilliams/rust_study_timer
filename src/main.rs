use stui_timer::util::{self, io, types};

//note from crossterm docs:
//      New line character will not be processed therefore println! can’t be used, use write! instead

// program entry point
fn main() -> types::UnitResult {
    //display welcome message and set terminal
    io::set_terminal();

    let mut terminal_event_reader = types::TerminalEventReader::new();

    //handle user input for starting timer -> start timer if yes
    let result: String =
        util::io::await_startup_choice(&mut terminal_event_reader).unwrap_or_else(|error| {
            io::clear_terminal();
            panic!("\n\rError while awaiting startup input: {}.", error);
        });

    util::io::handle_startup_choice(result, util::timer::timer).unwrap_or_else(|error| {
        io::clear_terminal();
        panic!("\n\rError handling startup input: {}", error);
    });

    util::io::blocking_await_keypress();

    util::io::clear_terminal();

    Ok(())
}
