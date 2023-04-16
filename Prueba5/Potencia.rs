//Potencia de un numero con recursividad

fn main(){
    let numero: f32 = 5.0;
    let exponente: i8 = 3;
    println!("{} al cubo es: {}", numero, potencia(1, numero, exponente));
}

fn potencia(i:i8, x:f32, n:i8) -> f32{
    if i<n {
        return potencia(i+1, x, n)*x;
    }
    return x;
}