use crate::comp::list::LinkedList as Stack;
use crate::comp::syn_ops::split_input;
use bigdecimal::BigDecimal;
use num_bigint::BigUint;
use num_traits::{pow, ToPrimitive};
use std::str::FromStr;
use tauri;

use super::syn_ops::format_output;

#[tauri::command]
pub fn process_conversion(input: &str, base1_str: &str, base2_str: &str) -> Result<String, String> {
    let mut res_str = convert_from_decimal(
        &convert_to_decimal(
            &input
                .chars()
                .filter(|&c| c.is_alphanumeric() || c == '.')
                .collect::<String>(),
            base1_str,
        )?,
        base2_str,
    )?;
    match (base1_str, base2_str) {
        ("8", "2") => format_output(&mut res_str, 3, ' '),
        (_, "2") => format_output(&mut res_str, 4, ' '),
        (_, "10") => format_output(&mut res_str, 3, ','),
        (_, "16") => format_output(&mut res_str, 4, ' '),
        _ => {}
    }
    Ok(res_str)
}

pub fn convert_to_decimal(input: &str, base_str: &str) -> Result<String, String> {
    let mut decimal = BigDecimal::from(0);
    if let Ok(base) = base_str.parse::<u8>() {
        let input_vec = split_input(input);
        decimal += to_decimal_integral(input_vec[0], base)?;
        if input_vec.len() > 1 {
            decimal += to_decimal_fractional(input_vec[1], base)?;
        }
        return Ok(decimal.to_string());
    }
    Err(String::from(
        "conversion to decimal failed because u8 could not be parsed from base",
    ))
}

pub fn convert_from_decimal(decimal: &str, base_str: &str) -> Result<String, String> {
    if let Ok(base) = base_str.parse::<u8>() {
        let mut res_str = String::new();
        let input_vec = split_input(decimal);
        from_decimal_integral(&mut res_str, input_vec[0], base)?;
        if input_vec.len() > 1 {
            from_decimal_fractional(&mut res_str, input_vec[1], base)?;
        }
        return Ok(if res_str.is_empty() { String::from("0") } else { res_str });
    }
    Err(String::from(
        "conversion from decimal failed because u8 could not be parsed from base",
    ))
}

fn to_decimal_integral(integral_str: &str, base: u8) -> Result<BigDecimal, String> {
    let mut decimal = BigDecimal::from(0);
    for (i, c) in integral_str.chars().rev().enumerate() {
        if !c.is_alphanumeric() {
            return Err(String::from(
                "conversion to decimal failed because of a non-alphanumeric character",
            ));
        }
        decimal += to_digit(c) * pow(BigDecimal::from(base), i);
    }
    Ok(decimal)
}

fn to_decimal_fractional(fractional_str: &str, base: u8) -> Result<BigDecimal, String> {
    let mut decimal = BigDecimal::from(0);
    for (i, c) in fractional_str.chars().enumerate() {
        if !c.is_alphanumeric() {
            return Err(String::from(
                "conversion to decimal failed because of a non-alphanumeric character",
            ));
        }
        decimal += to_digit(c) * BigDecimal::from(1) / pow(BigDecimal::from(base), i + 1);
    }
    Ok(decimal)
}

fn from_decimal_integral(res_str: &mut String, integral_str: &str, base: u8) -> Result<(), String> {
    if let Ok(mut integral) = BigUint::from_str(integral_str) {
        let mut stack = Stack::<char>::new();
        while &integral > &BigUint::from(0u8) {
            let rem = &integral % BigUint::from(base);
            if let Some(c) = biguint_to_char(&rem) {
                stack.push(c);
            } else {
                return Err(String::from(
                    "conversion from decimal failed because u8 could not be parsed from BigUint",
                ));
            }
            integral /= &BigUint::from(base);
        }
        while let Some(c) = stack.pop() {
            res_str.push(c);
        }
        return Ok(());
    }
    Err(String::from(
        "conversion failed because BigUint could not be parsed from &str"
    ))
}

fn from_decimal_fractional(
    res_str: &mut String,
    fractional_str: &str,
    base: u8,
) -> Result<(), String> {
    if let Ok(mut fractional) = BigDecimal::from_str(&format!("0.{}", fractional_str)) {
        res_str.push('.');
        let (mut places, limit) = (0u8, 64u8); // 64 is hard-coded but may be variable later on
        while &fractional != &BigDecimal::from(0) && places < limit {
            fractional *= &BigDecimal::from(base);
            let digit = fractional.with_scale(0);
            if let Some(c) = bigdecimal_to_char(&digit) {
                res_str.push(c);
            } else {
                return Err(String::from(
                    "conversion from decimal failed because i8 could not be parsed from BigUint",
                ));
            }
            fractional -= &digit;
            places += 1;
        }
        return Ok(());
    }
    Err(String::from(
        "conversion failed because BigDecimal could not be parsed from &str",
    ))
}

pub fn to_digit(c: char) -> BigDecimal {
    if c.is_alphabetic() {
        BigDecimal::from(c.to_ascii_uppercase() as u8 - 55)
    } else {
        BigDecimal::from(c as u8 - 48)
    }
}

pub fn biguint_to_char(biguint: &BigUint) -> Option<char> {
    if let Some(num) = biguint.to_u8() {
        return Some(if num > 9 { num + 55 } else { num + 48 } as char);
    }
    None
}

pub fn bigdecimal_to_char(bigdecimal: &BigDecimal) -> Option<char> {
    if let Some(num) = bigdecimal.to_i8() {
        return Some(if num > 9 { num + 55 } else { num + 48 } as u8 as char);
    } // logically this function will never take a signed integer, so as u8 as char shouldn't cause errors
    None
}
