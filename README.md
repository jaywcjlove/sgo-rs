sgo
===

帮助你为静态网站、单页应用程序或静态文件提供服务（无论它们是在你的设备上还是在本地网络上）。它还提供了一个整洁的界面，用于列出目录内容：

![sgo](https://github.com/user-attachments/assets/76797b83-0ff4-45da-bacf-114c1af1f16d)

### Command help

```sh
Usage: sgo [OPTIONS]

Options:
  -d, --dir <DIRECTORY>  Sets the directory to serve files from [default: ./static]
  -p, --port <PORT>      Sets the port number to listen on [default: 3030]
  -h, --help             Print help
  -V, --version          Print version
```

<br />

### 开发

```sh
cargo run   # 运行服务器，在浏览器中打开 http://127.0.0.1:3030/
cargo build # 编译项目
cargo build --release # 编译输出发布版本
```

编译输出目录

```rs
├── README.md
├── src
│   └── main.rs
└── target
    ├── debug
    │   ├── ...
    │   └── sgo // build 输出的二进制文件
    └── release
        ├── ...
        └── sgo // release 输出的二进制文件
```

<br />

### License

MIT © [Kenny Wong](https://wangchujiang.com/)