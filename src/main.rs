use std::convert::Infallible;
use std::path::Path;
use tokio::fs;
use warp::Filter;
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
  let base_dir = Arc::new(matches.get_one::<String>("dir").unwrap().to_string());
  let port: u16 = matches
      .get_one::<String>("port")
      .unwrap()
      .parse()
      .unwrap_or(3030);
    
  // 设置监听的 IP 和端口
  let address = ([127, 0, 0, 1], port);
  // 创建路由
  let route = warp::path::tail()
      .and_then({
          let base_dir = base_dir.clone();
          move |path| serve_files(path, base_dir.clone())
      });

  // 打印服务器启动信息
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

async fn serve_files(path: warp::path::Tail, base_dir: Arc<String>) -> Result<Response, Infallible> {
  let path_str = path.as_str();

  // 如果请求的是 CSS 文件，返回指定路径的 CSS 文件内容
  if path_str == "_.sgo_style.css" {
    let css_content = include_str!("style.css");
    return Ok(with_header(css_content, "Content-Type", "text/css").into_response());
  }

  let full_path = Path::new(&**base_dir).join(path_str);
  let full_path_clone = full_path.clone(); // 克隆 PathBuf

  if full_path.is_dir() {
      match fs::read_dir(full_path).await {
          Ok(mut entries) => {

              let relative_path = Path::new(path_str)
                  .to_str().unwrap_or(&base_dir).to_string();


              // 检查 relative_path 是否为空
              let relative_path = if relative_path.is_empty() {
                base_dir.to_string()
              } else {
                relative_path
              };

              let mut list = String::new();
              // 引入 CSS 文件
              list.push_str("<meta content=\"width=device-width,initial-scale=1.0,minimum-scale=1.0,shrink-to-fit=no\" name=\"viewport\">");
              list.push_str("<link rel=\"stylesheet\" type=\"text/css\" href=\"/_.sgo_style.css\">");
              list.push_str(&format!("<h1><i>Index of&nbsp;</i>{}</h1><ul>", relative_path));
              
              // 添加返回上一级目录的链接（如果不是根目录）
              if !path_str.is_empty() {
                  let parent_path = Path::new(path_str).parent().unwrap_or(Path::new(&**base_dir)).to_str().unwrap();
                  list.push_str(&format!("<li><a class=\"folder\" href=\"/{}\">../</a></li>", parent_path));
              }

              while let Some(entry) = entries.next_entry().await.unwrap() {
                  let file_name = entry.file_name().into_string().unwrap();
                  let entry_path = entry.path();
                  let relative_path = Path::new(path_str).join(&file_name).to_str().unwrap().to_string();
                  
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
              let error_message = "Directory not found".to_string();
              Ok(html(error_message).into_response())
          },
      }
  } else {
      match fs::read(full_path).await {
          Ok(content) => {
              let mime_type = mime_guess::from_path(&full_path_clone).first_or_octet_stream();
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


// async fn serve_files(path: warp::path::Tail, base_dir: Arc<String>) -> Result<Response, Infallible> {
//   let path_str = path.as_str();
//   let full_path = Path::new(&**base_dir).join(path_str);
//   //let full_path_clone = full_path.clone(); // 克隆 PathBuf

//   if full_path.is_dir() {
//       match fs::read_dir(full_path).await {
//           Ok(mut entries) => {
//               let mut list = String::new();
//               list.push_str("<h1>Directory Listing</h1><ul>");
              
//               // 添加返回上一级目录的链接（如果不是根目录）
//               if !path_str.is_empty() {
//                   let parent_path = path_str.rsplit('/').skip(1).collect::<Vec<&str>>().join("/");
//                   list.push_str(&format!("<li><a href=\"{}\">../</a></li>", parent_path));
//               }

//               while let Some(entry) = entries.next_entry().await.unwrap() {
//                   let file_name = entry.file_name().into_string().unwrap();
//                   let entry_path = entry.path();
//                   let relative_path = format!("{}/{}", path_str, file_name);
                  
//                   if entry_path.is_dir() {
//                       list.push_str(&format!("<li><strong><a href=\"{}\">{}/</a></strong></li>", relative_path, file_name));
//                   } else {
//                       list.push_str(&format!("<li><a href=\"{}\">{}</a></li>", relative_path, file_name));
//                   }
//               }
//               list.push_str("</ul>");
//               Ok(html(list).into_response())
//           },
//           Err(_) => {
//               let error_message = "Directory not found".to_string();
//               Ok(html(error_message).into_response())
//           },
//       }
//   } else {
//       let full_path_clone = full_path.clone(); // 克隆 PathBuf
//       match fs::read(full_path).await {
//           Ok(content) => {
//               let mime_type = mime_guess::from_path(&full_path_clone).first_or_octet_stream();
//               if mime_type == mime::TEXT_HTML || mime_type == mime::TEXT_PLAIN || mime_type == mime::TEXT_CSS || mime_type == mime::TEXT_JAVASCRIPT {
//                   // 对于文本文件直接展示内容
//                   Ok(with_header(content, "Content-Type", mime_type.to_string()).into_response())
//               } else {
//                   // 对于其他文件，提供下载
//                   Ok(with_header(content, "Content-Type", mime_type.to_string()).into_response())
//               }
//           },
//           Err(_) => {
//               let error_message = "File not found".to_string();
//               Ok(html(error_message).into_response())
//           },
//       }
//   }
// }


// async fn serve_files(path: warp::path::Tail, base_dir: Arc<String>) -> Result<Response, Infallible> {
//   let path_str = path.as_str();
//   let full_path = Path::new(&**base_dir).join(path_str);

//   if full_path.is_dir() {
//       match fs::read_dir(full_path).await {
//           Ok(mut entries) => {
//               let mut list = String::new();
//               list.push_str("<h1>Directory Listing</h1><ul>");
//               while let Some(entry) = entries.next_entry().await.unwrap() {
//                   let file_name = entry.file_name().into_string().unwrap();
//                   let entry_path = entry.path();
//                   if entry_path.is_dir() {
//                       list.push_str(&format!("<li><strong><a href=\"{}\">{}/</a></strong></li>", file_name, file_name));
//                   } else {
//                       list.push_str(&format!("<li><a href=\"{}\">{}</a></li>", file_name, file_name));
//                   }
//               }
//               list.push_str("</ul>");
//               Ok(html(list).into_response())
//           },
//           Err(_) => {
//               let error_message = "Directory not found".to_string();
//               Ok(html(error_message).into_response())
//           },
//       }
//   } else {
//       let full_path_clone = full_path.clone(); // 克隆 PathBuf
//       match fs::read(full_path).await {
//           Ok(content) => {
//               let mime_type = mime_guess::from_path(&full_path_clone).first_or_octet_stream();
//               if mime_type == mime::TEXT_HTML || mime_type == mime::TEXT_PLAIN || mime_type == mime::TEXT_CSS || mime_type == mime::TEXT_JAVASCRIPT {
//                   // 对于文本文件直接展示内容
//                   Ok(with_header(content, "Content-Type", mime_type.to_string()).into_response())
//               } else {
//                   // 对于其他文件，提供下载
//                   Ok(with_header(content, "Content-Type", mime_type.to_string()).into_response())
//               }
//           },
//           Err(_) => {
//               let error_message = "File not found".to_string();
//               Ok(html(error_message).into_response())
//           },
//       }
//   }
// }


// async fn serve_files(path: warp::path::Tail, base_dir: String) -> Result<Response, Infallible> {
//     let path_str = path.as_str();
//     let full_path = Path::new(&base_dir).join(path_str);
//     if full_path.is_dir() {
//         match fs::read_dir(full_path).await {
//             Ok(mut entries) => {
//                 let mut list = String::new();
//                 while let Some(entry) = entries.next_entry().await.unwrap() {
//                     let file_name = entry.file_name().into_string().unwrap();
//                     list.push_str(&format!("<a href=\"{}\">{}</a><br>", file_name, file_name));
//                 }
//                 Ok(html(list).into_response())
//             },
//             Err(_) => {
//                 let error_message = "Directory not found".to_string();
//                 Ok(html(error_message).into_response())
//             },
//         }
//     } else {
//         match fs::read(full_path).await {
//             Ok(content) => Ok(with_header(content, "Content-Type", "application/octet-stream").into_response()),
//             Err(_) => {
//                 let error_message = "File not found".to_string();
//                 Ok(html(error_message).into_response())
//             },
//         }
//     }
// }