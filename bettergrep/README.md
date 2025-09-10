# Bettergrep

A command-line file search tool written in Rust, inspired by the grep utility.
This project is an extended version of the minigrep example from The Rust Programming Language book, with added features for enhanced usability.

-----

## Features

- **Normal Search:** Find lines containing a specified query.

- **Case-Insensitive Search:** Use a command-line flag to perform a search that ignores case.

- **Strict Search:** Find a query as a whole word, not as a substring within a larger word.

- **Line Numbering:** Each matching line is prefixed with its corresponding line number, making it easy to locate results in the file.

- **Clearer Output:** The tool provides clear messages indicating if results were "Found" or "Not found!!!".

-----

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
| --strict | Performs a strict search |

> Notice that the command-line option arguments are all parallel!

-----

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

### Strict Search


This will search for "duct" in poem.txt, matching only "duct".

```Bash

cargo run duct poem.txt --strict

```

### Strict Case-Insensitive Search

Both of the commands below will search for "rUst" in poem.txt, matching both "Rust" and "rust".

```Bash

cargo run rUst poem.txt --ignore_case --strict

```

```Bash

cargo run rUst poem.txt --strict --ignore_case

```

-----

## Comparing with the Original minigrep Program in the Rust Book

### Functionality and Features

>>> **Original minigrep:** Supports only two search modes: a basic, case-sensitive search and a basic, case-insensitive search.

>>> **The bettergrep:** Adds two completely new search modes: a strict, whole-word search and a case-insensitive strict search. This is a major usability improvement. Your program also includes a word count for the query, a feature not in the original.

### Argument Handling

>>> **Original minigrep:** The case-insensitive flag is handled by checking a separate environment variable (CASE_INSENSITIVE). This works but isn't as common or intuitive for users as command-line flags.

>>> **The bettergrep:** All options are handled directly through standard command-line flags (--strict and --ignore-case). This is a much more idiomatic and user-friendly approach for command-line tools. Your use of a while let loop to process all optional flags is robust and professional.

### Code Structure and Design

>>> **Original minigrep:** The program has two separate search functions (search and search_case_insensitive), which leads to some code duplication.

>>> **The bettergrep:** You introduced a generic search_generic function that takes a closure. This is a key design improvement that shows a deep understanding of functional programming in Rust. It eliminates duplicated code and makes the program more flexible and easier to maintain. You also use a SearchMethod enum to handle the different search types in a clean and type-safe way.

### Dependencies

>>> **Original minigrep:** The program uses only the Rust standard library.

>>> **The bettergrep:** You correctly identified that a "strict" search is best handled by a regular expression library and integrated the regex crate. This is a smart choice that allows you to leverage a powerful tool for a specific problem.

### Punctuation Handling

>>> **Original minigrep:** It does not handle punctuation well for whole-word matching (since it doesn't have this feature).

>>> **The bettergrep:** You correctly diagnosed the issue with punctuation and implemented a clean_text helper function to handle it. This shows excellent problem-solving skills and attention to detail.