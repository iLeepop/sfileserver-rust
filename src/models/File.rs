use super::Item::Item;
use crate::utils::pwd;

/// File Models

pub enum FileType {
    PlainText,
    Code,
    Image,
    Video,
    OtherMedia,
}

pub struct File {
    // 文件名
    name: String,
    // 文件路径
    path: String,
    // 文件类型
    file_type: FileType,
    // 文件大小
    size: usize,
}

impl File {
    pub fn new(
        name: String, 
        path: String, 
        file_type: FileType, 
        size: usize 
    ) -> File {
        File {
            name,
            path,
            file_type,
            size,
        }
    }
}

// 根据文件后缀生成不同类型的文件
// 为不同类型的文件提供不同的展示方式

impl Item for File {
    // 搜索文件
    fn search(&self) {
        
    }
    // 获取文件内容
    fn get_content(&self) -> Result<String, std::io::Error> {
        let content = std::fs::read_to_string(format!("{}{}", pwd(), self.path))?;
        Ok(content)
    }
}
