use std::convert::Infallible;
use std::path::Path;
use tokio::fs;
use warp::Reply;
use std::sync::Arc;
use tokio_stream::wrappers::ReadDirStream;
use tokio_stream::StreamExt;
use percent_encoding::percent_decode_str;
use mime_guess::mime;

use std::ffi::OsStr;

pub async fn serve_files(
    path: warp::path::Tail,
    css_content: Arc<String>,
    base_dir: Arc<String>,
    enable_cors: bool,
) -> Result<warp::reply::Response, Infallible> {
    let path_str = path.as_str();
    // 解码路径
    let decoded_path_str = percent_decode_str(path_str).decode_utf8_lossy();
    // 转换 Cow<str> 为 &str
    let decoded_path = decoded_path_str.as_ref();
    let full_path = Path::new(&**base_dir).join(decoded_path);

    let response = if full_path.is_dir() {
        match fs::read_dir(full_path).await {
            Ok(entries) => {
                let mut dir_stream = ReadDirStream::new(entries);
                let mut entries_vec: Vec<_> = Vec::new();
                while let Some(entry) = dir_stream.next().await {
                    match entry {
                        Ok(entry) => entries_vec.push(entry),
                        Err(e) => eprintln!("Error reading entry: {}", e),
                    }
                }
                // Sort the entries
                entries_vec.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

                // 调用异步方法并获取结果
                let project_dir_name = get_parent_dir_name(&**base_dir).await;
                let relative_path: String = Path::new(decoded_path).to_str().unwrap_or("").to_string();
                
                // 检查 relative_path 是否为空
                let relative_path = if relative_path.is_empty() {
                    "".to_string()
                } else {
                    relative_path
                };
            
                let mut list = String::new();
                list.push_str("<meta content=\"width=device-width,initial-scale=1.0,minimum-scale=1.0,shrink-to-fit=no\" name=\"viewport\">");
                list.push_str(&format!("<title>Files within {}</title>", relative_path));
                list.push_str(&format!("<style>{}</style>", css_content));
                list.push_str(&format!("<h1><i>Index of&nbsp;</i>{}/{}</h1><ul>", project_dir_name, relative_path));

                // 添加返回上一级目录的链接（如果不是根目录）
                if !path_str.is_empty() {
                    let parent_path: &str = Path::new(path_str).parent().unwrap_or(Path::new(&**base_dir)).to_str().unwrap();
                    list.push_str(&format!("<li><a class=\"folder\" href=\"/{}\">../</a></li>", parent_path));
                }

                // 现在 entries_vec 包含了排序后的目录条目
                for entry in entries_vec {
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
        match fs::read(full_path.clone()).await {
            Ok(content) => {
                let mut mime_type: mime::Mime = mime_guess::from_path(full_path).first_or_octet_stream();
                if mime_type == mime::APPLICATION_OCTET_STREAM && std::str::from_utf8(&content).is_ok() {
                    mime_type = mime::TEXT_PLAIN;
                }
                let is_text = mime_type.to_string().starts_with("text/") || mime_type == mime::APPLICATION_JSON;
                let response = if is_text == true {
                    let content_str = String::from_utf8_lossy(&content).to_string();
                    warp::reply::with_header(content_str, "Content-Type", &format!("{}; charset=utf-8", mime_type.to_string())).into_response()
                } else {
                    warp::reply::with_header(content, "Content-Type", mime_type.to_string()).into_response()
                };
                response.into_response()
            }
            Err(_) => {
                let error_message = "File not found".to_string();
                warp::reply::with_status(error_message, warp::http::StatusCode::NOT_FOUND).into_response()
            }
        }
    };
    let response = if enable_cors {
        let response = warp::reply::with_header(response, "Access-Control-Allow-Origin", "*");
        let response = warp::reply::with_header(response, "Access-Control-Allow-Headers", "*");
        let response = warp::reply::with_header(response, "Access-Control-Allow-Credentials", "true");
        warp::reply::with_header(response, "Access-Control-Allow-Private-Network", "true").into_response()
    } else {
        response
    };

    Ok(response)
}


// MARK: - 获取父目录名称
async fn get_parent_dir_name(base_dir: &str) -> String {
    let current_dir = Path::new(base_dir);
    match fs::canonicalize(current_dir).await {
        Ok(parent_path) => {
            parent_path.file_name().as_deref().unwrap_or_else(|| OsStr::new("")).to_string_lossy().to_string()
        }
        Err(_) => "/".to_string(), // 错误时返回默认值 "/"
    }
}
