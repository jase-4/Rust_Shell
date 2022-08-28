use std::env;

pub fn print_cur_dir(){
    let crab = '\u{1F980}';
    let cur_dir = env::current_dir();
    print!("{} {} ",crab,cur_dir.expect("Error Print current directory")
        .display())
}
