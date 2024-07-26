use std::convert::Infallible;
use std::path::Path;
use tokio::fs;
use warp::Filter;
use warp::reply::{html, with_header, Response};
use warp::Reply;  // 导入 Reply trait 以使用 into_response 方法
use clap::{Arg, Command}; // 更新为 clap 4 的用法

#[tokio::main]
async fn main() {
    // MARK: - 解析命令行参数
    let matches = Command::new("Static File Server")
      .version("1.0")
      .author("Your Name <you@example.com>")
      .about("Serves static files")
      .arg(
          Arg::new("dir")
            .short('d')
            .long("dir")
            .value_name("DIRECTORY")
            .help("Sets the directory to serve files from")
            .default_value("./static"),
      )
      .arg(
          Arg::new("port")
            .short('p')
            .long("port")
            .value_name("PORT")
            .help("Sets the port number to listen on")
            .default_value("3030"),
      )
      .get_matches();

  // 读取命令行参数
  let base_dir: String = matches.get_one::<String>("dir").unwrap().to_string();
  // 读取端口参数，如果无法解析则使用默认值 3030
  let port: u16 = matches.get_one::<String>("port").unwrap().parse().unwrap_or(3030);

  // 设置监听的 IP 和端口
  let address = ([127, 0, 0, 1], port);

  // 创建路由
  let route = warp::path::tail()
      .and_then(move |path| serve_files(path, base_dir.to_string()));

  println!(
      "Starting server at http://{}:{}",
      address.0.iter().map(|b| b.to_string()).collect::<Vec<_>>().join("."),
      address.1
  );

  // 启动服务器
  warp::serve(route)
      .run(address)
      .await;
}

// MARK: - 处理请求
async fn serve_files(path: warp::path::Tail, base_dir: String) -> Result<Response, Infallible> {
    let path_str = path.as_str();
    let full_path = Path::new(&base_dir).join(path_str);
    if full_path.is_dir() {
        match fs::read_dir(full_path).await {
            Ok(mut entries) => {
                let mut list = String::new();
                while let Some(entry) = entries.next_entry().await.unwrap() {
                    let file_name = entry.file_name().into_string().unwrap();
                    list.push_str(&format!("<a href=\"{}\">{}</a><br>", file_name, file_name));
                }
                Ok(html(list).into_response())
            },
            Err(_) => {
                let error_message = "Directory not found".to_string();
                Ok(html(error_message).into_response())
            },
        }
    } else {
        match fs::read(full_path).await {
            Ok(content) => Ok(with_header(content, "Content-Type", "application/octet-stream").into_response()),
            Err(_) => {
                let error_message = "File not found".to_string();
                Ok(html(error_message).into_response())
            },
        }
    }
}