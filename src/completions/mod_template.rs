fn get_top_template() -> String {
    String::from(
        r#"pub mod process;

pub use process::CompletionProcess;

pub struct Completions {
    
}

impl Completions {
    pub fn bash() {
        println!("{}",r#"
"#,
    )
}

fn get_fish_template() -> String {
    String::from(
        "
\"#);
    }

    pub fn fish() {
        println!(\"{}\",r#\"
",
    )
}

fn get_zsh_template() -> String {
    String::from(
        "
\"#);
    }

    pub fn zsh() {
        println!(\"{}\",r#\"
",
    )
}

fn get_ps1_template() -> String {
    String::from(
        "
\"#);
    }

    pub fn powershell() {
        println!(\"{}\",r#\"
",
    )
}

fn get_elvish_template() -> String {
    String::from(
        "
\"#);
    }

    pub fn elvish() {
        println!(\"{}\",r#\"
        ",
    )
}

fn get_bottom_template() -> String {
    String::from(
        "
\"#);
    }
}
",
    )
}