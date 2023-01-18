use std::{
    fs, io,
    io::{BufWriter, Write},
    path,
};

pub fn open_file_or_create(file_path: &path::PathBuf) -> io::Result<fs::File> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env, fs};

    #[test]
    fn test_open_file_or_create() {
        let temp_dir = env::temp_dir();
        let test_file_path = temp_dir.join("test_file.txt");
        let results = open_file_or_create(&test_file_path);

        assert_eq!(true, test_file_path.exists());
        assert_eq!(true, test_file_path.is_file());
        assert_eq!(true, results.is_ok());
    }

    #[test]
    fn test_delete_file_if_exists() {
        let temp_dir = env::temp_dir();
        let delete_file_path = temp_dir.join("delete_file.txt");

        delete_file_if_exists(&delete_file_path);

        assert_eq!(false, delete_file_path.exists());
    }

    #[test]
    fn test_update_write_buffer() {
        let temp_dir = env::temp_dir();
        let test_file_path = temp_dir.join("test_write_file.txt");
        let test_file_result = fs::File::create(test_file_path);
        let mut test_write_buffer: Option<BufWriter<fs::File>> = None;

        if let Ok(test_file) = test_file_result {
            test_write_buffer = Some(BufWriter::new(test_file));
        }

        test_write_buffer = update_write_buffer(test_write_buffer, "test".as_bytes());

        assert_eq!(true, test_write_buffer.is_some());

        if let Some(writer) = test_write_buffer {
            assert_eq!("test".as_bytes(), writer.buffer())
        }
    }
}
