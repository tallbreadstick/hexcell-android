use crate::comp::{
    num_sys::{
        convert_from_decimal,
        convert_to_decimal
    },
    syn_ops::format_output
};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use std::str::FromStr;
use tauri;

#[tauri::command]
pub fn process_arithmetic(
    input1_str: &str,
    input2_str: &str,
    base_str: &str,
    operation: &str,
) -> Result<String, String> {
    match (
        BigDecimal::from_str(&convert_to_decimal(
            &input1_str
                .chars()
                .filter(|&c| c.is_alphanumeric() || c == '.')
                .collect::<String>(),
            base_str,
        )?),
        BigDecimal::from_str(&convert_to_decimal(
            &input2_str
                .chars()
                .filter(|&c| c.is_alphanumeric() || c == '.')
                .collect::<String>(),
            base_str,
        )?),
    ) {
        (Ok(input1), Ok(input2)) => {
            let res_num = match operation {
                "add" => &input1 + &input2,
                "multiply" => &input1 * &input2,
                "subtract" => {
                    let mut res = convert_from_decimal(&(&input1 - &input2).abs().to_string(), base_str)?;
                    format_from_base(&mut res, base_str);
                    if &input1 >= &input2 {
                        return Ok(res);
                    } else {
                        return Ok(format!("-{}", res.trim()));
                    }
                }
                "divide" => {
                    // handle returning a precise fractional or an integral with remainder
                    if input1_str.chars().any(|c| c == '.') || input2_str.chars().any(|c| c == '.')
                    {
                        &input1 / &input2
                    } else {
                        let (input1, input2) = (
                            BigInt::from_str(&convert_to_decimal(input1_str, base_str)?).unwrap(), // unwraps here should be safe because absence of fractionals has been confirmed
                            BigInt::from_str(&convert_to_decimal(input2_str, base_str)?).unwrap(),
                        );
                        let (mut res_str, rem) = (
                            convert_from_decimal(&(&input1 / &input2).to_string(), base_str)?,
                            (&input1 % &input2),
                        );
                        format_from_base(&mut res_str, base_str);
                        if rem != BigInt::from(0) {
                            let mut rem_str = convert_from_decimal(&rem.to_string(), base_str)?;
                            format_from_base(&mut rem_str, base_str);
                            res_str.push_str(&format!(
                                " r{}",
                                rem_str
                            ));
                        }
                        return Ok(res_str);
                    }
                }
                _ => {
                    return Err(String::from(
                        "arithmetic failed because of an invalid operator",
                    ))
                }
            };
            let mut res_str = convert_from_decimal(&res_num.to_string(), base_str)?;
            format_from_base(&mut res_str, base_str);
            return Ok(res_str);
        }
        _ => {
            return Err(String::from(
                "arithmetic failed because BigDecimal could not be parsed from inputs",
            ))
        }
    }
}

// DRY principle moment vvv

fn format_from_base(output: &mut String, base_str: &str) {
    match base_str {
        "2" => format_output(output, 4, ' '),
        "10" => format_output(output, 3, ','),
        "16" => format_output(output, 4, ' '),
        _ => {}
    }
}