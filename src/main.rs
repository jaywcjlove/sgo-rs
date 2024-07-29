use std::convert::Infallible;
use std::path::Path;
use colored::Colorize;
use tokio::fs;
use warp::Filter;
use warp::http::Method;
use warp::reply::{html, with_header, Response};
use warp::Reply;
use mime_guess::mime; // 添加 mime_guess 库用于检测文件类型
use std::sync::Arc;
use clap::{Arg, Command};

#[tokio::main]
async fn main() {
    // MARK: - 解析命令行参数
    let matches = Command::new("Static File Server")
      .version("1.0")
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
      .arg(
          Arg::new("no-request-logging")
            .long("no-request-logging")
            .value_name("LOGGING")
            .help("Do not log any request information to the console")
            .action(clap::ArgAction::SetTrue), // Define as a flag that sets the value to false
      )
      .get_matches();

    // 读取命令行参数
    //let no_request_logging = matches.contains_id("no-request-logging");
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
    // 创建路由
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
                serve_files(path, css_content_arc.clone(), base_dir.clone())
            }
        });

    // 打印服务器启动信息
    println!(
        "Starting server at http://{}:{}",
        address.0.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(".").on_blue(),
        address.1.to_string().on_blue()
    );

    // 启动服务器
    warp::serve(route).run(address).await;

}

// MARK: - 处理请求
async fn serve_files(path: warp::path::Tail, css_content: Arc<String>, base_dir: Arc<String>) -> Result<Response, Infallible> {
  let path_str = path.as_str();

  let full_path = Path::new(&**base_dir).join(path_str);
  let full_path_clone = full_path.clone(); // 克隆 PathBuf

  if full_path.is_dir() {
      match fs::read_dir(full_path).await {
          Ok(mut entries) => {
              let relative_path: String = Path::new(path_str).to_str().unwrap_or(&base_dir).to_string();

              // 检查 relative_path 是否为空
              let relative_path = if relative_path.is_empty() {
                base_dir.to_string()
              } else {
                relative_path
              };

              let mut list = String::new();
              // 引入 CSS 文件
              list.push_str("<meta content=\"width=device-width,initial-scale=1.0,minimum-scale=1.0,shrink-to-fit=no\" name=\"viewport\">");
              list.push_str(&format!("<style>{}</style>", css_content));
              list.push_str(&format!("<h1><i>Index of&nbsp;</i>{}</h1><ul>", relative_path));
              
              // 添加返回上一级目录的链接（如果不是根目录）
              if !path_str.is_empty() {
                  let parent_path = Path::new(path_str).parent().unwrap_or(Path::new(&**base_dir)).to_str().unwrap();
                  list.push_str(&format!("<li><a class=\"folder\" href=\"/{}\">../</a></li>", parent_path));
              }

              while let Some(entry) = entries.next_entry().await.unwrap() {
                  let file_name: String = entry.file_name().into_string().unwrap();
                  let entry_path: std::path::PathBuf = entry.path();
                  let relative_path: String = Path::new(path_str).join(&file_name).to_str().unwrap().to_string();
                  
                  if entry_path.is_dir() {
                      list.push_str(&format!("<li><a class=\"folder\" href=\"/{}\">{}/</a></li>", relative_path, file_name));
                  } else {
                      list.push_str(&format!("<li><a class=\"file\" href=\"/{}\">{}</a></li>", relative_path, file_name));
                  }
              }
              list.push_str("</ul>");
              Ok(html(list).into_response())
          },
          Err(_) => {
              let error_message: String = "Directory not found".to_string();
              Ok(html(error_message).into_response())
          },
      }
  } else {
      match fs::read(full_path).await {
          Ok(content) => {
              let mime_type: mime::Mime = mime_guess::from_path(&full_path_clone).first_or_octet_stream();
              if mime_type == mime::TEXT_HTML || mime_type == mime::TEXT_PLAIN || mime_type == mime::TEXT_CSS || mime_type == mime::TEXT_JAVASCRIPT {
                  // 对于文本文件直接展示内容
                  let content_str = String::from_utf8_lossy(&content).to_string();
                  Ok(with_header(content_str, "Content-Type", mime_type.to_string()).into_response())
              } else {
                  // 对于其他文件，提供下载
                  Ok(with_header(content, "Content-Type", mime_type.to_string()).into_response())
              }
          },
          Err(_) => {
              let error_message = "File not found".to_string();
              Ok(html(error_message).into_response())
          },
      }
  }
}
