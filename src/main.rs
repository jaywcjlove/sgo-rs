use std::convert::Infallible;
use std::path::Path;
use tokio::fs;
use warp::Filter;
use warp::reply::{html, with_header, Response};
use warp::Reply;  // 导入 Reply trait 以使用 into_response 方法

#[tokio::main]
async fn main() {
    let route = warp::path::tail()
        .and_then(serve_files);

    warp::serve(route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn serve_files(path: warp::path::Tail) -> Result<Response, Infallible> {
    let path_str = path.as_str();
    let base_dir = "./static"; // 静态文件的根目录

    let full_path = Path::new(base_dir).join(path_str);

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