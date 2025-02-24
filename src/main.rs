use std::io::Stdin;
use std::io::{self, Write};

fn main() {
    loop {
        prompt();
    }
}
fn prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();
    let stdin: Stdin = io::stdin();
    let mut input: String = String::new();
    stdin.read_line(&mut input).unwrap();
    let command: &str = input.trim();
    handle_command(command);
}

fn handle_command(command: &str) {}
