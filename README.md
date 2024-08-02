[简体中文](./README-zh.md) • <ruby> [sgo](https://github.com/jaywcjlove/sgo) <rt>Nodejs</rt></ruby>

<p align="center">
  <a href="https://github.com/jaywcjlove/sgo-rs">
    <img alt="sgo logo" src="https://github.com/user-attachments/assets/ec07f2ce-03cd-4d04-ab1b-f0bf6cf6d334">
  </a>
</p>

<p align="center">
  <a href="https://github.com/jaywcjlove/sgo-rs/actions/workflows/ci.yml">
    <img alt="CI" src="https://github.com/jaywcjlove/sgo-rs/actions/workflows/ci.yml/badge.svg">
  </a>
  <a href="https://github.com/jaywcjlove/sgo-rs/releases">
    <img alt="Releases" src="https://img.shields.io/github/release/jaywcjlove/sgo-rs.svg">
  </a>
</p>

This is a tool designed to help you serve static websites, single-page applications, or static files, whether they are on your device or on a local network. It is the Rust version of [sgo](https://github.com/jaywcjlove/sgo), rewritten in Rust.

Additionally, it provides a neat interface for listing directory contents:

<img width="557" alt="sgo" src="https://github.com/user-attachments/assets/76797b83-0ff4-45da-bacf-114c1af1f16d">

<br />

### Usage

```sh
sgo -d target -p 3001
```

<br />

### Command help

```sh
$ sgo --help

sgo - Static file serving and directory listing

Usage: sgo [OPTIONS]

Options:
  -d, --dir <DIRECTORY>     Sets the directory to serve files from [default: ./static]
  -p, --port <PORT>         Sets the port number to listen on [default: 3030]
  -L, --no-request-logging  Do not log any request information to the console
  -C, --cors                Enable CORS, sets `Access-Control-Allow-Origin` to `*`
  -h, --help                Print help
  -V, --version             Print version
```

<br />

### Development

```sh
cargo run   # Run the server, open http://127.0.0.1:3030/ in the browser
cargo build # Compile the project
cargo build --release # Compile the release version

cargo build --target aarch64-apple-darwin --release
cargo build --target aarch64-apple-ios --release
cargo build --target aarch64-apple-ios-sim --release
```

Compilation output directory

```rs
└── target
    ├── debug
    │   └── sgo // Binary file output from build
    └── release
        └── sgo // Binary file output from release
```

<br />

### License

MIT © [Kenny Wong](https://wangchujiang.com/)