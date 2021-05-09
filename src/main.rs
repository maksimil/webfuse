use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use anyhow::{anyhow, ensure, Context};
use clap::clap_app;
use parser::parse_html;

use crate::{errors::Error, load::LoadedHtml};

mod errors;
mod load;
pub mod parser;
mod test;

fn main() -> anyhow::Result<()> {
    let matches = clap_app!(webfuse =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: env!("CARGO_PKG_AUTHORS"))
        (about: "Fuses html file with all its dependencies one html file")
        (@arg FILE: +required "Html file with dependencies")
        (@arg TO: -t --to +takes_value "Name of generated file (is fused_index.html) by default")
    )
    .get_matches();

    let file: PathBuf = matches
        .value_of("FILE")
        .context(anyhow!("FILE argument was not provided"))?
        .into();

    let to = match matches.value_of("TO") {
        Some(to) => to,
        None => "fused_index.html",
    }
    .to_string();

    ensure!(file.is_file(), Error::NotFile(file.clone()));

    let root: PathBuf = file
        .parent()
        .context(anyhow!("FILE parent was not found"))?
        .into();

    // get and load html
    let html = {
        let contents =
            fs::read_to_string(&file).with_context(|| Error::FileNotFound(file.clone()))?;
        let html = parse_html(contents);
        LoadedHtml::load(root, html)?
    };

    // writing chunks to the output file
    let mut file = File::create(to).context(anyhow!("Failed to create output file"))?;

    for chunk in html.chunks() {
        file.write_all(chunk.as_bytes())
            .context(anyhow!("Failed to write file output"))?;
    }

    Ok(())
}
