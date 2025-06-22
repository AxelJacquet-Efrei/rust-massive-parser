use anyhow::Result;
use parser_core::DocumentParser;
use std::path::PathBuf;
use txt_parser::TxtParser;

fn main() -> Result<()> {
    let path: PathBuf = std::env::args()
        .nth(1)
        .expect("Usage: parser-cli <fichier.txt>")
        .into();

    let _doc = TxtParser::parse(&path)?;
    Ok(())
}
