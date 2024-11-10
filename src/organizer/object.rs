use std::{collections::HashMap, ffi::OsStr, fs::File, io::{BufRead, BufReader}, path::PathBuf};

use super::OrganizeError;

#[derive(Debug)]
pub struct Object {
    pub id: String,
    pub events: HashMap<String, String>,
}

#[derive(Debug)]
pub enum ObjectParseError {
    NoIdentification(String),
}

pub fn organize_objects(path: PathBuf) -> Result<Vec<Object>, OrganizeError> {
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
        let mut events: HashMap<String, String> = HashMap::new();

        let mut current_event: Option<String> = None;
        let mut line_buf: String = "".to_string();

        let mut lines = reader.lines().peekable();
        let mut i: usize = 0;
        while let Some(line) = lines.next() {
            let line = line.map_err(|e| OrganizeError::FailedReadingFromFileSystem(e))?;

            if i == 0 {
                if !line.starts_with("#id ") {
                    return Err(OrganizeError::ObjectParseError(ObjectParseError::NoIdentification(line)));
                }

                id = Some(line[4..].to_string());

                i += 1;
                continue;
            }

            if line.starts_with("#event ") {
                if current_event.is_some() {
                    events.insert(current_event.clone().unwrap(), line_buf.clone());
                }

                let event_name = &line[7..];
                current_event = Some(event_name.to_string());
                line_buf = "".to_string();

                i += 1;
                continue;
            }

            let mut needs_to_push_new_line = false;
            if lines.peek().is_none() {
                if current_event.is_some() {
                    events.insert(current_event.clone().unwrap(), line_buf.clone());
                }
            } else {
                needs_to_push_new_line = true;
            }

            line_buf.push_str(&line);
            if needs_to_push_new_line {
                line_buf.push('\n');
            }
            i += 1;
        }

        let id = id.expect("No Object ID");
        
        let object = Object {
            id,
            events,
        };

        objects.push(object);
    }

    Ok(objects)
}
