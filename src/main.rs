#[macro_use]
extern crate slog;

extern crate structopt;

mod cli;
mod logs;
mod completions;

use crate::completions::CompletionProcess;
use structopt::StructOpt;

fn main() {
    let cli_settings = cli::Opt::from_args();

    let root_log = logs::PrintlnDrain::get_logger(cli_settings.verbose);
      
    match cli_settings.commands {
        Some(command) => {
            match command {
                cli::Commands::Completions(x) => {
                    let ran_completion = CompletionProcess::run(x);
                    if ran_completion {
                        std::process::exit(0);
                    }
                }
            }
        },
        None => {
            trace!(root_log, "teapot is warming");
            debug!(root_log, "teapot is broken");
            info!(root_log, "teapot is working");
        }
    }
}

