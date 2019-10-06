#[macro_use]
extern crate slog;

extern crate structopt;

mod cli;
mod logs;
mod completions;

use logs::PrintlnDrain;
use completions::CompletionProcess;
use structopt::StructOpt;

fn main() {
    // This is the collection of settings sent from the request.
    let cli_options = cli::Opt::from_args();

    // This should be passed to any underlying modules and follow verbose logic rules for CLI.
    let log_config = PrintlnDrain::get_logger(cli_options.is_verbose());

    // If user request completion option else extract cli request.
    match cli_options.has_completion() {
        Some(completion) => CompletionProcess::run(log_config, completion),
        None => {
                    debug!(log_config, "You should do something with this CLI.");

                    // How you run the rest of the CLI is up to you. I have grown fond of a
                    // Model/Handler flow where request become Types with various Handlers for
                    // various use cases.
                },
    }
}
