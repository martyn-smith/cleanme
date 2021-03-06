use super::{clean, Helpers, CONFIG_FNAME};
use std::env;
use std::fs;
use std::process::Command;
use which::which;

/*
 * use std::iter;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

let mut rng = thread_rng();
let chars: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(7)
        .collect();
*/
/*
 * Caution:
 *
 * This program has not been written with multithreading in mind,
 * and by default is not multithreaded.
 *
 * In addition, the unit tests below test mutually incompatible behaviour.
 *
 * Future work: look at mutexes, or create more subdirectories
 * (although running in /tmp/ also allows us to check proper handling of
 * permission errors).
 *
 * In the meantime, run tests with -- --test-threads 1 as options.
 *
 *
 */

/*
struct TestClean;

impl Drop for TestClean {
    //if /tmp/testfile, /tmp/testdir, /tmp/test_file_* exist, remove
    //std::io::ErrorKind::AlreadyExists
    fn drop(&mut self) {}
}
*/

/*
 * CONSIDER:
 *
 * if we create a dir for each test, that guarantees reliability wrt permission errors.
 * (and improves auditability a little)
 * However, one thing we *want* to do is test graceful handling of permission errors.
 * The best solution is probably a halfway house with one test running in directory root
 * and the others in a specialised dir.
 *
 */

#[test]
fn cleans_file() {
    let root = env::temp_dir();
    env::set_current_dir(&root);
    fs::File::create(root.join("cleanme_test"))
        .expect("test creation failed; cannot create junk files");
    fs::write(root.join(CONFIG_FNAME), "cleanme_test")
        .expect("test creation failed; cannot create config file");
    let h = Helpers::new();
    clean(&root, &h, &None).expect("cleaning failed :-(");
    fs::remove_file(root.join(CONFIG_FNAME)).expect("cannot remove config file");
    //dbg!("{:?}", fs::read_dir(&root).unwrap().collect::<Vec<Result<fs::DirEntry, _>>>());
    assert!(!root.join("testfile").exists());
}

#[test]
fn does_not_clean_file() {
    let root = env::temp_dir();
    env::set_current_dir(&root);
    fs::File::create(root.join("testfile"))
        .expect("test creation failed; cannot create junk files");
    fs::write(root.join(CONFIG_FNAME), "")
        .expect("test creation failed; cannot create config file");
    let h = Helpers::new();
    clean(&root, &h, &None).expect("cleaning failed :-(");
    fs::remove_file(root.join(CONFIG_FNAME)).expect("cannot remove config file");
    assert!(root.join("testfile").exists());
    fs::remove_file(root.join("testfile")).unwrap() //unreachable;
}

#[test]
fn cleans_git() {
    let root = env::temp_dir().join("cleanme_dir");
    env::set_current_dir(&root);
    if which::which("git").is_ok() {
        Command::new("git").args(["init", "."]).spawn().expect("can't execute git!");
        let h = Helpers::new();
        clean(&root, &h, &None).expect("cleaning failed :-(");
        fs::remove_dir_all(root).expect("cleanup failed");
    }
}

#[test]
fn cleans_glob() {
    let root = env::temp_dir();
    env::set_current_dir(&root);
    fs::File::create(root.join("testfile_1"))
        .expect("test creation failed; cannot create junk files");
    fs::File::create(root.join("testfile_2"))
        .expect("test creation failed; cannot create junk files");
    fs::write(root.join(CONFIG_FNAME), "testfile*")
        .expect("test creation failed; cannot create config file");
    let h = Helpers::new();
    clean(&root, &h, &None).expect("cleaning failed :-(");
    fs::remove_file(root.join(CONFIG_FNAME)).expect("cannot remove config file");
    assert!(!root.join("testfile_1").exists());
    assert!(!root.join("testfile_2").exists());
}

#[test]
fn cleans_dir() {
    let root = env::temp_dir();
    env::set_current_dir(&root);
    fs::create_dir(root.join("testdir")).expect("test creation failed; cannot create directory");
    fs::write(root.join(CONFIG_FNAME), "testdir/")
        .expect("test creation failed; cannot create config file");
    let h = Helpers::new();
    clean(&root, &h, &None).expect("cleaning failed :-(");
    assert!(!root.join("testdir").exists());
    fs::remove_dir_all(root.join("testdir")).expect("can't remove test directory");
}


/*
#[test]
fn traverses() {
    let root = env::temp_dir();
    env::set_current_dir(&root);
    fs::create_dir(root.join("testdir")).expect("test creation failed; cannot create directory");
    fs::File::create(root.join("testdir/testfile"))
        .expect("test creation failed; cannot create junk files");
    fs::write(root.join("testdir/").join(CONFIG_FNAME), "testfile")
        .expect("test creation failed; cannot create config file");
    clean(&root, &None);
    fs::remove_file(root.join("testdir/.clean"));
    assert!(!root.join("testdir/testfile").exists());
    fs::remove_dir(root.join("testdir"));
}
*/
