# Project Structure & Modules

This document explains the file and directory structure of the `rust_study_timer`.

## Directory Tree

```
.
├── DOCS/                   # Documentation and related assets
├── src/                    # Source code
│   ├── main.rs             # Application entry point
│   ├── lib.rs              # Library root, module definitions
│   └── util/               # Utility modules (Application Logic)
│       ├── mod.rs          # util module definition
│       ├── io.rs           # Input/Output handling
│       ├── timer.rs        # Timer loop logic
│       ├── types.rs        # Data structures (TimeLog)
│       ├── generate_id.rs  # ID generation
│       └── secs_to_time_log.rs # Time conversion logic
├── Cargo.toml              # Project dependencies and metadata
└── time_log.txt            # (Generated) Log of study sessions
```

## Module Responsibilities

### `src/main.rs`
The executable entry point. It is responsible for:
*   Global error handling.
*   Initial terminal setup (Entering Alternate Screen).
*   Delegating control to `util::io`.

### `src/lib.rs`
The library entry point. It publicly exports the `util` module, making the application's internal logic testable and organized.

### `src/util/`
This directory contains the core logic, split into semantic files:
*   **`io.rs`**: Handles all user interaction (printing to screen, reading key presses) and file I/O (saving logs). Segregating I/O keeps logic pure in other modules.
*   **`timer.rs`**: Contains the "business logic" of the timer (counting, stopping).
*   **`types.rs`**: Defines the data model (`TimeLog`) shared across modules.
*   **`secs_to_time_log.rs`**: A pure transformation module that converts raw data (seconds) into the domain model (`TimeLog`).
*   **`generate_id.rs`**: A utility helper for unique identifiers.
