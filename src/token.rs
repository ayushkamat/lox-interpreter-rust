use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    QuestionMark,
    Colon,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords.
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

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Number(n) => write!(f, "{}", n),
            other => write!(
                f,
                "{}",
                match other {
                    TokenType::LeftParen => "'('",
                    TokenType::RightParen => "')'",
                    TokenType::LeftBrace => "'{'",
                    TokenType::RightBrace => "'}'",
                    TokenType::Comma => "',",
                    TokenType::Dot => "'.'",
                    TokenType::Minus => "'-'",
                    TokenType::Plus => "'+'",
                    TokenType::Semicolon => "';'",
                    TokenType::Slash => "'/'",
                    TokenType::Star => "'*'",
                    TokenType::QuestionMark => "'?'",
                    TokenType::Colon => "':'",

                    TokenType::Bang => "'!'",
                    TokenType::BangEqual => "'!='",
                    TokenType::Equal => "'='",
                    TokenType::EqualEqual => "'=='",
                    TokenType::Greater => "'>'",
                    TokenType::GreaterEqual => "'>='",
                    TokenType::Less => "'<'",
                    TokenType::LessEqual => "'<='",

                    TokenType::Identifier(i) => &i,
                    TokenType::String(s) => &s,
                    TokenType::Number(n) => "", // covered by above match

                    TokenType::And => "'and'",
                    TokenType::Class => "'class'",
                    TokenType::Else => "'else'",
                    TokenType::False => "'false'",
                    TokenType::Fun => "'fun'",
                    TokenType::For => "'for'",
                    TokenType::If => "'if'",
                    TokenType::Nil => "'nil'",
                    TokenType::Or => "'or'",
                    TokenType::Print => "'print'",
                    TokenType::Return => "'return'",
                    TokenType::Super => "'super'",
                    TokenType::This => "'this'",
                    TokenType::True => "'true'",
                    TokenType::Var => "'var'",
                    TokenType::While => "'while'",

                    TokenType::EOF => "<EOF>",
                }
            ),
        }
    }
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
}

impl Token {
    pub fn to_string(&self) -> String {
        format!("{}, Line {}", self.token_type, self.line)
    }
}
