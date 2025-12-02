// Simple HTML parser - just enough to extract text content
pub struct DomNode {
    pub tag: String,
    pub text: Option<String>,
    pub children: Vec<DomNode>,
}

pub fn parse_html(html: &str) -> Vec<DomNode> {
    // Very basic parser - just extracts text between tags
    let mut nodes = Vec::new();
    let mut current_text = String::new();
    let mut in_tag = false;
    let mut in_script = false;
    let mut in_style = false;
    
    for c in html.chars() {
        match c {
            '<' => {
                if !current_text.trim().is_empty() && !in_script && !in_style {
                    nodes.push(DomNode {
                        tag: "text".to_string(),
                        text: Some(current_text.trim().to_string()),
                        children: vec![],
                    });
                }
                current_text.clear();
                in_tag = true;
            }
            '>' => {
                if current_text.to_lowercase().starts_with("script") {
                    in_script = true;
                } else if current_text.to_lowercase().starts_with("/script") {
                    in_script = false;
                } else if current_text.to_lowercase().starts_with("style") {
                    in_style = true;
                } else if current_text.to_lowercase().starts_with("/style") {
                    in_style = false;
                }
                current_text.clear();
                in_tag = false;
            }
            _ => {
                if !in_tag {
                    current_text.push(c);
                } else {
                    current_text.push(c);
                }
            }
        }
    }
    
    if !current_text.trim().is_empty() && !in_script && !in_style {
        nodes.push(DomNode {
            tag: "text".to_string(),
            text: Some(current_text.trim().to_string()),
            children: vec![],
        });
    }
    
    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_parse() {
        let html = "<html><body>Hello World</body></html>";
        let nodes = parse_html(html);
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].text, Some("Hello World".to_string()));
    }
}
