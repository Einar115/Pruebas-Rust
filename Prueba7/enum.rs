enum Numeros{
    cero,
    uno,
    dos,
}

enum Color{
    Rojo=0xff0000,
    Verde=0x00ff00,
    Azul=0x0000ff,
}

fn main(){
    println!("cero es {}", Numeros::cero as i8);
    println!("uno es {}", Numeros::uno as i8);

    println!("Las rosas son #{:06x}", Color::Rojo as i32);
    println!("El cielo es #{:06x}", Color::Azul as i32);
}