use super::m_argv;

pub fn L_SetupConsoleMasks() {
    let (mut p, mut i) = (false, 0);
    let cena = "ICWEFDA";
    let mut pos = 0;
    p = m_argv::M_CheckParm('r');
    if p {
        
    }
}

pub fn D_DoomMainSetup() {
    let (mut p, mut slot) = (0, 0);

    L_SetupConsoleMasks();
}

pub fn D_DoomLoop() {}

/*
 *  Main Function
 * */
pub fn D_DoomMain() {
    D_DoomMainSetup();
    D_DoomLoop();
}
