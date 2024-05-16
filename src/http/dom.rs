pub fn content(path_list: Vec<String>) -> String {
    let mut content = String::new();
    for path in path_list {
        content += &format!("<a href=\"{}\">{}</a><br>", path, path);
    }
    content
}