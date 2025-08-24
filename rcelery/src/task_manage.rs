use std::{any::Any, collections::HashMap};

pub struct TaskManager {
    pub(crate) queues: HashMap<String, Box<dyn Any>>,
}
