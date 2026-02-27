# parsel

parsel is a terminal UI HTTP client for crafting requests and inspecting responses.

## Install

```sh
curl -fsSL https://raw.githubusercontent.com/StanleyMasinde/parsel/main/install.sh | sh
```

Optional: install a specific version

```sh
curl -fsSL https://raw.githubusercontent.com/StanleyMasinde/parsel/main/install.sh | sh -s v1.2.3
```

Install to a custom directory

```sh
curl -fsSL https://raw.githubusercontent.com/StanleyMasinde/parsel/main/install.sh | PARSEL_INSTALL=~/.local/bin sh
```

## Usage

Run:

```sh
parsel
```

Or start with a URL pre-filled from the first positional argument:

```sh
parsel https://httpbin.org/get
```

Only the first command-line argument is used as the initial URL.

Workflow:
- Enter a URL and configure the request on the left panels.
- Press Enter to send the request and view the response on the right panels.

## Keybindings

Global (Normal mode):
- `Tab` / `Shift+Tab`: next/previous panel
- `Enter`: send request
- `q`: quit

Editing:
- `i`: enter Edit mode on URL, Query, Headers, or Body panels
- `Esc`: back to Normal mode
- `Enter`: send request when editing URL
- `Ctrl+Enter`: send request from any edit panel

Request helpers:
- `m` / `M`: next/previous HTTP method
- `b` / `B`: next/previous body mode

Response:
- `j` / `k`: scroll response body down/up

## Request input format

Query params, headers, and body inputs are entered as `key: value` pairs, one per line.

Examples:

```text
token: 123
page: 2
```

Body modes:
- JSON: values are encoded into a flat JSON object
- Form: values are encoded as `application/x-www-form-urlencoded`

Header notes:
- Brotli workaround: parsel parses `Accept-Encoding` (including `q=` weights) and enables Brotli decompression only when `br` is the highest-preference encoding.
- Enabling Brotli disables `gzip` and `deflate` for that request.
- This is a workaround because Rust libcurl does not support Brotli by default.

## Build from source

```sh
cargo build --release
```

Binary will be at:

```sh
target/release/parsel
```

## License

MIT License. See `LICENSE`.
