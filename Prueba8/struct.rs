struct Alumno{
    id: String,
    nombre: String,
    apellidos: String,
    edad: u8,
    regular: bool,
}

fn main(){
    let alumno1 = Alumno{
        id: String::from("342352"),
        nombre: String::from("Victor"),
        apellidos: String::from("Martinez"),
        edad: 23,
        regular: true,
    };

    println!("Alumno:\n");
    println!("Id: {}", alumno1.id);
    println!("Nombre: {}", alumno1.nombre);
    println!("Apellido: {}", alumno1.apellidos);
    println!("Edad:{}", alumno1.edad);
    println!("Alumno regular: {}", alumno1.regular);
}