extern crate clap;
#[macro_use]
extern crate structopt;

use clap::Shell;

use std::env;
use std::fs;
use std::path::Path;

use structopt::StructOpt;

include!("src/cli/mod.rs");

fn main() {
    // create directory for output
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("Unable to find project root.");
    let project_root = Path::new(&manifest_dir);
    let build_profile = env::var("PROFILE").unwrap_or("debug".to_string());
    let output_dir = project_root.join("target").join(build_profile).join("completions.d");
    let project_name = env::var("CARGO_PKG_NAME").unwrap_or("nfty".to_string());

    if !output_dir.is_dir() {
        fs::create_dir(&output_dir).expect("Unable to create output directory.");
    }

    let mut app = Opt::clap();

    for variant in Shell::variants().iter() {
        let shell = match variant {
            &"fish"       => Shell::Fish,
            &"bash"       => Shell::Bash,
            &"powershell" => Shell::PowerShell,
            &"zsh"        => Shell::Zsh,
            _ => unreachable!("Unknown shell: {}", variant),
        };

        app.gen_completions(project_name.as_str(), shell, &output_dir);
    }
}
