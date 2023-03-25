use std::{process::Command, str};

pub fn get_staged_changes() {
    let git_staged_cmd = Command::new("git")
        .arg("diff")
        .arg("--staged")
        .output()
        .expect("Couldn't find diff.")
        .stdout;

    let git_staged_parsed = str::from_utf8(&git_staged_cmd).unwrap();
    println!("{}", git_staged_parsed);
}