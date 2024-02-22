use tide::Request;
use tide::prelude::*;
use tracing::{info, Level};
use tracing_subscriber;

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u8,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    let mut app = tide::new();
    app.at("/").get(hello_world);
    app.at("/orders/shoes").post(order_shoes);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn hello_world(_req: Request<()>) -> tide::Result {
    // tracing::collect::with_default("hello_world");
    info!("GET Hello World");
    Ok(format!("Hello, World!").into())
}

async fn order_shoes(mut req: Request<()>) -> tide::Result {
    let Animal { name, legs } = req.body_json().await?;
    Ok(format!("Hello, {}! I've put in an order for {} shoes", name, legs).into())
}
