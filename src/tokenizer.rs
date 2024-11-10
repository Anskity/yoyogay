use std::collections::HashMap;

use crate::text_data::TextRange;
use crate::Boxxable;

#[derive(Debug)]
pub struct Token {
    pub data: TokenData,
    pub text_range: TextRange,
}

#[derive(Debug, Clone)]
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
    Var,
    Const,
    ModAccess,
    NotEquals,
    IsEquals,
}

pub fn tokenize(src: &str) -> Result<Vec<Token>, char> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut ptr: usize = 0;

    let recognizers: &[Box<dyn TokenRecognizer>] = &[
        IdetifierRecognizer {}.to_box(),
        NumericLiteralRecognizer {}.to_box(),
        SymbolRecognizer::new().to_box(),
    ];

    while ptr < src.len() {
        let chr = &src.chars().nth(ptr).expect("Ptr was outside of src");

        if [' ', '\t', '\n'].contains(&chr) {
            ptr += 1;
            continue;
        }

        let recognizer = recognizers
            .iter()
            .find(|rec| rec.recognize(&src[ptr..]))
            .ok_or_else(|| chr.clone())?;
        let (token_data, numb) = recognizer.get_token(&src[ptr..]);
        assert_ne!(numb, 0);

        ptr += numb;

        let token = Token {
            data: token_data,
            text_range: TextRange::new_empty(),
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

        (TokenData::Identifier(id), len)
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
                ("{".to_string(), TokenData::OpenCurly),
                ("}".to_string(), TokenData::CloseCurly),
            ]),
        }
    }
}

impl TokenRecognizer for SymbolRecognizer {
    fn recognize(&self, code_left: &str) -> bool {
        self.map.keys().any(|k| code_left[0..k.len()] == *k)
    }

    fn get_token(&self, code_left: &str) -> (TokenData, usize) {
        let symbol = self.map.keys().find(|k| code_left[0..k.len()] == **k).expect("Symbol not found");
        let token_data = self.map.get(symbol).expect("Token Data not found");

        (token_data.clone(), symbol.len())
    }
}
