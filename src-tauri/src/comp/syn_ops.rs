use tauri;
use crate::comp::list::LinkedList as Stack;

/*

VARIOUS SYNTAX OPERATIONS

This module contains helper functions that ease various syntax operations, like
checking for illegal characters, cleaningc clutter, splitting numbers between
fractionals and integrals, and clamping values to fit bases.

*/

// splits an input string at the decimal point
pub fn split_input(input: &str) -> Vec<&str> {
    input.split(".").collect()
}

// clears all whitespace and commas within an input
pub fn clear_clutter(input: &str) -> String {
    input
        .chars()
        .filter(|&c| !c.is_whitespace() && c != ',')
        .collect()
}

// checks if a base is valid
pub fn base_valid(base_str: &str) -> bool {
    if let Ok(base) = base_str.parse::<u8>() {
        base <= 35 || base >= 2
    } else {
        false
    }
}

// checks if an input has valid dots
pub fn dots_valid(input: &str) -> bool {
    input.chars().filter(|&c| c == '.').count() <= 1
}

// checks if an input is valid according to its base
pub fn range_valid(input: &str, base_str: &str) -> bool {
    if let Ok(base) = base_str.parse::<u32>() {
        input.chars().all(|c| {
            if c.is_digit(base) {
                true
            } else if c.is_alphabetic() {
                let value = c.to_ascii_uppercase() as u32 - 55;
                value < base
            } else {
                false
            }
        })
    } else {
        false
    }
}

// clamps a base to 2-35
#[tauri::command]
pub fn clamp_base(base_str: &str) -> Result<String, String> {
    if let Ok(base) = base_str.parse::<u8>() {
        Ok(format!("{}", base.clamp(2, 35)))
    } else {
        Err(String::from(
            "base could not be clamped because it contains illegal characters",
        ))
    }
}

// clamps an input according to its base
#[tauri::command]
pub fn clamp_input(input: &str, base_str: &str) -> Result<String, String> {
    if let Ok(base) = base_str.parse::<u8>() {
        let mut clamped = String::new();
        for c in input.chars() {
            clamped.push(clamp_digit(c, base));
        }
        return Ok(clamped);
    }
    Err(String::from("failed to parse base as u8 from string"))
}

// checks if an input string is valid
#[tauri::command]
pub fn input_valid(input: &str) -> bool {
    input
        .chars()
        .all(|c| c.is_alphanumeric() || c.is_whitespace() || c == '.' || c == ',')
        && dots_valid(input)
}

// clamps a single char digit from a base
fn clamp_digit(c: char, base: u8) -> char {
    let u = c.to_ascii_uppercase() as u8;
    let base_max = if base > 10 { base + 54 } else { base + 47 } as char;
    if c.is_alphabetic() {
        if u - 55 < base {
            c
        } else {
            base_max
        }
    } else if c.is_ascii_digit() {
        if u - 48 < base {
            c
        } else {
            base_max
        }
    } else {
        c
    }
}

pub fn format_output(
    output: &mut String,
    group_size: usize,
    separator: char
) { // does not need to return an error since all possible ones should have been caight in previous function calls
    let mut res_str = String::new();
    let (input_vec, mut stack) = (split_input(output), Stack::<char>::new());
    for (i, c) in input_vec[0].chars().rev().enumerate() {
        if i != 0 && i % group_size == 0 {
            stack.push(separator);
        }
        stack.push(c);
    }
    while let Some(c) = stack.pop() {
        res_str.push(c);
    }
    if input_vec.len() > 1 {
        res_str.push('.');
        for (i, c) in input_vec[1].chars().enumerate() {
            if !(separator == ',' && i == 0) && i % group_size == 0 {
                res_str.push(separator);
            }
            res_str.push(c);
        }
    }
    *output = res_str;
}