use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};

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
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("read error");
    return contents.replace("\t", " ");
}

// ch02-12 1列目をcol1.txtに，2列目をcol2.txtに保存
// numは0始まりではなく、1始まりのほうがいい?
pub fn extract_column(input_file_name: &str, num: usize, output_file_name: &str) {
    let input_f = File::open(input_file_name).expect("file not found");
    let read_buf = BufReader::new(input_f);
    let mut output_f = OpenOptions::new()
        .write(true)
        .create(true)
        .open(output_file_name)
        .expect(format!("can't open file[{}] with write option", output_file_name).as_str());
    read_buf.lines().for_each(|line| match line {
        Ok(line) => {
            let columns: Vec<_> = line.split('\t').collect();
            writeln!(output_f, "{}", columns[num]).expect("error");
            output_f.flush().expect("Error during flush");
        }
        Err(_) => panic!("parse error "),
    });
}

// ch02-13 col1.txtとcol2.txtをマージ
pub fn merge_files(col1_file: &str, col2_file: &str, output_file_name: &str) {
    let col1_buf = BufReader::new(File::open(col1_file).expect("file not found"));
    let col2_buf = BufReader::new(File::open(col2_file).expect("file not found"));
    let mut output_f = OpenOptions::new()
        .write(true)
        .create(true)
        .open(output_file_name)
        .expect(format!("can't open file[{}] with write option", output_file_name).as_str());
    col1_buf
        .lines()
        .zip(col2_buf.lines())
        .for_each(|(col1, col2)| {
            let col1 = col1.expect("parse error col1");
            let col2 = col2.expect("parse error col2");
            writeln!(output_f, "{}\t{}", col1, col2).expect("error");
            output_f.flush().expect("Error during flush");
        });
}

// ch02-14 先頭からN行を出力
pub fn head(input_file_name: &str, lines: usize) -> String {
    let buf = BufReader::new(File::open(input_file_name).expect("file not found"));
    let mut head = String::new();
    buf.lines().take(lines).for_each(|line| {
        head.push_str(format!("{}\n", line.expect("parse error")).as_str());
    });
    return head;
}

// ch02-15 末尾のN行を出力
pub fn tail(input_file_name: &str, lines: usize) -> String {
    let buf = BufReader::new(File::open(input_file_name).expect("file not found"));
    let mut tail = String::new();
    let line_count = word_count(input_file_name);
    buf.lines().skip(line_count - lines).for_each(|line| {
        tail.push_str(format!("{}\n", line.expect("parse error")).as_str());
    });
    return tail;
}

// ch02-16 ファイルをN分割する
pub fn split_files(
    input_file_name: &str,
    num: usize,
    output_file_prefix: &str,
    output_file_suffix: &str,
) {
    let total = word_count(input_file_name) as f64;
    let lines_in_file = total / num as f64;
    let lines_in_file = lines_in_file.ceil() as usize; //
    let buf = BufReader::new(File::open(input_file_name).expect("file not found"));

    let output_files: Vec<File> = create_file_vec(output_file_prefix, num, output_file_suffix);

    println!("split file each {} lines.", lines_in_file);
    let mut lines = buf.lines();

    for mut output_f in output_files {
        let mut current = 1;
        while current < lines_in_file + 1 {
            let line = lines.next();
            if let Some(line_rs) = line {
                if let Ok(line_str) = line_rs {
                    writeln!(output_f, "{}", line_str).expect("error");
                }
            }
            current = current + 1;
        }
        output_f.flush().expect("error during flush");
    }
}

fn create_file_vec(output_file_prefix: &str, num: usize, output_file_suffix: &str) -> Vec<File> {
    let mut files = Vec::with_capacity(num);
    for i in 0..num {
        let output_file_name = format!("{}{}{}", output_file_prefix, i + 1, output_file_suffix);
        let output_f = OpenOptions::new()
            .write(true)
            .create(true)
            .open(output_file_name.as_str())
            .expect(format!("can't open file[{}] with write option", output_file_name).as_str());
        files.push(output_f);
    }
    return files;
}

