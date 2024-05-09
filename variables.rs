/*
Rust tiene que conocer el tipo de cada variable cuando tiene lugar
la compilacion, el compilador tambien puede deducir el tipo 
dependiendo del contexto

Las variables son inmutables, no se puede cambiar su valor posteriormente
*/

fn main() {
    println!("Hello, world!");
    println!("Mi primer programa en rust");

    //las variables son inmutables
    let o = 8_i64; // infiere el tipo de dato
    let e = 7_i32; //i es de integrer en este caso de 32 bits
    
    let mut re = 66_i32; //con mut las hago mutables
    re = 77_i32;  //le reasigno un valor
    println!(
        "El valor de re es: {re}"
    );
    
    let b = 85_i32;{
        let mut b = 87_i32;
        b = 666;
        println!("El valor de b es: {b}");
    }       
    println!("EL valor de b es {b}");

            //Constantes
    const PI: f32 = 31.1416; //palabra reservada + id + tipo de dato = result
    println!("EL valor de pi es {PI}");
    
            //Shadowing
    //Podemos crear una misma variable, para poder asignarse un nuevo valor, incluso un tipo distinto
    let u = 3;
    println!("EL valor de u sin cambios es {u}");
    
    let u = "d";
    println!("EL valor de u con cambios es {u}");
    
    let mut jn = "Luz"; // es de tipo carater
    println!("EL valor de jn como tipo string es {jn}");

    jn = "55";
    println!("EL valor de jn como tipo entero de 32 {jn}");
        
    let jn = jn.parse::<i32>().unwrap(); //shadowing, cambiando el tipo de dato a un entero.     

}
