use redis::Client;

use crate::error::CeleryError;

pub trait Broker {
    async fn connect(&mut self) -> Result<(), CeleryError>;
    async fn disconnect(&mut self) {}
    async fn check_broker(&self) -> bool;
    fn get_broker_url(&self) -> &String;

    async fn is_connected(&self) -> bool {
        self.check_broker().await
    }
    async fn reconnection(&mut self, max_re: u32, time_sleep: u32) -> Result<(), CeleryError> {
        let mut last_err = None;
        for attempt in 1..=max_re {
            // Limit the mutable borrow to this block
            let connect_result = self.connect().await;
            match connect_result {
                Ok(_) => return Ok(()),
                Err(e) => {
                    last_err = Some(e);
                    eprintln!(
                        "连接{} 失败 (尝试 {}/{}): {}",
                        self.get_broker_url(),
                        attempt,
                        max_re,
                        last_err.as_ref().unwrap()
                    );
                    if attempt < max_re {
                        tokio::time::sleep(tokio::time::Duration::from_secs(time_sleep as u64))
                            .await;
                    }
                }
            }
        }
        Err(CeleryError::ConnectionError(format!(
            "无法连接到{}，已尝试 {} 次: {:?}",
            self.get_broker_url(),
            max_re,
            last_err
        )))
    }
}

pub struct RedisBroker {
    pub broker_url: String,
    pub client: Option<redis::Client>,
}

impl RedisBroker {
    pub fn new(broker_url: String) -> Self {
        RedisBroker {
            broker_url,
            client: None,
        }
    }
}

impl Broker for RedisBroker {
    async fn connect(&mut self) -> Result<(), CeleryError> {
        println!("Connecting to Redis broker at {}", self.broker_url);
        match redis::Client::open(self.broker_url.as_str()) {
            Ok(client) => {
                self.client = Some(client);
                Ok(())
            }
            Err(e) => Err(CeleryError::ConnectionError(e.to_string())),
        }
    }

    async fn disconnect(&mut self) {
        if self.client.is_some() {
            println!("Disconnecting from Redis broker at {}", self.broker_url);
            self.client = None;
        }
    }

    async fn check_broker(&self) -> bool {
        if let Some(client) = &self.client {
            match client.get_connection() {
                Ok(_) => true,
                Err(_) => false,
            }
        } else {
            false
        }
    }

    fn get_broker_url(&self) -> &String {
        &self.broker_url
    }
}
