use crate::error::LockBoxResult;
use std::fs::File;
use std::path::Path;

// Open file for reading
pub fn open_input(path: &Path) -> LockBoxResult<File> {
    Ok(File::open(path)?)
}

// Open file for writing (overwrites if exists)
pub fn open_output(path: &Path) -> LockBoxResult<File> {
    Ok(File::create(path)?)
}
