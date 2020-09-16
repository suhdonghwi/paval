use std::fs;
use std::process::Command;

use crate::til;
use crate::env::*;

pub fn git_command(args: &[&str], dir: &str) {
    Command::new("git")
        .current_dir(dir)
        .args(args)
        .spawn()
        .expect("Failed process spawning")
        .wait()
        .expect("Failed git command");
}

fn get_til_path(til: &til::TIL) -> String {
    format!("./til/src/docs/{}/{}.md", til.category, til.title)
}

fn commit_and_push(commit_message: &str) {
    git_command(&["pull"], "./til");
    git_command(&["stage", "."], "./til");
    git_command(&["commit", "-m", commit_message], "./til");
    git_command(&["push", &*GIT_URL], "./til");
}

pub fn add_til(til: &til::TIL) -> std::io::Result<()> {
    let til_content = til.to_markdown();
    let til_path = get_til_path(&til);

    Command::new("mkdir")
        .arg(format!("./til/src/docs/{}", til.category))
        .spawn()
        .expect("Failed process spwaning")
        .wait()
        .expect("Failed mkdir");

    fs::write(til_path, til_content)?;

    let commit_message = &format!("chore(post): add '{}' post", til.title);
    commit_and_push(commit_message);

    Ok(())
}

pub fn delete_til(til: &til::TIL) -> std::io::Result<()> {
    let til_path = get_til_path(til);

    Command::new("rm")
        .arg(til_path)
        .spawn()
        .expect("Failed process spwaning")
        .wait()
        .expect("Failed rm");

    let commit_message = &format!("chore(post): delete '{}' post", til.title);
    commit_and_push(commit_message);

    Ok(())
}
