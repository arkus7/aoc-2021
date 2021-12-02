use std::fs;
use std::io::Result;
use std::path::Path;

pub fn read_input<P: AsRef<Path>>(path: P) -> Result<String> {
    fs::read_to_string(path)
}
