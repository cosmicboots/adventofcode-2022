use std::{fs::{File, self}, io::Write, path::PathBuf};

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(short, long, action, value_parser)]
    /// Generate a file for another day's problems
    generate: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let bins_folder = PathBuf::from("src/bin/");

    if let Some(fname) = cli.generate {
        let path = bins_folder.join(fname);
        fs::create_dir_all(&path).unwrap();
        let fpath = path.join("main.rs");
        if !fpath.exists() {
            let mut f = File::create(&fpath).expect("Failed to create new file");
            f.write_all(
                format!(
                    "fn main() {{
    println!(\"Hello from {}!\");
}}
",
                    path.file_stem().unwrap().to_str().unwrap().to_owned()
                )
                .as_bytes(),
            )
            .expect("Failed to write to new file");
            println!("Generated new file at {:?}", fpath);
        } else {
            println!("File already exists!");
        }
    } else {
        println!(
            "
=================================
||     Advent of Code 2022     ||
|| Solutions from Thom Dickson ||
=================================
Each day's work can be compliled and run with cargo using the --bin flag.
For example, to run day one's code: cargo run --bin day01"
        );
    }
}
