use std::fmt::Display;

// #[repr(transparent)]
// #[derive(Clone)]
// struct CharValue(char);
// #[repr(transparent)]
// #[derive(Clone)]
// struct StringValue(String);
// #[repr(transparent)]
// #[derive(Clone)]
// struct NumberValue(f64);

trait ValueType: Clone {
    type T;

    fn value(&self) -> Self::T;
}

impl ValueType for char {
    type T = char;

    fn value(&self) -> Self::T {
        self.0
    }
}

// impl Display for CharValue {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }

impl ValueType for String {
    type T = String;

    fn value(&self) -> Self::T {
        self.0.clone()
    }
}

// impl Display for StringValue {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }

impl ValueType for f64 {
    type T = f64;

    fn value(&self) -> Self::T {
        self.0
    }
}

// impl Display for NumberValue {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }

#[derive(Clone, Copy)]
struct Span {
    line: usize,
    offset: usize,
    len: usize,
}

impl Span {
    fn new(line: usize, offset: usize, len: usize) -> Self {
        Self { line, offset, len }
    }
}

#[derive(Clone)]
enum Lexeme {
    String(String),
    Number(f64),
    Identifier(String),
    Star(char),
    BangEq(String),
}

impl From<&'static str> for Lexeme {
    fn from(value: &'static str) -> Self {
        match value {
            "*" => Lexeme::Star('*'),
            _ => panic!(),
        }
    }
}

impl Display for Lexeme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match &self {
            Lexeme::String(_) => "STRING",
            Lexeme::Number(_) => "NUMBER",
            Lexeme::Identifier(_) => "IDENTIFIER",
            Lexeme::Star(_) => "STAR",
            Lexeme::BangEq(_) => "BANG_EQ",
        };
        write!(f, "{name}")
    }
}

struct Token {
    lexeme: Lexeme,
    span: Span,
}

impl Token {
    pub fn new(lexeme: Lexeme, span: Span) -> Token {
        Token { lexeme, span }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lexeme = &self.lexeme;
        match lexeme {
            Lexeme::String(value) => {
                write!(f, "{lexeme} \"{value}\" {value}")
            }
            Lexeme::Number(value) => {
                let val_as_str = &format!("{value}");
                let trimmed = strip_suffix(val_as_str, ".0");
                write!(f, "{lexeme} {trimmed} {val_as_str}")
            }
            Lexeme::Identifier(value) => {
                write!(f, "{lexeme} {value} null")
            }
            _ => write!(f, "{lexeme} {lexeme} null"),
        }
    }
}

fn strip_suffix(s: &str, p: &str) -> String {
    s.strip_suffix(p).unwrap_or(s).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_token() {
        let token = Token::new(
            Lexeme::String(StringValue("hello".to_string())),
            Span::new(1, 2, 3),
        );
        eprintln!("{token}");
    }
}
