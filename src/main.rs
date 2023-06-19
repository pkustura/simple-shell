use std::cmp::Ordering;
use std::env;
use std::path::Path;

mod shell;

fn main() {
    // startup directory arg
    let args: Vec<String> = env::args().collect();
    match args.len().cmp(&2) {
        Ordering::Greater => {
            println!("Intended usage: ./simple_shell optional:start_dir");
        }
        Ordering::Equal => {
            let path = Path::new(args.get(1).unwrap());
            assert!(env::set_current_dir(path).is_ok());
        }
        Ordering::Less => {} // nothing needed
    }

    // main loop for shell
    loop {
        //display prompt to user
        let input = shell::prompt_user();
        //execute respective commands!
        shell::exec_input(input);
    }
}
