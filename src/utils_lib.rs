use std::{fs, io, path};

pub fn open_file(file_path: path::PathBuf) -> io::Result<fs::File> {
    let file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_path);

    file
}
