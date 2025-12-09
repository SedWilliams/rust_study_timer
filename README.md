# Terminal Study Timer

At the core this is a simple terminal timer designed to time, and log, study sessions.

## Usage

Ensure rust is installed on your system. Then run the following commands in your terminal:

```bash
git clone https://github.com/SedWilliams/Terminal_Study_Log.git

cd Terminal_Study_Log

cargo run
```

Note: Program uses system date/time. Please ensure your system date/time is current.

## What I learned?

* terminal IO (Rust Specific)
* file IO
* chrono crate for date/time Handling
* serde/serde_json crates for serialization/deserialization (JSON in this project)
* crossterm crate for terminal manipulation and event handling

## Problems encountered

* Handling terminal input/output in Rust
    * The enter keypress when I entered the cargo run command persisted and ruined my input collection.

## Future Improvements

* Once the basic functionality is complete I plan to add:
    * Polish to the TUI
    * Polish to underlying code
    * Functionality to allow for adding past study sessions, or sessions tracked elsewhere.
    * Functionality to visualize and format past study sessions from within the program.
    * __Possibly__ Option to set custom study session length at runtime, and then implement sound cues for session end.
    * Official Builds for UNIX and Windows

### There will never be a pause/resume function.
I use this tool personally, and I base my own study system off of Deep Work principles laid out in the work of Cal Newport.
And adding a pause/resume feature would allow for fragmenting 'deep work' time which is not something I want to encourage
for myself or for users of the tool. So this will always be a simple start, stop, and then end the session.
