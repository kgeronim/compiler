use std::iter::Peekable;

fn main() {
    let lang = String::from("data=\"string藏\";data2=1;data3=.12939+546.23;empty=\"\"");
    let mut chars = lang.char_indices().peekable();

    while let Some((prev_index, c)) = chars.next() {
        let token = match c {
            '"' => scan_string(&lang, &mut chars, prev_index),
            '+' | '-' | '*' | '/' | '=' => Token::Operator(c),
            '(' | ')' | '{' | '}' | ';' => Token::Symbol(c),
            _ if c.is_ascii_alphabetic() => scan_identifier(&lang, &mut chars, prev_index),
            _ if c.is_digit(10) => scan_number(&lang, &mut chars, prev_index),
            _ => Token::Other(c),
        };

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

fn scan_string<'a, T: Iterator<Item = (usize, char)>>(
    lang: &'a str,
    chars: &mut T,
    prev_index: usize,
) -> Token<'a> {
    loop {
        if let Some((curr_index, c)) = chars.next() {
            if c == '"' {
                break Token::String(&lang[prev_index + 1..curr_index]);
            }
        } else {
            panic!("unterminated string literal")
        }
    }
}

fn scan_number<'a, T: Iterator<Item = (usize, char)>>(
    lang: &'a str,
    chars: &mut Peekable<T>,
    prev_index: usize,
) -> Token<'a> {
    loop {
        if let Some((future_index, c)) = chars.peek() {
            if c.is_digit(10) || *c == '.' {
                chars.next();
            } else {
                break Token::Number(&lang[prev_index..*future_index]);
            }
        }
    }
}

fn scan_identifier<'a, T: Iterator<Item = (usize, char)>>(
    lang: &'a str,
    chars: &mut Peekable<T>,
    prev_index: usize,
) -> Token<'a> {
    loop {
        if let Some((future_index, c)) = chars.peek() {
            if c.is_ascii_alphanumeric() {
                chars.next();
            } else {
                break Token::Identifier(&lang[prev_index..*future_index]);
            }
        }
    }
}
