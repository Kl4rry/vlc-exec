use std::env;
use std::process::*;

fn main() {
    let mut args = env::args();
    let first = args.next();
    if first.is_some() {
        let mut vec: Vec<String> = vec![String::from("--sout-all"), String::from("--sout"), String::from("#display %1%")];
        vec.extend(args);
        Command::new("vlc")
            .args(vec)
            .spawn()
            .unwrap();
    }
}