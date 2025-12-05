use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Label, Orientation, ProgressBar, glib};
use libadwaita as adw;
use adw::prelude::*;
use std::process::Command;
use std::fs;
use std::path::Path;
use std::sync::mpsc;

// Channel-specific configuration
#[cfg(feature = "stable")]
mod channel {
    pub const APP_ID: &str = "org.foamium.Installer";
    pub const APP_NAME: &str = "Foamium";
    pub const BINARY_NAME: &str = "foamium";
    pub const ICON_NAME: &str = "foamium";
    pub const CARGO_FEATURE: &str = "stable";
    pub const DESKTOP_APP_ID: &str = "org.foamium.Browser";
}

#[cfg(feature = "beta")]
mod channel {
    pub const APP_ID: &str = "org.foamium.Installer.Beta";
    pub const APP_NAME: &str = "Foamium Beta";
    pub const BINARY_NAME: &str = "foamium-beta";
    pub const ICON_NAME: &str = "foamium-beta";
    pub const CARGO_FEATURE: &str = "beta";
    pub const DESKTOP_APP_ID: &str = "org.foamium.Browser.Beta";
}

#[cfg(feature = "nightly")]
mod channel {
    pub const APP_ID: &str = "org.foamium.Installer.Nightly";
    pub const APP_NAME: &str = "Foamium Nightly";
    pub const BINARY_NAME: &str = "foamium-nightly";
    pub const ICON_NAME: &str = "foamium-nightly";
    pub const CARGO_FEATURE: &str = "nightly";
    pub const DESKTOP_APP_ID: &str = "org.foamium.Browser.Nightly";
}

#[derive(Clone)]
enum InstallMessage {
    Status(String, f64),
    Success,
    Error(String),
}

fn main() {
    env_logger::init();

    let app = adw::Application::builder()
        .application_id(channel::APP_ID)
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &adw::Application) {
    // Main Window
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title(&format!("{} Installer", channel::APP_NAME))
        .default_width(500)
        .default_height(450)
        .resizable(false)
        .build();

    // Create a navigation view for wizard-style pages
    let nav_view = adw::NavigationView::new();
    
    // Page 1: Welcome
    let welcome_page = create_welcome_page(&nav_view, &window);
    nav_view.add(&welcome_page);
    
    window.set_content(Some(&nav_view));
    window.present();
}

