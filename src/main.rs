use rust_study_timer::*;

//note from crossterm docs:
//      New line character will not be processed therefore println! can’t be used, use write! instead

// program entry point
fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    //display welcome message and set terminal
    util::io::set_terminal()?;

    util::io::handle_yes_no(util::timer::timer);

    util::io::clear_terminal()?;

    Ok(())
}
