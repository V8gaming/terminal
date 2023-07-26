use std::{io::{Read, Error, Write}, process::Command, os::unix::io::AsRawFd};
use pty::fork::{Fork, Master};
use nix::unistd::read;
use nix::sys::termios;

pub fn pty(command: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut parts = command.split_whitespace();
    let command = parts.next().unwrap();
    let args: Vec<&str> = parts.collect();
    let fork = Fork::from_ptmx()?;

    match fork {
        Fork::Parent(_, mut Master) => {
            let mut output = String::new();
            Master.read_to_string(&mut output)?;
            Ok(output)
        },
        Fork::Child(_) => {
            let mut child_command = Command::new(command);
            child_command.args(args);
            child_command.spawn()?;
            Ok(String::from("Child process spawned."))
        }
    }
}
