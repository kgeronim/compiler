use std::iter::Peekable;
use std::str::CharIndices;

fn main() {
    let lang = String::from("data=\"stRing藏\";data2=1;data3=.12939+546.23;empty=\"\"");
    let mut tokens = Tokens::from(&lang);

    while let Some(token) = tokens.next() {
        println!("{:?}", token)
    }
}

#[derive(Debug)]
enum Token<'a> {
    String(&'a str),
    Operator(char),
    Other(char),
    Number(&'a str),
    Identifier(&'a str),
    Symbol(char),
}

struct Tokens<'a> {
    lang: &'a String,
    chars: Peekable<CharIndices<'a>>,
}

impl<'a> Tokens<'a> {
    fn from(lang: &'a String) -> Self {
        Tokens {
            lang,
            chars: lang.char_indices().peekable(),
        }
    }

    fn scan_string(&mut self, index_start: usize) -> Token<'a> {
        loop {
            if let Some((index_end, c)) = self.chars.next() {
                if c == '"' {
                    break Token::String(&self.lang[index_start + 1..index_end]);
                }
            } else {
                panic!("unterminated string literal")
            }
        }
    }

    fn scan_number(&mut self, index_start: usize) -> Token<'a> {
        loop {
            if let Some((index_end, c)) = self.chars.peek() {
                if c.is_digit(10) || *c == '.' {
                    self.chars.next();
                } else {
                    break Token::Number(&self.lang[index_start..*index_end]);
                }
            }
        }
    }

    fn scan_identifier(&mut self, index_start: usize) -> Token<'a> {
        loop {
            if let Some((index_end, c)) = self.chars.peek() {
                if c.is_ascii_alphanumeric() {
                    self.chars.next();
                } else {
                    break Token::Identifier(&self.lang[index_start..*index_end]);
                }
            }
        }
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((index_start, c)) = self.chars.next() {
            match c {
                '"' => Some(self.scan_string(index_start)),
                '+' | '-' | '*' | '/' | '=' => Some(Token::Operator(c)),
                '(' | ')' | '{' | '}' | ';' => Some(Token::Symbol(c)),
                '0'..='9' => Some(self.scan_number(index_start)),
                'a'..='z' => Some(self.scan_identifier(index_start)),
                _ => Some(Token::Other(c)),
            }
        } else {
            None
        }
    }
}
