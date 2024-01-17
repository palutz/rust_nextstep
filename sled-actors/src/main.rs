mod actorsys;
mod eventbus;
mod commander;
mod eventer;

use std::error::Error;
use crate::actorsys::*;
// #[tokio::main]
#[tokio::main(flavor = "current_thread")]
pub async fn main() -> Result<(), Box<dyn Error>> {

    actor_system().await;
    Ok(())
}
