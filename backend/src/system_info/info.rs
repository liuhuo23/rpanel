use crate::base::Response;
use actix_web::{Responder, get};
use serde::Serialize;
use system_info::cpu;
#[derive(Serialize, serde::Deserialize)]
pub struct CpuInfo {
    pub cores: usize,
    pub usage: f32, // 百分比 0.0~100.0
}

#[derive(Serialize, serde::Deserialize)]
struct MemInfo {
    pub total_kb: u64,
    pub used_kb: u64,
    pub usage_ratio: f32,
}

#[derive(Serialize, serde::Deserialize)]
struct SwapInfo {
    pub total_kb: u64,
    pub used_kb: u64,
    pub usage_ratio: f32,
}

#[get("/cpu")]
pub async fn cpu_info() -> impl Responder {
    let info = CpuInfo {
        cores: cpu::count(),
        usage: cpu::usage() as f32 / 1000.0,
    };
    Response::new(Some(info), "Success".into(), 0)
}

#[get("/mem")]
pub async fn mem_info() -> impl Responder {
    let info = system_info::mem::get_mem_info();
    let response = MemInfo {
        total_kb: info.total_kb,
        used_kb: info.used_kb,
        usage_ratio: info.usage_ratio as f32,
    };
    Response {
        data: Some(response),
        msg: "Success".into(),
        code: 0,
    }
}

#[get("/swap")]
pub async fn swap_info() -> impl Responder {
    let info = system_info::swap::get_swap_info();
    let info = SwapInfo {
        total_kb: info.total_kb,
        used_kb: info.used_kb,
        usage_ratio: info.usage_ratio as f32,
    };
    Response {
        data: Some(info),
        msg: "Success".into(),
        code: 0,
    }
}
