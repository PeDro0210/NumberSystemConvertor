pub fn decimal_to_binary_(decimal: i32) -> String {
    let mut binary = String::new();
    let mut n = decimal;
    while n > 0 {
        binary = format!("{}{}", n % 2, binary); // agrega el residuo (el cual es un bit)
        n = n / 2; // divide el número entre 2 para obtener el siguiente residuo
    }
    return binary;
}

pub fn binary_to_c2(binary: &str) -> String {
    let mut c1 = String::new();
    let c2: String = "00000001".to_string(); // solo aplicable para 8 bits
    for c in binary.chars() {
        // Invertir bits
        match c {
            '0' => c1.push('1'),
            '1' => c1.push('0'),
            _ => panic!("Invalid character in binary string"),
        }
    }
    return binary_sum(c1, c2);
}

pub fn binary_sum(a: String, b: String) -> String {
    let mut carry = 0;
    let mut sum = String::new();
    for (char1, char2) in a.chars().rev().zip(b.chars().rev()) {
        let bit1 = char1.to_digit(10).unwrap(); // convierte el caracter a un número
        let bit2 = char2.to_digit(10).unwrap(); // convierte el caracter a un número
        let result = bit1 + bit2 + carry;
        // Pushea bits dependiendo del resultado
        match result {
            0 => {
                sum.push('0');
                carry = 0;
            }
            1 => {
                sum.push('1');
                carry = 0;
            }
            2 => {
                sum.push('0');
                carry = 1;
            }
            3 => {
                sum.push('1');
                carry = 1;
            }
            _ => panic!("Invalid result"),
        }
    }
    return sum.chars().rev().collect::<String>();
}

pub fn convetor_hexdec(decimal: i32) -> String {
    let mut hex = String::new();
    let mut n = decimal;
    while n > 0 {
        let residuo = n % 16;
        if residuo < 10 {
            hex = format!("{}{}", residuo, hex); // agrega el residuo al inicio de la cadena
        } else {
            hex = format!("{}{}", (residuo + 55) as u8 as char, hex); // agrega el residuo al inicio de la cadena, convirtiendo el número a su representación en ASCII
        }
        n = n / 16;
    }
    return hex;
}

pub fn convetor_dechex(hex: &str) -> u32 {
    let mut decimal = 0;
    for (i, c) in hex.chars().rev().enumerate() {
        let n = match c.to_digit(16) {
            Some(n) => n,
            None => panic!("Invalid character in hex string"),
        };
        decimal += n * 16u32.pow(i as u32);
    }
    return decimal;
}

