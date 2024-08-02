use colored::Colorize;
use warp::Filter;
use warp::http::Method;
use std::sync::Arc;
use percent_encoding::percent_decode_str;
use tokio::sync::oneshot;
use tokio::signal::ctrl_c;

mod file_server; 
mod cli;
mod utils;

#[tokio::main]
async fn main() {
    // MARK: - 解析命令行参数
    let matches = cli::get_matches();

    // 读取命令行参数
    let enable_cors = matches.get_flag("cors");
    let no_request_logging = matches.get_flag("no-request-logging");
    let base_dir = Arc::new(matches.get_one::<String>("dir").unwrap().to_string());
    let port: u16 = matches
        .get_one::<String>("port")
        .unwrap()
        .parse()
        .unwrap_or(3030);

    if let Err(err) = utils::manage_port(port).await {
        eprintln!("  Failed to manage port {}: {}", port, err.to_string().red());
    }

    // 设置监听的 IP 和端口
    let address = ([127, 0, 0, 1], port);
    // 将 CSS 文件内容嵌入到二进制中
    let css_content = include_bytes!("./style.css");
    // 将字节数组转换为字符串（假设 CSS 文件是有效的 UTF-8 编码）
    let css_content_str = String::from_utf8_lossy(css_content);
    // 将内容转换为 String 并包装到 Arc
    let css_content_arc = Arc::new(css_content_str.to_string());
    // MARK: 创建路由
    let route = warp::path::tail()
        .and(warp::method())
        .and_then({
            let base_dir = base_dir.clone();
            let css_content_arc = css_content_arc.clone();
            move |path: warp::path::Tail, method: Method| {
                if !no_request_logging {
                    // 解码路径
                   let request_url = percent_decode_str(path.as_str()).decode_utf8_lossy();
                    // 打印请求方法
                    println!("{}: {} {}",
                        " HTTP ".on_blue().white().bold(),
                        method.to_string().green(), 
                        if path.as_str().is_empty() { "/".green() } else { request_url.to_string().green() }
                    );
                }
                file_server::serve_files(path, css_content_arc.clone(), base_dir.clone(), enable_cors.clone())
            }
        });

    // 创建一个通道，用于发送和接收关闭信号
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    // 创建服务器并绑定地址，同时启动优雅关闭
    let (server_addr, server) = warp::serve(route)
        .bind_with_graceful_shutdown(address, async {
            shutdown_rx.await.ok();
        });

    println!("\n  Server running on {}{}\n", "http://".on_blue(), server_addr.to_string().on_blue());
    // 启动服务器
    tokio::spawn(server);

    // 捕捉 CTRL+C 信号
    ctrl_c().await.expect(&"failed to listen for event".to_string().as_str().red());

    // 发送关闭信号
    let _ = shutdown_tx.send(());

    println!("{}", "\n\n  Server has been shutdown gracefully\n\n".yellow());

}
