#[macro_use]
extern crate log;
extern crate env_logger;
extern crate nlp100_rust;

use nlp100_rust::chapter01::answer;

fn main() {
    env_logger::init();
    info!("start!!");

    // Chapter 01
    println!("-- Chapter01");
    let orig = "stressed";
    println!("---- 00 Reverse characters");
    println!("reverse_str(\"{}\") -> {}", orig, answer::reverse_str(orig));
    println!("---- 01 Mix two string");

    // Chapter 02

}
