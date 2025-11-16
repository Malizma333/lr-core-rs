# Line Rider Core (Rust)

An extendable version of the line rider physics engine, written in rust.

## Current Features

- Premade classic physics entities (normal and acceleration lines, rider and sled skeletons, and the rider-skeleton mount)
- Builders for custom line types, skeleton types, and mount types
- Timeline seeking (with iteration and subiteration views available)
- Timeline-based functions to modify physics parameter (such as gravity)
- Test fixtures borrowed from [lr-core-py]() to ensure compatibility with existing engines

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

### Quickstart

Clone this repository

```sh
git clone git@github.com:Malizma333/lr-core-rs.git
```

Install dependencies

```sh
make install
```

Run tests

```sh
make test
```

See other helpful commands

```sh
make help
```

## License

This project is licensed under [GPL v3.0](LICENSE).
