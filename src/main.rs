use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};

mod shell;

fn main() {

    // main loop for shell
    loop {

        // let mut input = String::new();
        //display prompt to user
        // input = prompt_user();
        let input = prompt_user();
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

                    if let Err(e) = change_dir(&path) {
                        eprintln!("Error: failed to change directory.");
                    }


                }
                cmd => {
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
                            eprintln!("Error: {}", e);
                            last_out = None;
                            }
                    };

                }
            };
        }

         if let Some(mut final_command) = last_out {
            // block until the final command has finished
            final_command.wait().unwrap();
        }
    }
}

fn prompt_user() -> String {
//    let path = env::current_dir().unwrap();
    print!("$ ");
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
