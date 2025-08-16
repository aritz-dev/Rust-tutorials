/* 
Shadowing → reaprovechar un nombre para refinar o transformar su valor, 
            incluso cambiando de tipo.
*/
/*
Scopes {} → crear zonas temporales donde declarar variables sin que “contaminen” 
            el resto del programa, y para liberar memoria más pronto.
            usar scope si hay menos de 10 lineas, si hay mas convierte a funcion normal
*/
pub fn variables_and_mutability(){
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}