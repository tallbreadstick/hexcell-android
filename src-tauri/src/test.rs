use rstest::rstest;

#[allow(unused_imports)]
use crate::comp::{
    num_sys::{
        convert_to_decimal,
        convert_from_decimal,
        process_conversion
    },
    syn_ops::{
        clamp_base,
        clamp_input
    },
    arithmetic::process_arithmetic,
    twos_comp::{
        process_complement,
        process_binary,
        process_complement_decimal,
        process_binary_decimal
    }
};

/*

THIS FILE CONTANS TEST CASES TO ENSURE VALIDITY OF ALL FUNCTIONS

Here I have primarily tested various math functions such as number system conversions,
arithmetic of numbers outside base-10, and converting between binary and twos complement.
This code does not get compiled during the release build, thanks to the #[rstest] macro,
which highlights various parts of the code to be tested during `cargo test` without building
them in the releases.

This file has saved me lots of time working on the backend without having to write test drivers
or any frontend implementations yet. Testing code is so much easier with Rust.

*/

#[rstest]

// from binary integrals only
#[case("1111", "2", "15")]
#[case("1010", "2", "10")]
#[case("1101", "2", "13")]

// from binary w fractionals
#[case("111.01", "2", "7.25")]
#[case("1011.0011", "2", "11.1875")]
#[case("0.111", "2", "0.875")]

// from octal integrals only
#[case("777", "8", "511")]
#[case("456", "8", "302")]
#[case("1234567", "8", "342391")]

// from octal w fractionals
#[case("707.00123", "8", "455.002532958984375")]
#[case("677.766", "8", "447.98046875")]
#[case("404.000007", "8", "260.000026702880859375")]

// from hex integrals only
#[case("9ffe", "16", "40958")]
#[case("7e2d5", "16", "516821")]
#[case("ff", "16", "255")]

// from hex w fractionals
#[case("ff.ff", "16", "255.99609375")]
#[case("a.a", "16", "10.625")]
#[case("1000.c", "16", "4096.75")]

pub fn ok_to_decimal_conversions(
    #[case] a: &str,
    #[case] b: &str,
    #[case] exp: &str
) {
    assert_eq!(convert_to_decimal(a, b), Ok(exp.to_string()));
}

#[rstest]

// tests that fail at u8 parsing from base
#[case("111", "-2", "conversion to decimal failed because u8 could not be parsed from base")]
#[case("1101", "v", "conversion to decimal failed because u8 could not be parsed from base")]
#[case("1234", ".", "conversion to decimal failed because u8 could not be parsed from base")]
#[case("777", "2eff", "conversion to decimal failed because u8 could not be parsed from base")]

// tests that fail due to an encountered non-alnum char
#[case("111,000.011,111", "2", "conversion to decimal failed because of a non-alphanumeric character")]
#[case("-45", "8", "conversion to decimal failed because of a non-alphanumeric character")]
#[case("!75", "16", "conversion to decimal failed because of a non-alphanumeric character")]
#[case("acd3%", "16", "conversion to decimal failed because of a non-alphanumeric character")]

pub fn err_to_decimal_conversions(
    #[case] a: &str,
    #[case] b: &str,
    #[case] exp: &str
) {
    assert_eq!(convert_to_decimal(a, b), Err(exp.to_string()));
}

#[rstest]

// to bin integrals only
#[case("15", "2", "1111")]
#[case("27", "2", "11011")]
#[case("34", "2", "100010")]

// to bin w fractionals
#[case("55.0625", "2", "110111.0001")]
#[case("255.009", "2", "11111111.0000001001001101110100101111000110101001111110111110011101101100")]
#[case("10.75", "2", "1010.11")]

// to octal integrals only
#[case("511", "8", "777")]
#[case("777", "8", "1411")]
#[case("123456789", "8", "726746425")]

// to octal w fractionals
#[case("670.109375", "8", "1236.07")]
#[case("99.875", "8", "143.7")]
#[case("100.22", "8", "144.1605075341217270243656050753412172702436560507534121727024365605")]

// to hex integrals only
#[case("255", "16", "FF")]
#[case("173", "16", "AD")]
#[case("40495", "16", "9E2F")]

// to hex w fractionals
#[case("255.99609375", "16", "FF.FF")]
#[case("34.0625", "16", "22.1")]
#[case("287.25", "16", "11F.4")]

// disclaimer: from_decimal tests assume a fractional precision of MAX 64, which may become variable in production
pub fn ok_from_decimal_conversions(
    #[case] a: &str,
    #[case] b: &str,
    #[case] exp: &str
) {
    assert_eq!(convert_from_decimal(a, b), Ok(exp.to_string()));
}

#[rstest]

// tests that fail at u8 parsing from base
#[case("39.72", "-2", "conversion from decimal failed because u8 could not be parsed from base")]
#[case("15", "2.5", "conversion from decimal failed because u8 could not be parsed from base")]
#[case("2871", "***", "conversion from decimal failed because u8 could not be parsed from base")]
#[case("104234", "2/11/2024", "conversion from decimal failed because u8 could not be parsed from base")]

