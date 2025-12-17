// src/progress.rs
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct ProgressTracker {
    pub completed: usize,
    pub failed: usize,
}

pub type SharedProgress = Arc<Mutex<ProgressTracker>>;
