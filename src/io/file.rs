use std::path::Path;

pub fn pwd() -> String {
    std::env::current_dir()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
}

pub fn read_file(path: &str) -> String {
    let file = std::fs::read_to_string(format!("{}{}", pwd(), path))
                                                    .expect("Error reading file");
    file
}

pub fn read_dir(dir_path: &str) -> Vec<String> {
    if dir_path == "/favicon.ico" {
        return vec![]
    }
    let mut list = Vec::<String>::new();
    for entry in std::fs::read_dir(format!("{}{}", pwd(), dir_path))
                                                                .expect("Error reading directory") {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name()
                                    .unwrap()
                                    .to_str()
                                    .unwrap();
        if dir_path == "/" {
            list.push(format!("{}{}", dir_path, file_name));
        } else {
            list.push(format!("{}/{}", dir_path, file_name));
        }
    }
    list
}

pub fn is_file(path: &str) -> bool {
    let path = format!("{}{}", pwd(), path);
    let path = Path::new(&path);
    path.is_file()
}

pub fn is_exists(path: &str) -> bool {
    let path = format!("{}{}", pwd(), path);
    let path = Path::new(&path);
    path.exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let file = read_file( "/src/io/test" );
        assert_eq!(file, "This file is a test file for io");
    }

    #[test]
    fn test_list_dir() {
        let list = read_dir("/src/io");
        for item in list {
            println!("{}", item);
        }
    }

    #[test]
    fn test_is_file() {
        assert_eq!(is_file("/src/io/test"), true);
    }
}