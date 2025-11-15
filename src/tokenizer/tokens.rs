use fancy_regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub struct CommentData {
    index: i128,
    value: String,
}

impl CommentData {
    pub fn new(index: i128, value: String) -> Self {
        Self { index, value }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct StringLiteralData {
    index: i128,
    value: String,
}
impl StringLiteralData {
    pub fn new(index: i128, value: String) -> Self {
        Self { index, value }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NumberLiteralData {
    index: i128,
    value: f64,
}

impl NumberLiteralData {
    pub fn new(index: i128, value: f64) -> Self {
        Self { index, value }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct IdentifierData {
    index: i128,
    value: String,
}
impl IdentifierData {
    pub fn new(index: i128, value: String) -> Self {
        Self { index, value }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Token {
    Whitespace,
    LineBreak { index: i128 },
    Comment(CommentData),
    StringLiteral(StringLiteralData),
    NumberLiteral(NumberLiteralData),
    Identifier(IdentifierData),

    // keywords / literals
    Null { index: i128 },
    True { index: i128 },
    False { index: i128 },
    As { index: i128 },
    For { index: i128 },
    While { index: i128 },
    Break { index: i128 },
    Continue { index: i128 },
    If { index: i128 },
    Else { index: i128 },
    Return { index: i128 },
    Let { index: i128 },
    Const { index: i128 },
    Start { index: i128 },
    OnLoop { index: i128 },
    IntKeyword { index: i128 },
    DecimalKeyword { index: i128 },
    StringKeyword { index: i128 },
    BooleanKeyword { index: i128 },

    // operators & symbols
    LBrace { index: i128 },
    RBrace { index: i128 },
    LBracket { index: i128 },
    RBracket { index: i128 },
    LParen { index: i128 },
    RParen { index: i128 },
    Semicolon { index: i128 },
    Colon { index: i128 },
    Dot { index: i128 },
    Comma { index: i128 },
    Pow { index: i128 },
    Star { index: i128 },
    DoubleEq { index: i128 },
    Arrow { index: i128 },
    Eq { index: i128 },
    NotEq { index: i128 },
    AndAnd { index: i128 },
    And { index: i128 },
    Caret { index: i128 },
    Tilde { index: i128 },
    Bang { index: i128 },
    OrOr { index: i128 },
    Or { index: i128 },
    PlusPlus { index: i128 },
    Plus { index: i128 },
    MinusMinus { index: i128 },
    Minus { index: i128 },
    Backslash { index: i128 },
    Percent { index: i128 },
    QMark { index: i128 },
    GreaterThanOrEqual { index: i128 },
    LessThanOrEqual { index: i128 },
    BitshiftRight { index: i128 },
    GreaterThan { index: i128 },
    BitShiftLeft { index: i128 },
    LessThan { index: i128 },

    // End of File
    Eof,
}

trait StringExt {
    fn remove_first_and_last(self) -> String;
}
impl StringExt for String {
    fn remove_first_and_last(self) -> String {
        let mut chars = self.chars();
        chars.next();
        chars.next_back();
        chars.collect()
    }
}

// macro to create regexes that start at the beginning of a line
macro_rules! start_regex {
    ($regex:expr) => {
        Regex::new(concat!("^", $regex)).unwrap()
    };
}

// macro for generating token creators from regex
macro_rules! token {
    // Version for when no variables are used
    ($regex: expr, |_, _| $body:expr) => {
        (start_regex!($regex), |_, _| $body)
    };

    // Version for when only one variable is used
    ($regex: expr, |$index:ident, _| $body:expr) => {
        (start_regex!($regex), |$index: i128, _| $body)
    };

    // Version for when both variables are used
    ($regex:expr, |$index:ident, $val:ident| $body:expr) => {
        (start_regex!($regex), |$index: i128, $val: String| $body)
    };
}

type TokenCreator = fn(i128, String) -> Token;

lazy_static::lazy_static! {
    pub static ref TOKEN_CONVERTERS: Vec<(Regex, TokenCreator)> = vec![
        token!(r"[ \t]+", |_, _| Token::Whitespace),
        token!(r"\r?\n", |index, _| Token::LineBreak { index }),
        token!(r"//(.*?)(?=\r?\n|$)", |index, val| Token::Comment(CommentData::new(index, val[2..].to_string() ))),
        token!(r#""[^"\r\n]+""#, |index, val| Token::StringLiteral(StringLiteralData::new(index, val.remove_first_and_last() ))),
        token!(r#"'[^'\r\n]+'"#, |index, val| Token::StringLiteral(StringLiteralData::new(index, val.remove_first_and_last() ))),
        token!(r#"`[^`]+`"#, |index, val| Token::StringLiteral(StringLiteralData::new(index, val.remove_first_and_last() ))),
        token!(r"-?[0-9]+\.?[0-9]*(?![a-zA-Z$_])", |index, val| Token::NumberLiteral(NumberLiteralData::new(index, val.parse::<f64>().unwrap() ))),
        //token!(r"^\d*.\d+", |index, val| Token::DecimalLiteral { index, value: val.parse::<f64>().unwrap() }),

        // punctuation / symbols
        token!(r"\{", |index, _| Token::LBrace { index }),
        token!(r"\}", |index, _| Token::RBrace { index }),
        token!(r"\[", |index, _| Token::LBracket { index }),
        token!(r"\]", |index, _| Token::RBracket { index }),
        token!(r"\(", |index, _| Token::LParen { index }),
        token!(r"\)", |index, _| Token::RParen { index }),
        token!(r";", |index, _| Token::Semicolon { index }),
        token!(r":", |index, _| Token::Colon { index }),
        token!(r"\.", |index, _| Token::Dot { index }),
        token!(r"\,", |index, _| Token::Comma { index }),
        token!(r"\*\*", |index, _| Token::Pow { index }),
        token!(r"\*", |index, _| Token::Star { index }),
        token!(r"==", |index, _| Token::DoubleEq { index }),
        token!(r"->", |index, _| Token::Arrow { index }),
        token!(r"=", |index, _| Token::Eq { index }),
        token!(r"!=", |index, _| Token::NotEq { index }),
        token!(r"&&", |index, _| Token::AndAnd { index }),
        token!(r"&", |index, _| Token::And { index }),
        token!(r"\^", |index, _| Token::Caret { index }),
        token!(r"~", |index, _| Token::Tilde { index }),
        token!(r"!", |index, _| Token::Bang { index }),
        token!(r"\|\|", |index, _| Token::OrOr { index }),
        token!(r"\|", |index, _| Token::Or { index }),
        token!(r"\+\+", |index, _| Token::PlusPlus { index }),
        token!(r"\+", |index, _| Token::Plus { index }),
        token!(r"\-\-", |index, _| Token::MinusMinus { index }),
        token!(r"\-", |index, _| Token::Minus { index }),
        token!(r"\\", |index, _| Token::Backslash { index }),
        token!(r"%", |index, _| Token::Percent { index }),
        token!(r"\?", |index, _| Token::QMark { index }),
        token!(r">=", |index, _| Token::GreaterThanOrEqual { index }),
        token!(r"<=", |index, _| Token::LessThanOrEqual { index }),
        token!(r">>", |index, _| Token::BitshiftRight { index }),
        token!(r">", |index, _| Token::GreaterThan { index }),
        token!(r"<<", |index, _| Token::BitShiftLeft { index }),
        token!(r"<", |index, _| Token::LessThan { index }),

        // keywords / literals
        token!(r"null", |index, _| Token::Null { index }),
        token!(r"start", |index, _| Token::Start { index }),
        token!(r"int", |index, _| Token::IntKeyword { index }),
        token!(r"decimal", |index, _| Token::DecimalKeyword { index }),
        token!(r"string", |index, _| Token::StringKeyword { index }),
        token!(r"bool", |index, _| Token::BooleanKeyword { index }),
        token!(r"true", |index, _| Token::True { index }),
        token!(r"false", |index, _| Token::False { index }),
        token!(r"as", |index, _| Token::As { index }),
        token!(r"for", |index, _| Token::For { index }),
        token!(r"while", |index, _| Token::While { index }),
        token!(r"on_loop", |index, _| Token::OnLoop { index }),
        token!(r"break", |index, _| Token::Break { index }),
        token!(r"continue", |index, _| Token::Continue { index }),
        token!(r"if", |index, _| Token::If { index }),
        token!(r"else", |index, _| Token::Else { index }),
        token!(r"return", |index, _| Token::Return { index }),
        token!(r"let", |index, _| Token::Let { index }),
        token!(r"const", |index, _| Token::Const { index }),

        // generic identifier last (catch-all)
        token!(r"[a-zA-Z$_][a-zA-Z0-9$_]*", |index, val| Token::Identifier(IdentifierData::new(index, val))),
    ];
}
