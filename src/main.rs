use std::fs;
use std::path::Path;

fn main() {
    let path: &Path = Path::new("C:/Users/valen/Desktop/BIO 01.md");
    let f = fs::read_to_string(path).unwrap();
    println!("{:?}", f);
    
}

struct Lexer<'a> {
    content: &'a str,
}

impl<'a> Lexer<'a> {
    fn new(content: &'a str) -> Self {
        Self { content }
    }

    
}