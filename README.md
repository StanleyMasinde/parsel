# Parsel

Test APIs from the terminal.

## Motivation

A lot of my work is in the terminal and sometimes Curl is too complex. I built this to allow me to switch into another tmux window for API work.

## MVP Features

* Sends requests
* Displays all responses as plain text
* Supports custom headers and body input
* _Modal interface (like Vim)_

## Installation

Download the appropriate binary from releases:

**Linux (x64)**

```bash
wget https://github.com/StanleyMasinde/parsel/releases/latest/download/parsel-linux-x86_64.tar.gz && tar -xzf parsel-linux-x86_64.tar.gz && rm parsel-linux-x86_64.tar.gz && sudo install -sm 755 parsel /usr/local/bin/
```


**Linux (ARM)**

```bash
wget https://github.com/StanleyMasinde/parsel/releases/latest/download/parsel-linux-arm.tar.gz && tar -xzf parsel-linux-arm.tar.gz && rm parsel-linux-arm.tar.gz && sudo install -sm 755 parsel /usr/local/bin/
```

**macOS (Intel)**

```bash
curl -L https://github.com/StanleyMasinde/parsel/releases/latest/download/parsel-darwin-x86_64.tar.gz | tar -xz && sudo install -sm 755 parsel /usr/local/bin/
```

**macOS (Apple Silicon)**

```bash
curl -L https://github.com/StanleyMasinde/parsel/releases/latest/download/parsel-darwin-aarch64.tar.gz | tar -xz && sudo install -sm 755 parsel /usr/local/bin/
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
