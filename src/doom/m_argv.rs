use std::env;

pub fn M_CheckParm(check: char) -> bool {
    let args: Vec<String> = env::args().collect();
    let mut b = true;
    for arg in args {
        if b {
            b = false;
            continue;
        }
        if check == arg.chars().nth(0).unwrap() { return true }
    }
    false
}