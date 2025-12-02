use foamium_dom::parse_html;
use foamium_net::NetworkManager;
use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Entry, Label, Orientation, ScrolledWindow};
use libadwaita as adw;
use adw::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

const APP_ID: &str = "org.foamium.Browser";

fn main() {
    env_logger::init();
    
    println!("Starting Foamium Engine...");

    let app = adw::Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &adw::Application) {
    // Create Adwaita window
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Foamium Browser")
        .default_width(900)
        .default_height(700)
        .build();

    // Create header bar
    let header = adw::HeaderBar::new();
    
    // Navigation buttons
    let nav_box = GtkBox::new(Orientation::Horizontal, 0);
    nav_box.add_css_class("linked");
    
    let back_btn = Button::from_icon_name("go-previous-symbolic");
    let forward_btn = Button::from_icon_name("go-next-symbolic");
    let reload_btn = Button::from_icon_name("view-refresh-symbolic");
    
    nav_box.append(&back_btn);
    nav_box.append(&forward_btn);
    nav_box.append(&reload_btn);
    
    header.pack_start(&nav_box);
    
    // URL Entry
    let url_entry = Entry::new();
    url_entry.set_placeholder_text(Some("Enter URL..."));
    url_entry.set_hexpand(true);
    url_entry.set_text("https://example.com");
    header.set_title_widget(Some(&url_entry));
    
    // Content area (will be updated when loading pages)
    let content_box = Rc::new(RefCell::new(GtkBox::new(Orientation::Vertical, 12)));
    content_box.borrow().set_margin_top(12);
    content_box.borrow().set_margin_bottom(12);
    content_box.borrow().set_margin_start(12);
    content_box.borrow().set_margin_end(12);

    // Wrap in scrolled window
    let scrolled = ScrolledWindow::new();
    scrolled.set_child(Some(&*content_box.borrow()));
    scrolled.set_vexpand(true);

    // Create main container
    let main_box = GtkBox::new(Orientation::Vertical, 0);
    main_box.append(&header);
    main_box.append(&scrolled);

    window.set_content(Some(&main_box));
    
    // Load initial page
    let content_box_clone = content_box.clone();
    load_url("https://example.com", &content_box_clone);
    
    // Connect URL entry activation
    let content_box_clone = content_box.clone();
    let url_entry_clone = url_entry.clone();
    url_entry.connect_activate(move |entry| {
        let url = entry.text().to_string();
        url_entry_clone.set_text(&url);
        load_url(&url, &content_box_clone);
    });
    
    // Connect reload button
    let content_box_clone = content_box.clone();
    let url_entry_clone = url_entry.clone();
    reload_btn.connect_clicked(move |_| {
        let url = url_entry_clone.text().to_string();
        load_url(&url, &content_box_clone);
    });

    window.present();
    println!("✓ Adwaita window ready!");
}

fn load_url(url: &str, content_box: &Rc<RefCell<GtkBox>>) {
    println!("Loading {}...", url);
    
    // Clear existing content
    while let Some(child) = content_box.borrow().first_child() {
        content_box.borrow().remove(&child);
    }
    
    // Fetch and parse
    let net = NetworkManager::new();
    let html_content = match net.fetch_text(url) {
        Ok(content) => {
            println!("✓ Fetched {} bytes", content.len());
            content
        },
        Err(e) => {
            eprintln!("Failed to fetch URL: {}", e);
            let error_label = Label::new(Some(&format!("Error loading page: {}", e)));
            error_label.add_css_class("error");
            content_box.borrow().append(&error_label);
            return;
        }
    };
    
    println!("Parsing HTML...");
    let dom_nodes = parse_html(&html_content);
    let text_lines: Vec<String> = dom_nodes
        .iter()
        .filter_map(|node| node.text.clone())
        .collect();
    
    println!("✓ Extracted {} text nodes", text_lines.len());

    // Add text content
    for text in text_lines {
        let label = Label::new(Some(&text));
        label.set_wrap(true);
        label.set_xalign(0.0);
        label.set_selectable(true);
        label.add_css_class("body");
        content_box.borrow().append(&label);
    }
}

