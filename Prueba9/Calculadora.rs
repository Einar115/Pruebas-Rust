use std::io;

fn main(){
    loop{
        let mut opc=String::new();
        let num1: f32=25.0;
        let num2: f32=5.0;

        println!("\nCalculadora\n");
        println!("Sumar........................1");
        println!("Restar.......................2");
        println!("Multiplicar..................3");
        println!("Dividir......................4");
        println!("Salir........................5");
        println!("Opcion:                       ");

        io::stdin().read_line(&mut opc).expect("Error al leer la entrada");

        match opc.trim().parse(){
            Ok(opc) => {
                match opc{
                    1 => {
                        println!("El resultado de la suma de {} y {} es: {}", num1, num2, sumar(num1,num2));
                    },
                    2 => {
                        println!("El resultado de la resta de {} y {} es: {}", num1, num2, restar(num1,num2));
                    },
                    3 => {
                        println!("El resultado de la multiplicacion de {} y {} es: {}", num1, num2, multiplicar(num1,num2));
                    },
                    4 => {
                        println!("El resultado de la division de {} y {} es: {}", num1, num2, dividir(num1,num2));
                    },
                    5 => {
                        println!("Saliendo...");
                    },
                    _ => {
                        println!("Opción inválida. Por favor, selecciona una opción válida.");
                    }
                }
            },
            Err(_) => {
                println!("Error al parsear la entrada. Por favor, ingresa un número válido.");
            }
        }
    }
}

fn sumar(num1:f32, num2:f32) -> f32{
    return num1+num2;
}

fn restar(num1:f32, num2:f32) -> f32{
    return num1-num2;
}

fn multiplicar(num1:f32, num2:f32) -> f32{
    return num1*num2;
}

fn dividir(num1:f32, num2:f32) -> f32{
    return num1/num2;
}