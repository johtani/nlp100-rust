use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, BufReader, BufRead};

// ch02 UNIXコマンド - https://nlp100.github.io/ja/ch02.html
// ch02-10 行数のカウント
pub fn word_count(file_name: &str) -> usize {
    let f = File::open(file_name).expect("file not found");
    let buf = BufReader::new(f);
    return buf.lines().count();
}

// ch02-11 タブをスペースに置換

pub fn tab_2_space(file_name: &str) -> String {
    let mut f = File::open(file_name).expect("file not found");
    let mut contents= String::new();
    f.read_to_string(&mut contents).expect("read error");
    return contents.replace("\t", " ");
}

// ch02-12 1列目をcol1.txtに，2列目をcol2.txtに保存

// ch02-13 col1.txtとcol2.txtをマージ

// ch02-14 先頭からN行を出力

// ch02-15 末尾のN行を出力

// ch02-16 ファイルをN分割する

// ch02-17 １列目の文字列の異なり

// ch02-18 各行を3コラム目の数値の降順にソート

// ch02-19 各行の1コラム目の文字列の出現頻度を求め，出現頻度の高い順に並べる



// -- Unit test -----
#[cfg(test)]
mod tests {
    use chapter02::answer;
    use chapter02::answer::{word_count, tab_2_space};
    use std::io::{Lines, Read, BufReader, BufRead};
    use std::fs::File;

    const INPUT_PATH: &str = "data/popular-names.txt";
    const EXPECTED_PATH: &str = "data/chap02_expected/";

    fn output_dir() -> &'static str {
        return "data/chap02_expected/";
    }

    fn read_expected_file_as_string(file_name: &str) -> String {
        let mut f = File::open(file_name)
            .expect(format!("file not found. {}", file_name).as_str());
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("read error");
        return contents;
    }

    fn read_expected_file_as_lines(file_name: &str) -> Vec<String> {
        let f = File::open(file_name)
            .expect(format!("file not found. {}", file_name).as_str());
        let buf = BufReader::new(f);
        let lines:Vec<String> = buf.lines().map(|l| l.unwrap()).collect();
        return lines;
    }

    #[test]
    fn success_10_word_count() {
        let count = word_count(INPUT_PATH);
        let expected = read_expected_file_as_string(
            format!("{}{}", EXPECTED_PATH, "10.txt").as_str());
        let expected:usize = expected.trim().parse().expect("parse error!");
        assert_eq!(expected, count);
    }

    #[test]
    fn success_11_tab_2_space() {
        let actual = tab_2_space(INPUT_PATH);
        let expected = read_expected_file_as_string(
            format!("{}{}", EXPECTED_PATH, "11_tr.txt").as_str());
        assert_eq!(expected, actual);
    }
}