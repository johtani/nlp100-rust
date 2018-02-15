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
    // 00
    let orig = "stressed";
    println!("---- 00 Reverse characters");
    println!("reverse_str(\"{}\") -> {}", orig, answer::reverse_str(orig));
    // 01
    let orig1 = "パトカー";
    let org2 = "タクシー";
    println!("---- 01 Mix two string");
    println!("mix_two_str(\"{}\") -> {}", orig, answer::mix_two_str(orig1, orig2));

    // Chapter 02
    println!("---- 01 Mix two string");


}
