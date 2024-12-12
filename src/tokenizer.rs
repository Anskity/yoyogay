use crate::ast::{OperatorType, VariableModificationType};
use crate::parser::utils::delimiter_checker::DelimiterChecker;
use crate::text_data::{TextPos, TextRange};
use crate::Boxxable;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub data: TokenData,
    pub text_range: TextRange,
}

pub trait TokensUtils {
    fn split_tks<T: AsRef<TokenData>>(&self, splitter: T) -> Vec<&[Token]>;
    fn find_free<T: AsRef<TokenData>>(&self, search_tk: T) -> Option<usize>;
    fn find_pair(&self, pos: usize) -> Option<usize>;
}

impl TokensUtils for [Token] {
    fn split_tks<'a, T: AsRef<TokenData>>(&'a self, splitter: T) -> Vec<&'a [Token]> {
        let splitter = splitter.as_ref();
        let mut slices: Vec<&[Token]> = Vec::new();
        let mut last_idx: usize = 0;

        for (i, _) in self.iter().enumerate() {
            let tk = &self[i];

            if tk.data == *splitter {
                slices.push(&self[last_idx..i]);
                last_idx = i + 1;
            } else if i == self.len() - 1 {
                slices.push(&self[last_idx..]);
            }
        }

        slices
    }
    fn find_free<T: AsRef<TokenData>>(&self, search_tk: T) -> Option<usize> {
        enum FindDelimiterCheckerMode {
            Normal,
            Curly,
            Paren,
            Brack,
        }
        let search_tk = search_tk.as_ref();

        assert!(!matches!(
            search_tk,
                | TokenData::CloseParenthesis
                | TokenData::CloseCurly
                | TokenData::CloseBracket
        ));

        let mode = match search_tk {
            TokenData::OpenParenthesis => FindDelimiterCheckerMode::Paren,
            TokenData::OpenCurly => FindDelimiterCheckerMode::Curly,
            TokenData::OpenBracket => FindDelimiterCheckerMode::Brack,
            _ => FindDelimiterCheckerMode::Normal,
        };

        let mut delimiter_checker = DelimiterChecker::new();
        for (i, tk) in self.iter().enumerate() {
            delimiter_checker.check(&tk).ok()?;
            if tk.data == *search_tk && delimiter_checker.is_free() {
                return Some(i);
            }

            if matches!((&tk.data, &mode, delimiter_checker.paren_level, delimiter_checker.brack_level, delimiter_checker.curly_level),
                        (TokenData::OpenParenthesis, FindDelimiterCheckerMode::Paren, 1, 0, 0) |
                        (TokenData::OpenBracket, FindDelimiterCheckerMode::Brack, 0, 1, 0) |
                        (TokenData::OpenCurly, FindDelimiterCheckerMode::Curly, 0, 0, 1)
                        ) {
                return Some(i);
            }
        }

        None
    }

    fn find_pair(&self, mut pos: usize) -> Option<usize> {
        assert!(pos < self.len());

        assert!(matches!(
            &self[pos].data,
            TokenData::OpenParenthesis
                | TokenData::OpenCurly
                | TokenData::OpenBracket
                | TokenData::CloseParenthesis
                | TokenData::CloseCurly
                | TokenData::CloseBracket
        ));
        let rev = match &self[pos].data {
            TokenData::OpenParenthesis | TokenData::OpenCurly | TokenData::OpenBracket => false,
            TokenData::CloseParenthesis | TokenData::CloseCurly | TokenData::CloseBracket => true,
            _ => panic!("?????????"),
        };

        let mut delimiter_checker = DelimiterChecker::new();
        loop {
            let tk = &self[pos];
            if rev {
                delimiter_checker.check_reverse(tk).ok()?;
            } else {
                delimiter_checker.check(tk).ok()?;
            }

            if delimiter_checker.is_free() {
                return Some(pos);
            }

            if rev {
                pos -= 1;
            } else {
                pos += 1;
            }

            if pos == 0 && rev {
                break;
            }

            if pos == self.len() && !rev {
                break;
            }
        }

        None
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match &self.data {
            TokenData::ModAccess => "::".to_string(),
            TokenData::NotEquals => "!=".to_string(),
            TokenData::IsEquals => "==".to_string(),
            TokenData::Equals => "=".to_string(),
            TokenData::IncreaseBy => "+=".to_string(),
            TokenData::DecreaseBy => "-=".to_string(),
            TokenData::MultiplyBy => "*=".to_string(),
            TokenData::DivideBy => "/=".to_string(),
            TokenData::Add => "+".to_string(),
            TokenData::Sub => "-".to_string(),
            TokenData::Mul => "*".to_string(),
            TokenData::Div => "/".to_string(),
            TokenData::Or => "||".to_string(),
            TokenData::OpenParenthesis => "(".to_string(),
            TokenData::CloseParenthesis => ")".to_string(),
            TokenData::OpenCurly => "{".to_string(),
            TokenData::CloseCurly => "}".to_string(),
            TokenData::OpenBracket => "[".to_string(),
            TokenData::CloseBracket => "]".to_string(),
            TokenData::Comma => ",".to_string(),
            TokenData::Semilicon => ";".to_string(),
            TokenData::Pipe => "|".to_string(),
            TokenData::Dot => ".".to_string(),
            TokenData::Var => "var".to_string(),
            TokenData::Const => "const".to_string(),
            TokenData::Fn => "fn".to_string(),
            TokenData::If => "if".to_string(),
            TokenData::Else => "else".to_string(),
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
    IncreaseBy,
    DecreaseBy,
    MultiplyBy,
    DivideBy,

    OpenParenthesis,
    CloseParenthesis,
    OpenCurly,
    CloseCurly,
    OpenBracket,
    CloseBracket,
    Var,
    Const,
    Fn,
    If,
    Else,
    ModAccess,
    NotEquals,
    IsEquals,
    Comma,
    Semilicon,
    Pipe,
    Dot,
    Or,
}

impl TokenData {
    pub fn operator_type(&self) -> Option<OperatorType> {
        match self {
            TokenData::Add => Some(OperatorType::Add),
            TokenData::Sub => Some(OperatorType::Sub),
            TokenData::Mul => Some(OperatorType::Mul),
            TokenData::Div => Some(OperatorType::Div),
            TokenData::NotEquals => Some(OperatorType::NotEquals),
            TokenData::IsEquals => Some(OperatorType::IsEquals),
            TokenData::Or => Some(OperatorType::Or),
            _ => None,
        }
    }

    pub fn variable_modification_type(&self) -> Option<VariableModificationType> {
        match self {
            TokenData::Equals => Some(VariableModificationType::Set),
            TokenData::IncreaseBy => Some(VariableModificationType::IncreaseBy),
            TokenData::DecreaseBy => Some(VariableModificationType::DecreaseBy),
            TokenData::MultiplyBy => Some(VariableModificationType::MultiplyBy),
            TokenData::DivideBy => Some(VariableModificationType::DivideBy),
            _ => None,
        }
    }
}

impl AsRef<TokenData> for TokenData {
    fn as_ref(&self) -> &TokenData {
        self
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
            "fn" => TokenData::Fn,
            "if" => TokenData::If,
            "else" => TokenData::Else,
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
    vec: Vec<(String, TokenData)>,
}

impl SymbolRecognizer {
    pub fn new() -> Self {
        SymbolRecognizer {
            vec: Vec::from([
                ("::".to_string(), TokenData::ModAccess),
                ("!=".to_string(), TokenData::NotEquals),
                ("==".to_string(), TokenData::IsEquals),
                ("+=".to_string(), TokenData::IncreaseBy),
                ("-=".to_string(), TokenData::DecreaseBy),
                ("*=".to_string(), TokenData::MultiplyBy),
                ("/=".to_string(), TokenData::DivideBy),
                ("||".to_string(), TokenData::Or),
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
                (".".to_string(), TokenData::Dot),
            ]),
        }
    }
}

impl TokenRecognizer for SymbolRecognizer {
    fn recognize(&self, code_left: &str) -> bool {
        self.vec.iter().any(|(k, _)| code_left[0..k.len()] == *k)
    }

    fn get_token(&self, code_left: &str) -> (TokenData, usize) {
        let idx = self
            .vec
            .iter()
            .position(|(k, _)| code_left[0..k.len()] == **k)
            .expect("Symbol not found");
        let (key, token_data) = self.vec.get(idx).expect("Token Data not found");

        (token_data.clone(), key.len())
    }
}
