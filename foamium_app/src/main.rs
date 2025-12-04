use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Entry, Orientation, glib};
use libadwaita as adw;
use adw::prelude::*;
use webkit6::prelude::*;
use webkit6::WebView;
use std::rc::Rc;
use gtk4::gdk;

mod database;
use database::Database;

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

fn generate_history_html(database: &Database) -> String {
    let history = database.get_history(100).unwrap_or_default();
    
    let items_html = if history.is_empty() {
        r#"<div class="empty-state">
            <h2>No History Yet</h2>
            <p>Pages you visit will appear here.</p>
        </div>"#.to_string()
    } else {
        history.iter().map(|entry| {
            let first_char = entry.title.chars().next().unwrap_or('?').to_uppercase().to_string();
            let time = entry.timestamp.format("%b %d, %H:%M").to_string();
            format!(
                r#"<a href="{}" class="history-item">
                    <div class="history-icon">{}</div>
                    <div class="history-content">
                        <div class="history-title">{}</div>
                        <div class="history-url">{}</div>
                    </div>
                    <div class="history-time">{}</div>
                </a>"#,
                html_escape(&entry.url),
                html_escape(&first_char),
                html_escape(&entry.title),
                html_escape(&entry.url),
                time
            )
        }).collect::<Vec<_>>().join("\n")
    };

    format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>History</title>
    <style>
        :root {{
            --bg-color: #fafafa;
            --surface-color: #ffffff;
            --text-color: #1c1c1c;
            --dim-text-color: #5e5e5e;
            --accent-color: #3584e4;
            --border-color: #d0d0d0;
        }}
        @media (prefers-color-scheme: dark) {{
            :root {{
                --bg-color: #1e1e1e;
                --surface-color: #303030;
                --text-color: #ffffff;
                --dim-text-color: #c0c0c0;
                --accent-color: #62a0ea;
                --border-color: #454545;
            }}
        }}
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{
            font-family: system-ui, -apple-system, sans-serif;
            background-color: var(--bg-color);
            color: var(--text-color);
            min-height: 100vh;
            padding: 2rem;
            line-height: 1.6;
        }}
        .container {{ max-width: 900px; margin: 0 auto; }}
        header {{
            display: flex;
            align-items: center;
            gap: 1rem;
            margin-bottom: 2rem;
            padding-bottom: 1rem;
            border-bottom: 1px solid var(--border-color);
        }}
        h1 {{ font-size: 1.75rem; font-weight: 600; }}
        .history-list {{ display: flex; flex-direction: column; gap: 0.5rem; }}
        .history-item {{
            display: flex;
            align-items: center;
            gap: 1rem;
            padding: 0.75rem 1rem;
            background: var(--surface-color);
            border-radius: 8px;
            border: 1px solid var(--border-color);
            text-decoration: none;
            color: inherit;
            transition: all 0.15s ease;
        }}
        .history-item:hover {{
            border-color: var(--accent-color);
            transform: translateX(4px);
        }}
        .history-icon {{
            width: 32px;
            height: 32px;
            border-radius: 6px;
            background: var(--accent-color);
            display: flex;
            align-items: center;
            justify-content: center;
            color: white;
            font-weight: 600;
            font-size: 0.875rem;
            flex-shrink: 0;
        }}
        .history-content {{ flex: 1; min-width: 0; }}
        .history-title {{
            font-weight: 500;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }}
        .history-url {{
            font-size: 0.85rem;
            color: var(--dim-text-color);
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }}
        .history-time {{
            font-size: 0.85rem;
            color: var(--dim-text-color);
            white-space: nowrap;
            flex-shrink: 0;
        }}
        .empty-state {{
            text-align: center;
            padding: 4rem 2rem;
            color: var(--dim-text-color);
        }}
        .empty-state h2 {{ margin-bottom: 0.5rem; }}
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>📜 History</h1>
        </header>
        <div class="history-list">
            {}
        </div>
    </div>
</body>
</html>"#, items_html)
}

