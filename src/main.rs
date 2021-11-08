use anyhow::Result;
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
     * if there is a ".clean" file in the current dir, use that;
     * otherwise, use the current list (which may be empty).
     */
    
    if let Ok(read) = fs::read_dir(wd) {

        let local = get_target(wd);

        let clean_tgt = match local {
            Some(_) => &local,
            None => clean_tgt,
        };

        if let Some(clean_tgt) = clean_tgt {
            for glob_pat in clean_tgt {
                let remove = glob(wd.join(&glob_pat).to_str().unwrap()).ok();

                if let Some(remove) = remove {
                    for tgt in remove {
                        let tgt = tgt?;
                        if tgt.is_dir() {
                            fs::remove_dir_all(tgt)?;
                        } else {
                            fs::remove_file(tgt)?;
                        }
                    }
                }
            }
        }

        for entry in read.into_iter() {
            if let Ok(d) = entry {
                if d.path().is_dir() {
                    clean(&d.path(), clean_tgt)?; 
                }
            }
        }

    } else {
        eprintln!("cannot read {}, skipping..", wd.display()); 
    }
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
