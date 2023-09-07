#![recursion_limit = "256"]

use clap::Parser;
use lalrpop_util::lalrpop_mod;
use miette::IntoDiagnostic;
use owo_colors::OwoColorize;

// The lalrpop module, it does generate the parser and lexer
// for the language.
lalrpop_mod! {
    #[allow(warnings)]
    /// The parsing module
    pub rinha
}

/// The abstract syntax tree for the language. The abstract
/// syntax tree is the tree that represents the program
/// in a tree form.
pub mod ast;

/// Parser LALRPOP module. It does uses a parse generator to
/// generate a parser and lexer for the language.
pub mod parser;

/// Simple program to run `rinha` language.
#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Command {
    #[clap(long, short, default_value = "false")]
    pub pretty: bool,

    /// The file we would like to run, type check, etc
    pub main: String,
}

/// Logger function for the fern logger.
///
/// It does format the log message to a specific format.
pub fn log(out: fern::FormatCallback, message: &std::fmt::Arguments, record: &log::Record) {
    let style = match record.level() {
        log::Level::Error => owo_colors::Style::new().red().bold(),
        log::Level::Warn => owo_colors::Style::new().yellow().bold(),
        log::Level::Info => owo_colors::Style::new().bright_blue().bold(),
        log::Level::Debug => owo_colors::Style::new().bright_red().bold(),
        log::Level::Trace => owo_colors::Style::new().bright_cyan().bold(),
    };
    let level = record.level().to_string().to_lowercase();
    let level = level.style(style);

    out.finish(format_args!("  {level:>7} {}", message))
}

/// The main function of the program.
pub fn program() -> miette::Result<()> {
    // Initialize the bupropion handler with miette
    bupropion::BupropionHandlerOpts::install(|| {
        // Build the bupropion handler options, for specific
        // error presenting.
        bupropion::BupropionHandlerOpts::new()
    })
    .into_diagnostic()?;

    // Initialize the logger
    fern::Dispatch::new() // Perform allocation-free log formatting
        .format(log) // Add blanket level filter -
        .level(log::LevelFilter::Debug) // - and per-module overrides
        .level_for("hyper", log::LevelFilter::Info) // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout())
        .apply()
        .into_diagnostic()?;

    // Parse the command line arguments
    let command = Command::parse();
    let file = std::fs::read_to_string(&command.main).into_diagnostic()?;
    let file = crate::parser::parse_or_report(&command.main, &file)?;

    let json = if command.pretty {
        serde_json::to_string_pretty(&file).into_diagnostic()?
    } else {
        serde_json::to_string(&file).into_diagnostic()?
    };

    println!("{json}");

    Ok(())
}
