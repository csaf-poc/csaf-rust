use csaf_lib::csaf::validation::*;
use csaf_lib::csaf::loader::load_document;
use std::{
    env,
    io::{Error, ErrorKind},
};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let path = match args.get(1) {
        None => {
            return Err(Error::new(
                ErrorKind::Other,
                "Please specify a file to validate",
            ))
        }
        Some(v) => v,
    };

    let p = load_document(path)?;

    validate_document(&p);

    Ok(())
}