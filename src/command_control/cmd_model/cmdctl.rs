use structopt::StructOpt;
use super::Commands;

#[derive(Debug, StructOpt, Default)]
#[structopt(raw(global_settings = "&[structopt::clap::AppSettings::DisableVersion, structopt::clap::AppSettings::DisableHelpSubcommand]"))]
pub struct CmdCtl {
    /// Enable verbose logging.
    #[structopt(long = "verbose", short = "v")]
    pub verbose: bool,

    #[structopt(subcommand)]
    pub commands: Option<Commands>,
}

impl CmdCtl {

    pub fn run_command_process(self) -> CmdCtl {
        match &self.commands {
            Some(commands) => {
                commands.process_completions();
                self
            },
            None => self
        }
    }

    pub fn is_verbose(&self) -> bool {
        match self.commands.clone() {
            Some(commands) => commands.is_verbose(),
            None => self.verbose
        }
    }

}
