use std::{fs::File, io::Write, path::{Path, PathBuf}};
use git2::Repository;


/**
 * attempts to create a file and write to it.
 */
pub fn make_file(path: &String, content: &String) -> Result<(), std::io::Error> {
    let mut file = File::create(&path).expect("there was an error creating the file");

    let bytes = content.clone().into_bytes();
    
    return file.write_all(&bytes);
}

pub fn clone_repo(url: &String, path: &String) -> git2::Repository {
    let res = path.as_str().trim();
    std::fs::create_dir(res).expect("unable to create path");
    let repo = match Repository::clone(&url, res) {
        Ok(repo) => repo,
        Err(e) => panic!("{}", format!("Unable to clone repo {url} to {path} because {e}")),
    };//https://docs.rs/git2/latest/git2/struct.Repository.html

    return repo;
}