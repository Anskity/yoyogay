use yoyogay::{gamemaker::GameMakerProject, organizer::{OrganizeError, YoyogayProject}, parser::{parse, ParseError}, tokenizer::{tokenize, TokenizeError}};

#[derive(Debug)]
enum Error {
    #[allow(unused)]
    OrganizeError(OrganizeError),
    #[allow(unused)]
    TokenizeError(TokenizeError),
    #[allow(unused)]
    ParseError(ParseError),
}

impl From<TokenizeError> for Error {
    fn from(value: TokenizeError) -> Self {
        Error::TokenizeError(value)
    }
}

impl From<OrganizeError> for Error {
    fn from(value: OrganizeError) -> Self {
        Error::OrganizeError(value)
    }
}

impl From<ParseError> for Error {
    fn from(value: ParseError) -> Self {
        Error::ParseError(value)
    }
}

fn main() -> Result<(), Error> {
    let yoyogay_project = YoyogayProject::create_from_directory("./test_project").map_err(|e| Error::OrganizeError(e))?;
    let gamemaker_project = GameMakerProject::new_from_yoyogay_project(&yoyogay_project);
    gamemaker_project.write_in_fs("./output_project").unwrap();
    Ok(())
}
