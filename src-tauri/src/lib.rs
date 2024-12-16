mod comp;

use tauri;
use crate::comp::{
    syn_ops::{
        clamp_base,
        clamp_input,
        input_valid
    },
    num_sys::process_conversion,
    conv_steps::process_steps,
    arithmetic::process_arithmetic,
    twos_comp::{
        process_complement,
        process_binary,
        process_complement_decimal,
        process_binary_decimal
    }
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            clamp_base,
            clamp_input,
            input_valid,
            process_conversion,
            process_steps,
            process_arithmetic,
            process_complement,
            process_binary,
            process_complement_decimal,
            process_binary_decimal
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
