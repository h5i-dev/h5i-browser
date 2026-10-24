# h5i-browser

A lightweight headless browser for coding agents, written in Rust. No Chromium,
no V8.

Headless, but it renders: unlike a DOM-only parser, it rasterises the page to
pixels, so it can take screenshots and stream a live view. The engine is
assembled rather than written — [Blitz](https://github.com/DioxusLabs/blitz)
owns the DOM and drives [Stylo](https://github.com/servo/stylo) for CSS,
`vello_cpu` rasterises on the CPU, and [Boa](https://github.com/boa-dev/boa)
runs page script. Every request is policy-checked and receipted before it
reaches the wire.

## Build

```sh
cargo build --release
```

The binary is `h5i-browser`. Run `h5i-browser --help`, or
`h5i-browser <command> --help`, for the full command line.

## Usage

### Open a page

Load a page and report what it says and what it tried to reach. Nothing remote
is reachable until an origin is granted with `--allow`:

```sh
# Render a local file (loopback and local files need no grant)
h5i-browser open ./page.html

# Fetch a remote page, granting just its origin
h5i-browser open --allow https://example.com https://example.com

# The page as a model should read it, plus a screenshot
h5i-browser open --allow https://example.com --text --screenshot shot.png \
  https://example.com

# Run page script (opt-in), with a request log on disk
h5i-browser open --allow https://example.com --script \
  --receipts requests.jsonl https://example.com
```

### Drive a live session

`serve` holds one page open and streams a live view over WebSocket; `session`
verbs act on that held page (this is the agent-facing half):

```sh
# In one terminal: hold a page open and advertise the control port
h5i-browser serve --allow https://example.com \
  --stream-file /tmp/view.json https://example.com

# In another: read the page, act on it, read it again
h5i-browser session snapshot
h5i-browser session type @e3 "hello"
h5i-browser session submit @e3
h5i-browser session snapshot

# Save what the session did, then replay it against a fresh serve
h5i-browser session script --save run.txt
h5i-browser replay run.txt
```

### Inspect the engine

```sh
# What this engine can and cannot do, as JSON
h5i-browser capabilities

# Fonts, proxy, and what the current policy would allow
h5i-browser doctor --allow https://example.com
```
