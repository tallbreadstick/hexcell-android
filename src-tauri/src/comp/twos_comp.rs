use tauri;

use crate::comp::{
    num_sys::{convert_from_decimal, convert_to_decimal},
    syn_ops::format_output,
};
use num_bigint::{BigInt, BigUint};
use num_traits::pow;
use std::str::FromStr;

#[tauri::command]
pub fn process_complement(binary_str: &str) -> Result<String, String> {
    if binary_str
        .chars()
        .all(|c| c == '1' || c == '0' || c.is_whitespace())
    {
        let mut res_str = binary_str
            .chars()
            .filter(|&c| c == '1' || c == '0')
            .collect::<String>();
        if let Ok(zero_test) = BigUint::from_str(&res_str) {
            if &zero_test == &BigUint::from(0u8) {
                return Ok(String::from("0"));
            }
        }
        let (mut width, mut padding) = (8usize, String::new());
        while width < res_str.len() {
            width *= 2;
        }
        for _ in 0..(width - res_str.len()) {
            padding.push('0');
        }
        res_str.insert_str(0, &padding);
        res_str = res_str
            .chars()
            .map(|c| if c == '1' { '0' } else { '1' })
            .collect::<String>();
        if let Ok(mut decimal_uint) = BigUint::from_str(&convert_to_decimal(&res_str, "2")?) {
            decimal_uint += &BigUint::from(1u8);
            res_str = convert_from_decimal(&decimal_uint.to_string(), "2")?;
            format_output(&mut res_str, 4, ' ');
            return Ok(res_str);
        }
    }
    Err(String::from(
        "binary to 2's complement conversion failed because of an illegal character",
    ))
}

#[tauri::command]
pub fn process_binary(complement_str: &str) -> Result<String, String> {
    let mut comp_decimal_str = process_complement_decimal(
        &complement_str
            .chars()
            .filter(|&c| c == '1' || c == '0')
            .collect::<String>()
    )?;
    comp_decimal_str = comp_decimal_str.replace("-", "");
    convert_from_decimal(&comp_decimal_str, "2")
}

#[tauri::command]
pub fn process_complement_decimal(complement_str: &str) -> Result<String, String> {
    if complement_str
        .chars()
        .all(|c| c == '1' || c == '0' || c.is_whitespace())
    {
        let mut decimal = BigInt::from(0);
        let res_str = complement_str
            .chars()
            .filter(|&c| c == '1' || c == '0')
            .collect::<String>();
        for (i, c) in res_str[1..].chars().rev().enumerate() {
            decimal += to_bit(c) * pow(BigInt::from(2), i);
        }
        decimal += &to_bit(res_str.chars().nth(0).unwrap())
            * pow(BigInt::from(2), res_str.len() - 1)
            * BigInt::from(-1);
        return Ok(decimal.to_string());
    }
    Err(String::from(
        "failed to convert decimal from complement because of illegal characters",
    ))
}

#[tauri::command]
pub fn process_binary_decimal(binary_str: &str) -> Result<String, String> {
    convert_to_decimal(binary_str, "2")
}

fn to_bit(c: char) -> BigInt {
    BigInt::from(if c == '1' { 1 } else { 0 })
}
