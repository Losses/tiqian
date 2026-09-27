pub struct Fs;

fn fail(path: &str, error: std::io::Error) -> ! {
    panic!("{}: {}", path, error);
}

impl Fs {
    pub fn exists(path: &str) -> bool {
        std::path::Path::new(path).exists()
    }

    pub fn read_text(path: &str) -> String {
        let bytes = std::fs::read(path).unwrap_or_else(|e| fail(path, e));
        String::from_utf8_lossy(&bytes).into_owned()
    }

    pub fn write_text(path: &str, data: &str) {
        std::fs::write(path, data).unwrap_or_else(|e| fail(path, e));
    }

    pub fn append_text(path: &str, data: &str) {
        use std::io::Write;
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap_or_else(|e| fail(path, e));
        file.write_all(data.as_bytes())
            .unwrap_or_else(|e| fail(path, e));
    }

    pub fn make_dirs(path: &str) {
        std::fs::create_dir_all(path).unwrap_or_else(|e| fail(path, e));
    }

    pub fn read_dir(path: &str) -> Vec<String> {
        let entries = std::fs::read_dir(path).unwrap_or_else(|e| fail(path, e));
        let mut names = Vec::new();
        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => fail(path, e),
            };
            names.push(entry.file_name().to_string_lossy().into_owned());
        }
        names
    }

    pub fn is_directory(path: &str) -> bool {
        match std::fs::metadata(path) {
            Ok(metadata) => metadata.is_dir(),
            Err(_) => false,
        }
    }
}
