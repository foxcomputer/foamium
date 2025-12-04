# Foamium Branding Assets

This directory contains the branding assets for Foamium Browser.

## Files

- **foamium.svg** - Main Foamium logo (SVG vector format)
- **foamium.png** - Main Foamium logo (PNG raster format, 256x256)
- **fox.svg** - Fox mascot logo (SVG vector format)
- **foxc.png** - Fox mascot logo (PNG raster format)

## Usage

### In HTML Pages
The custom pages in `../pages/` use these assets:
- `blank.html` - Uses `foamium.svg` as the main logo
- `error.html` - Uses `fox.svg` for the error icon
- `warning.html` - Uses `fox.svg` for the warning icon

### In Application
The application uses the icon name `org.foamium.Browser` which should reference these assets when installed.

## Installation

To install the icon for system-wide use:

```bash
# Install to user icon directory
mkdir -p ~/.local/share/icons/hicolor/256x256/apps/
cp foamium.png ~/.local/share/icons/hicolor/256x256/apps/org.foamium.Browser.png

# Install SVG version (scalable)
mkdir -p ~/.local/share/icons/hicolor/scalable/apps/
cp foamium.svg ~/.local/share/icons/hicolor/scalable/apps/org.foamium.Browser.svg

# Update icon cache
gtk-update-icon-cache ~/.local/share/icons/hicolor/
```

For system-wide installation (requires root):
```bash
sudo cp foamium.png /usr/share/icons/hicolor/256x256/apps/org.foamium.Browser.png
sudo cp foamium.svg /usr/share/icons/hicolor/scalable/apps/org.foamium.Browser.svg
sudo gtk-update-icon-cache /usr/share/icons/hicolor/
```

## Desktop Entry

A desktop entry file is provided at `../org.foamium.Browser.desktop` which references the `org.foamium.Browser` icon.

To install:
```bash
# User installation
cp ../org.foamium.Browser.desktop ~/.local/share/applications/

# System-wide installation (requires root)
sudo cp ../org.foamium.Browser.desktop /usr/share/applications/
```
