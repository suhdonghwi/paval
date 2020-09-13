use chrono::prelude::*;

struct TIL {
    title: String,
    content: String,
    tags: Vec<String>,
    date: Date<Utc>,
}

fn parseTIL(source: String, date: Date<Utc>) -> Option<TIL> {
    let lines: Vec<&str> = source.split('\n').collect();
    if lines.len() < 2 { return None }

    let title = String::from(lines[0]);
    let mut content = String::new();

    for i in 1..lines.len()-1 {
        content.push('\n');
        content.push_str(lines[i]);
    }

    let last_line = lines[lines.len()-1];
    let mut tags = Vec::new();
    match last_line.chars().nth(0) {
        Some('#') => {
            let items: Vec<&str> = last_line.split(' ').collect();
            for item in items {
                tags.push(item[1..].into());
            }
        }
        _ => {
            content.push('\n');
            content.push_str(last_line);
        }
    }

    let result = TIL {
        title,
        content,
        tags,
        date
    };

    Some(result)
}
