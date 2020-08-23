use std::env;
use std::process::{Command, Stdio};

#[derive(Debug, PartialEq)]
enum CLICommand {
    EditBootstrap,
    EditRC,
    Run,
    GitClone,
}

fn spawn_and_wait(command: &mut Command) {
    // let mut command = Command::new("ls");
    if let Ok(mut child) = command.spawn() {
        child.wait().expect("error during execution...");
        // println!("Child has finished its execution!");
    } else {
        println!("Command unable to start.");
    }
}

fn sh(command: &str, working_dir: &str) {
    let mut the_args = command.split(' '); // todo: support quoted strings
    let first: &str = the_args.next().unwrap();
    let rest: Vec<&str> = the_args.collect::<Vec<&str>>();

    spawn_and_wait(
        Command::new(first)
            .args(rest)
            .current_dir(working_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
    );
}

fn run_command(command: CLICommand) {
    let command_text;
    let home = std::env::var("HOME").unwrap();
    let mut working_dir = format!("{}/git/dotfiles", home);
    if command == CLICommand::EditBootstrap {
        command_text = format!("vim {}/git/dotfiles/bootstrap.sh", home);
    } else if command == CLICommand::EditRC {
        command_text = format!("vim {}/git/dotfiles/.zshrc", home);
    } else if command == CLICommand::GitClone {
        command_text = format!("git clone https://github.com/jsmith/dotfiles {}/git/dotfiles", home);
        working_dir = home;
    } else {
        command_text = format!("{}/git/dotfiles/bootstrap.sh", home);
    }

    println!("> {}", command_text);
    sh(&command_text, &working_dir);
}

fn check_command(command: &str) -> Result<CLICommand, ()> {
    match command {
        "bootstrap" => Ok(CLICommand::EditBootstrap),
        "rc" => Ok(CLICommand::EditRC),
        "run" => Ok(CLICommand::Run),
        "clone" => Ok(CLICommand::GitClone),
        _ => Err(()),
    }
}

fn get_first_arg<'a>(args: &'a Vec<String>) -> Result<&'a str, ()> {
    match args.len() {
        2 => Ok(&args[1]),
        _ => Err(()),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match Ok(&args).and_then(get_first_arg).and_then(check_command) {
        Ok(command)  => run_command(command),
        Err(_e) => println!("Error"),
    }
}
