mod commands;

use commands::Shell;
use std::io::{self, Write};

fn main() {
    let shell = Shell::new();
    loop {
        print!("$ ");

        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input = input.trim().to_string();
        shell.process_command(&input);
    }
}
