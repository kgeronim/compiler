use std::iter::Peekable;

fn main() {
    let lang = String::from("data=\"stRing藏\";data2=1;data3=.12939+546.23;empty=\"\"");
    let mut chars = lang.char_indices().peekable();

    while let Some((start_index, c)) = chars.next() {
        let token = match c {
            '"' => scan_string(&lang, &mut chars, start_index),
            '+' | '-' | '*' | '/' | '=' => Token::Operator(c),
            '(' | ')' | '{' | '}' | ';' => Token::Symbol(c),
            'a'..='z' => scan_identifier(&lang, &mut chars, start_index),
            '0'..='9' => scan_number(&lang, &mut chars, start_index),
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
    start_index: usize,
) -> Token<'a> {
    loop {
        if let Some((end_index, c)) = chars.next() {
            if c == '"' {
                break Token::String(&lang[start_index + 1..end_index]);
            }
        } else {
            panic!("unterminated string literal")
        }
    }
}

fn scan_number<'a, T: Iterator<Item = (usize, char)>>(
    lang: &'a str,
    chars: &mut Peekable<T>,
    start_index: usize,
) -> Token<'a> {
    loop {
        if let Some((end_index, c)) = chars.peek() {
            if c.is_digit(10) || *c == '.' {
                chars.next();
            } else {
                break Token::Number(&lang[start_index..*end_index]);
            }
        }
    }
}

fn scan_identifier<'a, T: Iterator<Item = (usize, char)>>(
    lang: &'a str,
    chars: &mut Peekable<T>,
    start_index: usize,
) -> Token<'a> {
    loop {
        if let Some((end_index, c)) = chars.peek() {
            if c.is_ascii_alphanumeric() {
                chars.next();
            } else {
                break Token::Identifier(&lang[start_index..*end_index]);
            }
        }
    }
}
