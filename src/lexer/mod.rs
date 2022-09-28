// ===================================================================
// This is not needed because we are using the Token enum in rust
// type Token struct {
//    Type    TokenType
//    Literal string
// }

// func newToken(tokenType token.TokenType, ch byte) token.Token {
//    return token.Token{Type: tokenType, Literal: string(ch)}
// }

// Question:
// In the book, using golang, we use the Token struct, where we
// use the "Literal" field, but in rust we use just the Token enum value,
// Why do we need the "Literal" value in golang then?

// ===================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal,
    Blank,
    Eof,

    // Identifiers + literals
    Ident(String),
    Int(i64),
    String(String),
    Bool(bool),

    // Statements
    Assign,
    If,
    Else,

    // Operators
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Equal,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,

    // Delimiters
    Comma,
    Colon,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Lbracket,
    Rbracket,

    // Reseved keywords
    Func,
    Let,
    Return,
}

#[derive(Default, Debug)]
struct Lexer {
    input: String,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: u8,               // current char under examination
}

impl Lexer {
    fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.chars().collect::<String>(),
            ..Default::default()
        };

        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.chars().count() {
            self.ch = 0
        } else {
            self.ch = self.input.as_bytes()[self.read_position]
        }
        self.position = self.read_position;
        self.read_position += 1
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.ch {
                b' ' | b'\t' => {
                    self.read_char();
                }
                _ => {
                    break;
                }
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            }
            b'/' => Token::Slash,
            b'*' => Token::Asterisk,
            b'<' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::LessThanEqual
                } else {
                    Token::LessThan
                }
            }
            b'>' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::GreaterThanEqual
                } else {
                    Token::GreaterThan
                }
            }
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b'{' => Token::Lbrace,
            b'}' => Token::Rbrace,
            b'[' => Token::Lbracket,
            b']' => Token::Rbracket,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            b':' => Token::Colon,
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                return self.consume_identifier();
            }
            b'0'..=b'9' => {
                return self.read_number();
            }
            b'"' => {
                return self.consume_string();
            }
            b'\n' => {
                if self.peek_char() == b'\n' {
                    Token::Blank
                } else {
                    self.read_char();
                    return self.next_token();
                }
            }
            0 => Token::Eof,
            _ => Token::Illegal,
        };

        self.read_char();

        return tok;
    }

    fn consume_identifier(&mut self) -> Token {
        let start_pos = self.position;

        loop {
            if is_letter(self.ch) {
                self.read_char();
            } else {
                break;
            }
        }

        let literal = &self.input[start_pos..self.position];

        match literal {
            "fn" => Token::Func,
            "let" => Token::Let,
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            _ => Token::Ident(String::from(literal)),
        }
    }

    fn peek_char(&mut self) -> u8 {
        if self.read_position >= self.input.chars().count() {
            return 0;
        } else {
            return self.input.as_bytes()[self.read_position];
        }
    }

    fn read_number(&mut self) -> Token {
        let start_pos = self.position;

        loop {
            if is_digit(self.ch) {
                self.read_char();
            } else {
                break;
            }
        }

        let literal = &self.input[start_pos..self.position];

        Token::Int(literal.parse::<i64>().unwrap())
    }

    fn consume_string(&mut self) -> Token {
        self.read_char();

        let start_pos = self.position;

        loop {
            match self.ch {
                b'"' | 0 => {
                    let literal = self.input[start_pos..self.position].to_string();
                    self.read_char();
                    return Token::String(literal);
                }
                _ => {
                    self.read_char();
                }
            }
        }
    }
}

pub fn is_letter(ch: u8) -> bool {
    return (b'a' <= ch && ch <= b'z') | (b'A' <= ch && ch <= b'Z') | (ch == b'_');
}

pub fn is_digit(ch: u8) -> bool {
    return b'0' <= ch && ch <= b'9';
}

#[cfg(test)]
mod tests {
    use super::is_digit;
    use super::is_letter;
    use super::Lexer;
    use super::Token;

    #[test]
    fn test_is_letter() {
        assert_eq!(true, is_letter(b'a'));
        assert_eq!(false, is_letter(b'1'));
        assert_eq!(false, is_letter(b'!'));
    }

    #[test]
    fn test_is_digit() {
        assert_eq!(true, is_digit(b'0'));
        assert_eq!(true, is_digit(b'5'));
        assert_eq!(false, is_digit(b'#'));
        assert_eq!(false, is_digit(b'!'));
    }

    #[test]
    fn test_next_token() {
        let input = r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
  return true;
} else {
  return false;
}

10 == 10;
10 != 9;
10 <= 10;
10 >= 10;
"foobar";
"foo bar";

[1, 2];


{"foo": "bar"};
"#;

        let tests = vec![
            Token::Let,
            Token::Ident(String::from("five")),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("ten")),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Blank,
            Token::Let,
            Token::Ident(String::from("add")),
            Token::Assign,
            Token::Func,
            Token::Lparen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::Rparen,
            Token::Lbrace,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::Semicolon,
            Token::Rbrace,
            Token::Semicolon,
            Token::Blank,
            Token::Let,
            Token::Ident(String::from("result")),
            Token::Assign,
            Token::Ident(String::from("add")),
            Token::Lparen,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::Rparen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(5),
            Token::Semicolon,
            Token::Int(5),
            Token::LessThan,
            Token::Int(10),
            Token::GreaterThan,
            Token::Int(5),
            Token::Semicolon,
            Token::Blank,
            Token::If,
            Token::Lparen,
            Token::Int(5),
            Token::LessThan,
            Token::Int(10),
            Token::Rparen,
            Token::Lbrace,
            Token::Return,
            Token::Bool(true),
            Token::Semicolon,
            Token::Rbrace,
            Token::Else,
            Token::Lbrace,
            Token::Return,
            Token::Bool(false),
            Token::Semicolon,
            Token::Rbrace,
            Token::Blank,
            Token::Int(10),
            Token::Equal,
            Token::Int(10),
            Token::Semicolon,
            Token::Int(10),
            Token::NotEqual,
            Token::Int(9),
            Token::Semicolon,
            Token::Int(10),
            Token::LessThanEqual,
            Token::Int(10),
            Token::Semicolon,
            Token::Int(10),
            Token::GreaterThanEqual,
            Token::Int(10),
            Token::Semicolon,
            Token::String(String::from("foobar")),
            Token::Semicolon,
            Token::String(String::from("foo bar")),
            Token::Semicolon,
            Token::Blank,
            Token::Lbracket,
            Token::Int(1),
            Token::Comma,
            Token::Int(2),
            Token::Rbracket,
            Token::Semicolon,
            Token::Blank,
            Token::Blank,
            Token::Lbrace,
            Token::String(String::from("foo")),
            Token::Colon,
            Token::String(String::from("bar")),
            Token::Rbrace,
            Token::Semicolon,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input);

        for expect in tests {
            let tok = lexer.next_token();

            assert_eq!(expect, tok);
        }
    }
}
