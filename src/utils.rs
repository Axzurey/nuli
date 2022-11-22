use std::{fs::File, io::Write};
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
    let repo = match Repository::clone(&url, &path) {
        Ok(repo) => repo,
        Err(e) => panic!("{}", format!("Unable to clone repo {url} to {path} because {e}")),
    };
    return repo;
}