use crate::cli::Completions;
use crate::logs;

pub struct CompletionProcess {}

impl CompletionProcess {
    pub fn run(log_config: logs::Logger, cli_settings: Completions) {
        debug!(log_config, "Checking is request is for shell script.");

        match cli_settings {
            Completions::Bash(x) => {
                debug!(log_config, "Request is for Bash script: {:?}", x);
                super::Completions::bash();
            }
            Completions::Fish(x) => {
                debug!(log_config, "Request is for Fish script: {:?}", x);
                super::Completions::fish();
            }
            Completions::Zsh(x) => {
                debug!(log_config, "Request is for Zsh script: {:?}", x);
                super::Completions::zsh();
            }
            Completions::PowerShell(x) => {
                debug!(log_config, "Request is for PowerShell script: {:?}", x);
                super::Completions::powershell();
            }
            Completions::Elvish(x) => {
                debug!(log_config, "Request is for Elvish script: {:?}", x);
                super::Completions::elvish();
            }
        }

    }
}
