use core::str;

use serde::{Deserialize, Serialize};

use crate::{celery_app::CelerApp, handler::FromJson};
pub trait TaskFactory {
    fn register(self, config: &mut CelerApp);
}

impl<T: TaskFactory> TaskFactory for Vec<T> {
    fn register(self, config: &mut CelerApp) {
        self.into_iter()
            .for_each(|factory| factory.register(config));
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    pub name: Option<String>,
}

impl FromJson for Task {
    type Error = String;
    type Future =
        std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self, Self::Error>> + Send>>;
    fn from_json_value(val: serde_json::Value) -> Self::Future {
        Box::pin(async move { serde_json::from_value(val).map_err(|e| e.to_string()) })
    }
}
