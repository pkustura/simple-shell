#![allow(dead_code)]
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
