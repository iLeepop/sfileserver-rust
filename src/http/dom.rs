use async_std::path::PathBuf;
use tokio::{io::Result, fs};

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

pub async fn generate_directory_listing(dir: &PathBuf, req_path: &str) -> std::io::Result<String> {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html><html><head><meta charset=\"utf-8\"><title>Index of ");
    html.push_str(req_path);
    html.push_str("</title>");
    // html.push_str(r#"<style>body</style>"#);
    html.push_str("</head><body><h1>Index of ");
    html.push_str(req_path);
    html.push_str("</h1><ul style=\" list-style: none; \">");

    if req_path != "" {
        html.push_str(&format!("<li><a href=\"../\">../</a></li>"));
    }

    let mut entries = fs::read_dir(dir).await?;
    while let Some(entry) = entries.next_entry().await? {
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();
        let path = entry.path();
        let display_path = if path.is_dir() {
            format!("{}/", name)
        } else {
            name.to_string()
        };

        html.push_str(&format!("<li><a href=\"{0}\">{0}</a></li>", display_path));
    }

    html.push_str("</ul></body></html>");
    Ok(html)
}
