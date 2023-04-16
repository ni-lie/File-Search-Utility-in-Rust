# File Search Utility in Rust
This is the first lab exercise in my CS 155 (Compiler Construction) class. We were tasked to create a Cargo-based **Rust** program
that takes in a *glob path* and a *regex string* as command line arguments and outputs a lexicographically sorted list of filepaths
with content matching the given regex string.

I created two implementations for this lab exercise:
1. Imperative way
   - For this, I used loops and vectors to traverse the directory tree 
   - If the content of the file matches with the given regex string, push it to an initialized vector *vec*
   - Then print all the elements that are contained in *vec*/
2. Functional Way
   - I used built-in Rust functions for this implementation:
     - `.into_iter()`: traverse the directory tree
     - `.filter` + `.is_match`: filters files that don't match with the given regex strings
     - `.collect`: collects all file paths into one vector

# How To Run
To run, go to the repository and run either implementations via:

```
# To run imp.rs:
cargo run --bin imp -- "**/*.txt" "\d+"

# To run fp.rs:
cargo run --bin fp -- "**/*.txt" "\d+"
```

# Sample calls + demo
```
cargo run --bin imp -- "**/*.txt" "\d+"

cargo run --bin fp -- "**/*.txt" "\d+"

```

<p align = "center">
    <img src="images\1.png" alt = "image">
</p>

The image above shows the result of a sample command for both **imperative** and **functional** implementations.
The files listed are files which have contents that match to the given regex *"\d+"*