pub mod completions;

use structopt::StructOpt;
pub use completions::{Completions};

#[derive(Debug, StructOpt, Default)]
#[structopt(raw(global_settings = "&[structopt::clap::AppSettings::DisableVersion, structopt::clap::AppSettings::DisableHelpSubcommand]"))]
pub struct Opt {
    /// Enable verbose logging.
    #[structopt(long = "verbose", short = "v")]
    pub verbose: bool,

    #[structopt(subcommand)]
    pub commands: Option<Commands>,
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(raw(
    global_settings = "&[structopt::clap::AppSettings::DeriveDisplayOrder, structopt::clap::AppSettings::DisableVersion, structopt::clap::AppSettings::DisableHelpSubcommand]"
))]
pub enum Commands {
    /// Completion scripts for various shells.
    #[structopt(
        raw(setting = "structopt::clap::AppSettings::DisableHelpSubcommand"),
        name = "completions"
    )]
    Completions(Completions),
}
