// backend 库主入口
mod base;
mod error;
mod file_api;
mod img_api;
mod middleware;
mod system_info;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use env_logger::Env;
use thiserror::Error;
#[derive(Debug)]
pub struct Server {
    host: String,
    port: u16,
}

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("invalid port: {0}")]
    InvalidPort(u16),
}

impl Server {
    pub fn new(host: String, port: u16) -> Self {
        Server { host, port }
    }

    pub async fn run(self) -> std::io::Result<()> {
        use crate::file_api;
        use crate::system_info;
        env_logger::init_from_env(Env::default().default_filter_or("info"));
        use crate::base::Response;
        use actix_web::web;
        HttpServer::new(|| {
            App::new()
                .wrap(actix_web::middleware::DefaultHeaders::new().add(("X-Version", "0.1")))
                .wrap(Logger::default())
                .service(
                    web::scope("/v1")
                        .service(web::scope("/system_info").configure(system_info::handle))
                        .service(web::scope("/file").configure(file_api::handle))
                        .service(web::scope("/img").configure(img_api::handle)),
                )
                .default_service(web::route().to(|| async {
                    Response::<()> {
                        data: None,
                        msg: "Not Found".to_string(),
                        code: 404,
                    }
                }))
        })
        .workers(2)
        .bind((self.host, self.port))?
        .run()
        .await
    }
}
