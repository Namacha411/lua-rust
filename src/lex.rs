use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::mem;

use crate::byte_code::ByteCode;
use crate::value::Value;

#[derive(Debug, PartialEq)]
pub enum Token {
    /// keywords
    And,
    Break,
    Do,
    Else,
    ElseIf,
    End,
    False,
    For,
    Function,
    Goto,
    If,
    In,
    Local,
    Nil,
    Not,
    Or,
    Repeat,
    Return,
    Then,
    True,
    Until,
    White,

    /// +
    Add,
    /// -
    Sub,
    /// *
    Mul,
    /// /
    Div,
    /// %
    Mod,
    /// ^
    Pow,
    /// #
    Len,
    /// &
    BitAnd,
    /// ~
    BitXor,
    /// |
    BitOr,
    /// <<
    ShiftLeft,
    /// >>
    ShiftRight,
    /// //
    IntDiv,
    /// ==
    Equal,
    /// ~=
    NotEq,
    /// <=
    LesEq,
    /// >=
    GreEq,
    /// <
    Less,
    /// >
    Greater,
    /// =
    Assign,
    /// (
    ParLeft,
    /// )
    ParRight,
    /// {
    CurlyLeft,
    /// }
    CurlyRight,
    /// [
    SqurLeft,
    /// ]
    SqurRight,
    /// ::
    DoubleColon,
    /// ;
    SemiColon,
    /// :
    Colon,
    /// ,
    Comma,
    /// .
    Dot,
    /// ..
    Concat,
    /// ...
    Dots,

    /// constant values
    Interger(i64),
    Float(f64),
    String(String),

    /// name of variables or table keys
    Name(String),

    // end
    Eos,
}

#[derive(Debug)]
pub struct Lex {
    input: File,
    ahead: Token,
}

impl Lex {
    pub fn new(input: File) -> Self {
        Lex {
            input,
            ahead: Token::Eos,
        }
    }

    pub fn peek(&mut self) -> &Token {
        if self.ahead == Token::Eos {
            self.ahead = self.do_next();
        }
        &self.ahead
    }

    pub fn next(&mut self) -> Token {
        if self.ahead == Token::Eos {
            self.do_next()
        } else {
            mem::replace(&mut self.ahead, Token::Eos)
        }
    }

    pub fn do_next(&mut self) -> Token {
        use Token::*;
        let ch = self.read_char();
        match ch {
            ' ' | '\r' | '\n' | '\t' => self.do_next(),
            '+' => Add,
            '*' => Mul,
            '%' => Mod,
            '^' => Pow,
            '#' => Len,
            '&' => BitAnd,
            '|' => BitOr,
            '(' => ParLeft,
            ')' => ParRight,
            '{' => CurlyLeft,
            '}' => CurlyRight,
            '[' => SqurLeft,
            ']' => SqurRight,
            ';' => SemiColon,
            ',' => Comma,
            '/' => self.check_ahead('/', IntDiv, Div),
            '=' => self.check_ahead('=', Equal, Assign),
            '~' => self.check_ahead('=', NotEq, BitXor),
            ':' => self.check_ahead(':', DoubleColon, Colon),

            '\0' => Eos,
            '"' => {
                let mut s = String::new();
                loop {
                    match self.read_char() {
                        '\0' => panic!("unfinished literal string"),
                        '"' => break,
                        ch => s.push(ch),
                    }
                }
                Token::String(s)
            }
            'A'..='Z' | 'a'..='z' | '_' => {
                let mut name = String::new();
                name.push(ch);
                loop {
                    match self.read_char() {
                        '\0' => break,
                        '_' => name.push('_'),
                        ch if ch.is_alphanumeric() => name.push(ch),
                        _ => {
                            self.input.seek(SeekFrom::Current(-1)).unwrap();
                            break;
                        }
                    }
                }
                Token::Name(name)
            }
            _ => panic!("unexpected char: {ch}"),
        }
    }

    fn read_char(&mut self) -> char {
        let mut buf = [0];
        self.input.read(&mut buf).unwrap();
        buf[0] as char
    }

    fn check_ahead(&mut self, ahead: char, long: Token, short: Token) -> Token {
        if self.read_char() == ahead {
            long
        } else {
            self.putback_char();
            short
        }
    }

    fn putback_char(&mut self) {
        self.input.seek(SeekFrom::Current(-1)).unwrap();
    }
}

#[derive(Debug)]
pub struct ParseProto {
    pub constants: Vec<Value>,
    pub byte_codes: Vec<ByteCode>,
}
