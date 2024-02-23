fn main(){
    // Ejemplo de uso de las funciones
    let a:i32 = 7;
    let binary_a = decimal_to_binary(a);
    println!("El número {} en binario es {}", a, binary_a);

    let binary_a_kimberly = decimal_to_binary_(a);
    println!("El número {} en binario es {}", a, binary_a_kimberly);

    let _c2_a = binary_to_c2(&binary_a);
    println!("El complemento a 2 de {} es {}", binary_a, _c2_a);

    let b = 42;
    let hex_b = convetor_hexdec(b);
    println!("El número {} en hexadecimal es {}", b, hex_b);

    let c = "2A";
    let decimal_c = convetor_dechex(c);
    println!("El número {} en decimal es {}", c, decimal_c);
}

// Easy way
fn decimal_to_binary(decimal: i32) -> String {
    // castea el número a binario, por medio de formateo 
    if decimal > 255 {
        panic!("El número debe ser menor a 255");
    }
    return format!("{:08b}", decimal)
}

fn decimal_to_binary_(decimal: i32) -> String {
    // castea el número a binario, por medio de divisiones
    let mut binary = String::new();
    let mut n = decimal;
    while n > 0 {
        // Residuo
        binary = format!("{}{}", n % 2, binary);
        // división 
        n = n / 2;
    }
    return binary;
}

fn binary_to_c2(binary: &str) -> String {
    // convierte el binario a complemento a 2
    let mut c1 = String::new();
    let c2:String = "00000001".to_string(); // 1 en binario 8 bits
    for c in binary.chars() {
        match c {
            '0' => c1.push('1'),
            '1' => c1.push('0'),
            _ => panic!("Invalid character in binary string"),
        }
    }
    return binary_sum(c1, c2);
}

fn binary_sum(a: String, b: String) -> String {
    let mut carry = 0;
    let mut sum = String::new();
    // agarra las dos cifras, las sumas con el carry y despues poner el resultado en binario dependiendo la respuesta
    for (char1, char2) in a.chars().rev().zip(b.chars().rev()) {
        let bit1 = char1.to_digit(10).unwrap(); // convierte el caracter a un número
        let bit2 = char2.to_digit(10).unwrap(); // convierte el caracter a un número
        let result = bit1 + bit2 + carry;
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

fn convetor_hexdec(decimal: i32) -> String {
    // convierte el número a hexadecimal
    let mut hex = String::new();
    let mut n = decimal;
    while n > 0 {
        let residuo = n % 16;
        if residuo < 10 {
            hex = format!("{}{}", residuo, hex);
        } else {
            hex = format!("{}{}", (residuo + 55) as u8 as char, hex);
        }
        n = n / 16;
    }
    return hex;
}

fn convetor_dechex(hex: &str) -> u32 {
    // convierte el hexadecimal a decimal
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


/*
Queriamos usar mas funciones usando std the Rust, pero para que no se viera tan lleno preferimos que no 
*/