use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

pub fn load_and_parse_neko() {
    let file_path = "./data/chap04/neko.txt";
    let file = File::open(file_path).unwrap();
    let buf = BufReader::new(file);
    let mut out = File::create("./data/chap04/neko.txt.lindera.json").unwrap();
    buf.lines().filter_map(|item| item.ok()).for_each(|line| {
        println!("{}", line);
        let tokens = tokenize(line.as_str());
        output_tokens(&tokens, &mut out);
    });
}

#[derive(Debug, Serialize, Deserialize)]
struct Token {
    surface: String,
    base: String,
    pos: String,
    pos1: String,
}

fn output_tokens(tokens: &Vec<Token>, buf: &mut File) {
    writeln!(buf, "{}", serde_json::to_string(tokens).unwrap()).expect("Error during output json");
}

fn tokenize(line: &str) -> Vec<Token> {
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

trait Command {
    fn execute(&mut self, tokens: &Vec<Token>);
}
// ch04-30. 形態素解析結果の読み込み
fn load_json<T: Command>(cmd: &mut T) {
    let file = File::open("./data/chap04/neko.txt.lindera.json").unwrap();
    let buf = BufReader::new(file);
    buf.lines().filter_map(|item| item.ok()).for_each(|line| {
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
                writeln!(self.out, "{}", token.surface);
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
                    writeln!(self.out, "{}{}", buffer.join(""), token.surface);
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
fn extract_max_conjunction_of_noun() {
    let mut cmd = ExtractMaxConjunctionNoun {
        out: File::create("./data/chap04/max_noun.txt").unwrap(),
        buffer: vec![],
    };
    load_json(&mut cmd);
    cmd.print_max();
}

struct ExtractMaxConjunctionNoun {
    out: File,
    buffer: Vec<Token>,
}

impl ExtractMaxConjunctionNoun {
    fn print_max(&self) {
        let mut max = String::new();
        for token in &self.buffer {
            max.push_str(token.surface.as_str());
        }
        writeln!(&self.out, "{}", max);
        println!("{}", max);
    }
}

impl Command for ExtractMaxConjunctionNoun {
    fn execute(&mut self, tokens: &Vec<Token>) {
        let mut nouns = vec![];
        tokens.iter().for_each(|token| {
            if token.pos == "名詞" {
                nouns.push(token);
            } else {
                if self.buffer.len() < nouns.len() {
                    self.buffer.clear();
                    //TODO 無駄なコピーしてる
                    for token in &nouns {
                        self.buffer.push(Token::from(token));
                    }
                }
                nouns.clear();
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
        hashmap: HashMap::new(),
    };
    load_json(&mut cmd);
    cmd.print();
}

struct TokenCounter {
    out: File,
    hashmap: HashMap<String, u32>,
}

impl TokenCounter {
    fn print(&self) {
        for (key, value) in &self.hashmap {
            writeln!(&self.out, "{}  {}", key, value);
            println!("{}  {}", key, value);
        }
    }
}

impl Command for TokenCounter {
    fn execute(&mut self, tokens: &Vec<Token>) {
        tokens.iter().for_each(|token| {
            let value = self.hashmap.get(token.surface.as_str());
            let count = match value {
                None => 1,
                Some(counter) => counter + 1,
            };
            self.hashmap.insert(token.surface.to_string(), count);
        });
    }
}

// ch04-36. 頻度上位10語
// ch04-37. 「猫」と共起頻度の高い上位10語
// ch04-38. ヒストグラム
// ch04-39. Zipfの法則

#[cfg(test)]
mod tests {
    use crate::chapter04::answer::{
        count_token_frequency, extract_a_and_b, extract_max_conjunction_of_noun, extract_verb,
        extract_verb_base, load_and_parse_neko, tokenize,
    };
    use std::fs::File;
    use std::path::Path;

    #[test]
    fn success_tokenize() {
        let text = "関西国際空港";
        let tokens = tokenize(text);
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
        load_and_parse_neko();
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
    fn success_output_max_noun() {
        extract_max_conjunction_of_noun();
    }

    #[test]
    fn success_output_token_freq() {
        count_token_frequency();
    }
}
