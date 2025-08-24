use std::process::Output;

use serde::{Serialize, de::DeserializeOwned};

use crate::error::CeleryError;

/// 定义异步任务

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

/// 任务元数据接口
pub trait TaskMeta: Send + Sync + 'static {
    /// 获取队列名称
    fn queue_name() -> &'static str;

    /// 获取任务名称
    fn task_name() -> &'static str;

    /// 输入类型
    type Input: Serialize + DeserializeOwned + 'static;
    /// 输出类型
    type Output: Serialize + DeserializeOwned + 'static;

    /// 处理任务数据
    fn handler(data: Self::Input) -> Result<Option<Self::Output>, CeleryError>;

    /// 重试次数
    fn max_retries() -> u32 {
        return 0;
    }
}

pub struct Task {
    pub id: String,
    pub status: TaskStatus,
    pub queue_name: String,
    pub task_name: String,
}
