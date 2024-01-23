use regex::Regex;
fn tokenize(input: &str) -> Vec<&str> {
    let word_pattern = Regex::new(r"\b\w+\b").unwrap();
    let tokens: Vec<&str> = word_pattern.find_iter(input).map(|mat| mat.as_str()).collect();
    tokens
}

fn main() {
    let tokens = tokenize(sentence);
    println!("Tokens: {:?}", tokens);
}
