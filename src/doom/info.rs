use super::d_think;

pub struct state_t {
    frame: i64,
    tics: i64,
    action: d_think::actionf_t,
    misc1: i64,
    misc2: i64
}