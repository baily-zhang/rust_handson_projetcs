use std::env;

/// Simple program that prints all environment variables.
fn main() {
    for (key, value) in env::vars() {
        println!("{key}={value}");
    }
}
