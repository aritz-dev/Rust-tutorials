fn _ownership() {
    // --- Regla básica de Ownership ---
    // Cada valor tiene un único dueño.
    // Cuando ese dueño sale de scope, el valor se ‘droppea’.
    {
        let _s = String::from("hola"); // dueño "s"
        // s sigue siendo válido aquí
    } // se llama automáticamente a drop. s ya no existe.

    // --- Tipos simples (stack) vs tipos complejos (heap) ---
    let x = 5;
    let y = x; // x se copia. Ambos siguen siendo válidos.
    println!("x = {x}, y = {y}");

    let s1 = String::from("hola");
    let _s2 = s1; // s1 se mueve a s2. s1 ya no es válido.
    // println!("{}", s1); // error: uso de valor movido.

    // --- Clonación explícita ---
    let s3 = String::from("hola");
    let s4 = s3.clone(); // copia profunda (heap).
    println!("s3 = {s3}, s4 = {s4}");

    // --- Reasignación libera memoria previa ---
    let mut _s5 = String::from("hola");
    _s5 = String::from("adios"); // se llama drop sobre el "hola"
    println!("{_s5}");

    // --- Ownership en funciones ---
    let s6 = String::from("hola");
    takes_ownership(s6); // s6 se mueve a la función. Ya no válido aquí.
    // println!("{}", s6); // error.

    let x2 = 42;
    makes_copy(x2); // i32 implementa Copy, x2 sigue válido.
    println!("x2 = {x2}");

    // --- Return y transferencia ---
    let _s7 = gives_ownership(); // recibe ownership del valor devuelto.
    let s8 = String::from("mundo");
    let _s9 = takes_and_gives_back(s8); // s8 movido y devuelto como s9.

    // println!("{}", s8); // error: ya movido.

    // --- Función que calcula longitud devolviendo ownership + otro valor ---
    let s10 = String::from("longitud");
    let (s11, len) = calculate_length(s10);
    println!("La longitud de '{}' es {}.", s11, len);

    // --- Ideal: usar referencias para evitar mover ---
    // (Se explica más en el capítulo siguiente)
}

// Función que toma ownership y luego su valor se droppea al salir.
fn takes_ownership(s: String) {
    println!("{}", s);
} // aquí se droppea "s".

// Función que copia tipo Copy. El argumento sigue válido después.
fn makes_copy(n: i32) {
    println!("{}", n);
}

// Devuelve un String. Ownership se mueve al llamador.
fn gives_ownership() -> String {
    String::from("tuyo")
}

// Recibe String y lo devuelve. Ownership pasa a s9.
fn takes_and_gives_back(a: String) -> String {
    a
}

// Devuelve ownership y un valor adicional (longitud).
fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}
