use std::path::Path;

pub fn pwd() -> String {
    std::env::current_dir()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
}

// 读文件
pub fn read_file(path: &str) -> String {
    // let path = format!("{}{}", pwd(), path);
    let file = std::fs::read_to_string(format!("{}{}", pwd(), path))
                                                    .expect("Error reading file");
    file
}

// 读目录
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
        list.push(file_name.to_string());
    }
    list
}

// 判断文件类型 文件或目录
pub fn is_file(path: &str) -> bool {
    let path = Path::new(path);
    path.is_file()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        // 确认当前执行路径
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
}