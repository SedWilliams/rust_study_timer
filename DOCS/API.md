# API Documentation

This document details the functions, structs, and modules available in the `rust_study_timer` project.

## Module: `src/util/io.rs`

Handles Input/Output operations, terminal manipulation, and file writing.

### `welcome_message()`
```rust
pub fn welcome_message()
```
*   **Description**: Prints the "Terminal Study Timer" banner to stdout.
*   **Usage**: Called at program start in `main.rs`.

### `handle_yes_no(callback: fn())`
```rust
pub fn handle_yes_no(callback: fn())
```
*   **Description**: Enables raw mode and waits for a single key press ('y' or 'n').
    *   If 'y': Prints a start message and executes the provided `callback` function.
    *   If 'n': Calls `exit_message()` and terminates (via normal flow).
*   **Parameters**:
    *   `callback`: A function pointer (e.g., `util::timer::timer`) to execute if the user confirms.

### `exit_message()`
```rust
pub fn exit_message()
```
*   **Description**: Prints a goodbye message.

### `update_time_log(session_details: &TimeLog)`
```rust
pub fn update_time_log(session_details: &TimeLog)
```
*   **Description**: Appends the completed session details to `time_log.txt`.
*   **Logic**:
    *   Determines the path of `time_log.txt` relative to the executable.
    *   Checks if the file exists.
    *   Opens in append mode (or creates if missing).
    *   Serializes `session_details` to a JSON string using `serde_json`.
    *   Writes the JSON line to the file.
    *   Waits for a final keypress before cleaning up terminal state (leaving raw mode/alternate screen).

---

## Module: `src/util/timer.rs`

Contains the core timing logic.

### `timer()`
```rust
pub fn timer()
```
*   **Description**: The main timer loop.
*   **Logic**:
    *   Initializes `elapsed_seconds` to 0.
    *   enters a `loop`:
        *   `event::poll` waits for 1 second.
        *   If `event::read` detects 'q', the loop breaks.
        *   Otherwise, `elapsed_seconds` increments.
    *   After the loop, converts time using `secs_to_time_log` and saves it via `update_time_log`.

---

## Module: `src/util/secs_to_time_log.rs`

### `secs_to_time_log(seconds_from_timer: u64) -> TimeLog`
```rust
pub fn secs_to_time_log(seconds_from_timer: u64) -> TimeLog
```
*   **Description**: Converts a raw count of seconds into a structured `TimeLog` object.
*   **Parameters**:
    *   `seconds_from_timer`: Total duration of the session in seconds.
*   **Returns**: `TimeLog` struct containing generated ID, calculated [H, M, S], and current date.

---

## Module: `src/util/generate_id.rs`

### `generate_id() -> u32`
```rust
pub fn generate_id() -> u32
```
*   **Description**: Generates a random 4-digit ID (1000-9999) for a session.
*   **Dependencies**: Uses `rand::rng` and `random_range`.

---

## Module: `src/util/types.rs`

### Struct `TimeLog`
```rust
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TimeLog {
    pub id: u32,
    pub time_spent: [i8; 3], // [Hours, Minutes, Seconds]
    pub date: String,        // "YYYY-MM-DD"
}
```
*   **Description**: Represents a completed study session.
*   **Fields**:
    *   `id`: Randomly generated session ID.
    *   `time_spent`: Array representing duration broken down into hours, minutes, and seconds. Note: Defined as `i8`, ensuring values are small (though theoretically `u8` or `u64` might be safer for strict typing, `i8` is the current implementation).
    *   `date`: Date string of when the session finished.
