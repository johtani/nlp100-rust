#[macro_use]
extern crate log;
extern crate env_logger;
extern crate serde_json;
extern crate nlp100_rust;

use nlp100_rust::chapter01::answer;
use std::collections::BTreeMap;

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

    // 03
    let orig03 = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.";
    println!("---- 03 Pi");
    println!("pi(\"{}\") -> {}", orig03, format!("{:?}", &answer::pi(orig03)));

    // 04
    let orig04 = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.";
    let idx_one_symbols: Vec<usize> = vec![1, 5, 6, 7, 8, 9, 15, 16, 19];
    println!("---- 04 Chemical Element");
    println!("symbol_of_element(\"{}\", {}) -> ", orig04, format!("{:?}", &idx_one_symbols));
    print_map_to_json(answer::chemical_symbols(orig04, idx_one_symbols));

    // 05
    let orig05 ="I am an NLPer";
    println!("---- 05 N-gram (word, char)");
    println!("word_ngram(\"{}\", 2) -> {}", orig05, format!("{:?}", &answer::word_ngram(orig05, 2)));
    println!("char_ngram(\"{}\", 2) -> {}", orig05, format!("{:?}", &answer::char_ngram(orig05, 2)));

    // Chapter 02
    //println!("-- Chapter01");
    //println!("---- 01 Mix two string");


}

fn print_map_to_json(map: BTreeMap<String, usize>) {
    println!("{}", serde_json::to_string_pretty(&map).unwrap());
}
