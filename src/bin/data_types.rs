use std::io;

fn main() {
    // let guess: u32 = "42".parse().expect("Not a number!");
    // println!("You guessed: {}", guess);

    // compound_type();

    invalid_element_access();
}

fn scalar_type() {
        // integer: i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize
    // i8 = -128 - 127
    // u8 = 0 - 255
    // i16 = -32768 - 32767
    // u16 = 0 - 65535

    // isize and usize types depend on the architecture of the computer program is running on

    // float
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    // f32 type is a single-precision float, and f64 has double precision

    // Numeric operaion

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // Boolean
    let t = true;
    let f: bool = false; // explicit type annotation

    // Character
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
}

fn compound_type() {
    // Tuple
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    // Tuples are used to return multiple values from a function
    let tup = (500, 6.4, 1);

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;
    println!("The value of one is: {}", one);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    // Array
    // Unlike a tuple, every element of an array must have the same type
    // Unlike arrays in some other languages, arrays in Rust have a fixed length
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    // let _invalid = a[6]; // invalid index; index out of bounds: the length is 5 but the index is 6
    println!("a[0]: {}, a[1]: {}", first, second);

    let arr = [3; 5];
    println!("arr: {:?}", arr);

    // Slice
    let slice = &a[1..3];
    println!("slice: {:?}", slice);
}

fn invalid_element_access() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}