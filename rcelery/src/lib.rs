pub mod error;
pub mod task;
pub mod task_manage;

pub struct CeleryApp {
    pub broker_url: String,
}

impl CeleryApp {
    pub fn new(broker_url: String) -> Self {
        CeleryApp { broker_url }
    }
}
