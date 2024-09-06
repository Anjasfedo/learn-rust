// generic type parameters, trait bounds, and lifetimes together
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// lifetime annotation in struct definition
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// lifetime annotations in method definitions
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // preventing dangling references with lifetimes
    // let r;

    // {
    //     let x = 5;
    //     r = &x;
    // }

    // println!("r: {r}");

    // the borrow checker
    // let r;                // ---------+-- 'a
    //                       //          |
    // {                     //          |
    //     let x = 5;        // -+-- 'b  |
    //     r = &x;           //  |       |
    // }                     // -+       |
    //                       //          |
    // println!("r: {r}");   //          |
    //                       // ---------+

    // let x = 5;            // ----------+-- 'b
    //                       //           |
    // let r = &x;           // --+-- 'a  |
    //                       //   |       |
    // println!("r: {r}");   //   |       |
    //                       // --+       |
    //                       // ----------+

    // generic lifetimes in functions
    // let string1 = String::from("abcd");
    // let string2 = "xyz";

    // let result = longest(string1.as_str(), string2);
    // println!("The longest string is {result}");

    // lifetime annotation syntax
    // &i32        // a reference
    // &'a i32     // a reference with an explicit lifetime
    // &'a mut i32 // a mutable reference with an explicit lifetime

    // 
    // let string1 = String::from("long string is long");

    // {
    //     let string2 = String::from("xyz");
    //     let result = longest(string1.as_str(), string2.as_str());
    //     println!("The longest string is {result}");
    // }

    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {result}");

    // the static lifetime
    let s: &'static str = "I have a static lifetime.";
}

// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// lifetime annotation in function signature
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// thinking in terms of lifetimes
// fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//     x
// }

// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

// lifetime elision
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
