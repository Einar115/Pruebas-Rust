//vectores y operaciones de cada uno de sus elementos

fn main(){
    let mut vector = vec![56,32,12,7,9,1,5];
    
    println!("Longitud del vector es de: {}", vector.len());
    println!("\nElementos del vector: \n");
    for i in &vector {
        println!("{}", i);
    }

    println!("\nElementos del vector mas 10:\n");
    for i in &mut vector {
        *i +=10;
    }
    
    for i in &vector {
        println!("{}", i);
    }

    println!("\nElementos del vector menos 10:\n");
    for i in &mut vector {
        *i -=10;
    }
    
    for i in &vector {
        println!("{}", i);
    }
    println!("\nElementos del vector multiplicados por 10:\n");
    for i in &mut vector {
        *i *=10;
    }
    
    for i in &vector {
        println!("{}", i);
    }

    println!("\nElementos del vector divididos entre 10;\n");
    for i in &mut vector {
        *i /=10;
    }
    
    for i in &vector {
        println!("{}", i);
    }
}