fn generate_bookmarks_html(database: &Database) -> String {
    let bookmarks = database.get_bookmarks().unwrap_or_default();
    
    let items_html = if bookmarks.is_empty() {
        r#"<div class="empty-state">
            <h2>No Bookmarks Yet</h2>
            <p>Save your favorite pages with the star icon.</p>
        </div>"#.to_string()
    } else {
        bookmarks.iter().map(|entry| {
            let first_char = entry.title.chars().next().unwrap_or('?').to_uppercase().to_string();
            format!(
                r#"<a href="{}" class="bookmark-item">
                    <div class="bookmark-icon">{}</div>
                    <div class="bookmark-content">
                        <div class="bookmark-title">{}</div>
                        <div class="bookmark-url">{}</div>
                    </div>
                </a>"#,
                html_escape(&entry.url),
                html_escape(&first_char),
                html_escape(&entry.title),
                html_escape(&entry.url)
            )
        }).collect::<Vec<_>>().join("\n")
    };

    format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Bookmarks</title>
    <style>
        :root {{
            --bg-color: #fafafa;
            --surface-color: #ffffff;
            --text-color: #1c1c1c;
            --dim-text-color: #5e5e5e;
            --accent-color: #3584e4;
            --border-color: #d0d0d0;
        }}
        @media (prefers-color-scheme: dark) {{
            :root {{
                --bg-color: #1e1e1e;
                --surface-color: #303030;
                --text-color: #ffffff;
                --dim-text-color: #c0c0c0;
                --accent-color: #62a0ea;
                --border-color: #454545;
            }}
        }}
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{
            font-family: system-ui, -apple-system, sans-serif;
            background-color: var(--bg-color);
            color: var(--text-color);
            min-height: 100vh;
            padding: 2rem;
            line-height: 1.6;
        }}
        .container {{ max-width: 900px; margin: 0 auto; }}
        header {{
            display: flex;
            align-items: center;
            gap: 1rem;
            margin-bottom: 2rem;
            padding-bottom: 1rem;
            border-bottom: 1px solid var(--border-color);
        }}
        h1 {{ font-size: 1.75rem; font-weight: 600; }}
        .bookmark-list {{ display: flex; flex-direction: column; gap: 0.5rem; }}
        .bookmark-item {{
            display: flex;
            align-items: center;
            gap: 1rem;
            padding: 0.75rem 1rem;
            background: var(--surface-color);
            border-radius: 8px;
            border: 1px solid var(--border-color);
            text-decoration: none;
            color: inherit;
            transition: all 0.15s ease;
        }}
        .bookmark-item:hover {{
            border-color: var(--accent-color);
            transform: translateX(4px);
        }}
        .bookmark-icon {{
            width: 32px;
            height: 32px;
            border-radius: 6px;
            background: #f6d32d;
            display: flex;
            align-items: center;
            justify-content: center;
            color: #1c1c1c;
            font-weight: 600;
            font-size: 0.875rem;
            flex-shrink: 0;
        }}
        .bookmark-content {{ flex: 1; min-width: 0; }}
        .bookmark-title {{
            font-weight: 500;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }}
        .bookmark-url {{
            font-size: 0.85rem;
            color: var(--dim-text-color);
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }}
        .empty-state {{
            text-align: center;
            padding: 4rem 2rem;
            color: var(--dim-text-color);
        }}
        .empty-state h2 {{ margin-bottom: 0.5rem; }}
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>⭐ Bookmarks</h1>
        </header>
        <div class="bookmark-list">
            {}
        </div>
    </div>
</body>
</html>"#, items_html)
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
     .replace('\'', "&#39;")
}

