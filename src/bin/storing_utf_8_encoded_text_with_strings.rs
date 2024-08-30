fn main() {
    // creating a new string
    let mut s = String::new();

    let data = "initial content";

    let s = data.to_string();

    let s = "initial content".to_string();

    let s = String::from("initial content");

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // updating a string
    // appending
    let mut s = String::from("foo");
    s.push_str("bar");
    
    let mut s = String::from("lo");
    s.push('l');
    
    // concatination
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    let s = format!("{s1}-{s2}-{s3}");

    // indexing string
    let s1 = String::from("hello");
    // let h = s1[0]; // cause error, rust string doesnt support indexing

    // internal representation
    let hello = String::from("Hola");

    let hello = String::from("Hola");

    let hello = "Здравствуйте";
    let answer = &hello[0];

    // slicing strings
    let hello = "Здравствуйте";

    let s = &hello[0..4];

    // methods for iterating over string
    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
    
}
