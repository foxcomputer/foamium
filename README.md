# Foamium Browser

A fast, lightweight, and native web browser for Linux built with Rust.

## Overview

Foamium is a modern web browser that combines the power of WebKit with a beautiful native interface. Built from the ground up using Rust, it delivers a secure and efficient browsing experience that feels right at home on your Linux desktop.

## Features

- **Tabbed Browsing**: Manage multiple web pages with ease using our intuitive tab system
- **Modern Navigation**: Fast and responsive navigation controls (Back, Forward, Reload)
- **Smart Address Bar**: Quickly navigate to any website
- **Native Interface**: Beautiful, native Adwaita-styled UI that integrates seamlessly with your desktop
- **Performance**: Built with Rust for speed, safety, and efficiency
- **WebKit Engine**: Powered by WebKitGTK 6.0 for full HTML5, CSS3, and JavaScript support

## Development Status

**Current Version: Alpha**

Foamium is in active development. While the core browsing functionality is stable, we're continuously adding new features and improvements.

### Roadmap to Beta 1.0

**Beta 1.0 Release Date: January 2nd, 2026**

Planned features to be completed before the launch of Beta 1.0:
- Bookmarks manager
- Browsing history
- Download manager
- Settings and preferences
- Custom search engines
- Privacy and security enhancements

## Keyboard Shortcuts

Foamium supports the following keyboard shortcuts for efficient browsing:

| Shortcut | Action |
|----------|--------|
| **Ctrl+T** | Open new tab |
| **Ctrl+W** | Close current tab |
| **Ctrl+R** or **F5** | Reload current page |
| **Alt+Left** or **Alt+Backspace** | Navigate back |
| **Alt+Right** | Navigate forward |
| **Ctrl+L** | Focus address bar (and select all text) |
| **Ctrl+Tab** | Switch to next tab |
| **Ctrl+Shift+Tab** | Switch to previous tab |
| **Ctrl+Q** | Quit Foamium |

## Building from Source

### Prerequisites

Foamium requires the following dependencies:

**Fedora/RHEL:**
```bash
sudo dnf install gcc gtk4-devel webkit2gtk4.1-devel libadwaita-devel webkitgtk6.0-devel
```

**Debian/Ubuntu:**
```bash
sudo apt install build-essential libgtk-4-dev libwebkit2gtk-4.1-dev libadwaita-1-dev
```

**Arch Linux:**
```bash
sudo pacman -S base-devel gtk4 webkit2gtk-4.1 libadwaita
```

### Build Instructions

1. **Clone the repository:**
   ```bash
   git clone https://github.com/yourusername/foamium.git
   cd foamium
   ```

2. **Build the project:**
   ```bash
   cargo build --release -p foamium_app
   ```

3. **Run Foamium:**
   ```bash
   cargo run --release -p foamium_app
   ```

   Or run the compiled binary directly:
   ```bash
   ./target/release/foamium_app
   ```

4. **(Optional) Install desktop integration:**
   ```bash
   # Install icon
   mkdir -p ~/.local/share/icons/hicolor/scalable/apps/
   cp resources/branding/foamium.svg ~/.local/share/icons/hicolor/scalable/apps/org.foamium.Browser.svg
   
   # Install desktop entry
   cp resources/org.foamium.Browser.desktop ~/.local/share/applications/
   
   # Update caches
   gtk-update-icon-cache ~/.local/share/icons/hicolor/
   update-desktop-database ~/.local/share/applications/
   ```

## Development

Foamium is organized as a Rust workspace with the following structure:

- **foamium_app**: Main browser application (GTK4/Libadwaita UI + WebKitGTK)
- **foamium_net**: Network and HTTP utilities (legacy)
- **foamium_dom**: DOM parser (legacy)
- **foamium_css**: CSS parser (legacy)
- **foamium_layout**: Layout engine (legacy)
- **foamium_render**: Rendering utilities (legacy)

The main application (`foamium_app`) uses WebKitGTK for rendering, while the other crates contain experimental parsing and layout code from earlier development phases.

### Contributing

We welcome contributions! Whether it's bug reports, feature requests, or code contributions, feel free to get involved.

## License

We use the [MIT License](LICENSE). Open source things can be used by everyone!

## Contact

Email: hellofox.computer@proton.me (Proton usage due to security)

---

**Note**: Foamium is in active development. While we strive for stability, expect occasional bugs and missing features in this alpha release.
