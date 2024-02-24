#[cfg(test)]
mod convertor_functions;
use convertor_functions::*;

#[test]
fn test_decimal_to_binary() {
    assert_eq!(decimal_to_binary_(255), "11111111");
}

#[test]
fn test_binary_to_c2() {
    assert_eq!(binary_to_c2("11111111"), "00000001");
}

#[test]
fn test_binary_sum() {
    assert_eq!(binary_sum("00000001".to_string(), "00000001".to_string()), "00000010");
}

#[test]
fn test_convetor_hexdec() {
    assert_eq!(convetor_hexdec(255), "FF");
}
