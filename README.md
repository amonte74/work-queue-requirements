Parallel File Processor Project Requirements
Project Overview
Create a thread pool-based system for processing multiple files concurrently with configurable processing rules.

Core Requirements
Thread Pool Implementation
Must implement a generic thread pool
Must support dynamic number of worker threads
Must handle task distribution efficiently
Must implement proper shutdown mechanism
File Processing Features
Must support processing files from multiple directories
Must implement these analyzers:
Word count
Line count
Character frequency
File size statistics
Technical Requirements
Must use Arc and Mutex for shared state
Must implement proper error handling
Must support cancellation
Must handle all file system errors gracefully
Progress Tracking
Real-time progress updates
Per-file processing status
Error reporting with context
Processing time statistics
Output Format
struct FileAnalysis {
    filename: String,
    stats: FileStats,
    errors: Vec<ProcessingError>,
    processing_time: Duration,
}

struct FileStats {
    word_count: usize,
    line_count: usize,
    char_frequencies: HashMap<char, usize>,
    size_bytes: u64,
}
Bonus Features
Support for different file encodings
Extensible analyzer system
Memory usage limiting
Progress persistence
Resume capability
Testing Requirements
Unit tests for thread pool
Integration tests for file processing
Performance benchmarks
Error handling scenarios