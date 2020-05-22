fn main() {
    let lang = String::from("v = \"string\"; o = \"string2\"");
    let mut chars = lang.chars().enumerate();

    while let Some((prev_index, c)) = chars.next() {
        let token = match c {
            '"' => scan_string(&lang, &mut chars, prev_index),
            _ => Token::Other(c),
        };

        println!("{:?}", token)
    }
}

#[derive(Debug)]
enum Token<'a> {
    String(&'a str),
    Other(char),
}

fn scan_string<'a, T: Iterator<Item = (usize, char)>>(lang: &'a str, chars: &mut T, prev_index: usize) -> Token<'a> {
    let value = loop {
        if let Some((curr_index, c)) = chars.next() {
            match c {
                '"' => break Token::String(&lang[prev_index + 1..curr_index]),
                _ => continue,
            }
        } else {
            panic!("unterminated string literal")
        }
    };
    value
}
