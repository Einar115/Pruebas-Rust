//Serie de taylor de seno de pi/4 con recursividad

fn main(){
    let x:f32 = 0.78539; //pi/4
    let i:f32 = 8.0; //numero de iteraciones
    let n:f32 = 3.0; //N de la serie
    let signo:f32 = -1.0; //Correccion de signo
    let senx:f32 = taylor(x, i, n, signo); //resultado de seno de pi/4
    println!("Seno de pi/4 es: {}", senx); //imprimir resultado
}

fn taylor(x:f32, i:f32, n:f32, signo:f32) -> f32{ //funcion de serie de taylor recursiva
    if n<i{
        return taylor(x,i,n+2.0,signo)+((potencia(1.0,x,n)*signo)/factorial(n));
    }
    return x;
}

fn potencia(i:f32, x:f32, n:f32) -> f32{ //funcion de la potencia recursiva
    if i<n {
        return potencia(i+1.0, x, n)*x;
    }
    return x;
}

fn factorial(i: f32) -> f32{ //funcion del factorial recursiva
    if i>1.0 {
        return factorial(i-1.0)*i;
    }
    return 1.0;
}