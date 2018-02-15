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
    let orig00 = "stressed";
    println!("---- 00 Reverse characters");
    println!("reverse_str(\"{}\") -> {}", orig00, answer::reverse_str(orig00));

    // 01
    let orig01 = "パタトクカシーー";
    println!("---- 01 Odd index characters");
    println!("odd_idx_str(\"{}\") -> {}", orig01, answer::odd_idx_str(orig01));

    // 02
    let orig02_1 = "パトカー";
    let orig02_2 = "タクシー";
    println!("---- 02 Mix two string");
    println!("mix_two_str(\"{}\", \"{}\") -> {}", orig02_1, orig02_2, answer::mix_two_str(orig02_1, orig02_2));

    // Chapter 02
    //println!("-- Chapter01");
    //println!("---- 01 Mix two string");


}
