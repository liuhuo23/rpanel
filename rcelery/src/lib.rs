pub mod celery_app;
pub mod error;
pub mod handler;
pub mod queue;
pub mod task;
pub mod task_manage;
pub mod types;
pub struct CeleryApp {
    pub broker_url: String,
}

impl CeleryApp {
    pub fn new(broker_url: String) -> Self {
        CeleryApp { broker_url }
    }
}
