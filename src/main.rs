use colored::Colorize;
use warp::Filter;
use warp::http::Method;
use std::sync::Arc;

mod file_server; 
mod cli;

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
                    // 打印请求方法
                    println!("{}: {} {}",
                        " HTTP ".on_blue().white().bold(),
                        method.to_string().green(), 
                        if path.as_str().is_empty() { "/".green() } else { path.as_str().green() }
                    );
                }
                file_server::serve_files(path, css_content_arc.clone(), base_dir.clone(), enable_cors.clone())
            }
        });

    // 打印服务器启动信息
    println!(
        "Starting server at http://{}:{}",
        address.0.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(".").on_blue(),
        address.1.to_string().on_blue()
    );

    // MARK: 启动服务器
    warp::serve(route).run(address).await;

}
