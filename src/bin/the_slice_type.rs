fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    // s.clear(); // empties the string, make it equal to ""

    println!("The first word is: {} / {}", word, s);

    // let s = String::from("hello world");

    // let len = s.len();

    // let hello = &s[0..5];
    // let world = &s[6..len];

    // println!("{} {}", hello, world);

    // let slice = &s[..];

    // println!("{}", slice);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
