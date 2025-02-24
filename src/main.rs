use std::io::Stdin;
use std::io::{self, Write};

use rust_shell::run_exit_command;

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

fn handle_command(command: &str) {
    match command {
        "exit 0" => run_exit_command(),
        _ => {}
    }
}
