use std::default;

pub fn decimal_to_binary_(decimal: i32) -> String {
    let mut binary = String::new();
    let mut n = decimal;
    while n > 0 {
        binary = format!("{}{}", n % 2, binary); // agrega el residuo (el cual es un bit)
        n = n / 2; // divide el número entre 2 para obtener el siguiente residuo
    }
    return binary;
}

pub fn decimal_to_8bit_binary(decimal: i32) -> String {
    return format!("{:08b}", decimal); // convierte el número a binario y lo rellena con ceros a la izquierda hasta que tenga 8 bits
}

fn binary_to_decimal(binary: String) -> i32 {
    
    return i32::from_str_radix(&binary, 2).unwrap(); //multiplica cada bit por 2 elevado a la potencia de su posición y suma todos los resultados
}

pub fn binaryIEEE754_to_decimal(Binary: String) -> f32{
    let mut mantissa = 0;
    let mut sign = 1.0;
    let binary = Binary.chars().collect::<Vec<char>>();
    let exponent = binary_to_decimal(binary[1..9].iter().collect::<String>()); // usa la funcion de binary_to_decimal para convertir el exponente a decimal
    let exponent_rest = exponent - 127; // resta 127 al exponente para obtener el exponente real

    if binary.len() == 32 { 

        if binary[0] == '1' {
            sign = -1.0;
        }

        match exponent {
            0 => {
                return sign * (2.0f32.powf(-126.0)) * ((binary[9..32].iter().enumerate().fold(0.0, |acc, (i, x)| { // lo demismo de abajo aplica, solo se su exponente es -126
                    if *x == '1' {
                        acc + 2.0f32.powf((i + 1) as f32 * -1.0)
                    } else {
                        acc
                    }
                })));
            }
            _default =>{
                return sign * (2.0f32.powf(exponent_rest as f32)) * (1.0 + (binary[9..32].iter().enumerate().fold(0.0, |acc, (i, x)| { // suma 1 al principio de la mantissa
                    if *x == '1' {
                        acc + 2.0f32.powf((i + 1) as f32 * -1.0) // multiplica cada bit por 2 elevado a la potencia de su posición y suma todos los resultados
                    } else {
                        acc // si el bit es 0, no suma nada
                    }
                })));
            }
        }

    }
    else {
        return 0.0;
    }
}

pub fn decimalwithpoint_to_binaryIEEE754(decimal: f32) -> String {
    let mut sign = String::from("0");
    let exponent = decimal_to_binary_(decimal.floor() as i32);
    let mut despues_punto = decimal.abs() - decimal.floor();
    let mut mantissa = String::new();

    if decimal < 0.0 {
        sign = String::from("1");
    }

    /*
    Hace la operacion para obtener la parte decimal del número en binario
    */
    while ((despues_punto - despues_punto.floor()) != 0.0) && mantissa.len() < 23 { 
        despues_punto *= 2.0;
        mantissa.push_str(&despues_punto.floor().to_string());
        despues_punto = despues_punto - despues_punto.floor();
    }

    let mut normalized = format!("{}{}", exponent, mantissa); // concatena el exponente y la mantissa
    normalized.insert(1, '.');
    mantissa = normalized.chars().skip(2).collect(); // quita el punto y el primer bit del exponente para tener la mantissa

    while mantissa.len() < 23 { // rellena la mantissa con ceros hasta que tenga 23 bits
        mantissa.push('0');
    }

    let exponent = 127 + exponent.len() as i32 - 1;

    return format!("{}{}{}", sign, decimal_to_8bit_binary(exponent), mantissa);

}
