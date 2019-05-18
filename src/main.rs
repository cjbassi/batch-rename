#[macro_use]
mod errors;

use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{exit, Command};

use structopt::StructOpt;

use errors::UnwrapOrExit;

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

    let temp_dir = tempfile::tempdir().unwrap_or_exit("create temp dir");
    let temp_filepath = temp_dir.path().join("tempfile");
    let mut temp_file = fs::File::create(&temp_filepath)
        .unwrap_or_exit(&format!("create temp file {}", &temp_filepath.display()));

    if let Some(dir) = args.directory.clone() {
        env::set_current_dir(&dir).unwrap_or_exit(&format!("navigate to {}", dir.display()));
    }
    let cwd = env::current_dir().unwrap_or_exit("get current directory");

    let mut filenames: Vec<String> = fs::read_dir(&cwd)
        .unwrap_or_exit(&format!("read directory {}", cwd.display()))
        .map(|dir_entry| {
            let filepath = dir_entry.unwrap().path();
            let filename = filepath.file_name().unwrap().to_string_lossy().to_string();
            filename
        })
        .collect();

    filenames.sort();

    filenames.iter().for_each(|filename| {
        writeln!(temp_file, "{}", filename,)
            .unwrap_or_exit(&format!("write to file {}", temp_filepath.display()));
    });

    let editor = env::var("EDITOR").unwrap_or_exit("read EDITOR env variable");
    let status = Command::new(&editor)
        .args(&[temp_filepath.to_string_lossy().to_string()])
        .status()
        .unwrap_or_exit(&format!("execute {}", editor));
    if !status.success() {
        exit(1);
    }

    let file_contents = fs::read_to_string(&temp_filepath)
        .unwrap_or_exit(&format!("read temp_file {}", temp_filepath.display()));
    let new_filenames: Vec<&str> = file_contents.lines().collect();
    if filenames.len() != new_filenames.len() {
        println!("error: incorrect number of file names");
        exit(1);
    }

    filenames
        .iter()
        .zip(new_filenames.iter())
        .for_each(|(from, to)| match fs::rename(from, to) {
            Ok(()) => {
                if args.verbose {
                    println!("renamed '{}' -> '{}'", from, to);
                }
            }
            Err(e) => {
                print_error!("rename '{}' to '{}'", from, to, e);
            }
        });
}
