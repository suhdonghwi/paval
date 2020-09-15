use std::fs;
use std::process::Command;

use crate::til;

pub fn add_til(til: &til::TIL) -> std::io::Result<()> {
    let til_content = til.to_markdown();
    let til_path = format!("./til/src/docs/{}/{}.md", til.category, til.title);

    fs::write(til_path, til_content)?;

    let commit_message = format!("chore(post): add '{}' post", til.title);

    let mut push_command = Command::new(format!(
        "git stage . && git commit -m \"{}\" && git push",
        commit_message
    ));
    push_command.current_dir("./til");
    push_command.spawn().and_then(|_| Ok(()))
}
