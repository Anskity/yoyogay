use std::{collections::HashMap, ffi::OsStr, fs::File, io::{BufRead, BufReader}, path::PathBuf};

use crate::{ast::Node, parser::{parse_tks, ParseError}, tokenizer::{tokenize, Token, TokenizeError}};

use super::OrganizeError;

#[derive(Debug)]
pub struct Object<'a> {
    pub id: String,
    pub create: Option<(Node<'a>, Vec<Token>)>,
    pub step: Option<(Node<'a>, Vec<Token>)>,
    pub draw: Option<(Node<'a>, Vec<Token>)>,
    pub draw_gui: Option<(Node<'a>, Vec<Token>)>,
    pub clean_up: Option<(Node<'a>, Vec<Token>)>,
}

#[derive(Debug)]
pub enum ObjectParseError {
    NoIdentification(String),
    UnknownEvent(String, usize),
    TokenizeError(TokenizeError),
    ParseError(ParseError),
}

impl Into<ObjectParseError> for TokenizeError {
    fn into(self) -> ObjectParseError {
        ObjectParseError::TokenizeError(self)
    }
}

impl<T:Into<ObjectParseError>> From<T> for OrganizeError {
    fn from(value: T) -> OrganizeError {
        value.into().into()
    }
}

impl Into<ObjectParseError> for ParseError {
    fn into(self) -> ObjectParseError {
        ObjectParseError::ParseError(self)
    }
}

pub fn organize_objects<'a>(path: PathBuf) -> Result<Vec<Object<'a>>, OrganizeError> {
    assert!(path.exists());
    assert!(path.is_dir());
    assert_eq!(path.file_name(), Some(OsStr::new("objects")));

    let dirs = path.read_dir().expect("Couldnt read objects directory");
    let mut objects: Vec<Object> = Vec::new();

    for entry in dirs {
        let entry = entry.expect("Couldnt read dir in read directory");
        let path = entry.path();
        if !path.is_file() {
            return Err(OrganizeError::UnexpectedFile(path));
        }
        let file = File::open(path).map_err(|e| OrganizeError::FailedReadingFromFileSystem(e))?;

        let reader = BufReader::new(file);
        let mut id: Option<String> = None;
        let mut events: HashMap<String, (String, usize)> = HashMap::new();

        let mut current_event: Option<String> = None;
        let mut line_buf: String = "".to_string();

        let mut lines = reader.lines().peekable();
        let mut current_line_idx: usize = 0;
        while let Some(line) = lines.next() {
            let line = line.map_err(|e| OrganizeError::FailedReadingFromFileSystem(e))?;

            if current_line_idx == 0 {
                if !line.starts_with("#id ") {
                    return Err(OrganizeError::ObjectParseError(ObjectParseError::NoIdentification(line)));
                }

                id = Some(line[4..].to_string());

                current_line_idx += 1;
                continue;
            }

            if line.starts_with("#event ") {
                if current_event.is_some() {
                    events.insert(current_event.clone().unwrap(), (line_buf.clone(), current_line_idx+1));
                }

                let event_name = &line[7..];
                current_event = Some(event_name.to_string());
                line_buf = "".to_string();

                current_line_idx += 1;
                continue;
            }

            let mut needs_to_push_new_line = false;
            if lines.peek().is_none() {
                if current_event.is_some() {
                    events.insert(current_event.clone().unwrap(), (line_buf.clone(), current_line_idx+1));
                }
            } else {
                needs_to_push_new_line = true;
            }

            line_buf.push_str(&line);
            if needs_to_push_new_line {
                line_buf.push('\n');
            }
            current_line_idx += 1;
        }

        let id = id.expect("No Object ID");

        let mut create: Option<String> = None;
        let mut step: Option<String> = None;
        let mut draw: Option<String> = None;
        let mut draw_gui: Option<String> = None;
        let mut clean_up: Option<String> = None;
        for event in events.keys() {
            let (src, line) = events.get(event).expect("?????????????????????");
            match event.as_str() {
                "create" => {
                    create = Some(src.to_string());
                }
                "step" => {
                    step = Some(src.to_string());
                }
                "draw" => {
                    draw = Some(src.to_string());
                }
                "draw_gui" => {
                    draw_gui = Some(src.to_string());
                }
                "clean_up" => {
                    clean_up = Some(src.to_string());
                }
                _ => {
                    return Err(ObjectParseError::UnknownEvent(src.to_string(), *line).into());
                }
            }
        }

        macro_rules! parse_event {
            ($event:ident) => {{
                let event_tokens: Option<Result<Vec<Token>, TokenizeError>> = if let Some(src) = &$event {
                    Some(tokenize(src))
                } else {
                    None
                };

                let event_tokens: Option<Vec<Token>> = if let Some(res) = event_tokens {
                    Some(res?)
                } else {
                    None
                };

                let event: Option<Node> = if let Some(tks) = event_tokens {
                    Some(parse_tks(&tks)?)
                } else {
                    None
                };

                (event, event_tokens)
            }}
        }

        macro_rules! change_option {
            ($event:ident) => {
                
            };
        }

        let create: Option<(Node, Vec<Token>)> = parse_event!(create);
        let step: Option<(Node, Vec<Token>)> = parse_event!(step);
        let draw: Option<(Node, Vec<Token>)> = parse_event!(draw);
        let draw_gui: Option<(Node, Vec<Token>)> = parse_event!(draw_gui);
        let clean_up: Option<(Node, Vec<Token>)> = parse_event!(clean_up);
        
        let object = Object {
            id,
            create,
            step,
            draw,
            draw_gui,
            clean_up,
        };

        objects.push(object);
    }

    Ok(objects)
}
