use std::fs;
use std::process::Command;

use crate::til;

pub fn git_command(args: &[&str]) {
    Command::new("git")
        .current_dir("./til")
        .args(args)
        .spawn()
        .expect("Failed process spawning")
        .wait()
        .expect("Failed git command");
}

pub fn add_til(til: &til::TIL, git_url: &str) -> std::io::Result<()> {
    let til_content = til.to_markdown();
    let til_path = format!("./til/src/docs/{}/{}.md", til.category, til.title);

    Command::new("mkdir")
        .arg(format!("./til/src/docs/{}", til.category))
        .spawn()
        .expect("Failed process spwaning")
        .wait()
        .expect("Failed mkdir");

    fs::write(til_path, til_content)?;

    let commit_message = &format!("chore(post): add '{}' post", til.title);

    git_command(&["stage", "."]);
    git_command(&["commit", "-m", commit_message]);
    git_command(&["push", git_url]);

    Ok(())
}
