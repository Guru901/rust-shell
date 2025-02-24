use pathsearch::find_executable_in_path;

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
