pub mod completions;

use structopt::StructOpt;
pub use completions::{Completions};

#[derive(Debug, StructOpt)]
#[structopt(raw(global_settings = "&[structopt::clap::AppSettings::DisableVersion, structopt::clap::AppSettings::DisableHelpSubcommand]"))]
pub enum Opt {
    /// Completion scripts for various shells.
    #[structopt(raw(setting = "structopt::clap::AppSettings::DisableHelpSubcommand"), name = "completions")]
    Completions(Completions),
}
