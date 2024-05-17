use crate::io::is_file;

pub fn content(path_list: Vec<String>) -> String {
    let mut content = String::new();
    for path in path_list {
        if is_file(&path[..]) {
            let file_name = path.split("/").last().unwrap();
            content += &format!("<a href=\"{}\">{}</a><br>", path, file_name);
        } else {
            let dir_name = format!("/{}", path.split("/").last().unwrap());
            content += &format!("<a href=\"{}\">{}</a><br>", path, dir_name);
        }
    }
    content
}