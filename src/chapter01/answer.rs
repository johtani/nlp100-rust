use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::iter::FromIterator;
use std::str::from_utf8;

// ch01 準備運動 - https://nlp100.github.io/ja/ch01.html
// ch01-00 文字列の逆順
pub fn reverse_str(original: &str) -> String {
    if original.len() > 0 {
        String::from_iter(original.chars().rev())
    } else {
        String::new()
    }
}

// ch01-01 「パタトクカシーー」 -> 「パトカー」
pub fn odd_idx_str(original: &str) -> String {
    let iter = original
        .chars()
        .enumerate()
        .filter(|(i, _x)| i % 2 == 0)
        .map(|(_i, x)| x);
    String::from_iter(iter)
}

// ch01-02 「パトカー」＋「タクシー」＝「パタトクカシーー」
pub fn mix_two_str(first_str: &str, second_str: &str) -> String {
    // TODO how to handle arrays if they don't have same length? error?
    let mut mixed = String::from_iter(
        first_str
            .chars()
            .zip(second_str.chars())
            .map(|(x, y)| format!("{}{}", x, y)),
    );
    // adjust characters if the length is different
    if first_str.chars().count() > second_str.chars().count() {
        first_str
            .chars()
            .skip(second_str.chars().count())
            .for_each(|x| mixed.push(x));
    } else if second_str.chars().count() > first_str.chars().count() {
        second_str
            .chars()
            .skip(first_str.chars().count())
            .for_each(|x| mixed.push(x));
    }
    return mixed;
}

// ch01-03 円周率
pub fn pi(original: &str) -> Vec<usize> {
    // TODO how to handle "."?
    original
        .split_whitespace()
        .map(|word| {
            word.chars()
                .filter(|x| x.is_alphabetic())
                .collect::<Vec<char>>()
                .len()
        })
        .collect::<Vec<usize>>()
}

// ch01-04 元素記号
pub fn chemical_symbols(sentence: &str, idx_one_symbols: Vec<usize>) -> BTreeMap<String, usize> {
    // FIXME how to handle unsorted idx_one_symbols? we should sort it first?
    let mut symbols: BTreeMap<String, usize> = BTreeMap::new();
    sentence
        .split_whitespace()
        .enumerate()
        .for_each(|(idx, word)| {
            let idx = idx + 1;
            if idx_one_symbols.contains(&idx) {
                symbols.insert(String::from_iter(word.chars().take(1)), idx);
            } else {
                symbols.insert(String::from_iter(word.chars().take(2)), idx);
            }
        });
    return symbols;
}

// ch01-05 n-gram - word
pub fn word_ngram(text: &str, n: usize) -> Vec<Vec<String>> {
    return text
        .split_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .windows(n)
        .map(|x| Vec::from(x.to_vec()))
        .collect::<Vec<Vec<String>>>();
}

// ch01-05 n-gram - char
pub fn char_ngram(text: &str, n: usize) -> Vec<String> {
    return text
        .chars()
        .collect::<Vec<char>>()
        .windows(n)
        .map(|x| String::from_iter(x.to_vec()))
        .collect::<Vec<String>>();
}

// ch01-06 char bi-gram set operations
pub fn char_ngram_set(text: &str, n: usize) -> BTreeSet<String> {
    let mut ngram_set = BTreeSet::new();
    char_ngram(text, n).iter().for_each(|x| {
        ngram_set.insert(x.to_string());
    });
    return ngram_set;
}

// char01-06 union
pub fn union_ngram_sets(
    source_set: BTreeSet<String>,
    target_set: &BTreeSet<String>,
) -> BTreeSet<String> {
    return source_set
        .union(target_set)
        .cloned()
        .collect::<BTreeSet<String>>();
}
// char01-06 intersection
pub fn intersection_ngram_sets(
    source_set: BTreeSet<String>,
    target_set: &BTreeSet<String>,
) -> BTreeSet<String> {
    return source_set
        .intersection(target_set)
        .cloned()
        .collect::<BTreeSet<String>>();
}
// char01-06 difference source_set - target_set
pub fn difference_ngram_sets(
    source_set: BTreeSet<String>,
    target_set: &BTreeSet<String>,
) -> BTreeSet<String> {
    return source_set
        .difference(target_set)
        .cloned()
        .collect::<BTreeSet<String>>();
}

