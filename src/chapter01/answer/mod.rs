use std::collections::BTreeMap;

// ch01 準備運動 - http://www.cl.ecei.tohoku.ac.jp/nlp100/#ch1
// ch01-00 文字列の逆順
pub fn reverse_str(original: &str) -> String {
    let mut reversed = String::new();
    if original.len() > 0 {
        let mut char_array = original.chars().collect::<Vec<char>>();
        while let Some(top) = char_array.pop() {
            reversed.push(top);
        }
    }
    return reversed;
}

// ch01-01 「パタトクカシーー」 -> 「パトカー」
pub fn odd_idx_str(original: &str) -> String {
    let mut transformed = String::new();
    let char_array = original.chars().collect::<Vec<char>>();
    for (i, &x) in char_array.iter().enumerate() {
        if i % 2 == 0 {
            transformed.push(x);
        }
    }
    return transformed;
}

// ch01-02 「パトカー」＋「タクシー」＝「パタトクカシーー」
pub fn mix_two_str(first_str: &str, second_str: &str) -> String {
    // TODO how to handle arrays if they don't have same length? error?
    let mut mixed = String::new();
    if first_str.len() > 0 && second_str.len() > 0 {
        let first_chars = first_str.chars().collect::<Vec<char>>();
        let second_chars = second_str.chars().collect::<Vec<char>>();
        let mut first_chars_itr = first_chars.iter();
        let mut second_chars_itr = second_chars.iter();
        loop {
            let first_option = first_chars_itr.next();
            let second_option = second_chars_itr.next();
            let mut first_done = false;
            let mut second_done = false;
            if let Some(top) = first_option {
                mixed.push(*top);
            } else {
                first_done = true;
            }
            if let Some(top) = second_option {
                mixed.push(*top);
            } else {
                second_done = true;
            }
            if first_done && second_done {
                break;
            }
        }
    }
    return mixed;
}

// ch01-03 円周率
pub fn pi(original: &str) -> Vec<usize> {
    let mut word_lengths = Vec::new();
    // TODO how to handle "."?
    let words_tmp = original.split_whitespace().collect::<Vec<&str>>();
    for word in words_tmp {
        let word_chars = word.chars().filter(
            |x| x.is_alphabetic()).collect::<Vec<char>>();
        word_lengths.push(word_chars.len());
    }
    return word_lengths;
}

// ch01-04 元素記号
pub fn chemical_symbols(sentence: &str, idx_one_symbols: Vec<usize>) -> BTreeMap<String, usize> {
    // FIXME how to handle unsorted idx_one_symbols? we should sort it first?
    let mut symbols: BTreeMap<String, usize> = BTreeMap::new();
    let words_tmp = sentence.split_whitespace().collect::<Vec<&str>>();
    if let Some(last) = idx_one_symbols.last() {
        if words_tmp.len() < *last {
            error!("idx_one_symbols has # of word [{}] in sentence < last [{}]", words_tmp.len(), last);
        } else {
            for (i, word) in words_tmp.iter().enumerate() {
                let idx = i + 1;
                match idx_one_symbols.contains(&idx) {
                    true => {
                        if let Some(first) = word.chars().next() {
                            symbols.insert(first.to_string(), idx);
                        } else {
                            // FIXME how to handle it? error?
                            error!("0 length word...");
                        }
                    },
                    false => {
                        let chars = word.chars().collect::<Vec<char>>();
                        if chars.len() < 2 {
                            error!("word[{}] is short...", word);
                        } else {
                            let mut symbol = String::new();
                            for x in chars[0..2].iter() {
                                symbol.push(*x);
                            }
                            symbols.insert(symbol, idx);
                        }
                    }
                }
            }
        }
    } else {
        error!("idx_one_symbols has no elements...");
    }
    return symbols;
}

// ch01-05 n-gram - word
pub fn word_ngram(text: &str, n: i32) -> Vec<String> {
    let tokens = Vec::new();
    error!("Not implemented");
    return tokens;
}
// ch01-05 n-gram - char ]
pub fn char_ngram(text: &str, n: i32) -> Vec<String> {
    let tokens = Vec::new();
    error!("Not implemented");
    return tokens;
}

// -- Unit test -----
#[cfg(test)]
mod tests {
    use chapter01::answer;
    use std::collections::BTreeMap;

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
        let expected_vec = vec!["H", "He", "Li", "Be", "B", "C", "N", "O", "F", "Ne", "Na", "Mi", "Al", "Si", "P", "S", "Cl", "Ar", "K", "Ca"];
        let mut expected = BTreeMap::new();
        for (i, symbol) in expected_vec.iter().enumerate() {
            let idx = i + 1;
            expected.insert(symbol.to_string(), idx);
        }

        let actual = answer::chemical_symbols(original, idx_one_symbols);
        assert_eq!(expected.keys().len(), actual.keys().len());
        for i in 0..actual.keys().len() {
            assert_eq!(expected.keys().nth(i), actual.keys().nth(i));
        }
        for key in actual.keys() {
            assert_eq!(expected.get(key), actual.get(key));
        }
    }

    #[test]
    fn success_05_ngram() {
        let original = "I am an NLPer";
    }
}