// all logic for input/output handling
pub mod io;

//data types used in the program
pub mod types;

//generate random id for TimeLog entries
pub mod generate_id;
use generate_id::generate_id;

//timer main logic
pub mod timer;

//convert elapsed seconds into type: TimeLog (struct)
pub mod secs_to_time_log;

#[cfg(test)]
mod tests {

    //TODO write tests

}

