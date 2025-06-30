use regex::Regex;

pub fn parse_input(input: &str) -> Option<(String, String)> {
    // Trim the brackets
    let trimmed = input.trim().trim_start_matches('[').trim_end_matches(']');

    // Split by comma
    let parts: Vec<&str> = trimmed.split(',').map(|s| s.trim()).collect();

    if parts.len() == 2 {
        Some((parts[0].to_string(), parts[1].to_string()))
    } else {
        None
    }
}

pub fn extract_output(content: &str) -> Option<String> {
    let output_re = Regex::new(r"OUTPUT:\s*(\[.*?\])").unwrap();
    output_re.captures(content)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().trim().to_string())
}

