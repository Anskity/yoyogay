use crate::text_data::TextRange;
use crate::Boxxable;

pub struct Token {
    pub data: TokenData,
    pub text_range: TextRange,
}

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
        IdetifierRecognizer{}.to_box()
    ];

    while ptr < src.len() {
        let chr = &src.chars().nth(ptr).expect("Ptr was outside of src");
        
        if [' ', '\t', '\n'].contains(&chr) {
            ptr += 1;
            continue;
        } 

        let recognizer = recognizers.iter().find(|rec| rec.recognize(&src[ptr..])).ok_or_else(|| chr.clone())?;
        let (token, numb) = recognizer.get_token(&src[ptr..]);
        assert_ne!(numb, 0);

        ptr += numb;
        tokens.push(token);
    }

    Ok(tokens)
}

trait TokenRecognizer {
    fn recognize(&self, _: &str) -> bool;
    fn get_token(&self, _: &str) -> (Token, usize);
}

struct IdetifierRecognizer;
impl TokenRecognizer for IdetifierRecognizer {
    fn recognize(&self, _: &str) -> bool {
        todo!()
    }

    fn get_token(&self, _: &str) -> (Token, usize) {
        todo!()
    }
}

