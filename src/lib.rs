use dirs::home_dir;
use pathsearch::find_executable_in_path;
use std::io::{self, Stdin, Write};
use std::process::Command;
use std::{env, path::Path};

pub fn prompt() {
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
        command if command.starts_with("echo ") => {
            run_echo_command(get_arguments_of_command(command))
        }
        command if command.starts_with("type ") => {
            run_type_command(get_arguments_of_command(command))
        }
        command if command.starts_with("pwd") => run_pwd_command(),
        command if command.starts_with("cd ") => run_cd_command(get_arguments_of_command(command)),
        command => {
            run_executable_command(command, get_arguments_of_command(command));
        }
    }
}

fn run_exit_command() {
    std::process::exit(0);
}

fn get_arguments_of_command(command: &str) -> Vec<&str> {
    let mut inp: Vec<&str> = command.split(" ").collect();
    inp.remove(0);

    return inp;
}

fn run_echo_command(argument: Vec<&str>) {
    println!("{}", argument.join(" "));
}

fn print_shell_builtin(command: Vec<&str>) {
    println!("{} is a shell builtin", command.get(0).unwrap());
}
fn print_not_found(command: &str) {
    println!("{command}: not found");
}

fn run_type_command(argument: Vec<&str>) {
    let builtins = ["type", "exit", "echo", "pwd", "cd"];
    if builtins.contains(&argument.get(0).unwrap()) {
        print_shell_builtin(argument);
    } else if let Some(executable) = find_executable_in_path(argument.get(0).unwrap()) {
        println!("{} is {}", argument.get(0).unwrap(), executable.display());
    } else {
        print_not_found(argument.get(0).unwrap());
    }
}

fn run_pwd_command() {
    match std::env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => eprintln!("pwd: error getting current directory: {}", e),
    }
}

fn run_cd_command(argument: Vec<&str>) {
    let path = argument.get(0).unwrap_or(&&"/");

    if path == &"~" {
        if let Some(path) = home_dir() {
            env::set_current_dir(path).expect("No such file or directory")
        } else {
            eprintln!("Could not determine home directory.");
        }
    } else {
        let result = Path::new(path);

        if result.exists() {
            env::set_current_dir(result).expect(&format!("cd: {}: No such file or directory", path))
        } else {
            println!("cd: {}: No such file or directory", path);
        }
    }
}

fn run_executable_command(command: &str, argument: Vec<&str>) {
    let command: Vec<&str> = command.split(" ").collect();
    let command = command.get(0).unwrap();

    match Command::new(command).args(argument).output() {
        Ok(output) => {
            print!("{}", String::from_utf8_lossy(&output.stdout));
            io::stdout().flush().unwrap();
            if !output.stderr.is_empty() {
                io::stderr().write_all(&output.stderr).unwrap();
            }
        }
        Err(_) => {
            eprintln!("{}: command not found", command);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::{stdout, Stdout};

    use super::*;

    #[test]
    fn test_get_arguments_of_command() {
        let command = "echo hello";
        let arguments = get_arguments_of_command(command);
        assert_eq!(arguments.len(), 1);
        assert_eq!(arguments[0], "hello");
    }

    #[test]
    fn test_run_cd_command() {
        let command = "cd ~";
        let arguments = get_arguments_of_command(command);
        run_cd_command(arguments);
        assert_eq!(env::current_dir().unwrap(), home_dir().unwrap());
    }
}
