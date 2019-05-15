pub fn stdout_and_stderr(out: std::process::Output) -> String {
    let out = if !out.stdout.is_empty() {
        out.stdout
    } else {
        out.stderr
    };

    String::from_utf8(out).unwrap_or_default()
}

pub fn remove_main(script: &mut String) {
    let main_start = match script.find("fn main() {") {
        Some(idx) => idx,
        None => return,
    };

    let open_tag = main_start + 11;

    // corrupted script `..fn main() {`
    if open_tag == script.len() - 1 {
        return;
    }

    let mut close_tag = None;

    // look for closing tag
    let mut tag_score = 1;
    for (idx, character) in script[open_tag + 1..].chars().enumerate() {
        if character == '{' {
            tag_score += 1;
        }
        if character == '}' {
            tag_score -= 1;
            if tag_score == 0 {
                close_tag = Some(idx);
            }
        }
    }
    if let Some(close_tag) = close_tag {
        script.remove(open_tag + close_tag + 1);
        script.replace_range(main_start..=open_tag, "");
    }
}

pub struct StringTools {}

impl StringTools {
    pub fn insert_at_char_idx(buffer: &mut String, idx: usize, charcter: char) {
        let mut buffer_chars: Vec<char> = buffer.chars().collect();
        buffer_chars.insert(idx, charcter);
        *buffer = buffer_chars.into_iter().collect();
    }

    pub fn remove_at_char_idx(buffer: &mut String, idx: usize) {
        let mut buffer_chars: Vec<char> = buffer.chars().collect();
        buffer_chars.remove(idx);
        *buffer = buffer_chars.into_iter().collect();
    }

    pub fn chars_count(buffer: &str) -> usize {
        buffer.chars().count()
    }

    pub fn is_multiline(string: &str) -> bool {
        string.chars().filter(|c| *c == '\n').count() > 1
    }
}

pub struct VecTools {}

impl VecTools {
    pub fn index<T: std::cmp::PartialEq + std::fmt::Debug>(
        vector: &[T],
        items: Vec<T>,
    ) -> Vec<usize> {
        let mut indices = vec![];

        for (idx, elem) in vector.iter().enumerate() {
            if items.contains(elem) {
                indices.push(idx);
            }
        }

        indices
    }
}