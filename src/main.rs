use yoyogay::{organizer::{organize_project, OrganizeError}, tokenizer::tokenize};

#[derive(Debug)]
enum Error {
    OrganizeError(OrganizeError),
}

fn main() -> Result<(), Error> {
    let _ = organize_project("./test_project").map_err(|e| Error::OrganizeError(e))?;
    let tokens = tokenize("2 + 2 = 22").unwrap();
    dbg!(tokens);

    Ok(())
}
