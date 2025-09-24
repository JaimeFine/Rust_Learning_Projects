# Bettergrep

A command-line file search tool written in Rust, inspired by the grep utility.
This project is an extended version of the minigrep example from The Rust Programming Language book, with added features for enhanced usability.

> To take a look at the original minigrep, please visit: [The Rust Programming Language](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)

-----

## ✨ Features

- **Normal Search:** Find lines containing a specified query.

- **Case-Insensitive Search:** Use a command-line flag to perform a search that ignores case.

- **Strict Search:** Find a query as a whole word, not as a substring within a larger word.

- **Line Numbering:** Each matching line is prefixed with its corresponding line number, making it easy to locate results in the file.

- **Clearer Output:** The tool provides clear messages indicating if results were "Found" or "Not found!!!".

-----

## 📂 Project structure

```text
.
├── src
│   ├── lib.rs      # Functionality code file
│   └── main.rs     # Main code file
├── text.txt        # Example text file      
└── Cargo.toml
```

-----

## 🚀 Getting started

### Prerequisites

  * **Rust (latest stable):** [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

### Build the Program

```bash
# Clone and enter the project
git clone [https://github.com/JaimeFine/Rust_Learning_Projects/bettergrep.git](https://github.com/JaimeFine/Rust_Learning_Projects/bettergrep.git)
cd bettergrep

# Run the server
cargo run
```


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

## 🧪 Examples

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

## 📖 Comparing with the Original minigrep Program in the Rust Book

### Functionality and Features

- **Original minigrep:** Supports only two search modes: a basic, case-sensitive search and a basic, case-insensitive search.

- **The bettergrep:** Adds two completely new search modes: a strict, whole-word search and a case-insensitive strict search. This is a major usability improvement. The program also includes a word count for the query, a feature not included in the original.

### Argument Handling

- **Original minigrep:** The case-insensitive flag is handled by checking a separate environment variable (CASE_INSENSITIVE). This works but isn't as common or intuitive for users as command-line flags.

- **The bettergrep:** All options are handled directly through standard command-line flags (--strict and --ignore-case). This is a much more idiomatic and user-friendly approach for command-line tools.

### Code Structure and Design

- **Original minigrep:** The program has two separate search functions (search and search_case_insensitive), which leads to some code duplication.

- **The bettergrep:** Introduced a generic search_generic function that takes a closure. It eliminates duplicated code and makes the program more flexible and easier to maintain. There is also a SearchMethod enum to handle the different search types in a clean and type-safe way.

### Dependencies

- **Original minigrep:** The program uses only the Rust standard library.

- **The bettergrep:** Correctly identified that a "strict" search is best handled by a regular expression library and integrated the regex crate.

### Punctuation Handling

- **Original minigrep:** It does not handle punctuation well for whole-word matching (since it doesn't have this feature).

- **The bettergrep:** Correctly diagnosed the issue with punctuation and implemented a clean_text helper function to handle it.
