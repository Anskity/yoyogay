use std::fs;

use yoyogay::{organizer::{organize_project, OrganizeError}, parser::expr::parse_expr, tokenizer::{tokenize, TokenizeError}};

#[derive(Debug)]
enum Error {
    #[allow(unused)]
    OrganizeError(OrganizeError),
    #[allow(unused)]
    TokenizeError(TokenizeError),
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

fn main() -> Result<(), Error> {
    let project = organize_project("./test_project").map_err(|e| Error::OrganizeError(e))?;

    for object in project.objects {
        for (_, src) in object.events.iter() {
            let tokens = tokenize(src)?;
            println!("Tokens: {}", tokens.iter().map(|tk| tk.to_string()).reduce(|acc, e| format!("{acc} {e}")).unwrap());
        }
    }

    let tks = tokenize(&fs::read_to_string("./code.gmpp").unwrap())?;
    dbg!(parse_expr(&tks));

    Ok(())
}