fn create_welcome_page(nav_view: &adw::NavigationView, window: &adw::ApplicationWindow) -> adw::NavigationPage {
    let content = GtkBox::new(Orientation::Vertical, 24);
    content.set_margin_top(48);
    content.set_margin_bottom(48);
    content.set_margin_start(48);
    content.set_margin_end(48);
    content.set_valign(gtk4::Align::Center);
    content.set_halign(gtk4::Align::Center);
    
    // Logo - use channel-specific icon
    let logo_path = std::env::current_dir()
        .ok()
        .and_then(|p| p.join(format!("resources/applogos/{}.svg", channel::ICON_NAME)).to_str().map(String::from));
    
    if let Some(path) = logo_path {
        if Path::new(&path).exists() {
            let logo = gtk4::Picture::for_filename(&path);
            logo.set_size_request(128, 128);
            logo.set_can_shrink(true);
            content.append(&logo);
        }
    }
    
    // Title
    let title = Label::new(Some(&format!("Welcome to {}", channel::APP_NAME)));
    title.add_css_class("title-1");
    content.append(&title);
    
    // Subtitle
    let subtitle = Label::new(Some("Fast, Native, Secure Web Browser"));
    subtitle.add_css_class("dim-label");
    content.append(&subtitle);
    
    // Description
    let desc = Label::new(Some(&format!(
        "This installer will set up {} on your system.\n\nThe following will be installed:",
        channel::APP_NAME
    )));
    desc.set_wrap(true);
    desc.set_justify(gtk4::Justification::Center);
    desc.set_margin_top(16);
    content.append(&desc);
    
    // Installation details
    let details_box = GtkBox::new(Orientation::Vertical, 4);
    details_box.set_margin_top(8);
    details_box.add_css_class("card");
    details_box.set_margin_start(16);
    details_box.set_margin_end(16);
    
    let details = [
        ("Binary", format!("/usr/local/bin/{}", channel::BINARY_NAME)),
        ("Resources", format!("/usr/local/share/{}/", channel::BINARY_NAME)),
        ("Desktop Entry", "~/.local/share/applications/".to_string()),
    ];
    
    for (item, path) in details {
        let row = GtkBox::new(Orientation::Horizontal, 8);
        row.set_margin_start(16);
        row.set_margin_end(16);
        row.set_margin_top(8);
        row.set_margin_bottom(8);
        
        let item_label = Label::new(Some(item));
        item_label.set_hexpand(true);
        item_label.set_halign(gtk4::Align::Start);
        item_label.add_css_class("heading");
        row.append(&item_label);
        
        let path_label = Label::new(Some(&path));
        path_label.add_css_class("dim-label");
        path_label.set_halign(gtk4::Align::End);
        row.append(&path_label);
        
        details_box.append(&row);
    }
    content.append(&details_box);
    
    // Buttons
    let button_box = GtkBox::new(Orientation::Horizontal, 12);
    button_box.set_halign(gtk4::Align::Center);
    button_box.set_margin_top(24);
    
    let cancel_btn = Button::with_label("Cancel");
    cancel_btn.add_css_class("pill");
    let window_clone = window.clone();
    cancel_btn.connect_clicked(move |_| {
        window_clone.close();
    });
    button_box.append(&cancel_btn);
    
    let install_btn = Button::with_label("Install");
    install_btn.add_css_class("suggested-action");
    install_btn.add_css_class("pill");
    
    let nav_view_clone = nav_view.clone();
    let window_clone = window.clone();
    install_btn.connect_clicked(move |_| {
        let progress_page = create_progress_page(&window_clone);
        nav_view_clone.push(&progress_page);
    });
    button_box.append(&install_btn);
    
    content.append(&button_box);
    
    adw::NavigationPage::builder()
        .title("Welcome")
        .child(&content)
        .build()
}

fn create_progress_page(window: &adw::ApplicationWindow) -> adw::NavigationPage {
    let content = GtkBox::new(Orientation::Vertical, 24);
    content.set_margin_top(48);
    content.set_margin_bottom(48);
    content.set_margin_start(48);
    content.set_margin_end(48);
    content.set_valign(gtk4::Align::Center);
    content.set_halign(gtk4::Align::Fill);
    
    // Status icon (spinner initially)
    let spinner = gtk4::Spinner::new();
    spinner.set_size_request(64, 64);
    spinner.start();
    content.append(&spinner);
    
    // Title
    let title = Label::new(Some(&format!("Installing {}...", channel::APP_NAME)));
    title.add_css_class("title-1");
    content.append(&title);
    
    // Progress bar
    let progress = ProgressBar::new();
    progress.set_margin_start(32);
    progress.set_margin_end(32);
    progress.set_show_text(true);
    content.append(&progress);
    
    // Status label
    let status = Label::new(Some("Preparing installation..."));
    status.add_css_class("dim-label");
    status.set_wrap(true);
    content.append(&status);
    
    // Button box (hidden initially, shown on completion)
    let button_box = GtkBox::new(Orientation::Horizontal, 12);
    button_box.set_halign(gtk4::Align::Center);
    button_box.set_margin_top(24);
    button_box.set_visible(false);
    content.append(&button_box);
    
    // Create channel for thread communication
    let (tx, rx) = mpsc::channel::<InstallMessage>();
    
    // Start installation in background thread
    std::thread::spawn(move || {
        run_installation(tx);
    });
    
    // Poll for messages from installation thread
    let title_clone = title.clone();
    let status_clone = status.clone();
    let progress_clone = progress.clone();
    let spinner_clone = spinner.clone();
    let button_box_clone = button_box.clone();
    let window_clone = window.clone();
    let content_clone = content.clone();
    
    glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
        match rx.try_recv() {
            Ok(InstallMessage::Status(msg, frac)) => {
                status_clone.set_text(&msg);
                progress_clone.set_fraction(frac);
                glib::ControlFlow::Continue
            }
            Ok(InstallMessage::Success) => {
                show_success(&title_clone, &status_clone, &progress_clone, &spinner_clone, &button_box_clone, &window_clone);
                glib::ControlFlow::Break
            }
            Ok(InstallMessage::Error(err)) => {
                show_error(&title_clone, &status_clone, &spinner_clone, &content_clone, &window_clone, &err);
                glib::ControlFlow::Break
            }
            Err(mpsc::TryRecvError::Empty) => glib::ControlFlow::Continue,
            Err(mpsc::TryRecvError::Disconnected) => glib::ControlFlow::Break,
        }
    });
    
    adw::NavigationPage::builder()
        .title("Installing")
        .child(&content)
        .can_pop(false)
        .build()
}

