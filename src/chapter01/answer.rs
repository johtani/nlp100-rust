// ch01 準備運動 - http://www.cl.ecei.tohoku.ac.jp/nlp100/#ch1
// ch01-00 文字列の逆順
pub fn reverse_str(original: &str) -> String {
    let mut reversed = String::new();
    let mut char_array = original.chars().collect::<Vec<char>>();
    char_array.reverse();
    for i in 0..char_array.len() {
        reversed.push(char_array[i]);
    }
    return reversed;
}

// ch01-01 「パタトクカシーー」
fn mix_two_str(first_str: &str, second_str: &str) -> String {
    let mut mixed = String::new();
    error!("Not implemented yet");
    return mixed;
}


#[cfg(test)]
mod tests {
    use chapter01::answer;

    #[test]
    fn success_reverse_str() {
        let original = "hoge";
        let expected = "egoh";
        assert_eq!(expected, answer::reverse_str(original));

        let original2 = "ほげ";
        let expected2 = "げほ";
        assert_eq!(expected2, answer::reverse_str(original2));
    }

    #[test]
    fn success_mix_two_str() {
        let original_1 = "パトカー";
        let original_2 = "タクシー";
        let expected = "パタトクカシーー";
        assert_eq!(expected, answer::mix_two_str(original_1, original_2));
    }

}