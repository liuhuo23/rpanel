use crate::{base::Response, error::AppError};
use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use actix_web::{HttpResponse, Responder, web};
use actix_web::{get, post};
use log::info;
use std::fs::create_dir;
use std::path::Path;
use uuid;

#[get("/{path}")]
async fn get_image(path: web::Path<String>) -> Result<impl Responder, AppError> {
    let path = path.into_inner();
    let img_root = Path::new("/mnt/Leven/img");
    let image_path = img_root.join(&path);
    if image_path.exists() == false || image_path.is_file() == false {
        return Err(AppError::NotFound("图片不存在".into()));
    } else {
        let image_type = image_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        return Ok(HttpResponse::Ok()
            .content_type(format!("image/{}", image_type))
            .body(std::fs::read(image_path).map_err(AppError::Io)?));
    }
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

#[post("/")]
async fn create_image(form: MultipartForm<UploadForm>) -> Result<impl Responder, AppError> {
    let img_root = Path::new("/mnt/Leven/img");
    if img_root.exists() == false {
        create_dir(img_root).map_err(AppError::Io)?;
        info!("Created image root directory: {:?}", img_root);
    }

    return save_files(form).await;
}

async fn save_files(
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<impl Responder, AppError> {
    let mut file_paths = Vec::<String>::new();
    for f in form.files {
        let file_name = f.file_name.clone().unwrap_or_else(|| "unknown".into());
        let file_ext = file_name
            .rsplit('.')
            .next()
            .map(|s| s.to_lowercase())
            .unwrap_or_else(|| "bin".into());
        if !["png", "jpg", "jpeg", "gif", "bmp"].contains(&file_ext.as_str()) {
            return Err(AppError::InvalidParam(format!(
                "Unsupported file type: {}",
                file_ext
            )));
        }
        let file_name = uuid::Uuid::new_v4().to_string() + "." + &file_ext;
        let path = Path::new("/mnt/Leven/img").join(&file_name);
        file_paths.push(file_name);
        f.file
            .persist(&path)
            .map_err(|e| AppError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
    }

    Ok(Response::new(Some(file_paths), "Success".into(), 0))
}
