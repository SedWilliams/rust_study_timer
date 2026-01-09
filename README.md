# Terminal Study Log

This is a simple terminal timer designed to time and log study sessions.

## Table of Contents (Project wide, source of truth)

1. [README.md](README.md#terminal-study-log)
   * [Usage: Quick Start](#usage-quick-start)
   * [What I learned](#what-i-learned)
   * [Problems Encountered](#problems-encountered)
   * [Future Improvements](#future-improvements)
3. [Overview of DOCS Directory](DOCS/DOCS.md)
   * [Table of Contents (DOCS Directory)](DOCS/DOCS.md#table-of-contents)
5. [Architecture & Program Flow](DOCS/ARCHITECTURE.md)
   * [High-Level Program Overview](DOCS/ARCHITECTURE.md#highlevel-overview)
   * [Program Flow](DOCS/ARCHITECTURE.md#program-flow)
   * [Data Flow Diagram](DOCS/ARCHITECTURE.md#data-flow-diagram)
2. [API Documentation](DOCS/API.md)
   * [Module by Module Breakdown](DOCS/API.md#api-documentation)
3. [Module Structure](DOCS/MODULES.md)
   * [Directory Tree](DOCS/MODULES.md#directory-tree)
   * [Module Responsibilities](DOCS/MODULE.md#module-responsibilities)

## Usage: Quick Start

Ensure rust is installed on your system. Then run the following commands in your terminal:

```bash
git clone https://github.com/SedWilliams/Terminal_Study_Log.git

cd Terminal_Study_Log

cargo run 

//or...
cargo build
//find the 'rust_study_timer' executable
./rust_study_timer
```

Session information is stored next to the executable in a file called time\_log.txt

Note: Program uses system date/time. Please ensure your system date/time is current.

## What I learned?

* terminal IO (Rust Specific)
* file IO
* 'Box' pointer type
* chrono crate for date/time Handling
* serde/serde\_json crates for serialization/deserialization (JSON in this project)
* crossterm crate for terminal manipulation and event handling
* test based development
    * OOP is harder to write testable code in versus functional
* Safer usage of VCS
* Moduling in Rust
* Safer/more effective AI usage
* Was reminded very quickly I know nothing about anything, and neither does anyone else.
    * Don't go into any project or learning endeavor thinking you know everything.
    * There is always more to learn, and more efficient/better ways to do things.
    * That does not mean you aren't capable or smart, it is just a part of being human. So don't be discouraged but go into every project with the mindset of what learning oppotunities are available.

## Problems encountered

* Handling terminal input/output in Rust
    * The enter keypress when I entered the cargo run command persisted and ruined my input collection. Parsing terminal input without blocking the main thread.
    * Proper terminal buffer handling
    * Platform dependent terminal IO
* Platform dependent storage
* Serializing/deserializing data with serde

## Future Improvements

Future improvements list was moved to [./DOCS/TODO.md](DOCS/TODO.md)

### There will never be a pause/resume function.
I use this tool personally, and I base my own study system off of Deep Work principles laid out in the work of Cal Newport.
And adding a pause/resume feature would allow for fragmenting 'deep work' time which is not something I want to encourage
for myself or for users of the tool. So this will always be a simple start, stop, and then end the session.
