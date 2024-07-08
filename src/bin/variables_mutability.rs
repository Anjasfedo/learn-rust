fn main() {
    // variables_and_mutability();

    // constant();

    shadowing();
}

fn variables_and_mutability() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6; // if line 2 not have `mut` will get cannot assign twice to immutable variable
    println!("The value of x is: {x}");
}

fn constant() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}

fn shadowing() {
    let x = 5;

    let x = x + 1; // change variable value by shadowing (create new variable with same name)

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // shadowing
    // let spaces = "   ";
    // let spaces = "abc";

    // mutable reference
    // let mut spaces = "   ";
    // spaces = "abc";

    // println!("{spaces}")
}