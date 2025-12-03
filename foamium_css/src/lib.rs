use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Selector {
    Simple(SimpleSelector),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>,
}

pub type Specificity = (usize, usize, usize);

impl Selector {
    pub fn specificity(&self) -> Specificity {
        let Selector::Simple(ref simple) = *self;
        let a = simple.id.iter().count();
        let b = simple.class.len();
        let c = simple.tag_name.iter().count();
        (a, b, c)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    pub name: String,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Unit {
    Px,
    Pt,
    Em,
    Rem,
    Percent,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
}

pub fn parse(source: &str) -> Stylesheet {
    let mut parser = Parser {
        pos: 0,
        input: source.to_string(),
    };
    Stylesheet {
        rules: parser.parse_rules(),
    }
}

struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    fn parse_rules(&mut self) -> Vec<Rule> {
        let mut rules = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() {
                break;
            }
            if let Some(rule) = self.parse_rule() {
                rules.push(rule);
            } else {
                // Skip invalid token to make progress
                self.consume_char();
            }
        }
        rules
    }

    fn parse_rule(&mut self) -> Option<Rule> {
        let selectors = self.parse_selectors();
        if selectors.is_empty() {
            return None;
        }
        let declarations = self.parse_declarations();
        Some(Rule {
            selectors,
            declarations,
        })
    }

    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();
        loop {
            if let Some(selector) = self.parse_simple_selector() {
                selectors.push(Selector::Simple(selector));
            }
            
            self.consume_whitespace();
            match self.next_char() {
                ',' => {
                    self.consume_char();
                    self.consume_whitespace();
                }
                '{' => break,
                _ => {
                    // Unexpected char, stop parsing selectors
                    break; 
                }
            }
            if self.eof() { break; }
        }
        selectors.sort_by(|a, b| b.specificity().cmp(&a.specificity()));
        selectors
    }

    fn parse_simple_selector(&mut self) -> Option<SimpleSelector> {
        let mut selector = SimpleSelector {
            tag_name: None,
            id: None,
            class: Vec::new(),
        };
        let mut found_something = false;
        
        while !self.eof() {
            match self.next_char() {
                '#' => {
                    self.consume_char();
                    selector.id = Some(self.parse_identifier());
                    found_something = true;
                }
                '.' => {
                    self.consume_char();
                    selector.class.push(self.parse_identifier());
                    found_something = true;
                }
                '*' => {
                    self.consume_char();
                    found_something = true;
                }
                c if valid_identifier_char(c) => {
                    selector.tag_name = Some(self.parse_identifier());
                    found_something = true;
                }
                _ => break,
            }
        }
        
        if found_something {
            Some(selector)
        } else {
            None
        }
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        if self.next_char() != '{' {
            return Vec::new();
        }
        self.consume_char(); // consume '{'
        
        let mut declarations = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() || self.next_char() == '}' {
                if !self.eof() { self.consume_char(); }
                break;
            }
            if let Some(decl) = self.parse_declaration() {
                declarations.push(decl);
            } else {
                // Skip to next semicolon or brace to recover
                self.consume_until_delimiter();
            }
        }
        declarations
    }
    
    fn consume_until_delimiter(&mut self) {
        while !self.eof() {
            let c = self.consume_char();
            if c == ';' || c == '}' {
                break;
            }
        }
    }

    fn parse_declaration(&mut self) -> Option<Declaration> {
        let property_name = self.parse_identifier();
        self.consume_whitespace();
        
        if self.next_char() != ':' {
            return None;
        }
        self.consume_char(); // consume ':'
        
        self.consume_whitespace();
        let value = self.parse_value();
        self.consume_whitespace();
        
        if self.next_char() == ';' {
            self.consume_char();
        }
        
        Some(Declaration {
            name: property_name,
            value,
        })
    }

    fn parse_value(&mut self) -> Value {
        match self.next_char() {
            '0'..='9' => self.parse_length(),
            '#' => self.parse_color(),
            _ => Value::Keyword(self.parse_identifier()),
        }
    }

    fn parse_length(&mut self) -> Value {
        Value::Length(self.parse_float(), self.parse_unit())
    }

    fn parse_float(&mut self) -> f32 {
        let s = self.consume_while(|c| matches!(c, '0'..='9' | '.'));
        s.parse().unwrap_or(0.0)
    }

    fn parse_unit(&mut self) -> Unit {
        let ident = self.parse_identifier();
        match &*ident.to_ascii_lowercase() {
            "px" => Unit::Px,
            "pt" => Unit::Pt,
            "em" => Unit::Em,
            "rem" => Unit::Rem,
            "%" => Unit::Percent,
            _ => Unit::Px, // Default to px
        }
    }

    fn parse_color(&mut self) -> Value {
        self.consume_char(); // consume '#'
        Value::ColorValue(Color {
            r: self.parse_hex_pair(),
            g: self.parse_hex_pair(),
            b: self.parse_hex_pair(),
            a: 255,
        })
    }

    fn parse_hex_pair(&mut self) -> u8 {
        if self.pos + 2 > self.input.len() {
            return 0;
        }
        let s = &self.input[self.pos..self.pos + 2];
        self.pos += 2;
        u8::from_str_radix(s, 16).unwrap_or(0)
    }

    fn parse_identifier(&mut self) -> String {
        self.consume_while(valid_identifier_char)
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
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
        if self.eof() { return '\0'; }
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        cur_char
    }

    fn next_char(&self) -> char {
        if self.eof() { return '\0'; }
        self.input[self.pos..].chars().next().unwrap()
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}

fn valid_identifier_char(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_stylesheet() {
        let css = "h1, h2 { color: #ff0000; margin: 10px; }";
        let stylesheet = parse(css);
        assert_eq!(stylesheet.rules.len(), 1);
        assert_eq!(stylesheet.rules[0].selectors.len(), 2);
        assert_eq!(stylesheet.rules[0].declarations.len(), 2);
    }
    
    #[test]
    fn test_robustness() {
        let css = "h1 { color: #zzzzzz; width: 100unknown; } invalid {";
        let stylesheet = parse(css);
        // Should not panic and produce something
        assert!(!stylesheet.rules.is_empty());
    }
}
