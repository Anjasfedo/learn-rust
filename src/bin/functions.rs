

fn main() {
    println!("Hello World");

    another_function(5);

    print_labeled_measurement(2, 'h');

    statement_expression();

    let x = five();
    println!("The value of x is: {x}");

    let y = plus_one(5);
    println!("The value of y is: {y}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}")
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn statement_expression() {
    // let y = 6;
    // Statements do not return values
    // you can’t assign a let statement to another variable, as the following code tries to do; you’ll get an error
    // let x = (let y = 7);

    // Expression returns a value
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    // if we place a semicolon at the end of the line containing x + 1
    // changing it from an expression to a statement, we’ll get an error
    x + 1
}