# Bettergrep

A command-line file search tool written in Rust, inspired by the grep utility.
This project is an extended version of the minigrep example from The Rust Programming Language book, with added features for enhanced usability.

## Features

- **Standard Search:** Find lines containing a specified query.

- **Case-Insensitive Search:** Use a command-line flag to perform a search that ignores case.

- **Line Numbering:** Each matching line is prefixed with its corresponding line number, making it easy to locate results in the file.

- **Clearer Output:** The tool provides clear messages indicating if results were "Found" or "Not found!!!".

## Usage

### Prerequisites

To build and run this project, you need to have Rust and Cargo installed on your system. You can get them from rustup.rs.

### Running the Program

To run the program, use the cargo run command followed by the arguments.

```Bash

cargo run <query> <file_path> [options]

```
* **<query>:** The string you want to search for.

* **<file_path>:** The path to the file you want to search within.

* **[options]:** Optional flags to modify the search behavior.

### Command-Line Options

| Option | Description |
| ------ | ----------- |
| --ignore_case	| Performs a case-insensitive search |

## Examples

### Standard Search

This will search for the literal string "three" in poem.txt.

```Bash

cargo run three poem.txt

```
### Case-Insensitive Search

This will search for "rUst" in poem.txt, matching both "Rust" and "Trust".

```Bash

cargo run rUst poem.txt --ignore_case

```
