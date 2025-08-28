mod api;
use actix_web::web::ServiceConfig;

pub fn handle(cfg: &mut ServiceConfig) {
    cfg.service(api::create_image);
    cfg.service(api::get_image);
    cfg.service(api::list_images).service(api::delete);
}
