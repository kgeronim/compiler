use std::iter::Peekable;
use std::str::CharIndices;

fn main() {
    let lang = String::from("data=\"stRing藏\";data2=1;data3=.12939+546.23;empty=\"\"");
    let tokens = Tokens::from(&lang).enumerate();

    for token in tokens {
        println!("{:?}", token)
    }
}

#[derive(Debug)]
enum Token<'a> {
    String(&'a str, (usize, usize)),
    Operator(char, usize),
    Other(char, usize),
    Number(&'a str, (usize, usize)),
    Identifier(&'a str, (usize, usize)),
    Symbol(char, usize),
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
                    break Token::String(
                        &self.lang[index_start + 1..index_end],
                        (index_start, index_end),
                    );
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
                    break Token::Number(
                        &self.lang[index_start..*index_end],
                        (index_start, *index_end),
                    );
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
                    break Token::Identifier(
                        &self.lang[index_start..*index_end],
                        (index_start, *index_end),
                    );
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
                '+' | '-' | '*' | '/' | '=' => Some(Token::Operator(c, index_start)),
                '(' | ')' | '{' | '}' | ';' => Some(Token::Symbol(c, index_start)),
                '0'..='9' => Some(self.scan_number(index_start)),
                'a'..='z' => Some(self.scan_identifier(index_start)),
                _ => Some(Token::Other(c, index_start)),
            }
        } else {
            None
        }
    }
}
