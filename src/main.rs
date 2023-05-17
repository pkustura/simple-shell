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
            //builtins implemented here.
                "exit" => return, 
                "cd" => {
                    let path = match args.next() {
                        Some(path) => path.trim(),
                        None => "/",
                    };

                    if let Err(e) = shell::change_dir(&path) {
                        // eprintln!("Error: failed to change directory.");
                        shell::err_log(e.to_string());
                    }


                }
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
                            shell::err_log(e.to_string());
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
