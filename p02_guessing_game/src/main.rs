use std::io;
use std::cmp::Ordering;
use rand::prelude::*;
//Code that gets a guess from the user and prints it
fn main() {
    println!("Guess the number");
    let secret_number:u32=rand::rng().random_range(0..=10);
    println!("Secret number is: {secret_number}");  

    loop {
        println!("please input your guess.");
    
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line!!");
        //let guess :u32=guess.trim().parse().expect("opss");
        let guess:u32= match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Mete bien el numero"); 
                continue;
            },
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!(" YOU WIN!!");
                break;// stop loop
            }
        }
    } 
    
}
