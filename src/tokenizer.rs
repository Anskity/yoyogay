use std::collections::HashMap;

use crate::ast::OperatorType;
use crate::text_data::{TextPos, TextRange};
use crate::Boxxable;

#[derive(Debug)]
pub struct Token {
    pub data: TokenData,
    pub text_range: TextRange,
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match &self.data {
            TokenData::ModAccess => "::".to_string(),
            TokenData::NotEquals => "!=".to_string(),
            TokenData::IsEquals => "==".to_string(),
            TokenData::Equals => "=".to_string(),
            TokenData::Add => "+".to_string(),
            TokenData::Sub => "-".to_string(),
            TokenData::Mul => "*".to_string(),
            TokenData::Div => "/".to_string(),
            TokenData::OpenParenthesis => "(".to_string(),
            TokenData::CloseParenthesis => ")".to_string(),
            TokenData::OpenCurly => "{".to_string(),
            TokenData::CloseCurly => "}".to_string(),
            TokenData::OpenBracket => "[".to_string(),
            TokenData::CloseBracket => "]".to_string(),
            TokenData::Comma => ",".to_string(),
            TokenData::Semilicon => ";".to_string(),
            TokenData::Pipe => "|".to_string(),
            TokenData::Var => "var".to_string(),
            TokenData::Const => "const".to_string(),
            TokenData::Identifier(id) => id.clone(),
            TokenData::NumericLiteral(num) => num.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenData {
    Identifier(String),
    NumericLiteral(usize),
    Equals,
    Add,
    Sub,
    Mul,
    Div,

    OpenParenthesis,
    CloseParenthesis,
    OpenCurly,
    CloseCurly,
    OpenBracket,
    CloseBracket,
    Var,
    Const,
    ModAccess,
    NotEquals,
    IsEquals,
    Comma,
    Semilicon,
    Pipe,
}

impl TokenData {
    pub fn operator_type(&self) -> Option<OperatorType> {
        match self {
            TokenData::Add => Some(OperatorType::Add),
            TokenData::Sub => Some(OperatorType::Sub),
            TokenData::Mul => Some(OperatorType::Mul),
            TokenData::Div => Some(OperatorType::Div),
            _ => None
        }
    }
}

#[derive(Debug)]
pub enum TokenizeError {
    UnknownCharacter(char),
}

pub fn tokenize(src: &str) -> Result<Vec<Token>, TokenizeError> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut ptr: usize = 0;
    let mut line: usize = 1;
    let mut pos: usize = 0;

    let recognizers: &[Box<dyn TokenRecognizer>] = &[
        IdetifierRecognizer {}.to_box(),
        NumericLiteralRecognizer {}.to_box(),
        SymbolRecognizer::new().to_box(),
    ];

    while ptr < src.len() {
        let chr = &src.chars().nth(ptr).expect("Ptr was outside of src");

        if *chr == '\n' {
            line += 1;
            pos = 0;
        } else {
            pos += 1;
        }

        if [' ', '\t', '\n'].contains(&chr) {
            ptr += 1;
            continue;
        }

        let recognizer = recognizers
            .iter()
            .find(|rec| rec.recognize(&src[ptr..]))
            .ok_or_else(|| TokenizeError::UnknownCharacter(chr.clone()))?;
        let (token_data, numb) = recognizer.get_token(&src[ptr..]);
        assert_ne!(numb, 0);

        let start_line = line.clone();
        let start_pos = pos.clone();
        let mut end_line = line.clone();
        let mut end_pos = pos.clone();

        for _ in 0..numb - 1 {
            if let Some('\n') = src.chars().nth(numb) {
                end_line += 1;
                end_pos = 0;
            } else {
                end_pos += 1;
            }
        }

        ptr += numb;

        let start = TextPos {
            line: start_line,
            pos: start_pos,
        };
        let end = TextPos {
            line: end_line,
            pos: end_pos,
        };

        let token = Token {
            data: token_data,
            text_range: TextRange { start, end },
        };
        tokens.push(token);
    }

    Ok(tokens)
}

trait TokenRecognizer {
    fn recognize(&self, _: &str) -> bool;
    fn get_token(&self, _: &str) -> (TokenData, usize);
}

struct IdetifierRecognizer;
impl TokenRecognizer for IdetifierRecognizer {
    fn recognize(&self, code_left: &str) -> bool {
        matches!(code_left.chars().nth(0).expect("Code is empty"), 'a'..'z' | 'A'..'Z' | '_')
    }

    fn get_token(&self, code_left: &str) -> (TokenData, usize) {
        let id: String = code_left
            .chars()
            .take_while(|c| matches!(c, 'a'..'z' | 'A'..'Z' | '_' | '0'..'9'))
            .collect();
        let len = id.len();

        let token_data = match id.as_str() {
            "const" => TokenData::Const,
            "var" => TokenData::Var,
            _ => TokenData::Identifier(id),
        };

        (token_data, len)
    }
}

struct NumericLiteralRecognizer;
impl TokenRecognizer for NumericLiteralRecognizer {
    fn recognize(&self, code_left: &str) -> bool {
        matches!(code_left.chars().nth(0).expect("Empty code"), '0'..'9')
    }

    fn get_token(&self, code_left: &str) -> (TokenData, usize) {
        let id: String = code_left
            .chars()
            .take_while(|c| matches!(c, '0'..'9'))
            .collect();
        let len = id.len();

        (
            TokenData::NumericLiteral(id.parse().expect("Numeric Literal error")),
            len,
        )
    }
}

struct SymbolRecognizer {
    map: HashMap<String, TokenData>,
}

impl SymbolRecognizer {
    pub fn new() -> Self {
        SymbolRecognizer {
            map: HashMap::from([
                ("::".to_string(), TokenData::ModAccess),
                ("!=".to_string(), TokenData::NotEquals),
                ("==".to_string(), TokenData::IsEquals),
                ("=".to_string(), TokenData::Equals),
                ("+".to_string(), TokenData::Add),
                ("-".to_string(), TokenData::Sub),
                ("*".to_string(), TokenData::Mul),
                ("/".to_string(), TokenData::Div),
                ("(".to_string(), TokenData::OpenParenthesis),
                (")".to_string(), TokenData::CloseParenthesis),
                ("[".to_string(), TokenData::OpenBracket),
                ("]".to_string(), TokenData::CloseBracket),
                ("{".to_string(), TokenData::OpenCurly),
                ("}".to_string(), TokenData::CloseCurly),
                (",".to_string(), TokenData::Comma),
                (";".to_string(), TokenData::Semilicon),
                ("|".to_string(), TokenData::Pipe),
            ]),
        }
    }
}

impl TokenRecognizer for SymbolRecognizer {
    fn recognize(&self, code_left: &str) -> bool {
        self.map.keys().any(|k| code_left[0..k.len()] == *k)
    }

    fn get_token(&self, code_left: &str) -> (TokenData, usize) {
        let symbol = self
            .map
            .keys()
            .find(|k| code_left[0..k.len()] == **k)
            .expect("Symbol not found");
        let token_data = self.map.get(symbol).expect("Token Data not found");

        (token_data.clone(), symbol.len())
    }
}
