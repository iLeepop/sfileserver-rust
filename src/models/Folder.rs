use super::Item::Item;

/// Item struct

pub struct Folder {
    name: String,
    path: String,
    items: Vec<Box<dyn Item>>
}

impl Item for Folder {
    // 搜索内容
    fn search(&self) {
        
    }
    // 获取内容
    fn get_content(&self) -> Result<String, std::io::Error> {
        Ok(String::from(""))
    }
}
