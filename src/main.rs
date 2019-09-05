extern crate structopt;

use crate::cli::Opt::Completions;
use crate::completions::CompletionProcess;
use structopt::StructOpt;

mod cli;
mod completions;

fn main() {
    let cli_settings = cli::Opt::from_args();
    
    match cli_settings {
        Completions(x) => {
            let ran_completion = CompletionProcess::run(x);
            if ran_completion {
                std::process::exit(0);
            }
        },
    }
}