use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};

// mod display; 
mod shell;

fn main() {

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

fn prompt_user() -> String {
//    let path = env::current_dir().unwrap();
    shell::print_prompt();
    stdout().flush().unwrap(); //flush stdout
    //read input
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    return buf;
}


fn change_dir(path: &str) -> Result<(), std::io::Error> {
    let path = Path::new(path);
    return env::set_current_dir(&path);
}

// execution of a command with arguments and piping. "commands" is an iterator of cmd Strings.
// fn exec_command(commands: &str, is_background: bool) -> Child {
//
// }
