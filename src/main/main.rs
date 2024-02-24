use std::io;
mod convertor_functions;
use convertor_functions::*;
    // Ejemplo de uso de las funciones
fn main() {
    print!("Bienvenido al programa de conversiones\n");
    loop {
        print!("1. Decimal a binario\n2. Binario a complemento a 2\n3. Decimal a hexadecimal\n4. Hexadecimal a decimal\n5. Salir\n");
        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("No se pudo leer");
        let option: i32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Opción inválida");
                continue;
            }
        };
        match option {
                        1 => {
                            print!("\x1B[2J\x1B[1;1H"); // Limpiar terminal
                            print!("Ingrese el número decimal: ");
                            let mut decimal = String::new();
                            io::stdin().read_line(&mut decimal).expect("No se pudo leer");
                            let decimal: i32 = match decimal.trim().parse() {
                                Ok(num) => num,
                                Err(_) => {
                                    println!("Número inválido");
                                    continue;
                                }
                            };
                            println!("El número {} en binario es {}", decimal, decimal_to_binary_(decimal));
                        }
            2 => {
                print!("\x1B[2J\x1B[1;1H"); // Limpiar terminal
                print!("Ingrese el número binario: ");
                let mut binary = String::new();
                io::stdin().read_line(&mut binary).expect("No se pudo leer");
                binary = binary.trim().to_string();
                println!("El complemento a 2 de {} es {}", binary, binary_to_c2(&binary));
            }
            3 => {
                print!("\x1B[2J\x1B[1;1H"); // Limpiar terminal
                print!("Ingrese el número decimal: ");
                let mut decimal = String::new();
                io::stdin().read_line(&mut decimal).expect("No se pudo leer");
                let decimal: i32 = match decimal.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Número inválido");
                        continue;
                    }
                };
                println!("El número {} en hexadecimal es {}", decimal, convetor_hexdec(decimal));
            }
            4 => {
                print!("\x1B[2J\x1B[1;1H"); // Limpiar terminal
                print!("Ingrese el número hexadecimal: ");
                let mut hex = String::new();
                io::stdin().read_line(&mut hex).expect("No se pudo leer");
                hex = hex.trim().to_string();
                println!("El número {} en decimal es {}", hex, convetor_dechex(&hex));
            }
            5 => {
                break;
            }
            _ => {
                println!("Opción inválida");
            }
        }
        // Clear terminal
    }
}

/*
Rust esta chido, but I have boilerplate
*/