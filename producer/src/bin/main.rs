use anyhow::Result;
use tanit::application::http::{HttpServer, HttpServerConfig};

fn generate_data() {
    
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");

    let server_config = HttpServerConfig::new("3333".to_string());
    let http_server = HttpServer::new(server_config).await?;

    http_server.run().await?;

    Ok(())
}
