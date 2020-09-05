use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};

pub fn load_and_parse_neko() {
    let file_path = "./data/chap04/neko.txt";
    let file = File::open(file_path).unwrap();
    let buf = BufReader::new(file);
    let out_buf = BufWriter::new(File::open("./data/chap04/neko.txt.lindera.json").unwrap());
    buf.lines().filter_map(|item| item.ok()).for_each(|line| {
        let tokens = tokenize(line.as_str());
        output_tokens(&tokens, &out_buf);
    });
}

#[derive(Debug, Serialize, Deserialize)]
struct Token {
    surface: String,
    base: String,
    pos: String,
    pos1: String,
}

fn tokenize(line: &str) -> Vec<Token> {
    let mut tokenizer = lindera::tokenizer::Tokenizer::new("normal", "");
    let lindera_tokens = tokenizer.tokenize(line);
    let tokens = lindera_tokens
        .iter()
        .map(|lindera_token| {
            let surface = lindera_token.text.to_string();
            let base = lindera_token.detail[6].to_string();
            let pos = lindera_token.detail[0].to_string();
            let pos1 = lindera_token.detail[1].to_string();
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

fn output_tokens(tokens: &Vec<Token>, buf: &BufWriter<File>) {}

#[cfg(test)]
mod tests {
    use crate::chapter04::answer::tokenize;

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
}
