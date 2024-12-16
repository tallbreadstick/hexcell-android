use crate::comp::{
    list::LinkedList as Stack,
    num_sys::{bigdecimal_to_char, biguint_to_char, to_digit},
    syn_ops::split_input,
};
use bigdecimal::BigDecimal;
use num_bigint::BigUint;
use num_traits::{pow, ToPrimitive};
use serde::Serialize;
use std::str::FromStr;
use tauri;

#[derive(Serialize)]
pub struct ToSteps {
    pub digit: char,
    pub base: u8,
    pub place: i8,
    pub res: String,
}

#[derive(Serialize)]
pub struct FromSteps {
    pub num: String,
    pub op: char,
    pub base: u8,
    pub res: String,
    pub rem: u8,
}

#[derive(Serialize)]
pub struct Steps {
    pub to_steps: Vec<ToSteps>,
    pub from_steps: Vec<FromSteps>,
}

// defined to explicitly specify a string being used as a json string
type JsonStr = String;

#[tauri::command]
pub fn process_steps(input: &str, base1_str: &str, base2_str: &str) -> Result<JsonStr, String> {
    let (mut to_steps, mut from_steps) = (Vec::<ToSteps>::new(), Vec::<FromSteps>::new());
    convert_from_decimal_steps(
        &mut from_steps,
        &convert_to_decimal_steps(
            &mut to_steps,
            &input
                .chars()
                .filter(|&c| c.is_alphanumeric() || c == '.')
                .collect::<String>(),
            base1_str)?,
        base2_str,
    )?;
    let jstr = serde_json::json!(Steps {
        to_steps,
        from_steps
    });
    Ok(jstr.to_string())
}

pub fn convert_to_decimal_steps(
    steps: &mut Vec<ToSteps>,
    input: &str,
    base_str: &str,
) -> Result<String, String> {
    let mut decimal = BigDecimal::from(0);
    if let Ok(base) = base_str.parse::<u8>() {
        let input_vec = split_input(input);
        if input_vec.len() > 1 {
            decimal += to_decimal_fractional_steps(steps, input_vec[1], base)?;
        }
        decimal += to_decimal_integral_steps(steps, input_vec[0], base)?;
        return Ok(decimal.to_string());
    }
    Err(String::from(
        "conversion to decimal failed because u8 could not be parsed from base",
    ))
}

pub fn convert_from_decimal_steps(
    steps: &mut Vec<FromSteps>,
    decimal: &str,
    base_str: &str,
) -> Result<(), String> {
    if let Ok(base) = base_str.parse::<u8>() {
        let mut res_str = String::new();
        let input_vec = split_input(decimal);
        from_decimal_integral_steps(steps, &mut res_str, input_vec[0], base)?;
        if input_vec.len() > 1 {
            from_decimal_fractional_steps(steps, &mut res_str, input_vec[1], base)?;
        }
        return Ok(());
    }
    Err(String::from(
        "conversion from decimal failed because u8 could not be parsed from base",
    ))
}

fn to_decimal_integral_steps(
    steps: &mut Vec<ToSteps>,
    integral_str: &str,
    base: u8,
) -> Result<BigDecimal, String> {
    let mut decimal = BigDecimal::from(0);
    for (i, c) in integral_str.chars().rev().enumerate() {
        if !c.is_alphanumeric() {
            return Err(String::from(
                "conversion to decimal failed because of a non-alphanumeric character",
            ));
        }
        let res = to_digit(c) * pow(BigDecimal::from(base), i);
        steps.push(ToSteps {
            digit: c,
            base,
            place: i as i8,
            res: res.to_string(),
        });
        decimal += res;
    }
    Ok(decimal)
}

fn to_decimal_fractional_steps(
    steps: &mut Vec<ToSteps>,
    fractional_str: &str,
    base: u8,
) -> Result<BigDecimal, String> {
    let mut decimal = BigDecimal::from(0);
    let mut stack = Stack::<ToSteps>::new();
    for (i, c) in fractional_str.chars().enumerate() {
        if !c.is_alphanumeric() {
            return Err(String::from(
                "conversion to decimal failed because of a non-alphanumeric character",
            ));
        }
        let res = to_digit(c) * BigDecimal::from(1) / pow(BigDecimal::from(base), i + 1);
        stack.push(ToSteps {
            digit: c,
            base,
            place: (i + 1) as i8 * -1,
            res: res.to_string(),
        });
        decimal += res;
    }
    while let Some(step) = stack.pop() {
        steps.push(step);
    }
    Ok(decimal)
}

fn from_decimal_integral_steps(
    steps: &mut Vec<FromSteps>,
    res_str: &mut String,
    integral_str: &str,
    base: u8,
) -> Result<(), String> {
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
            let res = &integral / &BigUint::from(base);
            steps.push(FromSteps {
                num: integral.to_string(),
                op: '/',
                base,
                res: res.to_string(),
                rem: rem.to_u8().unwrap(), // vey likely to be safe ?
            });
            integral = res;
        }
        while let Some(c) = stack.pop() {
            res_str.push(c);
        }
        return Ok(());
    }
    Err(String::from(
        "conversion failed because BigUint could not be parsed from &str",
    ))
}

fn from_decimal_fractional_steps(
    steps: &mut Vec<FromSteps>,
    res_str: &mut String,
    fractional_str: &str,
    base: u8,
) -> Result<(), String> {
    if let Ok(mut fractional) = BigDecimal::from_str(&format!("0.{}", fractional_str)) {
        res_str.push('.');
        let (mut places, limit) = (0u8, 64u8); // 64 is hard-coded but may be variable later on
        while &fractional != &BigDecimal::from(0) && places < limit {
            let num = BigDecimal::clone(&fractional);
            fractional *= &BigDecimal::from(base);
            let digit = fractional.with_scale(0);
            if let Some(c) = bigdecimal_to_char(&digit) {
                res_str.push(c);
            } else {
                return Err(String::from(
                    "conversion from decimal failed because i8 could not be parsed from BigUint",
                ));
            }
            let res = &fractional - &digit;
            steps.push(FromSteps {
                num: num.to_string(),
                op: '*',
                base,
                res: res.to_string(),
                rem: digit.to_u8().unwrap(), // should be safe to unwrap because fractionals are truncated?
            });
            fractional = res;
            places += 1;
        }
        return Ok(());
    }
    Err(String::from(
        "conversion failed because BigDecimal could not be parsed from &str",
    ))
}
