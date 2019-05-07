extern crate structopt;

use structopt::StructOpt;

mod cli;
mod completions;

fn main() {
    let cli_settings = cli::Opt::from_args();
    
    println!("{:#?}", cli_settings)
}