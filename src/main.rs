use std::iter::Peekable;
use std::str::CharIndices;

fn main() {
    let lang = String::from("ん data=\"stRing藏\";\n data2=1;data3=.12939+546.23; empty=\"\"");
    let tokens = Tokens::from(&lang).enumerate();

    for token in tokens {
        println!("{:?}", token)
    }
}

#[derive(Debug)]
enum Token<'a> {
    String(&'a str, (usize, usize)),
    Operator(char, (usize, usize)),
    Other(char, (usize, usize)),
    Number(&'a str, (usize, usize)),
    Identifier(&'a str, (usize, usize)),
    Symbol(char, (usize, usize)),
}

struct Tokens<'a> {
    lang: &'a str,
    chars: Peekable<CharIndices<'a>>,
}

impl<'a> Tokens<'a> {
    fn from(lang: &'a str) -> Self {
        Tokens {
            lang,
            chars: lang.char_indices().peekable(),
        }
    }

    fn scan_string(&mut self, idx_s: usize) -> Token<'a> {
        loop {
            if let Some((idx_e, c)) = self.chars.next() {
                if c == '"' {
                    break Token::String(&self.lang[idx_s + 1..idx_e], (idx_s, idx_e + 1));
                }
            } else {
                panic!("unterminated string literal")
            }
        }
    }

    fn scan_number(&mut self, idx_s: usize) -> Token<'a> {
        loop {
            if let Some((idx_e, c)) = self.chars.peek() {
                if c.is_digit(10) || *c == '.' {
                    self.chars.next();
                } else {
                    break Token::Number(&self.lang[idx_s..*idx_e], (idx_s, *idx_e));
                }
            }
        }
    }

    fn scan_identifier(&mut self, idx_s: usize) -> Token<'a> {
        loop {
            if let Some((idx_e, c)) = self.chars.peek() {
                if c.is_ascii_alphanumeric() {
                    self.chars.next();
                } else {
                    break Token::Identifier(&self.lang[idx_s..*idx_e], (idx_s, *idx_e));
                }
            }
        }
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((idx_s, c)) = self.chars.next() {
            match c {
                '"' => Some(self.scan_string(idx_s)),
                '+' | '-' | '*' | '/' | '=' => {
                    Some(Token::Operator(c, (idx_s, idx_s + c.len_utf8())))
                }
                '(' | ')' | '{' | '}' | ';' => {
                    Some(Token::Symbol(c, (idx_s, idx_s + c.len_utf8())))
                }
                '0'..='9' => Some(self.scan_number(idx_s)),
                'a'..='z' => Some(self.scan_identifier(idx_s)),
                _ => Some(Token::Other(c, (idx_s, idx_s + c.len_utf8()))),
            }
        } else {
            None
        }
    }
}
