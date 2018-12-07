#![recursion_limit="256"]

pub mod document_tree;
pub mod parser;
pub mod target;


use structopt::StructOpt;
use clap::{_clap_count_exprs, arg_enum};
use quicli::{
    fs::read_file,
    prelude::{CliResult,Verbosity},
};

use self::parser::{
    serialize_json,
    serialize_xml,
};

arg_enum! {
    #[derive(Debug)]
    enum Format { json, xml }
}

#[derive(Debug, StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
struct Cli {
    #[structopt(
        long = "format", short = "f", default_value = "json",  // xml is pretty defunct…
        raw(possible_values = "&Format::variants()", case_insensitive = "true"),
    )]
    format: Format,
    file: String,
    #[structopt(flatten)]
    verbosity: Verbosity,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    args.verbosity.setup_env_logger("rst")?;
    
    let content = read_file(args.file)?;
    let stdout = std::io::stdout();
    match args.format {
        Format::json => serialize_json(&content, stdout)?,
        Format::xml  => serialize_xml (&content, stdout)?,
    }
    Ok(())
}
