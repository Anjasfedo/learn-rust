`cargo run --bin [filename]`

# Cargo basic
- We can create a project using cargo new.
- We can build a project using cargo build.
- We can build and run a project in one step using cargo run.
- We can build a project without producing a binary to check for errors using cargo check.
- Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.

## renew the cargo package
`cargo build`

## update will ignore the Cargo.lock file and figure out all the latest versions that fit your specifications in Cargo.toml
`cargo update`

## open doc
`cargo doc --open`

## Guess Game
This project was a hands-on way to introduce you to many new Rust concepts: let, match, functions, the use of external crates, and more. 