use std::{process::Command, path::Path};

pub fn run_commands(command: String) -> String {
    //stdout().flush().unwrap();

    let input = command;
    //stdin().read_line(&mut input).unwrap();

    let mut parts = input.split_whitespace();
    let command = parts.next().unwrap();
    let mut args = parts;
    if command == "cd" {
        if args.clone().count() == 0 {
            std::env::set_current_dir(std::env::var("HOME").unwrap()).unwrap();
            println!("{:?}", std::env::current_dir().unwrap());
            return "".to_string();
        }
        if args.clone().count() > 1 {
            return "cd: too many arguments".to_string();
        }
        if !Path::new(args.clone().next().unwrap()).exists() {
            return "cd: no such file or directory".to_string();
        }

        std::env::set_current_dir(args.next().unwrap()).unwrap();
        println!("{:?}", std::env::current_dir().unwrap());
        "".to_string()
    } else {
        let output = Command::new(command).args(args).output();

        return match output {
            Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
            Err(e) => e.to_string(),
        };
    }
}
