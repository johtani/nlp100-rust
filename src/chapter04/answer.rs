use metered::{metered, ResponseTime, Throughput};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

#[derive(Default, Debug, Serialize)]
pub struct NekoParser {
    metric_reg: NekoParserMetricRegistry,
}

#[metered(registry = NekoParserMetricRegistry, /* default = self.metrics */ registry_expr = self.metric_reg)]
#[measure([ResponseTime, Throughput])]
impl NekoParser {
    #[measure]
    pub fn load_and_parse_neko(&self) {
        let file_path = "./data/chap04/neko.txt";
        let file = File::open(file_path).unwrap();
        let buf = BufReader::new(file);
        let mut out = File::create("./data/chap04/neko.txt.lindera.json").unwrap();
        buf.lines().filter_map(|item| item.ok()).for_each(|line| {
            let tokens = self.tokenize(line.as_str());
            self.output_tokens(&tokens, &mut out);
        });
    }

    #[measure]
    pub fn output_tokens(&self, tokens: &Vec<Token>, buf: &mut File) {
        writeln!(buf, "{}", serde_json::to_string(tokens).unwrap())
            .expect("Error during output json");
    }

    #[measure]
    pub fn tokenize(&self, line: &str) -> Vec<Token> {
        let mut tokenizer = lindera::tokenizer::Tokenizer::new("normal", "");
        let lindera_tokens = tokenizer.tokenize(line);
        let tokens = lindera_tokens
            .iter()
            .map(|lindera_token| {
                let surface = lindera_token.text.to_string();
                let pos = lindera_token.detail[0].to_string();
                let pos1 = if pos != "UNK" {
                    lindera_token.detail[1].to_string()
                } else {
                    String::new()
                };
                let base = if pos != "UNK" {
                    lindera_token.detail[6].to_string()
                } else {
                    String::new()
                };
                Token {
                    surface,
                    base,
                    pos,
                    pos1,
                }
            })
            .collect();
        return tokens;
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Token {
    surface: String,
    base: String,
    pos: String,
    pos1: String,
}

trait Command {
    fn execute(&mut self, tokens: &Vec<Token>);
}

trait Filter {
    fn is_target(&self, line: &str) -> bool;
}

struct NonFilter {}

impl Filter for NonFilter {
    fn is_target(&self, _line: &str) -> bool {
        true
    }
}

// ch04-30. 形態素解析結果の読み込み
fn load_json<T: Command>(cmd: &mut T) {
    load_json_with_filter(cmd, &NonFilter {});
}

fn load_json_with_filter<T: Command, U: Filter>(cmd: &mut T, filter: &U) {
    let file = File::open("./data/chap04/neko.txt.lindera.json").unwrap();
    let buf = BufReader::new(file);
    buf.lines()
        .filter_map(|item| item.ok())
        .filter(|line| filter.is_target(line))
        .for_each(|line| {
            let tokens = parse_line_json(line.as_str());
            cmd.execute(&tokens);
        });
}

fn parse_line_json(line: &str) -> Vec<Token> {
    return serde_json::from_str(line).unwrap();
}

// ch04-31. 動詞
fn extract_verb() {
    load_json(&mut ExtractVerv {
        out: File::create("./data/chap04/verb.txt").unwrap(),
    });
}

struct ExtractVerv {
    out: File,
}

impl Command for ExtractVerv {
    fn execute(&mut self, tokens: &Vec<Token>) {
        tokens
            .iter()
            .filter(|token| token.pos == "動詞")
            .for_each(|token| {
                writeln!(self.out, "{}", token.surface).expect("Error during writeln");
                println!("{}", token.surface);
            });
    }
}

// ch04-32. 動詞の原形
fn extract_verb_base() {
    load_json(&mut ExtractVerbBase {
        out: File::create("./data/chap04/verb_base.txt").unwrap(),
    });
}

struct ExtractVerbBase {
    out: File,
}

impl Command for ExtractVerbBase {
    fn execute(&mut self, tokens: &Vec<Token>) {
        tokens
            .iter()
            .filter(|token| token.pos == "動詞")
            .for_each(|token| {
                writeln!(self.out, "{}", token.base).expect("Error during writeln");
                println!("{}", token.base);
            })
    }
}

// ch04-33. 「AのB」
fn extract_a_and_b() {
    load_json(&mut ExtractAandB {
        out: File::create("./data/chap04/noun_a_and_b.txt").unwrap(),
    })
}

struct ExtractAandB {
    out: File,
}

impl Command for ExtractAandB {
    fn execute(&mut self, tokens: &Vec<Token>) {
        let mut buffer = vec![];
        tokens.iter().for_each(|token| {
            if token.pos == "名詞" {
                if buffer.is_empty() {
                    buffer.push(token.surface.to_string());
                } else if buffer.len() == 2 {
                    writeln!(self.out, "{}{}", buffer.join(""), token.surface)
                        .expect("Error during writeln");
                    println!("{}{}", buffer.join(""), token.surface);
                } else {
                    buffer.clear();
                    buffer.push(token.surface.to_string());
                }
            } else if token.surface == "の" && buffer.len() == 1 {
                buffer.push(token.surface.to_string());
            }
        });
    }
}

// ch04-34. 名詞の連接
fn extract_conjunction_of_nouns() {
    let mut cmd = ExtractMaxConjunctionNoun {
        out: File::create("./data/chap04/max_noun.txt").unwrap(),
        buffer: vec![],
    };
    load_json(&mut cmd);
    cmd.print_conjunction_nouns();
}

struct ExtractMaxConjunctionNoun {
    out: File,
    buffer: Vec<Vec<Token>>,
}

impl ExtractMaxConjunctionNoun {
    fn print_conjunction_nouns(&self) {
        self.buffer.iter().for_each(|nouns| {
            let mut max = String::new();
            for token in nouns.iter() {
                max.push_str(token.surface.as_str());
            }
            writeln!(&self.out, "{}", max).expect("Error during writeln");
            println!("{}", max);
        });
    }
}

impl Command for ExtractMaxConjunctionNoun {
    fn execute(&mut self, tokens: &Vec<Token>) {
        let mut nouns = vec![];
        // TODO 参照保持でどうにかしたいけどなぁ。
        tokens.iter().map(|token| token.clone()).for_each(|token| {
            if token.pos == "名詞" {
                nouns.push(token);
            } else {
                if nouns.len() > 1 {
                    self.buffer.push(nouns.clone());
                }
                nouns = vec![]
            }
        });
    }
}

impl Token {
    fn from(token: &Token) -> Token {
        Token {
            surface: token.surface.to_string(),
            base: token.base.to_string(),
            pos: token.pos.to_string(),
            pos1: token.pos1.to_string(),
        }
    }
}

// ch04-35. 単語の出現頻度
fn count_token_frequency() {
    let mut cmd = TokenCounter {
        out: File::create("./data/chap04/token_freq.txt").unwrap(),
        terms_count: BTreeMap::new(),
    };
    load_json(&mut cmd);
    cmd.print();
}

struct TokenCounter {
    out: File,
    terms_count: BTreeMap<String, u32>,
}

impl TokenCounter {
    fn print(&self) {
        for (key, value) in &self.terms_count {
            writeln!(&self.out, "{}, {}", key, value).expect("Error during writeln");
            println!("{}, {}", key, value);
        }
    }

    fn print_top10(&mut self) {
        let mut key_values: Vec<(&String, &u32)> =
            self.terms_count.iter().collect::<Vec<(&String, &u32)>>();
        key_values.sort_by(|x, y| y.1.cmp(&x.1));
        key_values.iter().take(10).for_each(|(key, value)| {
            writeln!(&self.out, "{}, {}", key, value).expect("Error during writeln");
            println!("{}, {}", key, value);
        });
    }
}

impl Command for TokenCounter {
    fn execute(&mut self, tokens: &Vec<Token>) {
        tokens.iter().for_each(|token| {
            let value = self.terms_count.get(token.surface.as_str());
            let count = match value {
                None => 1,
                Some(counter) => counter + 1,
            };
            self.terms_count.insert(token.surface.to_string(), count);
        });
    }
}

// ch04-36. 頻度上位10語
fn count_token_frequency_top10() {
    let mut cmd = TokenCounter {
        out: File::create("./data/chap04/token_freq_top10.txt").unwrap(),
        terms_count: BTreeMap::new(),
    };
    load_json(&mut cmd);
    cmd.print_top10();
}

// ch04-37. 「猫」と共起頻度の高い上位10語
fn count_co_occurrence_cat_top10() {
    let mut cmd = CoOccurrenceCat {
        out: File::create("./data/chap04/co_occurrence_cat_top10.txt").unwrap(),
        co_occurrence_term: BTreeMap::new(),
    };
    load_json_with_filter(&mut cmd, &CatFilter {});
    cmd.print_top10();
}

struct CatFilter {}

impl Filter for CatFilter {
    fn is_target(&self, line: &str) -> bool {
        line.contains("猫")
    }
}

struct CoOccurrenceCat {
    out: File,
    co_occurrence_term: BTreeMap<String, u32>,
}

impl Command for CoOccurrenceCat {
    fn execute(&mut self, tokens: &Vec<Token>) {
        tokens
            .iter()
            .filter(|token| token.surface != "猫")
            .for_each(|token| {
                let value = self.co_occurrence_term.get(token.surface.as_str());
                let count = match value {
                    None => 1,
                    Some(counter) => counter + 1,
                };
                self.co_occurrence_term
                    .insert(token.surface.to_string(), count);
            });
    }
}

impl CoOccurrenceCat {
    fn print(&self) {
        for (key, value) in &self.co_occurrence_term {
            writeln!(&self.out, "{}, {}", key, value).expect("Error during writeln");
            println!("{}, {}", key, value);
        }
    }

    fn print_top10(self) {
        let mut key_values: Vec<(&String, &u32)> = self
            .co_occurrence_term
            .iter()
            .collect::<Vec<(&String, &u32)>>();
        key_values.sort_by(|x, y| y.1.cmp(&x.1));
        key_values.iter().take(10).for_each(|(key, value)| {
            writeln!(&self.out, "{}, {}", key, value).expect("Error during writeln");
            println!("{}, {}", key, value);
        });
    }
}

// ch04-38. ヒストグラム
fn count_co_occurrence_cat() {
    let mut cmd = CoOccurrenceCat {
        out: File::create("./data/chap04/co_occurrence_cat.txt").unwrap(),
        co_occurrence_term: BTreeMap::new(),
    };
    load_json_with_filter(&mut cmd, &CatFilter {});
    cmd.print();
}

// ch04-39. Zipfの法則

#[cfg(test)]
mod tests {
    use crate::chapter04::answer::{
        count_co_occurrence_cat, count_co_occurrence_cat_top10, count_token_frequency,
        count_token_frequency_top10, extract_a_and_b, extract_conjunction_of_nouns, extract_verb,
        extract_verb_base, NekoParser,
    };
    use std::path::Path;

    #[test]
    fn success_tokenize() {
        let text = "関西国際空港";
        let parser = NekoParser::default();
        let tokens = parser.tokenize(text);
        assert_eq!(tokens.len(), 1);
        for token in tokens {
            assert_eq!(token.surface, "関西国際空港");
            assert_eq!(token.base, "関西国際空港");
            assert_eq!(token.pos, "名詞");
            assert_eq!(token.pos1, "固有名詞");
        }
    }

    #[test]
    fn success_output_tokenlists() {
        let parser = NekoParser::default();
        parser.load_and_parse_neko();
        let serialized = serde_json::to_string(&parser).unwrap();
        println!("{}", serialized);
        assert!(Path::new("./data/chap04/neko.txt.lindera.json").exists());
    }

    #[test]
    fn success_output_verv() {
        extract_verb();
    }

    #[test]
    fn success_output_verv_base() {
        extract_verb_base();
    }

    #[test]
    fn success_output_noun_a_and_b() {
        extract_a_and_b();
    }

    #[test]
    fn success_output_conjunction_noun() {
        extract_conjunction_of_nouns();
    }

    #[test]
    fn success_output_token_freq() {
        count_token_frequency();
    }

    #[test]
    fn success_output_token_freq_top10() {
        count_token_frequency_top10();
    }

    #[test]
    fn success_output_co_occurrence_cat_top10() {
        count_co_occurrence_cat_top10();
    }

    #[test]
    fn success_output_co_occurrence_cat() {
        count_co_occurrence_cat();
    }
}
