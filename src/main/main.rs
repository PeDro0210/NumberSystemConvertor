use std::io;
mod convertor_functions;
use convertor_functions::*;


    // Ejemplo de uso de las funciones

fn main() {
    println!("1. Convertir un binarioIEEE754 a decimal\n2. Convertir un decimal a binarioIEEE754");
    println!("Ingrese una opción: ");
    let mut option = String::new();
    io::stdin().read_line(&mut option).expect("Failed to read line");
    let option: u32 = option.trim().parse().expect("Please type a number!");

    match option {
        1 => {
            println!("Ingrese un número binarioIEEE754: ");
            let mut binary = String::new();
            io::stdin().read_line(&mut binary).expect("Failed to read line");
            let binary = binary.trim().to_string();
            let decimal = binaryIEEE754_to_decimal(binary);
            println!("El número decimal es: {}", decimal);
        }
        2 => { 
            println!("Ingrese un número decimal: ");
            let mut decimal = String::new();
            io::stdin().read_line(&mut decimal).expect("Failed to read line");
            let decimal: f32 = decimal.trim().parse().expect("Please type a number!");
            let binary = decimalwithpoint_to_binaryIEEE754(decimal);
            println!("El número binarioIEEE754 es: {}", binary);
        }
        _ =>
            println!("Opción no válida")
    }

}

/*
Rust esta chido, but boilerplate gos brrrrrr
*/