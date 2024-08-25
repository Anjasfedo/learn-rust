// fn main() {
//     println!("Hello, world!");
// }
// use garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = garden::vegetables::Asparagus {};
    println!("I'm growing {:?}", plant);
}