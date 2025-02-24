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
