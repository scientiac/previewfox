# previewfox

Launch Firefox with no UI chrome, just the content view. Made for live previews.

## Why?

When working on web projects, I want a browser window that shows only the page content without tabs, toolbars, bookmarks, or sidebars getting in the way. A clean content-only view for live-previewing designs, kiosk setups, or embedded displays.

PreviewFox creates a minimal Firefox profile that strips away all UI elements and opens a URL in a clean window. When you close previewfox, Firefox closes with it.

## Usage

```sh
# Open a URL in preview mode:
previewfox http://localhost:3000

# Open the default page (about:logo):
previewfox

# Force-rebuild the profile:
previewfox --rebuild

# Check that all profile files are intact:
previewfox --health

# See detailed output:
previewfox --verbose http://localhost:3000
```

Run `previewfox --help` for all arguments and options.

## Installation

```sh
cargo install previewfox
```

Requires Firefox to be installed and accessible on your PATH.

## License

GPL-3.0
