extern crate cc;
use std::path::Path;
fn main() {
    //*
    cc::Build::new()
        .file("prboomsrc/test.c")
        .compile("prboom.o");
    // */
}