// ch02-17 １列目の文字列の異なり
pub fn count_uniq_words(input_file_name: &str, col: usize) -> usize {
    let mut words = HashSet::new();
    let buf = BufReader::new(File::open(input_file_name).expect("file not found"));
    buf.lines().for_each(|line| match line {
        Ok(line_str) => {
            let columns: Vec<_> = line_str.split('\t').collect();
            words.insert(columns[col].to_string());
        }
        Err(_) => panic!("parse error "),
    });
    return words.len();
}

// ch02-18 各行を3コラム目の数値の降順にソート
pub fn sort_on_col3(input_file_name: &str) -> String {
    let mut lines: BTreeSet<Line> = BTreeSet::new();
    let buf = BufReader::new(File::open(input_file_name).expect("file not found"));
    buf.lines().for_each(|line| match line {
        Ok(line_str) => {
            let columns: Vec<_> = line_str.split('\t').collect();
            let num: u32 = columns[2].parse().expect("parse error");
            let line = Line {
                line: line_str,
                num,
            };
            lines.insert(line);
        }
        Err(_) => panic!("parse error"),
    });
    let mut sorted = String::new();
    lines.iter().for_each(|line| {
        sorted.push_str(format!("{}\n", line.line).as_str());
    });

    return sorted;
}

#[derive(Eq)]
struct Line {
    line: String,
    num: u32,
}

impl Ord for Line {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = other.num.cmp(&self.num);
        if let Ordering::Equal = ord {
            other.line.cmp(&self.line)
        } else {
            ord
        }
    }
}

impl PartialOrd for Line {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.eq(other)
    }
}

// ch02-19 各行の1コラム目の文字列の出現頻度を求め，出現頻度の高い順に並べる
pub fn sort_on_frequency(input_file_name: &str) -> String {
    let mut names: HashMap<String, u32> = HashMap::new();
    let buf = BufReader::new(File::open(input_file_name).expect("file not found"));
    buf.lines().for_each(|line| match line {
        Ok(line_str) => {
            let columns: Vec<_> = line_str.split('\t').collect();
            let name_str = columns[0].to_string();
            let count = names.entry(name_str).or_insert(0);
            *count += 1;
        }
        Err(_) => panic!("parse error"),
    });
    let mut sorted = String::new();
    let mut sorted_names: Vec<(&String, &u32)> = names.iter().collect();
    sorted_names.sort_by(|(aname, acount), (bname, bcount)| {
        let ord = bcount.cmp(acount);
        if let Ordering::Equal = ord {
            bname.cmp(aname)
        } else {
            ord
        }
    });
    sorted_names.iter().for_each(|(name, count)| {
        sorted.push_str(format!("{} {}\n", count, name).as_str());
    });
    return sorted;
}

// -- Unit test -----
#[cfg(test)]
mod tests {
    use crate::chapter02::answer::{
        count_uniq_words, extract_column, head, merge_files, sort_on_col3, sort_on_frequency,
        split_files, tab_2_space, tail, word_count,
    };
    use std::fs::{create_dir, remove_file, File};
    use std::io::{BufRead, BufReader, Read};

    const INPUT_PATH: &str = "data/popular-names.txt";
    const EXPECTED_PATH: &str = "data/chap02_expected/";
    const TMP_PATH: &str = "data/chap02_tmp/";
    const N: usize = 5;

    fn read_file_as_string(file_name: &str) -> String {
        let mut f = File::open(file_name).expect(format!("file not found. {}", file_name).as_str());
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("read error");
        return contents;
    }

    fn read_file_as_lines(file_name: &str) -> Vec<String> {
        let f = File::open(file_name).expect(format!("file not found. {}", file_name).as_str());
        let buf = BufReader::new(f);
        let lines: Vec<String> = buf.lines().map(|l| l.unwrap()).collect();
        return lines;
    }

    #[test]
    fn success_10_word_count() {
        let count = word_count(INPUT_PATH);
        let expected = read_file_as_string(format!("{}{}", EXPECTED_PATH, "10.txt").as_str());
        let expected: usize = expected.trim().parse().expect("parse error!");
        assert_eq!(expected, count);
    }

    #[test]
    fn success_11_tab_2_space() {
        let actual = tab_2_space(INPUT_PATH);
        let expected = read_file_as_string(format!("{}{}", EXPECTED_PATH, "11_tr.txt").as_str());
        assert_eq!(expected, actual);
    }

