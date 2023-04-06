// This file handles the input buffer

use std::sync::{Mutex, OnceLock};

struct Line {
    idx: usize,
    line: String,
}

static LINE: OnceLock<Mutex<Line>> = OnceLock::new();

pub fn init() {
    LINE.get_or_init(|| {
        Mutex::new(Line {
            idx: 0,
            line: String::new(),
        })
    });
}

pub fn insert(c: char) {
    let mut line = LINE.get().unwrap().lock().unwrap();
    let aux = line.idx;
    line.line.insert(aux, c);
    line.idx += 1;
}

// Returns the rest of the string to print from the cursor
pub fn backspace() -> Option<String> {
    let mut line = LINE.get().unwrap().lock().unwrap();
    let aux = line.idx;
    if aux == 0 {
        return None; // Just don't
    }

    line.line.remove(aux - 1);
    line.idx -= 1;
    return Some("".to_string()); // Temporal! This will be done nicely
}
