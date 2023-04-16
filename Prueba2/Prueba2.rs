//Perimetro y area de un circulo

fn main(){
    const PI: f32 = 3.1416;
    let diam: f32 = 20.0;
    let area: f32; 
    let peri: f32;
    peri=2.0*PI*(diam/2.0);
    area=PI*((diam/2.0)*(diam/2.0));

    println!("El perimetro del circulo es de: {}", peri);
    println!("El area del circulo es de : {}", area);
}