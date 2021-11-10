//! A simple cleaning utility.
//!
//! `clean` takes an initial ".clean" file as input, with glob-like syntax.
//! It then traverses the file tree from its current location, deleting matching files.
//! For each directory is traverses, if it finds a local ".clean" file it will use
//! that as input, for that directory and all child directories.

/*
 * TODO:
 *
 * add automatic invocation of `cargo clean` and `pyclean` 
 * when such processes exist
 */

use anyhow::{Context,Result};
use glob::glob;
use std::{env, fs, io::ErrorKind, path::Path, path::PathBuf};

#[cfg(test)]
mod test;

fn get_target(pwd: &Path) -> Option<Vec<String>> {
    Some(
        fs::read_to_string(pwd.join(Path::new(".clean")))
            .ok()?
            .lines()
            .filter(|&line| !(line.is_empty() || line.starts_with('#')))
            .map(|line| String::from(line))
            .collect(),
    )
}

fn clean(wd: &Path, clean_tgt: &Option<Vec<String>>) -> Result<()> {

    /*
     * Error handling: if we e.g. cannot access a directory, log to stderr and carry on.
     */
    let read = fs::read_dir(wd)
        .context(format!("cannot read {} - check permissions", wd.display()));
    /*
     * if there is a ".clean" file in the current dir, use that;
     * otherwise, use the current list (which may be empty).
     */
    let local = get_target(wd);
    let clean_tgt = match local {
        Some(_) => &local,
        None => clean_tgt,
    };

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
                        fs::remove_dir_all(&tgt)
                            .context(format!("failed to remove {:?}; maybe directory is write-protected", &tgt))?;
                    } else {
                        fs::remove_file(&tgt)
                            .context(format!("failed to remove {:?}; maybe file is write-protected", &tgt))?;
                    }
                }
            }
        }
    }

    /*
     * And recurse.
     */
    for entry in read.unwrap().into_iter() {
        if let Ok(d) = entry {
            if d.path().is_dir() {
                //TODO: do we want to return error here? Or continue the "fire and forget"
                //philosophy?
                clean(&d.path(), clean_tgt)?; 
            }
        }
    }

    //} else {
    //    eprintln!("cannot read {}, skipping..", wd.display()); 
    //}
    Ok(())
}

fn main() -> Result<()> {
    let clean_tgt = match env::var("HOME") {
        Ok(home) => get_target(&Path::new(&home)),
        _ => None,
    };
    let pwd = Path::new("./");
    clean(pwd, &clean_tgt)
}
