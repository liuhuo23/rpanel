use actix_web::web::ServiceConfig;

pub mod file;

pub fn handle(cfg: &mut ServiceConfig) {
    cfg.service(file::file_list);
}