// ch01-07
pub fn generate_sentence(x: i32, y: &str, z: f32) -> String {
    return format!("{}時の{}は{:?}", x, y, z);
}

// ch01-08
pub fn cipher(text: &str) -> String {
    return String::from_iter(text.chars().map(|x| {
        if x.is_ascii_alphanumeric() && x.is_lowercase() {
            let mut b = [0; 2];
            x.encode_utf8(&mut b);
            b[0] = 219 - b[0];
            char::from(b[0])
        } else {
            x
        }
    }));
}

// ch01-09
pub fn typoglycemia(text: &str) -> String {
    return text.split_whitespace().map(|word| {
        if word.len() <= 4 {
            word.to_string()
        } else {
            let original = word.chars().collect::<Vec<char>>();
            let first = original.get(0).unwrap();
            let last = original.last().unwrap();
            let mut typo =
                original[1..original.len() - 1].iter().map(|x| x.clone()).collect::<Vec<char>>();
            let mut rng = thread_rng();
            typo.shuffle(&mut rng);
            let mut typo = String::from_iter(typo.iter());
            typo.insert(0,first.clone());
            typo.push(last.clone());
            typo
        }
    }).collect::<Vec<String>>().join(" ");
}

// -- Unit test -----
#[cfg(test)]
mod tests {
    use chapter01::answer;
    use chapter01::answer::{cipher, typoglycemia};
    use std::collections::BTreeMap;
    use std::iter::FromIterator;

    #[test]
    fn success_00_reverse_str() {
        let original = "hoge";
        let expected = "egoh";
        assert_eq!(expected, answer::reverse_str(original));

        let original2 = "ほげ";
        let expected2 = "げほ";
        assert_eq!(expected2, answer::reverse_str(original2));

        let original3 = "";
        let expected3 = "";
        assert_eq!(expected3, answer::reverse_str(original3));
    }

    #[test]
    fn success_01_odd_idx_str() {
        let original = "パタトクカシーー";
        let expected = "パトカー";
        assert_eq!(expected, answer::odd_idx_str(original));

        let original1 = "ほげほ";
        let expected1 = "ほほ";
        assert_eq!(expected1, answer::odd_idx_str(original1));
    }

    #[test]
    fn success_02_mix_two_str() {
        let original_1 = "パトカー";
        let original_2 = "タクシー";
        let expected = "パタトクカシーー";
        assert_eq!(expected, answer::mix_two_str(original_1, original_2));

        let original1_1 = "パトカ";
        let original1_2 = "タクシ？";
        let expected1 = "パタトクカシ？";
        assert_eq!(expected1, answer::mix_two_str(original1_1, original1_2));

        let original2_1 = "パトカ！！";
        let original2_2 = "タクシ";
        let expected2 = "パタトクカシ！！";
        assert_eq!(expected2, answer::mix_two_str(original2_1, original2_2));
    }

    #[test]
    fn success_03_pi() {
        let original = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.";
        let expected: Vec<usize> = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9];
        assert_eq!(expected, answer::pi(original));

