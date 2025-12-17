// src/analyzers.rs
use std::collections::HashMap;

pub fn analyze_contents(contents: &str) -> (usize, usize, HashMap<char, usize>) {
    let word_count = contents.split_whitespace().count();
    let line_count = contents.lines().count();

    let mut freq = HashMap::new();
    for ch in contents.chars() {
        *freq.entry(ch).or_insert(0) += 1;
    }

    (word_count, line_count, freq)
}
