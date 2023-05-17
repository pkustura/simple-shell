 # simple-shell
A proof-of-concept minimal functional shell written in rust

# Implemented Features:
- Built-in commands: exit, cd
- Basic command piping (with "|" but not yet supporting "&|")
- Basic text coloring (for error output and user prompt) via crate owo-colors


# Features to-do:
- Simple signal handling
- Background processes ("&")
- Redirections
- configuration file (for colors, etc)

# Using simple-shell
Clone this repo. To build the binary:
'''
cargo build
'''
And to run (dev):
'''
cargo run
'''

