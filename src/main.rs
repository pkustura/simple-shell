use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};

// mod display; 
mod shell;

fn main() {

    // startup directory arg
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Intended usage: ./simple_shell optional:start_dir")
    } else if args.len() == 2 {
        let path = Path::new(args.get(1).unwrap());
        env::set_current_dir(&path);
    }

    // main loop for shell
    loop {
        // let mut input = String::new();
        //display prompt to user
    // input = prompt_user();
        let input = shell::prompt_user();
        //handle piping: split commands in array by pipes.
        shell::exec_input(input);
    }
    
}


