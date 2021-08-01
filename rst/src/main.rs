use clap::arg_enum;
use clap_verbosity_flag::Verbosity;
use quicli::fs::read_file;
use structopt::StructOpt;

use rst_parser::parse;
use rst_renderer::{render_html, render_json, render_xml};

arg_enum! {
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    enum Format { json, xml, html }
}

#[derive(Debug, StructOpt)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
struct Cli {
    #[structopt(
		long = "format", short = "f", default_value = "html",  // xml is pretty defunct…
		possible_values = &Format::variants(), case_insensitive = true,
	)]
    format: Format,
    file: String,
    #[structopt(flatten)]
    verbosity: Verbosity,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    // args.verbosity.setup_env_logger("rst")?;
    env_logger::builder()
        .filter(None, log::LevelFilter::Warn)
        .filter(
            Some("rst"),
            args.verbosity
                .log_level()
                .unwrap_or(log::Level::Warn)
                .to_level_filter(),
        )
        .init();

    // TODO: somehow make it work without replacing tabs
    let content = read_file(args.file)?.replace('\t', " ".repeat(8).as_ref());
    let document = parse(&content)?;
    let stdout = std::io::stdout();
    match args.format {
        Format::json => render_json(&document, stdout)?,
        Format::xml => render_xml(&document, stdout)?,
        Format::html => render_html(&document, stdout, true)?,
    }
    Ok(())
}
