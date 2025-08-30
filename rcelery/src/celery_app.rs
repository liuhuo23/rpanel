use crate::queue::Queue;
use serde::Deserialize;
use std::collections::HashMap;
use tokio::signal;

#[derive(Debug, Clone, Deserialize)]
pub struct CeleryConfig {
    pub broker_url: String,
    pub result_backend: Option<String>,
    pub worker_threads: usize,
    pub connection_max_retries: usize,
    pub connection_retry_delay: u64, // in seconds
}

impl CeleryConfig {
    pub fn new(broker_url: String) -> Self {
        CeleryConfig {
            broker_url,
            result_backend: None,
            worker_threads: 1,
            connection_max_retries: 5,
            connection_retry_delay: 5, // default 5 seconds
        }
    }
    pub fn with_result_backend(mut self, backend: String) -> Self {
        self.result_backend = Some(backend);
        self
    }
    pub fn with_worker_threads(mut self, threads: usize) -> Self {
        self.worker_threads = threads;
        self
    }
}

pub struct CelerApp {
    pub queue_map: HashMap<String, Queue>,
    pub celery_config: CeleryConfig,
}

impl CelerApp {
    pub fn new(broker_url: String) -> Self {
        CelerApp {
            queue_map: HashMap::new(),
            celery_config: CeleryConfig::new(broker_url),
        }
    }
    pub fn works(&mut self, threads: usize) -> &mut Self {
        self.celery_config.worker_threads = threads;
        self
    }

    pub fn with_config(config: CeleryConfig) -> Self {
        CelerApp {
            queue_map: HashMap::new(),
            celery_config: config,
        }
    }

    /// 直接运行整个 celery app（集成 tokio runtime）
    pub fn run(self) {
        let threads = self.celery_config.worker_threads;
        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(threads)
            .enable_all()
            .build()
            .expect("Failed to build Tokio runtime");
        rt.block_on(async move {
            self.start().await;
        });
    }
    /// 检查 broker 连接状态的占位函数
    async fn check_broker(&self) {}
    pub async fn start(&self) {
        // 不断从指定的 broker 中获取消息，并分发给对应的 handler 进行处理
        println!(
            "CeleryApp 启动，连接到 broker: {}",
            self.celery_config.broker_url
        );
        println!("CeleryApp 已准备好接收任务，按 Ctrl+C 退出");
        loop {
            self.check_broker().await;
            // 等待 ctrl+c 信号
            match signal::ctrl_c().await {
                Ok(()) => {
                    println!("收到 Ctrl+C，正在优雅退出...");
                    break;
                }
                Err(e) => {
                    eprintln!("等待 Ctrl+C 信号失败: {}", e);
                    continue;
                }
            }
        }
        println!("CeleryApp 已退出");
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::celery_app::CelerApp;
    use crate::queue::Queue;

    #[test]
    fn test_celery_app() {
        let mut app = CelerApp::new("redis://redis:6379/".to_string());
        let queue = Queue {
            queue_name: "default".to_string(),
            tasks: HashMap::new(),
        };
        app.queue_map.insert(queue.queue_name.clone(), queue);
        if let Some(q) = app.queue_map.get_mut("default") {
            assert_eq!(q.queue_name, "default");
            q.tasks.insert("hello".to_string(), "1".to_string());
        } else {
            panic!("Queue not found");
        }
        assert_eq!(app.queue_map.len(), 1);
    }
}
