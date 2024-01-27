use serde::Deserialize;
use similar::TextDiff;

#[derive(Debug, Deserialize)]
pub struct DiffFormData {
    pub diff_type: String,
    pub left: String,
    pub right: String,
    pub unified: Option<String>,
}

pub enum DiffType {
    Lines,
    Words,
    Chars,
}

impl DiffType {
    pub fn from_str(s: &str) -> DiffType {
        match s {
            "lines" => DiffType::Lines,
            "words" => DiffType::Words,
            "chars" => DiffType::Chars,
            _ => DiffType::Lines,
        }
    }
}

pub fn differ_by_type(diff_type: DiffType, left: &str, right: &str) -> String {
    let diff = match diff_type {
        DiffType::Lines => TextDiff::from_lines(left, right),
        DiffType::Words => TextDiff::from_words(left, right),
        DiffType::Chars => TextDiff::from_chars(left, right),
    };
    let diff1 = diff.unified_diff();
    diff1.to_string()
}
