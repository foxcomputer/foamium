# Foamium Engine Walkthrough

We have started building the **Foamium Browser Engine** from scratch in Rust.

## Architecture: The "Brick by Brick" Approach

The project is a **Rust Workspace** containing multiple crates, each responsible for a specific part of the browser pipeline:

| Crate                | Responsibility                              | Status          |
| :------------------- | :------------------------------------------ | :-------------- |
| **`foamium_app`**    | The main application window and event loop. | ✅ Basic Window |
| **`foamium_net`**    | Networking layer (fetching URLs).           | ✅ HTTP GET     |
| **`foamium_dom`**    | HTML Parsing and DOM Tree.                  | 🚧 Planned      |
| **`foamium_css`**    | CSS Parsing and Style Calculation.          | 🚧 Planned      |
| **`foamium_layout`** | Layout algorithms (Box Model, Flexbox).     | 🚧 Planned      |
| **`foamium_render`** | Painting to the screen (WebGPU).            | 🚧 Planned      |

## Current Capability: "View Source"

Currently, Foamium can:

1.  Launch a native window using `winit`.
2.  Fetch the HTML source code of `https://example.com` using `foamium_net`.
3.  Print the raw HTML to the console.

## How to Run

```bash
cargo run -p foamium_app
```

## Next Steps (Phase 3)

The next major milestone is **Rendering Text**. We will:

1.  Implement a basic HTML parser in `foamium_dom`.
2.  Build a simple Render Tree.
3.  Draw the text to the window using `wgpu`.
