fn main() {
    println!("Ejemplos de variables");
    let mut x =3;
    println!("El valor de x es {x}");
    //Si deseamos que una variable cambie de valor posteriormente habra que hacerlo del siguiente modo:
    x = 5;
    println!("El valor ahora de x es {x}");

    //================ Constantes ================//
    //TODO:A diferencia de las variables no pueden llevar la palabra mut y hayq que definir su tipo de dato obligatoriamente
    //Se pueden declarar en cualquier ambito, incluso global
    //Solo pueden almacenar un valor, no el resultado de un calculo
    //Ejemplo: const PI: f32 = 31.1416;

    const PI: f32 = 3.1416;
    println!("El valor de PI es: {}",PI);

    //Podemos volver a crear una misma variable, para poder asignarle un nuevo valor, incluso un tipo disntito
    let y = 7;
    println!("El valor de y es: {}",y);
    let y = "a";
    println!("El nuevo valor de y es: {}",y);


}
