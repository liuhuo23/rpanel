use rcelery::error::CeleryError;
use rcelery::task::TaskMeta;
use rcelery::task::TaskStatus;
use rcelery_macros::queue_task;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskInput {
    pub value: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskOutput {
    pub result: String,
}

#[queue_task(queue_name = "test_queue", task_name = "test_task", max_retries = 3)]
pub fn test_task(_: TaskInput) -> Result<Option<TaskOutput>, CeleryError> {
    Err(CeleryError::Unknown)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_output() {
        let input = TaskInput { value: 42 };
        let _ = TestTaskTask::handler(input);
    }
}
