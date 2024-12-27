use std::path::Path;

pub fn pwd() -> String {
    std::env::current_dir()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
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