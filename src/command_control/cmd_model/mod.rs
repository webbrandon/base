use structopt::StructOpt;
pub mod cmdctl;
pub mod completions;

use super::completion_handler::CompletionProcess;
pub use cmdctl::*;
pub use completions::Completions;

#[derive(Debug, StructOpt, Clone)]
#[structopt(raw(
    global_settings = "&[
        structopt::clap::AppSettings::DeriveDisplayOrder,
        structopt::clap::AppSettings::DisableVersion,
        structopt::clap::AppSettings::DisableHelpSubcommand
    ]"
))]
pub enum Commands {
    /// Completion scripts for various shells.
    #[structopt(
        raw(setting = "structopt::clap::AppSettings::DisableHelpSubcommand"),
        name = "completions"
    )]
    Completions(Completions),
}

impl Commands {
    pub fn process_completions(&self) {
        match self.clone() {
            Commands::Completions(completion) => {
                CompletionProcess::run(completion.clone());
            },
        }
    }

    pub fn is_verbose(&self) -> bool {
        match self.to_owned() {
            // This is one of the few option that shouldn't have verbose output.
            Commands::Completions(_) => {
                false
            },
        }
    }
}
