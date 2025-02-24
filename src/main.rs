use std::io::Stdin;
use std::io::{self, Write};

use rust_shell;

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
        "exit 0" => rust_shell::run_exit_command(),
        command if command.starts_with("echo ") => {
            rust_shell::run_echo_command(rust_shell::get_arguments_of_command(command))
        }
        command if command.starts_with("type ") => {
            rust_shell::run_type_command(rust_shell::get_arguments_of_command(command))
        }
        _ => {}
    }
}