// tests that fail because BigUint / BigDecimal could not parse from &str
// no tests here lmao bcus some errors here may never happen at runtime

pub fn err_from_decimal_conversions(
    #[case] a: &str,
    #[case] b: &str,
    #[case] exp: &str
) {
    assert_eq!(convert_from_decimal(a, b), Err(exp.to_string()));
}

#[rstest]

// conversions from base 2 to base 16
#[case("1111", "2", "16", "F")]
#[case("1101", "2", "16", "D")]
#[case("11111111", "2", "16", "FF")]
#[case("10010001", "2", "16", "91")]

// binary conversions with spacing
#[case("111 111 111", "2", "8", "777")]
#[case("1111 1111", "2", "16", "FF")]
#[case("1001 0110 1010 0011", "2", "16", "96A3")]
#[case("1011.01", "2", "10", "11.25")]

// conversions with comma separated digits
#[case("2,048", "10", "2", "1000 0000 0000")]
#[case("1,023,546", "10", "2", "1111 1001 1110 0011 1010")]
#[case("24,625", "10", "16", "6031")]
#[case("4,200,500", "10", "16", "40 1834")]

pub fn ok_process_conversion(
    #[case] input: &str,
    #[case] base_1: &str,
    #[case] base_2: &str,
    #[case] exp: &str
) {
    assert_eq!(process_conversion(input, base_1, base_2), Ok(exp.to_string()));
}

#[rstest]

// addition tests
#[case("11", "1100", "2", "add", "1111")]
#[case("1010", "101", "2", "add", "1111")]
#[case("123", "456", "8", "add", "601")]
#[case("777", "555", "8", "add", "1554")]
#[case("ABC", "DEF", "16", "add", "18AB")]
#[case("5", "4", "10", "add", "9")]

// subtraction tests
#[case("1111", "11", "2", "subtract", "1100")]
#[case("11", "1111", "2", "subtract", "-1100")]
#[case("777", "777", "8", "subtract", "0")]
#[case("115", "511", "8", "subtract", "-374")]
#[case("11", "1111", "16", "subtract", "-1100")]
#[case("4,672,122", "65,777", "10", "subtract", "4,606,345")]

// multiplication tests
#[case("111", "10", "2", "multiply", "1110")]
#[case("1101", "0", "2", "multiply", "0")]
#[case("77", "3", "8", "multiply", "275")]
#[case("777", "777", "8", "multiply", "776001")]
#[case("ABC", "DEF", "16", "multiply", "95 9184")]
#[case("11", "11", "10", "multiply", "121")]

// division tests
#[case("1100.01", "10", "2", "divide", "110. 001")]
#[case("1111", "100", "2", "divide", "11 r11")]
#[case("633", "7", "8", "divide", "72 r5")]
#[case("77.07", "7", "8", "divide", "11.01")]
#[case("FEFE", "A", "16", "divide", "197F r8")]
#[case("100.0", "8", "10", "divide", "12.5")]

pub fn ok_process_arithmetic(
    #[case] input1: &str,
    #[case] input2: &str,
    #[case] base: &str,
    #[case] op: &str,
    #[case] exp: &str
) {
    assert_eq!(process_arithmetic(input1, input2, base, op), Ok(exp.to_string()));
}

#[rstest]

#[case("1111", "1111 0001")]
#[case("1101", "1111 0011")]
#[case("1010", "1111 0110")]
#[case("0001", "1111 1111")]
#[case("0", "0")]

pub fn ok_process_complement(#[case] a: &str, #[case] exp: &str) {
    assert_eq!(process_complement(a), Ok(exp.to_string()));
}

#[rstest]

#[case("1111 0001", "1111")]
#[case("1111 1111", "1")]
#[case("1111 0110", "1010")]
#[case("1111 0011", "1101")]
#[case("0", "0")]

pub fn ok_process_binary(#[case] a: &str, #[case] exp: &str) {
    assert_eq!(process_binary(a), Ok(exp.to_string()));
}

#[rstest]

#[case("1111 0001", "-15")]
#[case("1111 0011", "-13")]
#[case("1111 0110", "-10")]
#[case("1111 1111", "-1")]
#[case("0", "0")]

pub fn ok_process_complement_decimal(#[case] a: &str, #[case] exp: &str) {
    assert_eq!(process_complement_decimal(a), Ok(exp.to_string()));
}

#[rstest]

#[case("2947001809034", "2", "1111001101011")]
#[case("zzz", "16", "FFF")]
#[case("ccc", "8", "777")]
#[case("0123456789ABCDEF", "10", "0123456789999999")]

pub fn ok_clamp_input(
    #[case] a: &str,
    #[case] b: &str,
    #[case] exp: &str
) {
    assert_eq!(clamp_input(a, b), Ok(exp.to_string()));
}