        let original1 = "This is    a \
        pen...";
        let expected1 = vec![4, 2, 1, 3];
        assert_eq!(expected1, answer::pi(original1));
    }

    #[test]
    fn success_04_symbol_of_element() {
        let original = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.";
        let idx_one_symbols: Vec<usize> = vec![1, 5, 6, 7, 8, 9, 15, 16, 19];
        let expected_vec = vec![
            "H", "He", "Li", "Be", "B", "C", "N", "O", "F", "Ne", "Na", "Mi", "Al", "Si", "P", "S",
            "Cl", "Ar", "K", "Ca",
        ];
        let mut expected = BTreeMap::new();
        for (i, symbol) in expected_vec.iter().enumerate() {
            let idx = i + 1;
            expected.insert(symbol.to_string(), idx);
        }

        let actual = answer::chemical_symbols(original, idx_one_symbols);
        assert_eq!(expected, actual);
        // FIXME add failure test case
    }

    #[test]
    fn success_05_ngram() {
        let original = "I am an NLPer";
        let n = 2;
        let expected_word_tokens: Vec<Vec<&str>> =
            vec![vec!["I", "am"], vec!["am", "an"], vec!["an", "NLPer"]];
        let actual_word_tokens = answer::word_ngram(original, n);
        assert_eq!(expected_word_tokens, actual_word_tokens);

        // char_ngram
        let expected_char_tokens: Vec<&str> = vec![
            "I ", " a", "am", "m ", " a", "an", "n ", " N", "NL", "LP", "Pe", "er",
        ];
        let actual_char_tokens = answer::char_ngram(original, n);
        assert_eq!(expected_char_tokens, actual_char_tokens);
    }

    #[test]
    fn success_06_set_operations() {
        let original1 = "paraparaparadise";
        let original2 = "paragraph";

        let expected1_values = vec!["ad", "ap", "ar", "di", "is", "pa", "ra", "se"];
        let expected2_values = vec!["ag", "ap", "ar", "gr", "pa", "ph", "ra"];

        assert_eq!(
            expected1_values,
            answer::char_ngram_set(original1, 2)
                .into_iter()
                .collect::<Vec<String>>()
        );
        assert_eq!(
            expected2_values,
            answer::char_ngram_set(original2, 2)
                .into_iter()
                .collect::<Vec<String>>()
        );

        let expected_union = vec![
            "ad", "ag", "ap", "ar", "di", "gr", "is", "pa", "ph", "ra", "se",
        ];
        assert_eq!(
            expected_union,
            answer::union_ngram_sets(
                answer::char_ngram_set(original1, 2),
                &answer::char_ngram_set(original2, 2)
            )
            .into_iter()
            .collect::<Vec<String>>()
        );

        let expected_intersection = vec!["ap", "ar", "pa", "ra"];
        assert_eq!(
            expected_intersection,
            answer::intersection_ngram_sets(
                answer::char_ngram_set(original1, 2),
                &answer::char_ngram_set(original2, 2)
            )
            .into_iter()
            .collect::<Vec<String>>()
        );

        // origina1 - original2
        let expected_difference_1_minus_2 = vec!["ad", "di", "is", "se"];
        assert_eq!(
            expected_difference_1_minus_2,
            answer::difference_ngram_sets(
                answer::char_ngram_set(original1, 2),
                &answer::char_ngram_set(original2, 2)
            )
            .into_iter()
            .collect::<Vec<String>>()
        );

        // original2 - origina1
        let expected_difference_2_minus_1 = vec!["ag", "gr", "ph"];
        assert_eq!(
            expected_difference_2_minus_1,
            answer::difference_ngram_sets(
                answer::char_ngram_set(original2, 2),
                &answer::char_ngram_set(original1, 2)
            )
            .into_iter()
            .collect::<Vec<String>>()
        );

        // find "se" from each set
        assert_eq!(true, answer::char_ngram_set(original1, 2).contains("se"));
        assert_eq!(false, answer::char_ngram_set(original2, 2).contains("se"));
    }

    #[test]
    fn success_07_generate_sentence() {
        let original_x = 0;
        let original_y = "y";
        let original_z = 2.0;
        assert_eq!(
            "0時のyは2.0",
            answer::generate_sentence(original_x, original_y, original_z)
        );
    }

    #[test]
    fn success_08_cipher() {
        let original = "AaBbCc";
        let expected = "AzByCx";
        assert_eq!(expected, cipher(original));
    }

    #[test]
    fn success_09_typoglycemia() {
        let original = "I couldn’t believe that I could actually understand what I was reading : the phenomenal power of the human mind.";
        let expected = [
            "I",
            "couldn’t",
            "believe",
            "that",
            "I",
            "could",
            "actually",
            "understand",
            "what",
            "I",
            "was",
            "reading",
            ":",
            "the",
            "phenomenal",
            "power",
            "of",
            "the",
            "human",
            "mind.",
        ];
        let actual = typoglycemia(original);
        println!("[{}]", actual);
        let check_tuple = expected.iter().zip(actual.split_whitespace());
        for (expected, actual) in check_tuple {
            if expected.len() <= 4 {
                assert_eq!(*expected, actual);
            } else {
                assert_eq!(expected.len(), actual.len());
                let expected_first = expected.chars().next().unwrap();
                let expected_last = expected.chars().last().unwrap();
                let actual_first = actual.chars().next().unwrap();
                let actual_last = actual.chars().last().unwrap();
                assert_eq!(expected_first, actual_first);
                assert_eq!(expected_last, actual_last);
            }
        }
    }
}
