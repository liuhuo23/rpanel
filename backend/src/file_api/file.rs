use actix_web::{HttpResponse, Responder, get, web};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::error::AppError;

#[derive(Deserialize)]
struct Info {
    dir: String,
    page: Option<u32>,
    page_size: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileInfo {
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
    pub created_at: u64,
    pub modified_at: u64,
}

#[get("/")]
pub async fn file_list(info: web::Query<Info>) -> Result<impl Responder, AppError> {
    let dir = info.dir.clone();
    let page = info.page.unwrap_or(1);
    let page_size = info.page_size.unwrap_or(10);
    let root = Path::new("/home");
    let path = root.join(dir);
    if path.exists() == false || path.is_dir() == false {
        return Err(AppError::NotFound("目录不存在".into()));
    }
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let meta = entry.metadata();
            let name = entry.file_name().to_string_lossy().to_string();
            let (size, is_dir, created_at, modified_at) = if let Ok(meta) = meta {
                let size = if meta.is_file() { meta.len() } else { 0 };
                let is_dir = meta.is_dir();
                let created_at = meta
                    .created()
                    .ok()
                    .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                    .map(|d| d.as_secs())
                    .unwrap_or(0);
                let modified_at = meta
                    .modified()
                    .ok()
                    .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                    .map(|d| d.as_secs())
                    .unwrap_or(0);
                (size, is_dir, created_at, modified_at)
            } else {
                return Err(AppError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "无法读取文件元数据",
                )));
            };
            files.push(FileInfo {
                name,
                size,
                is_dir,
                created_at,
                modified_at,
            });
        }
    }
    Ok(crate::base::Response::new(Some(files), "Success".into(), 0))
}