fn build_ui(app: &adw::Application) {
    // Initialize database
    let database = match Database::new() {
        Ok(db) => Rc::new(db),
        Err(e) => {
            eprintln!("Failed to initialize database: {}", e);
            // In a real app, we might want to show a dialog or exit
            // For now, we'll proceed but history/bookmarks won't work
            // We can use a dummy implementation or just handle the Option
            return; 
        }
    };

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

    // Bookmark Button (Star)
    let bookmark_btn = Button::from_icon_name("starred-symbolic");
    bookmark_btn.set_tooltip_text(Some("Toggle Bookmark"));
    header.pack_end(&bookmark_btn);

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
        let database = database.clone();
        Rc::new(move |url: Option<&str>| {
            let webview = WebView::new();
            webview.set_hexpand(true);
            webview.set_vexpand(true);
            
            let default_url = "foamium:newtab";
            let url_to_load = url.unwrap_or(default_url);
            
            // Handle special dynamic pages
            if url_to_load == "foamium:history" {
                let html = generate_history_html(&database);
                webview.load_html(&html, Some("foamium:history"));
            } else if url_to_load == "foamium:bookmarks" {
                let html = generate_bookmarks_html(&database);
                webview.load_html(&html, Some("foamium:bookmarks"));
            } else {
                // Resolve custom URIs
                let resolved_url = resolve_uri(url_to_load);
                webview.load_uri(&resolved_url);
            }
            
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
            
            // Update URL bar when URL changes and record history
            let url_entry = url_entry.clone();
            let tab_view = tab_view.clone();
            let page_weak = page.downgrade();
            let database = database.clone();
            
            webview.connect_load_changed(move |wv, load_event| {
                if load_event == webkit6::LoadEvent::Committed {
                    if let Some(uri) = wv.uri() {
                        // Skip internal pages from history
                        let is_internal = uri.contains("/resources/pages/") 
                            || uri.starts_with("foamium:");
                        
                        if !is_internal {
                            // Record in history
                            let title = wv.title()
                                .map(|t| t.to_string())
                                .unwrap_or_else(|| uri.to_string());
                            
                            if let Err(e) = database.add_visit(&uri, &title) {
                                log::warn!("Failed to record history: {}", e);
                            }
                        }
                        
                        // Only update URL bar if this is the currently selected tab
                        if let Some(page) = page_weak.upgrade() {
                            if tab_view.selected_page().as_ref() == Some(&page) {
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

    // Bookmark Button Handler
    let tab_view_clone = tab_view.clone();
    let database_clone = database.clone();
    let bookmark_btn_clone = bookmark_btn.clone();
    bookmark_btn.connect_clicked(move |btn| {
        if let Some(page) = tab_view_clone.selected_page() {
            let child = page.child();
            if let Some(webview) = child.downcast_ref::<WebView>() {
                if let Some(uri) = webview.uri() {
                    // Skip internal pages
                    if uri.contains("/resources/pages/") || uri.starts_with("foamium:") {
                        return;
                    }
                    
                    let title = webview.title()
                        .map(|t| t.to_string())
                        .unwrap_or_else(|| uri.to_string());
                    
                    // Toggle bookmark
                    if let Ok(is_bookmarked) = database_clone.is_bookmarked(&uri) {
                        if is_bookmarked {
                            let _ = database_clone.remove_bookmark(&uri);
                            btn.set_icon_name("non-starred-symbolic");
                        } else {
                            let _ = database_clone.add_bookmark(&uri, &title);
                            btn.set_icon_name("starred-symbolic");
                        }
                    }
                }
            }
        }
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
    let database_clone = database.clone();
    url_entry.connect_activate(move |entry| {
        let input = entry.text();
        
        if let Some(page) = tab_view_clone.selected_page() {
            let child = page.child();
            if let Some(webview) = child.downcast_ref::<WebView>() {
                // Handle dynamic foamium: pages
                if input.as_str() == "foamium:history" {
                    let html = generate_history_html(&database_clone);
                    webview.load_html(&html, Some("foamium:history"));
                    return;
                } else if input.as_str() == "foamium:bookmarks" {
                    let html = generate_bookmarks_html(&database_clone);
                    webview.load_html(&html, Some("foamium:bookmarks"));
                    return;
                }
                
                // Determine if input is a URL or a search query
                let url_str = if input.starts_with("foamium:") {
                    // Custom foamium: URI (newtab, error, warning)
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
                
                webview.load_uri(&url_str);
            }
        }
    });

    // Update URL bar and bookmark button when switching tabs
    let url_entry_clone = url_entry.clone();
    let bookmark_btn_clone = bookmark_btn.clone();
    let database_clone = database.clone();
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
                    } else if uri == "foamium:history" || uri == "foamium:bookmarks" {
                        uri.to_string()
                    } else {
                        uri.to_string()
                    };
                    url_entry_clone.set_text(&display_uri);
                    
                    // Update bookmark button icon
                    let is_internal = uri.contains("/resources/pages/") || uri.starts_with("foamium:");
                    if is_internal {
                        bookmark_btn_clone.set_icon_name("non-starred-symbolic");
                    } else {
                        let is_bookmarked = database_clone.is_bookmarked(&uri).unwrap_or(false);
                        if is_bookmarked {
                            bookmark_btn_clone.set_icon_name("starred-symbolic");
                        } else {
                            bookmark_btn_clone.set_icon_name("non-starred-symbolic");
                        }
                    }
                } else {
                    url_entry_clone.set_text("");
                    bookmark_btn_clone.set_icon_name("non-starred-symbolic");
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
