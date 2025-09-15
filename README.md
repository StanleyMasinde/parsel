# Parsel

Test APIs from the terminal.

## Motivation

A lot of my work is in the terminal and sometimes Curl is too complex. I built this to allow me to switch into another tmux window for API work.

## MVP Features

* Sends GET requests only
* Displays all responses as plain text
* TUI-based interface for sending requests and viewing responses
* Supports custom headers and body input

## Installation

Download the appropriate binary from releases:

**Linux (x64)**

```bash
wget https://github.com/StanleyMasinde/parsel/releases/latest/download/parsel-linux-x64.tar.gz && tar -xzf parsel-linux-x64.tar.gz && rm parsel-linux-x64.tar.gz
sudo mv parsel /usr/local/bin/
sudo chmod +x /usr/local/bin/parsel
```

**macOS (Intel)**

```bash
curl -L https://github.com/StanleyMasinde/parsel/releases/latest/download/parsel-darwin-x64.tar.gz | tar -xz
sudo mv parsel /usr/local/bin/
sudo chmod +x /usr/local/bin/parsel
```

**macOS (Apple Silicon)**

```bash
curl -L https://github.com/StanleyMasinde/parsel/releases/latest/download/parsel-darwin-arm64.tar.gz | tar -xz
sudo mv parsel /usr/local/bin/
sudo chmod +x /usr/local/bin/parsel
```

## Usage

Run Parsel from your terminal:

```bash
parsel
```

Navigate the TUI to select endpoints, send GET requests, and view responses as text.

## Contributing

Not accepting contributions until there is a stable codebase. For now, please submit issues only.

## License

MIT
