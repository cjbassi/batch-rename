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
    pub paths: Vec<PathBuf>,

    /// Overwrite existing files/directories
    #[structopt(short = "f", long = "force")]
    pub force: bool,

    /// Explain what is being done
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,
}

fn main() {
    let args = Args::from_args();

    let temp_dir = tempfile::tempdir().unwrap_or_exit("create temp dir");
    let temp_filepath = temp_dir.path().join("tempfile");
    let mut temp_file = fs::File::create(&temp_filepath)
        .unwrap_or_exit(&format!("create temp file {}", &temp_filepath.display()));

    let mut filenames = if args.paths.is_empty() {
        fs::read_dir(env::current_dir().unwrap())
            .unwrap()
            .map(|entry| PathBuf::from(entry.unwrap().path().file_name().unwrap()))
            .collect()
    } else {
        args.paths.clone()
    };

    filenames.sort();
    filenames.iter().for_each(|filename| {
        writeln!(temp_file, "{}", filename.display())
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

    let new_filenames: Vec<PathBuf> = fs::read_to_string(&temp_filepath)
        .unwrap_or_exit(&format!("read temp_file {}", temp_filepath.display()))
        .lines()
        .map(PathBuf::from)
        .collect();
    if filenames.len() != new_filenames.len() {
        eprintln!("error: incorrect number of file names");
        exit(1);
    }

    filenames
        .iter()
        .zip(new_filenames.iter())
        .filter(|(from, to)| from != to)
        .for_each(|(from, to)| {
            if to.exists() && !args.force {
                eprintln!(
                    "error: failed to rename '{}' to '{}': file already exists",
                    from.display(),
                    to.display()
                );
            } else {
                match fs::rename(from, to) {
                    Ok(()) => {
                        if args.verbose {
                            println!("renamed '{}' -> '{}'", from.display(), to.display());
                        }
                    }
                    Err(e) => {
                        print_error!("rename '{}' to '{}'", from.display(), to.display(), e);
                    }
                }
            }
        });
}
