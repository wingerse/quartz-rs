use std::fmt;
use std::iter::Iterator;

pub mod chat;
use self::chat::Color;

mod parse;
pub use self::parse::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Code {
    Black,
    DarkBlue,
    DarkGreen,
    DarkCyan,
    DarkRed,
    Purple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    BrightGreen,
    Cyan,
    Red,
    Pink,
    Yellow,
    White,
    Reset,

    Obfuscated,
    Bold,
    StrikeThrough,
    Underlined,
    Italic,
}

impl Code {
    fn to_char(&self) -> char {
        match *self {
            Code::Black => '0',
            Code::DarkBlue => '1',
            Code::DarkGreen => '2',
            Code::DarkCyan => '3',
            Code::DarkRed => '4',
            Code::Purple => '5',
            Code::Gold => '6',
            Code::Gray => '7',
            Code::DarkGray => '8',
            Code::Blue => '9',
            Code::BrightGreen => 'a',
            Code::Cyan => 'b',
            Code::Red => 'c',
            Code::Pink => 'd',
            Code::Yellow => 'e',
            Code::White => 'f',
            Code::Reset => 'r',
            Code::Obfuscated => 'k',
            Code::Bold => 'l',
            Code::StrikeThrough => 'm',
            Code::Underlined => 'n',
            Code::Italic => 'o',
        }
    }

    fn to_color(&self) -> Color {
        match *self {
            Code::Black => Color::Black,
            Code::DarkBlue => Color::DarkBlue,
            Code::DarkGreen => Color::DarkGreen,
            Code::DarkCyan => Color::DarkCyan,
            Code::DarkRed => Color::DarkRed,
            Code::Purple => Color::Purple,
            Code::Gold => Color::Gold,
            Code::Gray => Color::Gray,
            Code::DarkGray => Color::DarkGray,
            Code::Blue => Color::Blue,
            Code::BrightGreen => Color::BrightGreen,
            Code::Cyan => Color::Cyan,
            Code::Red => Color::Red,
            Code::Pink => Color::Pink,
            Code::Yellow => Color::Yellow,
            Code::White => Color::White,
            Code::Reset => Color::Reset,
            _ => Color::Reset,
        }
    }

    fn is_formatting(&self) -> bool {
        match *self {
            Code::Obfuscated | Code::Bold | Code::StrikeThrough | Code::Underlined | Code::Italic => true,
            _ => false
        }
    }

    fn is_color(&self) -> bool {
        !self.is_formatting()
    }

    fn from_char(c: char) -> Option<Code> {
        match c {
            '0' => Some(Code::Black),
            '1' => Some(Code::DarkBlue),
            '2' => Some(Code::DarkGreen),
            '3' => Some(Code::DarkCyan),
            '4' => Some(Code::DarkRed),
            '5' => Some(Code::Purple),
            '6' => Some(Code::Gold),
            '7' => Some(Code::Gray),
            '8' => Some(Code::DarkGray),
            '9' => Some(Code::Blue),
            'a' => Some(Code::BrightGreen),
            'b' => Some(Code::Cyan),
            'c' => Some(Code::Red),
            'd' => Some(Code::Pink),
            'e' => Some(Code::Yellow),
            'f' => Some(Code::White),
            'r' => Some(Code::Reset),
            'k' => Some(Code::Obfuscated),
            'l' => Some(Code::Bold),
            'm' => Some(Code::StrikeThrough),
            'n' => Some(Code::Underlined),
            'o' => Some(Code::Italic),
            _ => None,
        }
    }

    fn is_valid(c: char) -> bool {
        Self::from_char(c).is_some()
    }
}

pub const LEGACY_CHAR: char = '§';

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", LEGACY_CHAR, self.to_char())
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Token {
    String(String),
    Codes(Vec<Code>),
}

struct Tokenizer {
    index: isize,
    chars: Vec<char>,
    has_invalid_code: bool,
    len: usize,
    control_char: char,
}

impl Tokenizer {
    fn new(s: &str, control_char: char) -> Tokenizer {
        Tokenizer {
            index: -1, chars: s.chars().collect(), 
            has_invalid_code: false, 
            len: s.len(), 
            control_char
        }
    }

    fn has_next(&self) -> bool {
        self.index != (self.len - 1) as isize
    }

    fn next(&mut self) -> char {
        self.index += 1;
        self.chars[self.index as usize]
    }

    fn back(&mut self) {
        self.index -= 1;
    }

    fn peek(&self) -> char {
        self.chars[(self.index + 1) as usize]
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.has_next() {
            return None;
        }
        let mut codes = Vec::new();
        while self.has_next() {
            if self.has_invalid_code {
                break;
            }
            if self.peek() != self.control_char {
                break;
            }
            self.next();
            if !self.has_next() || !Code::is_valid(self.peek()) {
                self.back();
                self.has_invalid_code = true;
                break;
            }

            codes.push(Code::from_char(self.next()).unwrap())
        }

        if !codes.is_empty() {
            return Some(Token::Codes(codes));
        }  

        let mut chars = Vec::new();
        while self.has_next() {
            if self.has_invalid_code {
                chars.push(self.next());
                if self.has_next() {
                    chars.push(self.next());
                }
                self.has_invalid_code = false;
                continue;
            }
            if self.peek() == self.control_char {
                break;
            }
            chars.push(self.next());
        } 
        Some(Token::String(chars.into_iter().collect()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tokenizer() {
        let s = "&6&l&kii&4&lWigit&6&l&kii";
        let mut t = Tokenizer::new(s, '&');
        let mut o = Iterator::next(&mut t).unwrap();
        assert_eq!(Token::Codes(vec![Code::Gold, Code::Bold, Code::Obfuscated]), o);
        o = Iterator::next(&mut t).unwrap();
        assert_eq!(Token::String("ii".into()), o);
        o = Iterator::next(&mut t).unwrap();
        assert_eq!(Token::Codes(vec![Code::DarkRed, Code::Bold]), o);
        o = Iterator::next(&mut t).unwrap();
        assert_eq!(Token::String("Wigit".into()), o);
        o = Iterator::next(&mut t).unwrap();
        assert_eq!(Token::Codes(vec![Code::Gold, Code::Bold, Code::Obfuscated]), o);
        o = Iterator::next(&mut t).unwrap();
        assert_eq!(Token::String("ii".into()), o);
    }
}