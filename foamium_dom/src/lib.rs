// Proper HTML parser with DOM tree structure
#[derive(Debug, Clone)]
pub struct DomNode {
    pub node_type: NodeType,
    pub children: Vec<DomNode>,
}

#[derive(Debug, Clone)]
pub enum NodeType {
    Element(ElementData),
    Text(String),
}

#[derive(Debug, Clone)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap,
}

pub type AttrMap = std::collections::HashMap<String, String>;

impl DomNode {
    pub fn text(data: String) -> DomNode {
        DomNode {
            node_type: NodeType::Text(data),
            children: vec![],
        }
    }

    pub fn elem(name: String, attrs: AttrMap, children: Vec<DomNode>) -> DomNode {
        DomNode {
            node_type: NodeType::Element(ElementData {
                tag_name: name,
                attributes: attrs,
            }),
            children,
        }
    }
}

pub fn parse_html(source: &str) -> DomNode {
    let mut parser = Parser {
        pos: 0,
        input: source.to_string(),
    };
    let nodes = parser.parse_nodes();
    
    // Wrap in root element
    DomNode::elem("html".to_string(), AttrMap::new(), nodes)
}

struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    fn parse_nodes(&mut self) -> Vec<DomNode> {
        let mut nodes = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        nodes
    }

    fn parse_node(&mut self) -> DomNode {
        if self.next_char() == '<' {
            self.parse_element()
        } else {
            self.parse_text()
        }
    }

    fn parse_element(&mut self) -> DomNode {
        // Opening tag
        assert_eq!(self.consume_char(), '<');
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert_eq!(self.consume_char(), '>');

        // Contents
        let children = self.parse_nodes();

        // Closing tag
        assert_eq!(self.consume_char(), '<');
        assert_eq!(self.consume_char(), '/');
        assert_eq!(self.parse_tag_name(), tag_name);
        assert_eq!(self.consume_char(), '>');

        DomNode::elem(tag_name, attrs, children)
    }

    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| c.is_alphanumeric())
    }

    fn parse_attributes(&mut self) -> AttrMap {
        let mut attributes = AttrMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }
        attributes
    }

    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert_eq!(self.consume_char(), '=');
        let value = self.parse_attr_value();
        (name, value)
    }

    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        assert_eq!(self.consume_char(), open_quote);
        value
    }

    fn parse_text(&mut self) -> DomNode {
        DomNode::text(self.consume_while(|c| c != '<'))
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(|c| c.is_whitespace());
    }

    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        cur_char
    }

    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}

// Simple fallback parser for malformed HTML
pub fn parse_html_simple(html: &str) -> DomNode {
    let mut children = Vec::new();
    let mut current_text = String::new();
    let mut in_tag = false;
    let mut in_script = false;
    let mut in_style = false;
    
    for c in html.chars() {
        match c {
            '<' => {
                if !current_text.trim().is_empty() && !in_script && !in_style {
                    children.push(DomNode::text(current_text.trim().to_string()));
                }
                current_text.clear();
                in_tag = true;
            }
            '>' => {
                let tag_lower = current_text.to_lowercase();
                
                if tag_lower.starts_with("script") {
                    in_script = true;
                } else if tag_lower.starts_with("/script") {
                    in_script = false;
                } else if tag_lower.starts_with("style") {
                    in_style = true;
                } else if tag_lower.starts_with("/style") {
                    in_style = false;
                }
                
                current_text.clear();
                in_tag = false;
            }
            _ => {
                if !in_tag && !in_script && !in_style {
                    current_text.push(c);
                } else if in_tag {
                    current_text.push(c);
                }
            }
        }
    }
    
    if !current_text.trim().is_empty() && !in_script && !in_style {
        children.push(DomNode::text(current_text.trim().to_string()));
    }
    
    DomNode::elem("body".to_string(), AttrMap::new(), children)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_parse() {
        let html = "<html><body>Hello World</body></html>";
        let dom = parse_html(html);
        assert!(matches!(dom.node_type, NodeType::Element(_)));
    }
}
