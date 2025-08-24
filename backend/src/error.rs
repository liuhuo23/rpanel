use thiserror::Error;

use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("无效参数: {0}")]
    InvalidParam(String),

    #[error("未授权访问")]
    Unauthorized,

    #[error("找不到资源: {0}")]
    NotFound(String),

    #[error("未知错误: {0}")]
    Unknown(String),
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InvalidParam(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Unknown(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(crate::base::Response::<()> {
            data: None,
            msg: self.to_string(),
            code: self.status_code().as_u16(),
        })
    }
}

impl From<anyhow::Error> for AppError {
    fn from(e: anyhow::Error) -> Self {
        AppError::Unknown(e.to_string())
    }
}
