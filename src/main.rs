use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{exit, Command};

use anyhow::{anyhow, bail, ensure, Context, Result};
use structopt::StructOpt;

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

fn main() -> Result<()> {
    let args = Args::from_args();

    let temp_dir = tempfile::tempdir().context("failed to create a temp dir")?;
    let temp_filepath = temp_dir.path().join("tempfile");
    let mut temp_file = fs::File::create(&temp_filepath)
        .with_context(|| format!("failed to create temp file {}", &temp_filepath.display()))?;

    let mut filenames = if args.paths.is_empty() {
        fs::read_dir(env::current_dir()?)?
            .map(|entry| PathBuf::from(entry.unwrap().path().file_name().unwrap()))
            .collect()
    } else {
        args.paths.clone()
    };

    filenames.sort();
    writeln!(
        temp_file,
        "{}",
        filenames
            .iter()
            .map(|pathbuf| pathbuf.to_string_lossy().to_string())
            .collect::<Vec<String>>()
            .join("\n")
    )
    .with_context(|| format!("failed to write to file {}", temp_filepath.display()))?;

    let editor = env::var("EDITOR").context("failed to read EDITOR env variable")?;
    ensure!(editor != "", "EDITOR is set to empty string");
    let status = Command::new(&editor)
        .args(&[temp_filepath.to_string_lossy().to_string()])
        .status()
        .with_context(|| format!("failed to execute {}", editor))?;
    if !status.success() {
        exit(1);
    }

    let new_filenames: Vec<PathBuf> = fs::read_to_string(&temp_filepath)
        .with_context(|| format!("failed to read temp file {}", temp_filepath.display()))?
        .lines()
        .map(PathBuf::from)
        .collect();
    if filenames.len() != new_filenames.len() {
        bail!("incorrect number of file names");
    }

    filenames
        .iter()
        .zip(new_filenames.iter())
        .filter(|(from, to)| from != to)
        .for_each(|(from, to)| {
            if to.exists() && !args.force {
                eprintln!(
                    "{}",
                    anyhow!(
                        "failed to rename '{}' to '{}': file already exists",
                        from.display(),
                        to.display()
                    )
                );
            } else {
                match fs::rename(from, to).with_context(|| {
                    anyhow!(
                        "failed to rename '{}' to '{}'",
                        from.display(),
                        to.display()
                    )
                }) {
                    Ok(()) => {
                        if args.verbose {
                            println!("renamed '{}' -> '{}'", from.display(), to.display());
                        }
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            }
        });

    Ok(())
}
