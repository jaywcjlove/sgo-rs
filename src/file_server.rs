use std::convert::Infallible;
use std::path::Path;
use tokio::fs;
use warp::Reply;
use std::sync::Arc;
use mime_guess::mime;

pub async fn serve_files(
  path: warp::path::Tail,
  css_content: Arc<String>,
  base_dir: Arc<String>,
  enable_cors: bool,
) -> Result<warp::reply::Response, Infallible> {
  let path_str = path.as_str();

  let full_path = Path::new(&**base_dir).join(path_str);
  let full_path_clone = full_path.clone(); // 克隆 PathBuf

  let response = if full_path.is_dir() {
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
              list.push_str("<meta content=\"width=device-width,initial-scale=1.0,minimum-scale=1.0,shrink-to-fit=no\" name=\"viewport\">");
              list.push_str(&format!("<title>Files within {}</title>", relative_path));
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
              warp::reply::html(list).into_response()
          }
          Err(_) => {
              let error_message: String = "Directory not found".to_string();
              warp::reply::html(error_message).into_response()
          }
      }
  } else {
      match fs::read(full_path).await {
          Ok(content) => {
              let mime_type: mime::Mime = mime_guess::from_path(&full_path_clone).first_or_octet_stream();
              if mime_type == mime::TEXT_HTML || mime_type == mime::TEXT_PLAIN || mime_type == mime::TEXT_CSS || mime_type == mime::TEXT_JAVASCRIPT {
                  let content_str = String::from_utf8_lossy(&content).to_string();
                  warp::reply::with_header(content_str, "Content-Type", mime_type.to_string()).into_response()
              } else {
                  warp::reply::with_header(content, "Content-Type", mime_type.to_string()).into_response()
              }
          }
          Err(_) => {
              let error_message = "File not found".to_string();
              warp::reply::html(error_message).into_response()
          }
      }
  };

  let response = if enable_cors {
      warp::reply::with_header(response, "Access-Control-Allow-Origin", "*").into_response()
  } else {
      response
  };

  Ok(response)
}