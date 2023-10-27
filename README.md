# gcmh

[![GitHub Actions](https://img.shields.io/github/actions/workflow/status/jwcub/gcmh/rust.yml)](https://github.com/jwcub/gcmh/actions)
[![Crates.io](https://img.shields.io/crates/v/gcmh)](https://crates.io/crates/gcmh)
[![Downloads](https://img.shields.io/crates/d/gcmh)](https://crates.io/crates/gcmh)
[![License](https://img.shields.io/github/license/jwcub/gcmh)](https://github.com/jwcub/gcmh/blob/main/LICENSE)

**G**enerals **C**ustom **M**aps **H**elper

`gcmh` is a CLI tool to improve your custom maps on [generals.io](https://generals.io).

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
  -v, --verbose...           More output per occurrence
  -q, --quiet...             Less output per occurrence
  -h, --help                 Print help
  -V, --version              Print version
```

## Examples

- Search for three maps in keyword `maze`:

    ```sh
    gcmh -m maze -c 3 search
    ```

- Add 1606 upvotes to a map:

    ```sh
    gcmh -m "1*1 Ultimate" -c 1606 upvote
    ```

- Play a map for 161 times:

    ```sh
    gcmh -m "[From Kana]Maze 2" -c 161 play
    ```

## Installation

### Installing from Crates.io (Recommended)

```sh
$ cargo install gcmh
$ gcmh --version
gcmh 1.2.0
```

### Building from Source

```sh
$ git clone https://github.com/jwcub/gcmh
$ cd gcmh
$ cargo build --release
$ ./target/release/gcmh --version
gcmh 1.2.0
```

## License

This project is licensed under the [Unlicense](https://github.com/jwcub/gcmh/blob/main/LICENSE).
