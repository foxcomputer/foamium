# Foamium Browser

A fast, lightweight, and native web browser for Linux built with Rust.

## Overview

Foamium is a modern web browser that combines the power of WebKit with a beautiful native interface. Built from the ground up using Rust, it delivers a secure and efficient browsing experience that feels right at home on your Linux desktop.

## Release Channels

Foamium is available in three release channels:

| Channel | Description | Stability |
|---------|-------------|-----------|
| **Foamium Nightly** | Daily development builds with cutting-edge features | Experimental |
| **Foamium Beta** | Pre-release builds for testing before stable | Testing |
| **Foamium** | Production-ready stable releases | Stable |

### Nightly Launch

Foamium Nightly is launching today and will receive weekly updates. Download the latest version from the [releases page](https://github.com/foxcomputer/foamium/releases) to get the latest features and bug fixes.

## Features

- **Tabbed Browsing**: Manage multiple web pages with ease using our intuitive tab system
- **Modern Navigation**: Fast and responsive navigation controls (Back, Forward, Reload)
- **Smart Address Bar**: Quickly navigate to any website with site security indicators
- **Site Info Panel**: View connection security status and manage cookies
- **Bookmarks & History**: Persistent bookmarks and browsing history
- **Native Interface**: Beautiful, Adwaita-styled UI that integrates seamlessly with GNOME
- **Performance**: Built with Rust for speed, safety, and efficiency
- **WebKit Engine**: Powered by WebKitGTK 6.0 for full HTML5, CSS3, and JavaScript support

## Installation

### Using the GUI Installer

The easiest way to install Foamium is using the graphical installer:

```bash
# Clone and build
git clone https://github.com/foxcomputer/foamium.git
cd foamium

# Run the Nightly installer (default)
cargo run -p foamium_installer

# Or run the Beta/Stable installer
cargo run -p foamium_installer --no-default-features --features beta
cargo run -p foamium_installer --no-default-features --features stable
```

The installer will:
- Build the release binary
- Install to `/usr/local/bin/`
- Set up desktop integration
- Install the application icon

### Manual Installation

See [Building from Source](#building-from-source) below.

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| **Ctrl+T** | Open new tab |
| **Ctrl+W** | Close current tab |
| **Ctrl+R** or **F5** | Reload current page |
| **Alt+Left** or **Alt+Backspace** | Navigate back |
| **Alt+Right** | Navigate forward |
| **Ctrl+L** | Focus address bar |
| **Ctrl+Tab** | Switch to next tab |
| **Ctrl+Shift+Tab** | Switch to previous tab |
| **Ctrl+H** | Open History |
| **Ctrl+B** | Open Bookmarks |
| **Ctrl+D** | Toggle bookmark for current page |
| **Ctrl+Q** | Quit Foamium |

## Building from Source

### Prerequisites

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
   git clone https://github.com/foxcomputer/foamium.git
   cd foamium
   ```

2. **Build for your desired channel:**
   ```bash
   # Nightly (default)
   cargo build --release -p foamium_app
   
   # Beta
   cargo build --release -p foamium_app --no-default-features --features beta
   
   # Stable
   cargo build --release -p foamium_app --no-default-features --features stable
   ```

3. **Run Foamium:**
   ```bash
   ./target/release/foamium_app
   ```

## Development

Foamium is organized as a Rust workspace:

| Crate | Description |
|-------|-------------|
| **foamium_app** | Main browser application (GTK4/Libadwaita + WebKitGTK) |
| **foamium_installer** | GUI installer for all release channels |
| **foamium_net** | Network utilities (legacy) |
| **foamium_dom** | DOM parser (legacy) |
| **foamium_css** | CSS parser (legacy) |
| **foamium_layout** | Layout engine (legacy) |
| **foamium_render** | Rendering utilities (legacy) |

### Development Status

**Current Version: Alpha**

Foamium is in active development. Core browsing functionality is stable, with new features being added continuously.

### Roadmap to Beta 1.0

**Target: January 2nd, 2026**

- [x] Bookmarks manager
- [x] Browsing history  
- [x] Site security indicators
- [x] Cookie management
- [ ] Download manager
- [ ] Settings and preferences
- [ ] Custom search engines
- [ ] Privacy enhancements

### Contributing

We welcome contributions! Whether it's bug reports, feature requests, or code contributions, feel free to get involved.

## License

[MIT License](LICENSE) - Open source for everyone!

## Contact

Email: hellofox.computer@proton.me

---

**Note**: Foamium is in active development. While we strive for stability, expect occasional bugs and missing features in this alpha release.
