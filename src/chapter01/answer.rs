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
    for i in 0..char_array.len() {
        if i % 2 == 0 {
            transformed.push(char_array[i]);
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


#[cfg(test)]
mod tests {
    use chapter01::answer;

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

}