use yoyogay::{organizer::{organize_project, OrganizeError}, parser::{parse, ParseError}, tokenizer::{tokenize, TokenizeError}};

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
    let project = organize_project("./test_project").map_err(|e| Error::OrganizeError(e))?;
    for object in &project.objects {
        let events_to_parse = ["create", "step"];

        for event in events_to_parse {
            println!("{event}");
            let tks = tokenize(object.events.get(event).unwrap())?;
            let node = parse(&tks).unwrap();

            println!("{}", node.to_string());
        }
    }

    Ok(())
}
