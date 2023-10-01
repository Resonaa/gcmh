# gcmh

**G**enerals **C**ustom **M**aps **H**elper

`gcmh` is a CLI tool to improve your custom maps on [generals.io](https://generals.io).

**Note: This project is currently work in progress. Some features are missing and will be implemented soon.**

## Usage

```plaintext
$ gcmh --help
A CLI tool to improve your custom maps on generals.io.

Usage: gcmh.exe [OPTIONS] <COMMAND>

Commands:
  search  Search for maps
  upvote  Upvote a map
  play    Play a map
  help    Print this message or the help of the given subcommand(s)

Options:
  -m, --map <MAP>            Map name [default: "1*1 Ultimate"]
  -c, --count <COUNT>        Operations count [default: 10]
  -i, --interval <INTERVAL>  Interval (ms) between two operations [default: 1606]
  -v, --verbose...           More output per occurrence
  -q, --quiet...             Less output per occurrence
  -h, --help                 Print help
  -V, --version              Print version
```

## Examples
- Search for three maps in keyword `maze`:
    ```sh
    $ gcmh -m maze -c 3 search
    ```

- Add 1606 upvotes to a map:
    ```sh
    $ gcmh -m "1*1 Ultimate" -c 1606 upvote
    ```

- Play a map for 161 times:
    ```sh
    $ gcmh -m "[From Kana]Maze 2" -c 161 play
    ```

## Installation

### Install From Cargo (Recommended)
```sh
$ cargo install gcmh
$ gcmh --version
gcmh 0.1.0
```

### Build From Source
```sh
$ git clone https://github.com/jwcub/gcmh
$ cd gcmh
$ cargo build --release
$ ./target/release/gcmh --version
gcmh 0.1.0
```
