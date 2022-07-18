use khoj::{parse_config, run};
use std::env::args;
use std::process::exit;

fn main() {
    let cmd_args = args().into_iter().collect();
    if let Err(e) = parse_config(cmd_args).and_then(|c| run(&c)) {
        eprintln!("{}", e);
        exit(1);
    }
}
