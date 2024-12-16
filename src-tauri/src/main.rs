// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(test)] // exclude test file from release build
pub mod test;
pub mod comp;

fn main() {
    hexcell_android_lib::run()
}
