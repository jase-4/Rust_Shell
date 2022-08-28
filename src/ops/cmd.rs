use std::{str::{SplitWhitespace}, process::{Command, Stdio}};

pub fn run_cmd(args: SplitWhitespace,command:&str,stdin:Stdio,stdout:Stdio)->Result<std::process::Child, std::io::Error>{
    
    let output = Command::new(command)
        .args(args)
        .stdin(stdin)
        .stdout(stdout)
        .spawn();
    output
}