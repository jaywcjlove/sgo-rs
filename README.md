SGO
===


## 运行服务器

```sh
cargo run 
```

在浏览器中打开 http://127.0.0.1:3030/

## 编译项目

```sh
cargo build
```

编译输出发布版本

```sh
cargo build --release
```

编译输出目录

```rs
├── README.md
├── src
│   └── main.rs
└── target
    ├── CACHEDIR.TAG
    ├── debug
    │   ├── ...
    │   └── sgo // build 输出的二进制文件
    └── release
        ├── ...
        └── sgo // release 输出的二进制文件
```