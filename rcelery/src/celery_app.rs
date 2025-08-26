use crate::queue::Queue;
use std::collections::HashMap;
pub struct CelerApp {
    pub queue_map: HashMap<String, Queue>,
}

impl CelerApp {
    pub fn new() -> Self {
        CelerApp {
            queue_map: HashMap::new(),
        }
    }

    async fn start(&self) {
        // 不断从指定的 broker 中获取消息，并分发给对应的 handler 进行处理
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::celery_app::CelerApp;
    use crate::queue::Queue;

    #[test]
    fn test_celery_app() {
        let mut app = CelerApp::new();
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
