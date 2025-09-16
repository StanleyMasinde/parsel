# Parsel

Test APIs from the terminal.

## Motivation

A lot of my work is in the terminal and sometimes Curl is too complex. I built this to allow me to switch into another tmux window for API work.

## MVP Features

* Sends GET requests only
* Displays all responses as plain text
* TUI-based interface for sending requests and viewing responses
* Supports custom headers and body input
* **Modal interface (like Vim)**

## Installation

Download the appropriate binary from releases:

**Linux (x64)**

```bash
wget https://github.com/StanleyMasinde/parsel/releases/latest/download/parsel-linux-x64.tar.gz && tar -xzf parsel-linux-x64.tar.gz && rm parsel-linux-x64.tar.gz
sudo mv parsel /usr/local/bin/
sudo chmod +x /usr/local/bin/parsel
```

**Linux (x64 Static)**

```bash
wget https://github.com/StanleyMasinde/parsel/releases/latest/download/parsel-linux-x64-static.tar.gz && tar -xzf parsel-linux-x64.tar.gz && rm parsel-linux-x64.tar.gz
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

```bash
parsel
```

Parsel uses **modal editing**:

* Use `Tab` to switch between sections.
* In the **Request tab** (default):

  * `m` cycles HTTP method forward, `M` cycles back.
  * `Enter` sends the request.
* In **Query Params** or **Headers** tab:

  * `i` enters edit mode
  * `a` adds a new item
  * Type the key → `Tab` → type the value → `Return` to save
  * `D` deletes the selected item in normal mode
* In the **URL** or **Request Body** tab:

  * `i` to enter edit mode, type your input, `Esc` to go back to normal mode

## Contributing

Not accepting contributions until there is a stable codebase. For now, please submit issues only.

## License

MIT
