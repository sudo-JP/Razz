use std::fs;
use owo_colors::OwoColorize;
use similar::{ChangeTag, TextDiff};

pub fn load_fixture(path: &str) -> (String, String) {
    let input = fs::read_to_string(format!("{}/input.rz", path))
        .unwrap();
    let expected = std::fs::read_to_string(format!("{}/expected.txt", path))
        .unwrap_or_default()
        .lines()
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n");
    (input, expected)
}

pub fn colored_assert(actual: &str, expected: &str) {
    if actual != expected {
        let diff = TextDiff::from_lines(actual, expected);

        println!("{}", "===== DIFF (Actual vs Expected) =====".yellow().bold());

        for change in diff.iter_all_changes() {
            match change.tag() {
                ChangeTag::Delete => print!("{}{}", "-".red(), change.value().red()),
                ChangeTag::Insert => print!("{}{}", "+".green(), change.value().green()),
                ChangeTag::Equal => println!(" {}", change.value()),
            };
        }

        println!();
        panic!("{}", "Assertion failed".red().bold());
    }
}

pub fn colored_assert_debug<T: std::fmt::Debug>(actual: &T, expected: &T) {
    let actual_str = format!("{:#?}", actual);
    let expected_str = format!("{:#?}", expected);
    colored_assert(&actual_str, &expected_str);
}
