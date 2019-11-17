extern crate slog;

extern crate structopt;

mod command_control;
mod toolbelt;

use toolbelt::logs::RootLog;
use structopt::StructOpt;

fn main() {
    // This is the collection of settings sent from the request.
    let cli_options = command_control::CmdCtl::from_args()
        .run_command_process();

    // This should be passed to any underlying modules and follow verbose logic rules for CLI.
    let log_config = RootLog::get_logger(
        cli_options.is_verbose()
    );

}
