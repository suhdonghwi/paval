use chrono::prelude::*;
use indoc::indoc;

#[derive(PartialEq, Debug)]
pub struct TIL {
    title: String,
    content: String,
    tags: Vec<String>,
    date: Date<Utc>,
}

pub fn parse_til(source: &String, date: Date<Utc>) -> Option<TIL> {
    let lines: Vec<&str> = source.trim().split('\n').collect();
    if lines.len() < 2 {
        return None;
    }

    let title = String::from(lines[0]);
    let mut content = String::new();

    for i in 1..lines.len() - 1 {
        if lines[i].trim().is_empty() {
            continue;
        } else {
            content.push_str(lines[i]);
            content.push('\n');
        }
    }

    if content.is_empty() {
        return None;
    }

    let last_line = lines[lines.len() - 1];
    let mut tags = Vec::new();
    match last_line.chars().nth(0) {
        Some('#') => {
            let items: Vec<&str> = last_line.split(' ').collect();
            for item in items {
                if item.trim().is_empty() {
                    continue;
                }
                tags.push(String::from(&item[1..]));
            }
        }
        _ => {
            content.push_str(last_line);
            content.push('\n');
        }
    }

    Some(TIL {
        title,
        content,
        tags,
        date,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_til() {
        let now = Utc::today();
        let input = indoc! {"
            Title

            This is a sample content.
            Lorem ipsum.
        "}
        .to_string();

        assert_eq!(
            parse_til(&input, now),
            Some(TIL {
                title: "Title".to_string(),
                content: "This is a sample content.\nLorem ipsum.\n".to_string(),
                tags: vec![],
                date: now,
            }),
        );

        let input = indoc! {"
            Title

            This is a sample content.
            Lorem ipsum.

            #tag1 #tag2
        "}
        .to_string();

        assert_eq!(
            parse_til(&input, now),
            Some(TIL {
                title: "Title".to_string(),
                content: "This is a sample content.\nLorem ipsum.\n".to_string(),
                tags: vec!["tag1".to_string(), "tag2".to_string()],
                date: now,
            }),
        );

        let input = indoc! {"
            Title

            This is a sample content.


            #tag1   #tag2
        "}
        .to_string();

        assert_eq!(
            parse_til(&input, now),
            Some(TIL {
                title: "Title".to_string(),
                content: "This is a sample content.\n".to_string(),
                tags: vec!["tag1".to_string(), "tag2".to_string()],
                date: now,
            }),
        );

        let input = indoc! {"
            테스트

            이것은 예시 컨텐츠입니다.
            동해물과 백두산이 마르고 닳도록.

            #태그1 #태그2
        "}
        .to_string();

        assert_eq!(
            parse_til(&input, now),
            Some(TIL {
                title: "테스트".to_string(),
                content: "이것은 예시 컨텐츠입니다.\n동해물과 백두산이 마르고 닳도록.\n"
                    .to_string(),
                tags: vec!["태그1".to_string(), "태그2".to_string()],
                date: now,
            }),
        );
    }

    #[test]
    fn test_parse_bad_til() {
        let now = Utc::today();
        let input = indoc! {"
            Title
        "}
        .to_string();

        assert_eq!(parse_til(&input, now), None);

        let input = indoc! {"
            Title

            #tag1 #tag2

        "}
        .to_string();

        assert_eq!(parse_til(&input, now), None);
    }
}
