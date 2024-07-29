[简体中文](./README-zh.md)

sgo
===

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