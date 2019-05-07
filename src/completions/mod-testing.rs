use std::io::prelude::*;
use std::fs::File;

fn print_file(file: &'static str) {
    let mut file = File::open(file).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    println!("{}", contents);
}

pub struct S6Completions {
    
}

impl S6Completions {
    pub fn bash() {
        print_file("./src/completions/complete.bash");
    }
    
    pub fn fish() {
        print_file("./src/completions/complete.fish");
    }
    
    pub fn zsh() {
        print_file("./src/completions/_complete");
    }
    
    pub fn powershell() {
        print_file("./src/completions/_complete.ps1");
    }
    
    pub fn elvish() {
        print_file("./src/completions/complete.elv");
    }
}