fn run_installation(tx: mpsc::Sender<InstallMessage>) {
    let current_dir = std::env::current_dir().unwrap_or_default();
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home".to_string());
    
    // Step 1: Build release binary with correct feature
    let _ = tx.send(InstallMessage::Status(format!("Building {} release binary...", channel::APP_NAME), 0.1));
    
    let build_result = Command::new("cargo")
        .args([
            "build", "--release", "-p", "foamium_app",
            "--no-default-features", "--features", channel::CARGO_FEATURE
        ])
        .current_dir(&current_dir)
        .output();
    
    if !build_result.map(|o| o.status.success()).unwrap_or(false) {
        let _ = tx.send(InstallMessage::Error("Failed to build. Make sure Rust is installed.".to_string()));
        return;
    }
    
    let _ = tx.send(InstallMessage::Status("Build complete!".to_string(), 0.4));
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    // Step 2: Install binary (requires pkexec)
    let _ = tx.send(InstallMessage::Status("Installing binary (requires authentication)...".to_string(), 0.5));
    
    let binary_src = current_dir.join("target/release/foamium_app");
    let binary_dest = format!("/usr/local/bin/{}", channel::BINARY_NAME);
    
    let install_bin = Command::new("pkexec")
        .args(["cp", binary_src.to_str().unwrap_or(""), &binary_dest])
        .output();
    
    if !install_bin.map(|o| o.status.success()).unwrap_or(false) {
        let _ = tx.send(InstallMessage::Error("Failed to install binary. Authentication may have been cancelled.".to_string()));
        return;
    }
    
    // Make executable
    let _ = Command::new("pkexec")
        .args(["chmod", "+x", &binary_dest])
        .output();
    
    // Step 3: Install resources
    let _ = tx.send(InstallMessage::Status("Installing resources...".to_string(), 0.65));
    
    let resources_dest = format!("/usr/local/share/{}", channel::BINARY_NAME);
    let _ = Command::new("pkexec")
        .args(["mkdir", "-p", &resources_dest])
        .output();
    
    // Copy resources directory
    let resources_src = current_dir.join("resources");
    let _ = Command::new("pkexec")
        .args(["cp", "-r", 
               resources_src.join("branding").to_str().unwrap_or(""),
               resources_src.join("fonts").to_str().unwrap_or(""),
               resources_src.join("pages").to_str().unwrap_or(""),
               resources_src.join("applogos").to_str().unwrap_or(""),
               &format!("{}/", resources_dest)])
        .output();
    
    // Step 4: Create desktop entry
    let _ = tx.send(InstallMessage::Status("Creating desktop entry...".to_string(), 0.8));
    
    let desktop_dir = Path::new(&home).join(".local/share/applications");
    let _ = fs::create_dir_all(&desktop_dir);
    
    let desktop_entry = format!(r#"[Desktop Entry]
Name={}
Comment=Fast, Native, Secure Web Browser
Exec={}
Icon={}
Terminal=false
Type=Application
Categories=Network;WebBrowser;
MimeType=text/html;text/xml;application/xhtml+xml;x-scheme-handler/http;x-scheme-handler/https;
StartupWMClass={}
"#, channel::APP_NAME, binary_dest, channel::ICON_NAME, channel::DESKTOP_APP_ID);
    
    let desktop_path = desktop_dir.join(format!("{}.desktop", channel::BINARY_NAME));
    if let Err(e) = fs::write(&desktop_path, desktop_entry) {
        let _ = tx.send(InstallMessage::Error(format!("Failed to create desktop entry: {}", e)));
        return;
    }
    
    // Step 5: Install icon
    let _ = tx.send(InstallMessage::Status("Installing icon...".to_string(), 0.9));
    
    let icon_dir = Path::new(&home).join(".local/share/icons/hicolor/scalable/apps");
    let _ = fs::create_dir_all(&icon_dir);
    
    let icon_src = current_dir.join(format!("resources/applogos/{}.svg", channel::ICON_NAME));
    let icon_dest = icon_dir.join(format!("{}.svg", channel::ICON_NAME));
    let _ = fs::copy(&icon_src, &icon_dest);
    
    // Update caches
    let _ = Command::new("gtk-update-icon-cache")
        .args(["-f", "-t", &format!("{}/.local/share/icons/hicolor", home)])
        .output();
    
    let _ = Command::new("update-desktop-database")
        .arg(&format!("{}/.local/share/applications", home))
        .output();
    
    let _ = tx.send(InstallMessage::Status("Finishing up...".to_string(), 1.0));
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    let _ = tx.send(InstallMessage::Success);
}

fn show_success(
    title: &Label,
    status: &Label,
    progress: &ProgressBar,
    spinner: &gtk4::Spinner,
    button_box: &GtkBox,
    window: &adw::ApplicationWindow,
) {
    spinner.stop();
    spinner.set_visible(false);
    progress.set_fraction(1.0);
    
    title.set_text("Installation Complete!");
    status.set_text(&format!(
        "{} has been successfully installed.\nYou can find it in your application menu.",
        channel::APP_NAME
    ));
    
    // Show buttons
    button_box.set_visible(true);
    
    let close_btn = Button::with_label("Close");
    close_btn.add_css_class("pill");
    let window_clone = window.clone();
    close_btn.connect_clicked(move |_| {
        window_clone.close();
    });
    button_box.append(&close_btn);
    
    let launch_btn = Button::with_label(&format!("Launch {}", channel::APP_NAME));
    launch_btn.add_css_class("suggested-action");
    launch_btn.add_css_class("pill");
    let window_clone = window.clone();
    let binary_path = format!("/usr/local/bin/{}", channel::BINARY_NAME);
    launch_btn.connect_clicked(move |_| {
        let _ = Command::new(&binary_path).spawn();
        window_clone.close();
    });
    button_box.append(&launch_btn);
}

fn show_error(
    title: &Label,
    status: &Label,
    spinner: &gtk4::Spinner,
    content: &GtkBox,
    window: &adw::ApplicationWindow,
    error: &str,
) {
    spinner.stop();
    spinner.set_visible(false);
    
    title.set_text("Installation Failed");
    status.set_text(error);
    
    // Add close button
    let close_btn = Button::with_label("Close");
    close_btn.add_css_class("pill");
    close_btn.set_halign(gtk4::Align::Center);
    close_btn.set_margin_top(24);
    let window_clone = window.clone();
    close_btn.connect_clicked(move |_| {
        window_clone.close();
    });
    content.append(&close_btn);
}
