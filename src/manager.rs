use std::fs;
use std::process::Command;

use crate::til;

pub fn git_command(args: &[&str]) {
    &[1, 2];
    Command::new("git")
        .current_dir("./til")
        .args(args)
        .spawn()
        .expect("git command failed");
}

pub fn add_til(til: &til::TIL) -> std::io::Result<()> {
    let til_content = til.to_markdown();
    let til_path = format!("./til/src/docs/{}/{}.md", til.category, til.title);

    fs::write(til_path, til_content)?;

    let commit_message = &format!("\"chore(post): add '{}' post\"", til.title);

    git_command(&["stage", "."]);
    git_command(&["commit", "-m", commit_message]);
    git_command(&["push"]);

    Ok(())
}
