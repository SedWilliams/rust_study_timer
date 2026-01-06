# Rust Study Timer Documentation

Welcome to the documentation for the **Rust Study Timer**. This project is a command-line interface (CLI) tool designed to track study sessions.

## Table of Contents

1.  [Architecture & Program Flow](ARCHITECTURE.md)
    *   High-level overview of how the application runs.
    *   Flowcharts and logic description.
2.  [API Documentation](API.md)
    *   Detailed breakdown of functions, structs, and modules.
3.  [Module Structure](MODULES.md)
    *   Explanation of the file organization and source code layout.

## Quick Start

### Running the Project
```bash
cargo run
```

### Usage
1.  The program will launch in a dedicated terminal screen.
2.  Press `y` to start the timer.
3.  The timer runs in the background.
4.  Press `q` to stop the timer.
5.  The session is saved to `time_log.txt` in the same directory as the executable.
6.  Press any key to exit.