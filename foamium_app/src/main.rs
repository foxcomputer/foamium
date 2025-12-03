use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Entry, Orientation, glib};
use libadwaita as adw;
use adw::prelude::*;
use webkit6::prelude::*;
use webkit6::WebView;
use std::rc::Rc;

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
    
    // Helper to create a new tab
    let create_tab = {
        let tab_view = tab_view.clone();
        Rc::new(move |url: Option<&str>| {
            let webview = WebView::new();
            webview.set_hexpand(true);
            webview.set_vexpand(true);
            
            let url_to_load = url.unwrap_or("about:blank");
            webview.load_uri(url_to_load);
            
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
            
            // Update URL bar when switching to this tab or when URL changes
            // Note: This is a simplified implementation. In a real app, we'd need to track the active tab more carefully.
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
        let url = entry.text();
        let url_str = if url.contains("://") {
            url.to_string()
        } else {
            format!("https://{}", url)
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
                    url_entry_clone.set_text(&uri);
                } else {
                    url_entry_clone.set_text("");
                }
            }
        }
    });
    
    // Update URL bar when current tab navigates
    // This requires a bit more complex signal handling which we'll simplify for now by just updating on switch
    // Ideally we'd connect to load-changed on every webview and check if it's the active one.

    window.present();
}
