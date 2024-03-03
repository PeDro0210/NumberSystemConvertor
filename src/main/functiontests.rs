#[cfg(test)]
mod convertor_functions;
use convertor_functions::*;

#[test]
fn test_decimal_to_binary() {
    assert_eq!(decimal_to_binary_(255), "11111111");
}

#[test]
fn test_binaryIEEE754_to_decimal(){
    assert_eq!(binaryIEEE754_to_decimal("11000000000000000000000000000000".to_string()), -2.0);
}

#[test]
fn test_decimalwithpoint_to_binaryIEEE754(){
    assert_eq!(decimalwithpoint_to_binaryIEEE754(5.5), "01000000101100000000000000000000");
}