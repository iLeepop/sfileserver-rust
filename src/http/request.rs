pub fn parse_request_path(request: &str) -> &str {
    request.lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap_or("/")
        .trim_start_matches('/')
}