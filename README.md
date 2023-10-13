 # simple-shell
A proof-of-concept minimal functional shell written in rust
Written while learning rust, and my first rust program! Hello, world!

# Implemented Features:
- Optional path argument to set current working directory
- Basic built-in commands: exit, cd
- Command piping (with "|" but not yet supporting "&|")
- Text coloring (for error output and user prompt) via crate owo-colors


# Features to-do:
- Signal handling
- Background processes ("&")
- Redirections
- configuration file (for colors, aliases, etc)

# Using simple-shell
Clone this repo. To build the binary:
```
cargo build
```
And to run (dev):
```
cargo run
```

# For CS377:

My video presentation is located here: https://drive.google.com/file/d/1ZZmIbefCgjPwfUYxlCmPQjpjwOT5_7ay/view

