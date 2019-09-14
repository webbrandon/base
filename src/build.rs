extern crate clap;
extern crate structopt;

include!("cli/mod.rs");
include!("completions/mod_template.rs");
use clap::Shell;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
    let app_name: Option<&'static str> = option_env!("CARGO_PKG_NAME");
    
    create_completion_scripts(app_name.unwrap_or("app"));
    create_completion_mod(app_name.unwrap_or("app"));
    clean(app_name.unwrap_or("app"));
}

fn completion_scripts(name: &'static str) -> Vec<PathBuf> {
    let src_dir = String::from("./src/completions/");
    let mut bash = src_dir.clone();
    bash.push_str(name);
    bash.push_str(".bash");
    let mut fish = src_dir.clone();
    fish.push_str(name);
    fish.push_str(".fish");
    let mut zsh = src_dir.clone();
    zsh.push_str("_");
    zsh.push_str(name);
    let mut ps = src_dir.clone();
    ps.push_str("_");
    ps.push_str(name);
    ps.push_str(".ps1");
    let mut elvish = src_dir.clone();
    elvish.push_str(name);
    elvish.push_str(".elv");
    
    vec![
        Path::new(&bash.clone()).to_path_buf(),
        Path::new(&fish.clone()).to_path_buf(),
        Path::new(&zsh.clone()).to_path_buf(),
        Path::new(&ps.clone()).to_path_buf(),
        Path::new(&elvish.clone()).to_path_buf(),
    ]
}

fn create_completion_mod(name: &'static str) {
    let file_path = Path::new("./src/completions/mod.rs");    
    let completion_scripts = completion_scripts(name);
    let templates = vec![
        get_top_template(),
        get_fish_template(),
        get_zsh_template(),
        get_ps1_template(),
        get_elvish_template(),
    ];
    
    File::create(&file_path).unwrap();
    for i in 0..5 {
        merge_files_to_completion(
            file_path.clone().to_path_buf(),
            completion_scripts[i].to_path_buf(),
            templates[i].clone(),
        );
    }
    file_to_completion(file_path.to_path_buf(), get_bottom_template());
}

fn create_completion_scripts(name: &'static str) {
    let out_dir = Path::new("./src/completions/");
    let mut app = Opt::clap();
    
    app.gen_completions(name, Shell::Bash, out_dir);
    app.gen_completions(name, Shell::Fish, out_dir);
    app.gen_completions(name, Shell::Zsh, out_dir);
    app.gen_completions(name, Shell::PowerShell, out_dir);
    app.gen_completions(name, Shell::Elvish, out_dir);
}

fn clean(name: &'static str) {
    let files = completion_scripts(name);
    for i in 0..5 {
        match std::fs::remove_file(files[i].to_path_buf()) {
            Ok(x) => println!("{:#?}", x),
            Err(e) => eprint!("Error removing file: {}", e),
        }
    }
}

fn merge_files_to_completion(out: PathBuf, script: PathBuf, template: String) {
    let mut tmp_script = String::new();
    let mut file_script = File::open(script).expect("");

    file_script.read_to_string(&mut tmp_script).expect("");

    let mut outfile = OpenOptions::new()
        .write(true)
        .append(true)
        .open(out)
        .unwrap();
    outfile
        .write_fmt(format_args!("{}{}", template, tmp_script))
        .expect("Error writing updated completions.rs module.");
}

fn file_to_completion(out: PathBuf, template: String) {
    let mut outfile = OpenOptions::new()
        .write(true)
        .append(true)
        .open(out)
        .unwrap();
    outfile
        .write_fmt(format_args!("{}", template))
        .expect("Error writing updated completions.rs module.");
}
