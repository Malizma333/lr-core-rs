# Line Rider Format Converter

> [!WARNING]
> This project is in an unfinished state. Use at your own risk.

A CLI for converting between Line Rider track formats, written in rust.

## Quickstart

```bash
# MacOS/Linux
cargo build --release --target-dir target
target/release/format_converter ./samples/HAM.trk json
target/release/format_converter ./samples/Silk_Road.sol json
target/release/format_converter ./samples/Omniverse2.track.json sol
```