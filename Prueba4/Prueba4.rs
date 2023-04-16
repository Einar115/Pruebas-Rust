//funciones factorial

fn main(){
    let x = 7;
    println!("El factorial de {} es: {}", x, factorial(x));
}

fn factorial(i: i32) -> i32{
    if i>1 {
        return factorial(i-1)*i;
    }
    return 1;
}