use async_std::path::PathBuf;

pub fn get_content_type(path: &PathBuf) -> &str {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("html") => "text/html",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("mp4") => "video/mp4",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        _ => "application/octet-stream",
    }
}