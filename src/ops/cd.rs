use std::{str::SplitWhitespace, path::Path, env};

pub fn cd_cmd(args: SplitWhitespace){
    let user= whoami::username();
    let mut default_path = String::from("/Users/");
    default_path.push_str(&user);
    let new_dir = args.peekable().peek().map_or(default_path,|x| (*x).to_string());
    let root = Path::new(&new_dir);
    if let Err(e) = env::set_current_dir(&root) {
            eprintln!("{}", e);
    }
}