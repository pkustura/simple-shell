use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use home::home_dir;

pub fn prompt_user() -> String {
//    let path = env::current_dir().unwrap();
    print_prompt();
    stdout().flush().unwrap(); //flush stdout
    //read input
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    return buf;
}

pub fn exec_input(input: String) {

    //handle piping: split commands in array by pipes.
    let mut commands = input.split("|").peekable();

    //we need to keep track of last output for piping
    //and eventually outputting.
    let mut last_out = None;
        
    while let Some(cmd) = commands.next() {
        let mut args = cmd.trim().split_whitespace();
       
        //avoid panic on empty input
        let cmd = match args.next() {
            Some(cmd) => cmd,
            None => continue,
        };
        
        match cmd {
            // --- builtins implemented here. ---
            "exit" => {
                std::process::exit(0);
            } 
            "cd" => {
                change_dir(args.next());
            }
            // ---
            // general command execution
            cmd => {
                //for piping stdout from last command into stdin of next
                let stdin = last_out.map_or(Stdio::inherit(),
                                            |out: Child| {
                                                Stdio::from(out.stdout.unwrap())
                                            });            
        

                let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };
                    
                    // Child struct
                    // command is like builder for child (running/exited process)
                let output = Command::new(cmd)
                    .args(args)
                    .stdin(stdin)
                    .stdout(stdout)
                    .spawn();
                
                match output {
                    Ok(output) => {
                        last_out = Some(output);
                        }
                    Err(e) => {
                        // eprintln!("Error: {}", e);
                        err_log(e.to_string());
                        last_out = None;
                        }
                    };
                }
            };
         }
        if let Some(mut dependent) = last_out {
            // block until the final command has finished
            dependent.wait().unwrap();
        }
}


pub fn change_dir(dir_str: Option<&str>) {
                let pathbuf = match dir_str {
                    Some(str) => Path::new(str.trim()).to_path_buf(),
                    None => home_dir().unwrap(),
                };
                if let Err(e) = env::set_current_dir(&pathbuf) {
                    err_log(e.to_string());
                }
}


/* handling colorized output to terminal */

use owo_colors::OwoColorize;

pub fn err_log(s: String) {
    println!("{}", s.on_red());
}

pub fn suc_log(s: String) {
    println!("{}", s.dimmed());
}

pub fn print_prompt() {
    let path = env::current_dir().unwrap();
    print!("{}{}", path.display().green(), " $ ".green());
}