    #[test]
    fn success_12_extract_column() {
        let actual_file1 = format!("{}{}", TMP_PATH, "col1.txt");
        let actual_file2 = format!("{}{}", TMP_PATH, "col2.txt");

        create_dir(TMP_PATH);
        remove_file(actual_file1.as_str());
        remove_file(actual_file2.as_str());

        extract_column(INPUT_PATH, 0, actual_file1.as_str());
        extract_column(INPUT_PATH, 1, actual_file2.as_str());
        let actual = read_file_as_string(actual_file1.as_str());
        let expected = read_file_as_string(format!("{}{}", EXPECTED_PATH, "12_col1.txt").as_str());
        assert_eq!(expected, actual);
        let actual = read_file_as_string(actual_file2.as_str());
        let expected = read_file_as_string(format!("{}{}", EXPECTED_PATH, "12_col2.txt").as_str());
        assert_eq!(expected, actual);
    }

    #[test]
    fn success_13_merge_files() {
        let actual_file = format!("{}{}", TMP_PATH, "col12.txt");
        create_dir(TMP_PATH);
        remove_file(actual_file.as_str());

        //make sure existing col1.txt and col2.txt
        success_12_extract_column();

        merge_files(
            format!("{}{}", TMP_PATH, "col1.txt").as_str(),
            format!("{}{}", TMP_PATH, "col2.txt").as_str(),
            actual_file.as_str(),
        );

        let actual = read_file_as_string(actual_file.as_str());
        let expected = read_file_as_string(format!("{}{}", EXPECTED_PATH, "13.txt").as_str());
        assert_eq!(expected, actual);
    }

    #[test]
    fn success_14_head() {
        let actual = head(INPUT_PATH, N);
        let expected = read_file_as_string(format!("{}{}", EXPECTED_PATH, "14.txt").as_str());
        assert_eq!(expected, actual);
    }

    #[test]
    fn success_15_tail() {
        let actual = tail(INPUT_PATH, N);
        let expected = read_file_as_string(format!("{}{}", EXPECTED_PATH, "15.txt").as_str());
        assert_eq!(expected, actual);
    }

    #[test]
    fn success_16_split_files() {
        // TODO more efficient way...
        let suffixes = vec!["a", "b", "c", "d", "e"];
        let expected_prefix = format!("{}{}", EXPECTED_PATH, "16_");
        let actual_prefix = format!("{}{}", TMP_PATH, "16_");
        let actual_suffix = ".txt";

        create_dir(TMP_PATH);
        for i in 0..N {
            remove_file(format!("{}{}{}", actual_prefix.as_str(), i + 1, actual_suffix).as_str());
        }

        split_files(INPUT_PATH, N, actual_prefix.as_str(), actual_suffix);

        for i in 0..N {
            let actual = read_file_as_string(
                format!("{}{}{}", actual_prefix.as_str(), i + 1, actual_suffix).as_str(),
            );
            let expected =
                read_file_as_string(format!("{}{}", expected_prefix, suffixes[i]).as_str());
            assert_eq!(expected, actual, "current file is {}", i + 1);
        }
    }

    #[test]
    fn success_17_count_uniq_words() {
        let actual = count_uniq_words(INPUT_PATH, 0);
        let expected: usize =
            read_file_as_string(format!("{}{}", EXPECTED_PATH, "17.txt").as_str())
                .trim()
                .parse()
                .expect("parse error");
        assert_eq!(expected, actual);
    }

    #[test]
    fn success_18_sort_on_col3() {
        let actual = sort_on_col3(INPUT_PATH);
        let expected = read_file_as_string(format!("{}{}", EXPECTED_PATH, "18.txt").as_str());
        assert_eq!(expected, actual);
    }

    #[test]
    fn success_19_sort_on_frequency() {
        let actual = sort_on_frequency(INPUT_PATH);
        println!("{}", actual);
        let expected = read_file_as_string(format!("{}{}", EXPECTED_PATH, "19.txt").as_str());
        // Don't worry about output format
        assert_eq!(expected.replace(" ", ""), actual.replace(" ", ""));
    }
}
