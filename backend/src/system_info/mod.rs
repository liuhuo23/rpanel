mod info;

use actix_web::web::ServiceConfig;

pub fn handle(cfg: &mut ServiceConfig) {
    cfg.service(info::cpu_info);
    cfg.service(info::mem_info);
    cfg.service(info::swap_info);
}
