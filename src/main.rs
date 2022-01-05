//! A simple cleaning utility.
//!
//! `clean` takes an initial ".clean" file as input, with glob-like syntax.
//! It then traverses the file tree from its current location, deleting matching files.
//! For each directory is traverses, if it finds a local ".clean" file it will use
//! that as input, for that directory and all child directories.

use anyhow::{Context, Result};
use dirs;
use glob::glob;
use std::{env, fs, io::ErrorKind, path::Path, path::PathBuf, process::Command};

//use structopt::StructOpt;
//use dirs;
use which::which;

const CONFIG_FNAME: &str = "./cleanme";

#[cfg(test)]
mod test;

pub struct Helpers {
    cargo: bool,
    git: bool,
    npm: bool,
    poetry: bool,
    pyclean: bool
}

impl Helpers {

    fn new() -> Self {
            Self {
            cargo: which("cargo").is_ok(),
            git: which("git").is_ok(),
            npm: which("npm").is_ok(),
            poetry: which("poetry").is_ok(),
            pyclean: which("python3").is_ok()
        }
    }

    fn run(&self, path: &Path) {
        if self.cargo && Path::new("Cargo.toml").exists() {
            //TODO: rm Cargo.lock as well
            Command::new("cargo").args(&["clean"]).spawn().context("Cargo.toml but cargo failed");
        }
        if self.git && Path::new(".git").exists() {
            Command::new("git").args(&["gc", "--aggressive", "--prune"]).spawn().context(".git file but git process failed");
        }
        if self.npm && Path::new("package.json").exists() {
            Command::new("npm cache clean").args(&["force"]).spawn().context("package.json but npm failed");
        }
        if self.poetry && Path::new("pyproject.toml").exists() {
            //TODO: poetry env remove here
            Command::new("poetry cache clear").args(&["all"]).spawn().context("pyproject file but poetry failed");
        }
    }
}

fn get_target(pwd: &Path) -> Option<Vec<String>> {
    Some(
        fs::read_to_string(pwd.join(Path::new(CONFIG_FNAME)))
            .ok()?
            .lines()
            .filter(|&line| !(line.is_empty() || line.starts_with('#')))
            .map(String::from)
            .collect(),
    )
}

fn clean(wd: &Path, h: &Helpers, clean_tgt: &Option<Vec<String>>) -> Result<()> {
    /*
     * Error handling: if we e.g. cannot access a directory, log to stderr and carry on.
     */
    //dbg!(&wd);
    let read =
        fs::read_dir(wd).context(format!("cannot read {} - check permissions", wd.display()))?;
    /*
     * if there is a ".clean" file in the current dir, use that;
     * otherwise, use the current list (which may be empty).
     */
    let local = get_target(wd);
    let clean_tgt = match local {
        Some(_) => &local,
        None => clean_tgt,
    };

    h.run(wd);

    //None here is unlikely but not impossible.
    if let Some(clean_tgt) = clean_tgt {
        /*
         * Each glob_line creates its own iterator.
         * Joining those into a single list is possible, but janky,
         * so we'll handle each glob result separately.
         */
        for glob_line in clean_tgt {
            //ignore pattern error (TODO: switch to logging)
            let remove = glob(wd.join(&glob_line).to_str().unwrap()).ok();
            if let Some(remove) = remove {
                for tgt in remove {
                    /*
                     * tgt is a GlobResult, so IoErrors are handled
                     * (permission errors are not). Again, we should default to
                     * logging and try the next.
                     */
                    let tgt = tgt?;
                    //.context(format!("unknown IO error at {}", tgt))?;
                    if tgt.is_dir() {
                        fs::remove_dir_all(&tgt).context(format!(
                            "failed to remove {:?}; maybe directory is write-protected",
                            &tgt
                        ))?;
                    } else {
                        fs::remove_file(&tgt).context(format!(
                            "failed to remove {:?}; maybe file is write-protected",
                            &tgt
                        ))?;
                    }
                }
            }
        }
    }

    /*
     * And recurse.
     */
    for d in read.into_iter().flatten() {
        if d.path().is_dir() {
            match clean(&d.path(), h, clean_tgt) {
                Ok(()) => {},
                Err(e) => {eprintln!("{}", e);}
            };
        }
    }

    //} else {
    //    eprintln!("cannot read {}, skipping..", wd.display());
    //}
    Ok(())
}

fn main() -> Result<()> {
    /*
     * order of priorities: search stdin (io::read_to_string(&mut io::stdin())?), then $HOME, then $HOME/.config
     */
    let clean_tgt = match dirs::home_dir() {
        Some(home) => get_target(Path::new(&home)),
        _ => None,
    };
    let aux = Helpers::new();
    let pwd = Path::new("./");
    clean(pwd, &aux, &clean_tgt)
}
