extern crate libc;
use super::m_argv;


pub fn D_DoomMainSetup() {
    let (mut p, mut slot) = (0, 0);

    let mut rsp_found = true;

    while rsp_found {
        rsp_found = false;
        for arg in m_argv::myargv() {
            let chr = arg.as_str().chars().nth(0).unwrap();
            if (chr == '@') {
                println!("Found");
            }
        }
    }

}

pub fn D_DoomLoop() {}

/*
 *  Main Function
 * */
pub fn D_DoomMain() {
    D_DoomMainSetup();
    D_DoomLoop();
}
