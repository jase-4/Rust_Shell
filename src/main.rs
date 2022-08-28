use std::io::stdout;
use std::io::stdin;
use std::io::Write;
use std::process::Child;
use std::process::Stdio;

mod utils;
mod ops;

use ops::cd::cd_cmd;
use ops::cmd::run_cmd;
use utils::print_cur_dir;

fn main() {
    
    loop{
        print_cur_dir();
        stdout().flush().unwrap();
        let mut input = String::new();
        
        stdin()
            .read_line(&mut input)
            .expect("error reading input");

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command= None;

        while let Some(command) = commands.next()  {

            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

        match command {
            "cd" => {
                cd_cmd(args);
                previous_command = None;
            },

            "exit" =>{
                print!("exiting rsshell...");
                return
            },

            command => {
                let stdin = previous_command
                .map_or(
                    Stdio::inherit(),
                    |output: Child| Stdio::from(output.stdout.unwrap())
                );

                let stdout = if commands.peek().is_some() {
                    Stdio::piped()
                    } else {
                    Stdio::inherit()
                };

                let output = run_cmd(args, command, stdin ,stdout);

                    match output {
                        Ok(output) => { previous_command = Some(output); },
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                         },
                    };
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            final_command.wait().expect("error");
        }
    }
}
