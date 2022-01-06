# URL Encoder

A tiny util to URL encode strings given on standard input.

## Installation

```
cargo build --release
cp ./target/release/urlencoder /usr/local/bin
```

## Usage

On my mac I am using a snippet that url encodes a string in clipboard:

```
clippaste | urlencoder | clipcopy
```

As zsh alias

```
# ~/.zshrc
alias clip_url_encode='clippaste | urlencoder | clipcopy'
```
