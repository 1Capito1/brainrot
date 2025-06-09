use strum_macros::Display;

#[derive(Display, Debug, Clone, PartialEq)]
pub enum Literal {
    Number(f32),
    String(String),
    Boolean(bool),
    Nil,
}

#[derive(Display, Clone, Copy, Debug, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub enum TokenType {
    // Single Character Tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,

    // one or two Character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals
    String,
    Number,

    // keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    r#type: TokenType,
    pub lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

impl Token {
    pub fn new(r#type: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Self {
        Self {
            r#type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn get_type(&self) -> &TokenType {
        &self.r#type
    }
    pub fn get_literal(&mut self) -> &Literal {
        self.literal.as_ref().unwrap()
    }
    pub fn get_line(&mut self) -> usize {
        self.line
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.lexeme.clone())
    }
}
