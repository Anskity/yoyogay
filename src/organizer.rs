use std::path::PathBuf;

pub mod object;
pub mod info;

#[derive(Debug)]
pub struct YoyogayProject {
    pub objects: Vec<object::Object>,
}

#[derive(Debug)]
pub enum OrganizeError {
    FailedReadingFromFileSystem(std::io::Error),
    UnexpectedFile(PathBuf),
    ObjectParseError(object::ObjectParseError),
}

impl YoyogayProject {
    pub fn create_from_directory(path: impl Into<PathBuf>) -> Result<YoyogayProject, OrganizeError> {
        let path = path.into();
        let mut objects: Option<Vec<object::Object>> = None;

        let dirs = path
            .read_dir()
            .map_err(|e| OrganizeError::FailedReadingFromFileSystem(e))?;
        for dir in dirs {
            let dir = dir.map_err(|e| OrganizeError::FailedReadingFromFileSystem(e))?;
            let path = dir.path();

            if !path.is_dir() {
                return Err(OrganizeError::UnexpectedFile(path));
            }

            let name = dir.file_name();

            match name.as_os_str().to_str().unwrap_or("") {
                "objects" => {
                    objects = Some(object::organize_objects(path)?);
                }
                _ => panic!("Unexpected directory: {:?}", path),
            }
        }
        let objects = objects.unwrap_or(Vec::new());

        Ok(YoyogayProject { objects })
    }
}
