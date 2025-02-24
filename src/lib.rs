use dirs::home_dir;
use pathsearch::find_executable_in_path;
use std::io::{self, Write};
use std::process::Command;
use std::{env, path::Path};

pub fn run_exit_command() {
    std::process::exit(0);
}

pub fn get_arguments_of_command(command: &str) -> Vec<&str> {
    let mut inp: Vec<&str> = command.split(" ").collect();
    inp.remove(0);

    return inp;
}

pub fn run_echo_command(argument: Vec<&str>) {
    println!("{}", argument.join(" "));
}

pub fn print_shell_builtin(command: Vec<&str>) {
    println!("{} is a shell builtin", command.get(0).unwrap());
}
pub fn print_not_found(command: &str) {
    println!("{command}: not found");
}

pub fn run_type_command(argument: Vec<&str>) {
    let builtins = ["type", "exit", "echo", "pwd", "cd"];
    if builtins.contains(&argument.get(0).unwrap()) {
        print_shell_builtin(argument);
    } else if let Some(executable) = find_executable_in_path(argument.get(0).unwrap()) {
        println!("{} is {}", argument.get(0).unwrap(), executable.display());
    } else {
        print_not_found(argument.get(0).unwrap());
    }
}

pub fn run_pwd_command() {
    match std::env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => eprintln!("pwd: error getting current directory: {}", e),
    }
}

pub fn run_cd_command(argument: Vec<&str>) {
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

pub fn run_executable_command(command: &str, argument: Vec<&str>) {
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
