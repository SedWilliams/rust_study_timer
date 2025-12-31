use rust_study_timer::util::*;

// program entry point
fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    io::program_welcome();
    
    //parse a yes or no response, else display exit message
    io::handle_yes_no(timer::timer);

    Ok(())
}
