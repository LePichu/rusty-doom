pub mod d_main;
pub mod m_argv;

extern "C" {
    fn test();
}

pub fn setup() {
    unsafe {
        test();
    }
    d_main::D_DoomMain();
}
