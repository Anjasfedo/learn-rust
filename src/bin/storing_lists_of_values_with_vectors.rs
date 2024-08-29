fn main() {
    // empty vector
    let v: Vec<i32> = Vec::new();

    // vector with values
    let v = vec![1, 2, 3];

    // updating vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // reading elements
    // via indexing or get method
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // let does_not_exist = &v[100]; // cause error
    // let does_not_exist = v.get(100); // return None

    // 

    // let mut v = vec![1, 2, 3, 4, 5];

    // let first = &v[0]; // immutable borrow occurs here

    // v.push(6);

    // println!("The first element is: {first}"); // immutable borrow later used here

    // 

    // iterating over values
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // iterate the mutable
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{i}");
    }

    // using an enum to store multiple types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // dropping a vector its element
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
}