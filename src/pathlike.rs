#[cfg(not(windows))]
pub const DEFAULT_DELIM: char = ':';

#[cfg(windows)]
pub const DEFAULT_DELIM: char = ';';

pub fn seems_pathlike(s: &str, delimitator: char) -> bool {
    let parts: Vec<&str> = s.split(delimitator).collect();
    if parts.len() < 2 {
        return false;
    }
    for part in parts {
        if part.trim().is_empty() {
            return false;
        }
    }
    true
}

pub fn add_to_pathlike(original: &str, value: &str, delimitator: char) -> String {
    let mut parts: Vec<&str> = original.split(delimitator).collect();
    if !parts.contains(&value) {
        parts.push(value);
    }
    parts.join(&delimitator.to_string())
}

pub fn remove_from_pathlike(original: &str, value: &str, delimitator: char) -> String {
    let parts: Vec<&str> = original
        .split(delimitator)
        .filter(|&part| part != value)
        .collect();
    parts.join(&delimitator.to_string())
}
