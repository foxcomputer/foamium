# Foamium Custom Pages

This directory contains custom HTML pages for Foamium browser that follow the Adwaita design system.

## Pages

### blank.html
The default new tab page featuring:
- Foamium branding with wave emoji (🌊)
- Clean, minimal design
- Keyboard shortcut hint (Ctrl+L)
- Automatic dark/light theme support

### error.html
Displayed when a website cannot be found or fails to load:
- Clear error messaging
- Failed URL display
- Suggestions for troubleshooting
- "Go Back" and "Try Again" buttons
- Automatic dark/light theme support

### warning.html
Security warning page for SSL/TLS certificate issues:
- Prominent warning icon and messaging
- Details about the security issue
- "Go Back to Safety" (recommended) button
- "Proceed Anyway" (unsafe) button with warning
- Information about why the warning is shown
- Automatic dark/light theme support

## Theme Support

All pages automatically adapt to the system theme using CSS `prefers-color-scheme`:
- **Light theme**: Clean whites and grays with blue accents
- **Dark theme**: Dark backgrounds with lighter text and adjusted colors
- **System**: Follows the OS/desktop environment theme preference

## Design System

The pages follow the Adwaita design language:
- **Typography**: Cantarell font family (GNOME default)
- **Colors**: Adwaita color palette
  - Light: `#3584e4` (blue), `#e01b24` (red), `#f6d32d` (yellow)
  - Dark: `#62a0ea` (blue), `#ff6b6b` (red), `#f8e45c` (yellow)
- **Spacing**: Consistent padding and margins
- **Borders**: Rounded corners (6-12px radius)
- **Shadows**: Subtle elevation effects
- **Buttons**: Pill-shaped with hover effects

## Usage

These pages are loaded automatically by the browser:
- `blank.html`: Loaded for new tabs
- `error.html`: Loaded when navigation fails (with `?url=` parameter)
- `warning.html`: Loaded for security warnings (with `?reason=` parameter)

## Customization

To customize these pages:
1. Edit the HTML/CSS directly
2. Maintain the CSS variables for theme support
3. Keep the Adwaita design principles
4. Test in both light and dark modes
