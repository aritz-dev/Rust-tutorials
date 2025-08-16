pub fn is_equal(par1:u8,par2:u8)->bool{

    if par1 == par2 {
        println!("condition was true");
        true
    } else {
        println!("condition was false");
        false
    }
}

// Rust has three kinds of loops: loop, while, and for
pub fn loop_counter(){
    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("{counter}");
        if counter == 10 {
            break counter * 2; // para que al terminar el loop devuelva algo
        }
    };

    println!("The result is {result}");
}

pub fn while_counter(){
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

pub fn for_fn(){
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
    println!("===========");
    for number in (1..4).rev() { // rev to reverse the range: 1..4 no funciona, hay que usar .rev
        println!(" a: {number}!");
    }
    println!("LIFTOFF!!!");
}