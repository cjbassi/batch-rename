use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{exit, Command};

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Args {
    pub directory: Option<PathBuf>,

    #[structopt(short = "f", long = "force")]
    pub force: bool,

    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,
}

fn main() {
    let args = Args::from_args();

    let temp_dir = tempfile::tempdir().unwrap();
    let temp_filepath = temp_dir.path().join("tempfile");
    let mut temp_file = fs::File::create(&temp_filepath).unwrap();
    if let Some(dir) = args.directory {
        env::set_current_dir(dir).unwrap();
    }
    let cwd = env::current_dir().unwrap();
    let mut filenames: Vec<String> = fs::read_dir(cwd)
        .unwrap()
        .map(|dir_entry| {
            let filepath = dir_entry.unwrap().path();
            let filename = filepath.file_name().unwrap().to_string_lossy().to_string();
            filename
        })
        .collect();
    filenames.sort();
    let longest_filename_len = match filenames.iter().map(|filename| filename.len()).max() {
        Some(x) => x,
        None => return,
    };
    filenames.iter().for_each(|filename| {
        writeln!(
            temp_file,
            "{:width$} {}",
            filename,
            filename,
            width = longest_filename_len + 1
        )
        .unwrap();
    });

    let editor = env::var("EDITOR").unwrap();
    let status = Command::new(editor)
        .args(&[temp_filepath.to_string_lossy().to_string()])
        .status()
        .unwrap();
    if !status.success() {
        exit(1);
    }

    for line in fs::read_to_string(temp_filepath).unwrap().lines() {
        let filenames = line.split_whitespace().collect::<Vec<&str>>();
        let from = filenames[0];
        let to = filenames[1];
        fs::rename(from, to).unwrap();
        if args.verbose {
            println!("renamed '{}' -> '{}'", from, to);
        }
    }
}
