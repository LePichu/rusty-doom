use std::env;

pub fn myargv() -> Vec<String> {
    env::args().collect()
}
pub fn myargc() -> usize {
    myargv().len()
}

pub fn M_CheckParm(check: &str) -> usize {
    let args = myargv();
    for i in 1..myargc() {
        if args[i] == check {
            return i;
        }
    }
    0
}