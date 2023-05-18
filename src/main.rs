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
        //display prompt to user
        let input = shell::prompt_user();
        shell::exec_input(input);
    }
    
}


