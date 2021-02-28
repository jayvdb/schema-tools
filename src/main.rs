#[macro_use]
extern crate lazy_static;

use clap::Clap;

pub mod codegen;
pub mod commands;
pub mod discovery;
pub mod error;
pub mod process;
pub mod resolver;
pub mod schema;
pub mod scope;
pub mod tools;
pub mod validate;

#[derive(Clap)]
#[clap(version = "0.0.1", author = "Kacper S. <kacper@stasik.eu>")]

struct Opts {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Clap)]
enum Command {
    #[clap(
        version = "0.0.1",
        about = "Schema pre-processing",
        author = "Kacper S. <kacper@stasik.eu>"
    )]
    Process(commands::process::Opts),

    #[clap(
        version = "0.0.1",
        about = "Schema validation",
        author = "Kacper S. <kacper@stasik.eu>"
    )]
    Validate(commands::validate::Opts),

    #[clap(
        version = "0.0.1",
        about = "Schema to code transformations",
        author = "Kacper S. <kacper@stasik.eu>"
    )]
    Codegen(commands::codegen::Opts),

    #[clap(
        version = "0.0.1",
        about = "Chain different operations in one process",
        author = "Kacper S. <kacper@stasik.eu>"
    )]
    Chain(commands::chain::Opts),
}

fn main() {
    let opts: Opts = Opts::parse();

    let result = match opts.command {
        Command::Process(opts) => commands::process::execute(opts),
        Command::Codegen(opts) => commands::codegen::execute(opts),
        Command::Validate(opts) => commands::validate::execute(opts),
        Command::Chain(opts) => commands::chain::execute(opts),
    };

    std::process::exit(match result {
        Ok(_) => 0,
        Err(e) => {
            println!("\x1b[0;31mError occured:\x1b[0m {}", e);
            1
        }
    })
}
