use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
// mod display;
pub fn prompt_user() -> String {
//    let path = env::current_dir().unwrap();
    print_prompt();
    stdout().flush().unwrap(); //flush stdout
    //read input
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    return buf;
}


pub fn change_dir(path: &str) -> Result<(), std::io::Error> {
    let path = Path::new(path);
    return env::set_current_dir(&path);
}


/* file handing colorized output to terminal */

use owo_colors::OwoColorize;

pub fn err_log(s: String) {
    println!("{}", s.on_red());
}

pub fn suc_log(s: String) {
    println!("{}", s.dimmed());
}

pub fn print_prompt() {
//    let path = env::current_dir().unwrap();
    print!("{}", "$ ".green());
}
