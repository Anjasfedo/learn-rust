fn main() {
    let mut s = String::from("hewrow");

    let length = calculate_length(&s);

    println!("The length of '{}' is {}.", s, length);

    change(&mut s);

    println!("{}", s);

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);


    // {
    //     let r1 = &mut s;
    // } // r1 goes out of scope here, so we can make a new reference with no problems.

    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);


    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");

    let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }
fn dangle() -> String {
    let s = String::from("hello");

    s
}
