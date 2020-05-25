use std::iter::Peekable;
use std::str::CharIndices;

fn main() {
    let lang = String::from("ん data=\"stRing藏\";\ndata2=1;data3=.12939+546.23; empty=\"\"");
    let tokens = Tokens::from(&lang)
        .filter(|x| *x != Token::Whitespace)
        .enumerate();

    for token in tokens {
        println!("{:?}", token)
    }
}

#[derive(Debug, PartialEq)]
enum Token<'a> {
    String(&'a str, (usize, usize)),
    Operator(char, (usize, usize)),
    Other(char, (usize, usize)),
    Number(&'a str, (usize, usize)),
    Identifier(&'a str, (usize, usize)),
    Symbol(char, (usize, usize)),
    Whitespace,
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

    fn scan_string(&mut self) -> usize {
        loop {
            if let Some((idx_e, c)) = self.chars.next() {
                if c == '"' {
                    break idx_e;
                }
            } else {
                panic!("unterminated string literal")
            }
        }
    }

    fn scan_number(&mut self) -> usize {
        loop {
            if let Some((idx_e, c)) = self.chars.peek() {
                if c.is_digit(10) || *c == '.' {
                    self.chars.next();
                } else {
                    break *idx_e;
                }
            }
        }
    }

    fn scan_ident(&mut self) -> usize {
        loop {
            if let Some((idx_e, c)) = self.chars.peek() {
                if c.is_ascii_alphanumeric() {
                    self.chars.next();
                } else {
                    break *idx_e;
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
                '"' => {
                    let idx_e = self.scan_string();
                    let value = &self.lang[idx_s + 1..idx_e];
                    Some(Token::String(value, (idx_s, idx_e + 1)))
                }
                '+' | '-' | '*' | '/' | '=' => {
                    Some(Token::Operator(c, (idx_s, idx_s + c.len_utf8())))
                }
                '(' | ')' | '{' | '}' | ';' => {
                    Some(Token::Symbol(c, (idx_s, idx_s + c.len_utf8())))
                }
                '0'..='9' => {
                    let idx_e = self.scan_number();
                    let value = &self.lang[idx_s..idx_e];
                    Some(Token::Number(value, (idx_s, idx_e)))
                }
                'a'..='z' => {
                    let idx_e = self.scan_ident();
                    let value = &self.lang[idx_s..idx_e];
                    Some(Token::Identifier(value, (idx_s, idx_e)))
                }
                _ if c.is_ascii_whitespace() => Some(Token::Whitespace),
                _ => Some(Token::Other(c, (idx_s, idx_s + c.len_utf8()))),
            }
        } else {
            None
        }
    }
}
