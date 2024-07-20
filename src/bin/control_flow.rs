
fn main() {
    for_loop_range();
}

fn if_expression() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn must_bool() {
    // the condition in this code must be a bool
    let number = 3;

    // This will cause error
    // if number {
    //     println!("number was three");
    // }

    // Unlike languages such as Ruby and JavaScript, Rust will not automatically try to convert non-Boolean types to a Boolean. 
    // You must be explicit and always provide if with a Boolean as its condition

    if number != 0 {
        println!("number was something other than zero");
    }
}

fn multiple_condition() {
    let number  = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn inline_condition() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // the values that have the potential to be results from each arm of the if must be the same type
    // Cause error
    // let number = if condition { 5 } else { "six" };

    println!("The value of number is: {}", number);
}

fn repetition_loop() {
    // Cause infinite loop
    loop {
        println!("again!");
    }
}

fn return_value_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn label_to_multiple_loop() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    println!("End count = {}", count);
}

fn conditional_while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn collection_while_loop(){
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn collection_for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }

}

fn for_loop_range() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}