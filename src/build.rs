extern crate clap;
extern crate structopt;

include!("cli/mod.rs");

use std::io::Read;
use clap::Shell;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let app_name = "complete";
    let mut app = Opt::clap();

    let out_dir = Path::new("./src/completions/");
    let file_path = Path::new(&out_dir).join("mod.rs");
    
    let mut gen = File::create(&file_path).unwrap();
    
    app.gen_completions(app_name, Shell::Bash,       "./src/completions/");     
    app.gen_completions(app_name, Shell::Fish,       "./src/completions/");     
    app.gen_completions(app_name, Shell::Zsh,        "./src/completions/");   
    app.gen_completions(app_name, Shell::PowerShell, "./src/completions/");   
    app.gen_completions(app_name, Shell::Elvish,     "./src/completions/");      
    
    let mut top_rs_template = File::open("./src/completions/top.rs.template").expect("");
    let mut fish_rs_template = File::open("./src/completions/fish.rs.template").expect("");
    let mut zsh_rs_template = File::open("./src/completions/zsh.rs.template").expect("");
    let mut ps1_rs_template = File::open("./src/completions/ps1.rs.template").expect("");
    let mut elvish_rs_template = File::open("./src/completions/elvish.rs.template").expect("");
    let mut bottom_rs_template = File::open("./src/completions/bottom.rs.template").expect("");
    
    let mut bash_file = File::open("./src/completions/complete.bash").expect("");
    let mut fish_file = File::open("./src/completions/complete.fish").expect("");
    let mut zsh_file = File::open("./src/completions/_complete").expect("");
    let mut powershell_file = File::open("./src/completions/_complete.ps1").expect("");
    let mut elvish_file = File::open("./src/completions/complete.elv").expect("");
    
    let mut start_temp = String::new();
    let mut bash_script = String::new();
    let mut fish_temp = String::new();
    let mut fish_script = String::new();
    let mut zsh_temp = String::new();
    let mut zsh_script = String::new();
    let mut ps1_temp = String::new();
    let mut powershell_script = String::new();
    let mut elvish_temp = String::new();
    let mut elvish_script = String::new();
    let mut bottom_temp = String::new();
    
    top_rs_template.read_to_string(&mut start_temp).expect("");
    bash_file.read_to_string(&mut bash_script).expect("");
    fish_rs_template.read_to_string(&mut fish_temp).expect("");
    fish_file.read_to_string(&mut fish_script).expect("");
    zsh_rs_template.read_to_string(&mut zsh_temp).expect("");
    zsh_file.read_to_string(&mut zsh_script).expect("");
    ps1_rs_template.read_to_string(&mut ps1_temp).expect("");
    powershell_file.read_to_string(&mut powershell_script).expect("");
    elvish_rs_template.read_to_string(&mut elvish_temp).expect("");
    elvish_file.read_to_string(&mut elvish_script).expect("");
    bottom_rs_template.read_to_string(&mut bottom_temp).expect("");

    gen.write_fmt(format_args!("{}{}{}{}{}{}{}{}{}{}{}", 
        start_temp, bash_script, 
        fish_temp, fish_script, 
        zsh_temp, zsh_script, 
        ps1_temp, powershell_script, 
        elvish_temp, elvish_script, 
        bottom_temp
    )).expect("Error writing updated completions.rs module.");
}