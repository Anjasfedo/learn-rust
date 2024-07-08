use std::{cmp::Ordering, io}; // standard input/output
use rand::Rng; // random number generator

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // generate random number

    println!("Secret number is: {secret_number}");

    loop {
        println!("Input your guess");

        let mut guess = String::new(); // mutable string (mutable is variable that can be changed)

        // let apples = 5; // immutable
        // let mut bananas = 5; // mutable

        io::stdin() // standard input
            .read_line(&mut guess) // read line from stdin, &mut for mutable string
            .expect("Failed to read line"); // handling error

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guess: {}", guess); // print the guess
        // println!("You guess: {guess}");

        // comparing guess with secret number
        match guess.cmp(&secret_number) { // cmp method compares two values and can be called on anything that can be compared
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}