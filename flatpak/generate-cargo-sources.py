#!/usr/bin/env python3
"""
Generate cargo-sources.json for Flatpak builds.
This script creates a JSON file containing all Cargo dependencies
that can be used by flatpak-builder for offline builds.
"""

import json
import subprocess
import sys

def main():
    # Run cargo fetch to ensure all dependencies are downloaded
    subprocess.run(["cargo", "fetch"], check=True)
    
    # Get cargo metadata
    result = subprocess.run(
        ["cargo", "metadata", "--format-version=1"],
        capture_output=True,
        text=True,
        check=True
    )
    
    metadata = json.loads(result.stdout)
    sources = []
    
    # For now, create an empty sources file
    # In production, you'd use flatpak-cargo-generator
    # pip install flatpak-cargo-generator
    # flatpak-cargo-generator Cargo.lock -o flatpak/cargo-sources.json
    
    print("Note: For full Flatpak builds, run:")
    print("  pip install flatpak-cargo-generator")
    print("  flatpak-cargo-generator Cargo.lock -o flatpak/cargo-sources.json")
    
    with open("flatpak/cargo-sources.json", "w") as f:
        json.dump(sources, f, indent=2)
    
    print("Created empty flatpak/cargo-sources.json")
    print("Run flatpak-cargo-generator for proper offline builds.")

if __name__ == "__main__":
    main()
