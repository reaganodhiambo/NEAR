#![allow(unused)]
use std::io;
// use std::io::Write;
// use std::io::ErrorKind;

#[warn(dead_code)]
// #[warn(unused_must_use)]
fn main() {
    write_message();
}
fn write_message() -> io::Result<()>{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("You typed: {}", input.trim());
    Ok(())

}