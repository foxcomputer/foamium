// Layout tree - combines StyledNode with box model
use foamium_css::{Stylesheet, Rule, Selector, SimpleSelector, Value, Specificity, Unit, Color};
use foamium_dom::{DomNode, NodeType, ElementData};
use std::collections::{HashMap, HashSet};

pub type PropertyMap = HashMap<String, Value>;

#[derive(Debug)]
pub struct LayoutBox<'a> {
    pub node: &'a DomNode,
    pub display: Display,
    pub dimensions: Dimensions,
    pub style: ComputedStyle,
    pub children: Vec<LayoutBox<'a>>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Dimensions {
    pub content: Rect,
    pub padding: EdgeSizes,
    pub border: EdgeSizes,
    pub margin: EdgeSizes,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct EdgeSizes {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

#[derive(Debug, Clone)]
pub struct ComputedStyle {
    pub color: Option<Color>,
    pub background_color: Option<Color>,
    pub font_size: f32,
    pub display: Display,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Display {
    Inline,
    Block,
    None,
}

pub fn build_layout_tree<'a>(root: &'a DomNode, stylesheet: &'a Stylesheet) -> LayoutBox<'a> {
    let specified_values = match root.node_type {
        NodeType::Element(ref elem) => specified_values(elem, stylesheet),
        NodeType::Text(_) => HashMap::new(),
    };
    
    let display = get_display(&specified_values);
    let style = compute_style(&specified_values);
    
    let mut children = Vec::new();
    for child in &root.children {
        children.push(build_layout_tree(child, stylesheet));
    }
    
    LayoutBox {
        node: root,
        display,
        dimensions: Dimensions::default(), // Layout calculation happens later
        style,
        children,
    }
}

fn get_display(values: &PropertyMap) -> Display {
    match values.get("display") {
        Some(Value::Keyword(s)) => match s.as_str() {
            "block" => Display::Block,
            "none" => Display::None,
            _ => Display::Inline,
        },
        _ => Display::Inline,
    }
}

fn compute_style(values: &PropertyMap) -> ComputedStyle {
    let color = match values.get("color") {
        Some(Value::ColorValue(c)) => Some(*c),
        _ => None,
    };
    
    let background_color = match values.get("background-color") {
        Some(Value::ColorValue(c)) => Some(*c),
        _ => None,
    };
    
    let font_size = match values.get("font-size") {
        Some(Value::Length(v, Unit::Px)) => *v,
        Some(Value::Length(v, Unit::Pt)) => *v * 1.33, // Approx pt to px
        _ => 16.0, // Default
    };
    
    ComputedStyle {
        color,
        background_color,
        font_size,
        display: get_display(values),
    }
}

fn specified_values(elem: &ElementData, stylesheet: &Stylesheet) -> PropertyMap {
    let mut values = HashMap::new();
    let mut rules = matching_rules(elem, stylesheet);

    rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
    for (_, rule) in rules {
        for declaration in &rule.declarations {
            values.insert(declaration.name.clone(), declaration.value.clone());
        }
    }
    values
}

type MatchedRule<'a> = (Specificity, &'a Rule);

fn matching_rules<'a>(elem: &ElementData, stylesheet: &'a Stylesheet) -> Vec<MatchedRule<'a>> {
    stylesheet
        .rules
        .iter()
        .filter_map(|rule| match_rule(elem, rule))
        .collect()
}

fn match_rule<'a>(elem: &ElementData, rule: &'a Rule) -> Option<MatchedRule<'a>> {
    rule.selectors
        .iter()
        .find(|selector| matches(elem, *selector))
        .map(|selector| (selector.specificity(), rule))
}

fn matches(elem: &ElementData, selector: &Selector) -> bool {
    match *selector {
        Selector::Simple(ref simple_selector) => matches_simple_selector(elem, simple_selector),
    }
}

fn matches_simple_selector(elem: &ElementData, selector: &SimpleSelector) -> bool {
    if selector.tag_name.iter().any(|name| elem.tag_name != *name) {
        return false;
    }

    if selector.id.iter().any(|id| elem_id(elem) != Some(id)) {
        return false;
    }

    let elem_classes = elem_classes(elem);
    if selector
        .class
        .iter()
        .any(|class| !elem_classes.contains(&**class))
    {
        return false;
    }

    true
}

// Helper functions for ElementData
fn elem_id(elem: &ElementData) -> Option<&String> {
    elem.attributes.get("id")
}

fn elem_classes(elem: &ElementData) -> HashSet<&str> {
    match elem.attributes.get("class") {
        Some(classlist) => classlist.split(' ').collect(),
        None => HashSet::new(),
    }
}

