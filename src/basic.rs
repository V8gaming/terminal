use std::{path::Path, os::unix::process::ExitStatusExt};

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
        //println!("{:?}", std::env::current_dir().unwrap());
        "".to_string()
    } else {
        let output = std::process::Command::new(command)
            .args(args)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .output()
            .unwrap_or(std::process::Output {
                status: std::process::ExitStatus::from_raw(0),
                stdout: Vec::new(),
                stderr: Vec::new(),
            });
        String::from_utf8(output.stdout).unwrap()
        
    }
}
