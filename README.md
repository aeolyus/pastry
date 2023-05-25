# pastry
A sweet command line tool to interact with multiple pastebin backends.

## Usage
Pastry will read from `stdin` until EOF and upload the input to a
pastebin backend and return the URL of the pastebin link.
```
echo Hello World! | pastry
```
See `pastry --help` for more options


## Install
cargo
```
cargo install --git https://github.com/aeolyus/pastry.git
```
Homebrew
```
brew tap aeolyus/tap
brew install pastry
```
