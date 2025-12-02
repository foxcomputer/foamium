use foamium_dom::parse_html;
use foamium_net::NetworkManager;
use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Label, Orientation, ScrolledWindow};
use libadwaita as adw;
use adw::prelude::*;

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
    // Fetch and parse HTML
    let net = NetworkManager::new();
    let url = "https://example.com";
    
    println!("Fetching {}...", url);
    let html_content = match net.fetch_text(url) {
        Ok(content) => {
            println!("✓ Fetched {} bytes", content.len());
            content
        },
        Err(e) => {
            eprintln!("Failed to fetch URL: {}", e);
            "<html><body>Error loading page</body></html>".to_string()
        }
    };
    
    println!("Parsing HTML...");
    let dom_nodes = parse_html(&html_content);
    let text_lines: Vec<String> = dom_nodes
        .iter()
        .filter_map(|node| node.text.clone())
        .collect();
    
    println!("✓ Extracted {} text nodes", text_lines.len());

    // Create Adwaita window
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Foamium Browser")
        .default_width(800)
        .default_height(600)
        .build();

    // Create header bar with Adwaita styling
    let header = adw::HeaderBar::new();
    
    // Create content area
    let content_box = GtkBox::new(Orientation::Vertical, 12);
    content_box.set_margin_top(12);
    content_box.set_margin_bottom(12);
    content_box.set_margin_start(12);
    content_box.set_margin_end(12);

    // Add text content
    for text in text_lines {
        let label = Label::new(Some(&text));
        label.set_wrap(true);
        label.set_xalign(0.0);
        label.add_css_class("body");
        content_box.append(&label);
    }

    // Wrap in scrolled window
    let scrolled = ScrolledWindow::new();
    scrolled.set_child(Some(&content_box));
    scrolled.set_vexpand(true);

    // Create main container
    let main_box = GtkBox::new(Orientation::Vertical, 0);
    main_box.append(&header);
    main_box.append(&scrolled);

    window.set_content(Some(&main_box));
    window.present();
    
    println!("✓ Adwaita window ready!");
}
