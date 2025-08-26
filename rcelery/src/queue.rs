use std::collections::HashMap;

use crate::handler;

pub struct Queue {
    pub queue_name: String,
    pub tasks: HashMap<String, String>,
}
