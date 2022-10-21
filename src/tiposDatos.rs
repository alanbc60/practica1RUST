fn main() {
    // Usando mut, puede cambiar el valor. En algunos casos, una variable 
    // mutable es más conveniente que una implementación que solo usa variables inmutables
    //Operaciones numericas
        // suma
        let mut a = 5;
        let mut b = 10; 
        let suma = a + b;
        println!("La suma de {} + {} es: {}",a,b,suma);

        // resta
        a = 98;
        b = 4;
        let resta = a - b;
        println!("La resta de {} + {} es: {}",a,b,resta);
        // multiplicacion
        a = 4;
        b = 30;
        let producto = 4 * 30;
        println!("El producto de {} + {} es: {}",a,b,producto);
        //Division
        a = 56;
        b = 32;
        let division = 56.7 / 32.2;
        println!("La division de {} + {} es: {}",a,b,division);
        //Division 2
        a = 2;
        b = 3;
        let _division2 = 2 / 3; // Results in 0
        println!("La division de {} + {} es: {}",a,b,division);
        // residuo
        a = 43;
        b = 5;
        let residuo = 43 % 5;
        println!("El residuo de {} + {} es: {}",a,b,residuo);

        //
        let tup = (500, 6.4, 1);

        let (x, y, z) = tup;
    
        println!("El valor de y es: {y}");
}
