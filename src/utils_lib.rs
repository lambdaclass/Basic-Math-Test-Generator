use std::{
    fs, io,
    io::{BufWriter, Write},
    path,
};

pub fn open_file(file_path: &path::PathBuf) -> io::Result<fs::File> {
    let file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_path);

    file
}

pub fn delete_file_if_exists(file_path: &path::PathBuf) {
    if file_path.exists() {
        if fs::remove_file(file_path).is_err() {
            eprintln!(
                "Failed to removed old file at {}",
                file_path.to_string_lossy()
            );
        };
    }
}

pub fn update_write_buffer(
    write_buffer: Option<BufWriter<fs::File>>,
    bytes: &[u8],
) -> Option<BufWriter<fs::File>> {
    let Some(mut writer) = write_buffer else {
        return None
    };

    if let Err(e) = writer.write_all(bytes) {
        eprintln!("{}", e);
    };

    Some(writer)
}
