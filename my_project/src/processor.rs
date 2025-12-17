// src/processor.rs
use std::{fs, time::Instant};
use crate::{types::*, analyzers::*, errors::*};

pub fn process_file(path: &str) -> FileAnalysis {
    let start = Instant::now();
    let mut errors = Vec::new();

    let result = fs::read_to_string(path)
        .map_err(|e| (path.to_string(), e));

    let mut stats = FileStats::default();

    match result {
        Ok(contents) => {
            stats.size_bytes = contents.len() as u64;
            let (w, l, f) = analyze_contents(&contents);
            stats.word_count = w;
            stats.line_count = l;
            stats.char_frequencies = f;
        }
        Err(e) => errors.push(e.into()),
    }

    FileAnalysis {
        filename: path.to_string(),
        stats,
        errors,
        processing_time: start.elapsed(),
    }
}
