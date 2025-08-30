use rcelery::celery_app::{CelerApp, CeleryConfig};

fn main() {
    let config = CeleryConfig::new("redis://127.0.0.1:6379/".to_string()).with_worker_threads(4); // 线程数可按需设置
    let app = CelerApp::with_config(config.clone());
    app.run();
}
