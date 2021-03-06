use chrono::prelude::*;

#[derive(PartialEq, Debug)]
pub struct TIL {
    pub title: String,
    pub content: String,
    pub category: String,
    pub date: Date<Utc>,
}

impl TIL {
    pub fn to_markdown(&self) -> String {
        format!("# {}\n\n{}\n", self.title, self.content)
    }
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
            content.push('\n');
        } else {
            content.push_str(lines[i]);
            content.push('\n');
        }
    }

    content = content.trim().to_string();

    if content.is_empty() {
        return None;
    }

    let last_line = lines[lines.len() - 1];
    match last_line.chars().nth(0) {
        Some('#') => {
            let category = String::from(&last_line[1..]);

            Some(TIL {
                title,
                content,
                category,
                date,
            })
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_til() {
        let now = Utc::today();

        let input = indoc! {"
            Title

            This is a sample content.
            Lorem ipsum.

            #category
        "}
        .to_string();

        assert_eq!(
            parse_til(&input, now),
            Some(TIL {
                title: "Title".to_string(),
                content: "This is a sample content.\nLorem ipsum.".to_string(),
                category: "category".to_string(),
                date: now,
            }),
        );

        let input = indoc! {"
            Title

            This is a sample content.


            #tag1  
        "}
        .to_string();

        assert_eq!(
            parse_til(&input, now),
            Some(TIL {
                title: "Title".to_string(),
                content: "This is a sample content.".to_string(),
                category: "tag1".to_string(),
                date: now,
            }),
        );

        let input = indoc! {"
            Title

            This is a sample content.

            This is new line content.


            #tag1  
        "}
        .to_string();

        assert_eq!(
            parse_til(&input, now),
            Some(TIL {
                title: "Title".to_string(),
                content: "This is a sample content.\n\nThis is new line content.".to_string(),
                category: "tag1".to_string(),
                date: now,
            }),
        );

        let input = indoc! {"
            테스트

            이것은 예시 컨텐츠입니다.
            동해물과 백두산이 마르고 닳도록.

            #태그
        "}
        .to_string();

        assert_eq!(
            parse_til(&input, now),
            Some(TIL {
                title: "테스트".to_string(),
                content: "이것은 예시 컨텐츠입니다.\n동해물과 백두산이 마르고 닳도록.".to_string(),
                category: "태그".to_string(),
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

            This is a content.
        "}
        .to_string();

        assert_eq!(parse_til(&input, now), None);

        let input = indoc! {"
            Title

            #tag

        "}
        .to_string();

        assert_eq!(parse_til(&input, now), None);
    }
}
