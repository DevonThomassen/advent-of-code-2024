use std::fs;
use std::path::Path;

pub struct FileReader;

impl FileReader {
    pub fn read_file(file_path: &str) -> Result<String, String> {
        let file_path = Path::new(file_path);

        if !file_path.exists() {
            return Err(format!("File does not exist: {:?}", file_path));
        }

        match fs::read_to_string(file_path) {
            Ok(content) => Ok(content),
            Err(e) => Err(format!("Something went wrong reading the file: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = FileReader::read_file("../example.txt");
        assert_eq!(result.ok().unwrap(), "Lorem ipsum dolor sit amet");
    }
}
