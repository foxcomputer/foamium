use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Entry, Orientation, glib};
use libadwaita as adw;
use adw::prelude::*;
use webkit6::prelude::*;
use webkit6::WebView;
use std::rc::Rc;
use gtk4::gdk;

const APP_ID: &str = "org.foamium.Browser";

fn main() {
    env_logger::init();
    
    // Initialize GTK and WebKit
    let app = adw::Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &adw::Application) {
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Foamium Browser")
        .default_width(1024)
        .default_height(768)
        .build();

    // Set window icon name (icon should be installed in system icon theme)
    window.set_icon_name(Some("org.foamium.Browser"));

    // Create Header Bar
    let header = adw::HeaderBar::new();
    
    // Navigation Buttons
    let nav_box = GtkBox::new(Orientation::Horizontal, 0);
    nav_box.add_css_class("linked");
    
    let back_btn = Button::from_icon_name("go-previous-symbolic");
    let forward_btn = Button::from_icon_name("go-next-symbolic");
    let reload_btn = Button::from_icon_name("view-refresh-symbolic");
    
    nav_box.append(&back_btn);
    nav_box.append(&forward_btn);
    nav_box.append(&reload_btn);
    header.pack_start(&nav_box);
    
    // URL Bar
    let url_entry = Entry::new();
    url_entry.set_placeholder_text(Some("Search or enter address"));
    url_entry.set_hexpand(true);
    url_entry.set_input_purpose(gtk4::InputPurpose::Url);
    header.set_title_widget(Some(&url_entry));

    // New Tab Button
    let new_tab_btn = Button::from_icon_name("tab-new-symbolic");
    header.pack_end(&new_tab_btn);

    // Tab View and Tab Bar
    let tab_view = adw::TabView::new();
    let tab_bar = adw::TabBar::new();
    tab_bar.set_view(Some(&tab_view));

    // Main Layout
    let main_box = GtkBox::new(Orientation::Vertical, 0);
    main_box.append(&header);
    main_box.append(&tab_bar);
    main_box.append(&tab_view);

    window.set_content(Some(&main_box));
    
    // Helper to resolve custom foamium: URIs to file paths
    let resolve_uri = |uri: &str| -> String {
        if uri.starts_with("foamium:") {
            let page = uri.strip_prefix("foamium:").unwrap_or("");
            let page_file = match page {
                "newtab" => "blank.html",
                "error" => "error.html",
                "warning" => "warning.html",
                _ => "blank.html",
            };
            
            std::env::current_dir()
                .ok()
                .and_then(|p| p.join(format!("resources/pages/{}", page_file)).to_str().map(String::from))
                .map(|path| format!("file://{}", path))
                .unwrap_or_else(|| "about:blank".to_string())
        } else {
            uri.to_string()
        }
    };

    // Helper to create a new tab
    let create_tab = {
        let tab_view = tab_view.clone();
        let url_entry = url_entry.clone();
        Rc::new(move |url: Option<&str>| {
            let webview = WebView::new();
            webview.set_hexpand(true);
            webview.set_vexpand(true);
            
            let default_url = "foamium:newtab";
            let url_to_load = url.unwrap_or(default_url);
            
            // Resolve custom URIs
            let resolved_url = resolve_uri(url_to_load);
            webview.load_uri(&resolved_url);
            
            let page = tab_view.append(&webview);
            page.set_title("New Tab");
            page.set_live_thumbnail(true);
            
            // Connect signals for this webview
            let page_weak = page.downgrade();
            
            webview.connect_title_notify(move |wv| {
                if let (Some(title), Some(page)) = (wv.title(), page_weak.upgrade()) {
                    page.set_title(&title);
                }
            });
            
            // Handle load failures
            webview.connect_load_failed(move |_wv, _load_event, failing_uri, error| {
                log::warn!("Failed to load {}: {}", failing_uri, error);
                
                // Use foamium:error with the failed URL as a parameter
                let error_url = format!("{}?url={}", 
                    resolve_uri("foamium:error"),
                    urlencoding::encode(failing_uri));
                
                _wv.load_uri(&error_url);
                
                true
            });
            
            // Update URL bar when URL changes
            let url_entry = url_entry.clone();
            let tab_view = tab_view.clone();
            let page_weak = page.downgrade();
            
            webview.connect_load_changed(move |wv, load_event| {
                if load_event == webkit6::LoadEvent::Committed {
                    // Only update if this is the currently selected tab
                    if let Some(page) = page_weak.upgrade() {
                        if tab_view.selected_page().as_ref() == Some(&page) {
                            if let Some(uri) = wv.uri() {
                                let display_uri = if uri.contains("/resources/pages/blank.html") {
                                    "".to_string()
                                } else if uri.contains("/resources/pages/error.html") {
                                    "foamium:error".to_string()
                                } else if uri.contains("/resources/pages/warning.html") {
                                    "foamium:warning".to_string()
                                } else {
                                    uri.to_string()
                                };
                                url_entry.set_text(&display_uri);
                            }
                        }
                    }
                }
            });
        })
    };

    // Create initial tab
    create_tab(None);

    // New Tab Action
    let create_tab_clone = create_tab.clone();
    new_tab_btn.connect_clicked(move |_| {
        create_tab_clone(None);
    });

    // Handle Tab Closing
    tab_view.connect_close_page(move |view, page| {
        view.close_page_finish(page, true);
        glib::Propagation::Proceed
    });

    // Connect Navigation Buttons to Active Tab
    let tab_view_clone = tab_view.clone();
    back_btn.connect_clicked(move |_| {
        if let Some(page) = tab_view_clone.selected_page() {
            let child = page.child();
            if let Some(webview) = child.downcast_ref::<WebView>() {
                webview.go_back();
            }
        }
    });

    let tab_view_clone = tab_view.clone();
    forward_btn.connect_clicked(move |_| {
        if let Some(page) = tab_view_clone.selected_page() {
            let child = page.child();
            if let Some(webview) = child.downcast_ref::<WebView>() {
                webview.go_forward();
            }
        }
    });

    let tab_view_clone = tab_view.clone();
    reload_btn.connect_clicked(move |_| {
        if let Some(page) = tab_view_clone.selected_page() {
            let child = page.child();
            if let Some(webview) = child.downcast_ref::<WebView>() {
                webview.reload();
            }
        }
    });

    // URL Bar Enter
    let tab_view_clone = tab_view.clone();
    url_entry.connect_activate(move |entry| {
        let input = entry.text();
        
        // Determine if input is a URL or a search query
        let url_str = if input.starts_with("foamium:") {
            // Custom foamium: URI
            resolve_uri(&input)
        } else if input.contains("://") {
            // Already has a protocol (http://, https://, etc.)
            input.to_string()
        } else if input.contains('.') && !input.contains(' ') {
            // Looks like a domain (has a dot and no spaces)
            // Check if it has a TLD-like ending
            let parts: Vec<&str> = input.split('.').collect();
            if parts.len() >= 2 && parts.last().unwrap().len() >= 2 && parts.last().unwrap().len() <= 6 {
                // Likely a domain like "google.com" or "example.co.uk"
                format!("https://{}", input)
            } else {
                // Has a dot but doesn't look like a domain, search it
                format!("https://www.google.com/search?q={}", urlencoding::encode(&input))
            }
        } else {
            // No dots or has spaces - definitely a search query
            format!("https://www.google.com/search?q={}", urlencoding::encode(&input))
        };
        
        if let Some(page) = tab_view_clone.selected_page() {
            let child = page.child();
            if let Some(webview) = child.downcast_ref::<WebView>() {
                webview.load_uri(&url_str);
            }
        }
    });

    // Update URL bar when switching tabs
    let url_entry_clone = url_entry.clone();
    tab_view.connect_selected_page_notify(move |view| {
        if let Some(page) = view.selected_page() {
            let child = page.child();
            if let Some(webview) = child.downcast_ref::<WebView>() {
                if let Some(uri) = webview.uri() {
                    // Convert file:// URIs back to foamium: scheme for display
                    let display_uri = if uri.contains("/resources/pages/blank.html") {
                        "".to_string()
                    } else if uri.contains("/resources/pages/error.html") {
                        "foamium:error".to_string()
                    } else if uri.contains("/resources/pages/warning.html") {
                        "foamium:warning".to_string()
                    } else {
                        uri.to_string()
                    };
                    url_entry_clone.set_text(&display_uri);
                } else {
                    url_entry_clone.set_text("");
                }
            }
        }
    });
    
    
    // Update URL bar when current tab navigates
    // This requires a bit more complex signal handling which we'll simplify for now by just updating on switch
    // Ideally we'd connect to load-changed on every webview and check if it's the active one.

    // Keyboard Shortcuts
    let key_controller = gtk4::EventControllerKey::new();
    
    // Clone references for keyboard shortcuts
    let window_kb = window.clone();
    let tab_view_kb = tab_view.clone();
    let url_entry_kb = url_entry.clone();
    let create_tab_kb = create_tab.clone();
    let back_btn_kb = back_btn.clone();
    let forward_btn_kb = forward_btn.clone();
    let reload_btn_kb = reload_btn.clone();
    
    key_controller.connect_key_pressed(move |_controller, key, _code, modifiers| {
        let ctrl = modifiers.contains(gdk::ModifierType::CONTROL_MASK);
        let alt = modifiers.contains(gdk::ModifierType::ALT_MASK);
        let shift = modifiers.contains(gdk::ModifierType::SHIFT_MASK);
        
        match key {
            // Ctrl+T: New Tab
            gdk::Key::t | gdk::Key::T if ctrl && !shift && !alt => {
                create_tab_kb(None);
                return glib::Propagation::Stop;
            }
            
            // Ctrl+W: Close Tab
            gdk::Key::w | gdk::Key::W if ctrl && !shift && !alt => {
                if tab_view_kb.n_pages() == 1 {
                    if let Some(app) = window_kb.application() {
                        app.quit();
                    }
                } else {
                    if let Some(page) = tab_view_kb.selected_page() {
                        tab_view_kb.close_page(&page);
                    }
                }
                return glib::Propagation::Stop;
            }
            
            // Ctrl+R or F5: Reload
            gdk::Key::r | gdk::Key::R if ctrl && !shift && !alt => {
                reload_btn_kb.emit_clicked();
                return glib::Propagation::Stop;
            }
            gdk::Key::F5 if !ctrl && !shift && !alt => {
                reload_btn_kb.emit_clicked();
                return glib::Propagation::Stop;
            }
            
            // Alt+Left or Backspace: Back
            gdk::Key::Left if alt && !ctrl && !shift => {
                back_btn_kb.emit_clicked();
                return glib::Propagation::Stop;
            }
            gdk::Key::BackSpace if alt && !ctrl && !shift => {
                back_btn_kb.emit_clicked();
                return glib::Propagation::Stop;
            }
            
            // Alt+Right: Forward
            gdk::Key::Right if alt && !ctrl && !shift => {
                forward_btn_kb.emit_clicked();
                return glib::Propagation::Stop;
            }
            
            // Ctrl+L: Focus Address Bar
            gdk::Key::l | gdk::Key::L if ctrl && !shift && !alt => {
                url_entry_kb.grab_focus();
                url_entry_kb.select_region(0, -1); // Select all text
                return glib::Propagation::Stop;
            }
            
            // Ctrl+Tab: Next Tab
            gdk::Key::Tab if ctrl && !shift && !alt => {
                let n_pages = tab_view_kb.n_pages();
                if n_pages > 1 {
                    if let Some(current_page) = tab_view_kb.selected_page() {
                        let current_pos = tab_view_kb.page_position(&current_page);
                        let next_pos = (current_pos + 1) % n_pages;
                        let next_page = tab_view_kb.nth_page(next_pos);
                        tab_view_kb.set_selected_page(&next_page);
                    }
                }
                return glib::Propagation::Stop;
            }
            
            // Ctrl+Shift+Tab: Previous Tab
            gdk::Key::ISO_Left_Tab if ctrl && shift && !alt => {
                let n_pages = tab_view_kb.n_pages();
                if n_pages > 1 {
                    if let Some(current_page) = tab_view_kb.selected_page() {
                        let current_pos = tab_view_kb.page_position(&current_page);
                        let prev_pos = if current_pos == 0 { n_pages - 1 } else { current_pos - 1 };
                        let prev_page = tab_view_kb.nth_page(prev_pos);
                        tab_view_kb.set_selected_page(&prev_page);
                    }
                }
                return glib::Propagation::Stop;
            }
            
            // Ctrl+Q: Quit
            gdk::Key::q | gdk::Key::Q if ctrl && !shift && !alt => {
                if let Some(app) = window_kb.application() {
                    app.quit();
                }
                return glib::Propagation::Stop;
            }
            
            _ => {}
        }
        
        glib::Propagation::Proceed
    });
    
    window.add_controller(key_controller);

    window.present();
}
