/*
    Hifumi Technologies (https://github.com/hifumitech)
    File: src/main.rs

    Author(s): {
        Hifumi1337 (https://github.com/Hifumi1337)
    }
*/

mod core;
mod modules;
mod catherine;

fn main() {
    catherine::init("Catherine initializing...");
    catherine::shutdown("Shutdown successful...");
}