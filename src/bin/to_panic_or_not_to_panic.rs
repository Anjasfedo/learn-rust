
use std::net::IpAddr;

fn main() {
    // examples, prototype code, and tests

    // case in wich you have information than the compiler
    let home: IpAddr = "127.0.0.1"
    .parse()
    .expect("Hardcoded IP address should be valid");

    // guidelines for error handling

    // creating custom types for validation
    // loop {
    //     let guess: &str = "2";

    //     let guess: i32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };

    //     if guess < 1 || guess > 100 {
    //         println!("The secret number will be between 1 and 100.");
    //         continue;
    //     }

    //     // match guess.cmp(&secret_number) {
    //     //     // --snip--
    // }

    // 

    pub struct Guess {
        value: i32,
    }
    
    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {value}.");
            }
    
            Guess { value }
        }
    
        pub fn value(&self) -> i32 {
            self.value
        }
    }

    let guess = Guess::new(200);
    println!("Guess: {}", guess.value());
}