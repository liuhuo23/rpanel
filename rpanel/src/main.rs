use backend::Server;

#[tokio::main]
async fn main() {
    let server = Server::new("127.0.0.1".to_string(), 8080);
    if let Err(e) = server.run().await {
        eprintln!("服务启动失败: {}", e);
    }
}
