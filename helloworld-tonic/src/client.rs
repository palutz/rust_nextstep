use std::time::Duration;

use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

use std::sync::{Arc, Mutex};
use tokio::runtime::Handle;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Arc::new(Mutex::new(
        GreeterClient::connect("http://[::1]:50051").await?,
    ));
    let mut handlers = vec![];

    for i in 1..10 {
        let asynchnadle = Handle::current();
        let arcclient = Arc::clone(&client);
        let handle = std::thread::spawn(move || {
            let request = tonic::Request::new(HelloRequest {
                name: format!("Tonic, from Stefano {i}").into(),
            });

            let mut client = arcclient.lock().unwrap();
            let response = asynchnadle.block_on(client.say_hello(request));

            println!("RESPONSE={:?}", response);
            std::thread::sleep(Duration::from_secs(2));
        });
        handlers.push(handle);
    }

    for h in handlers {
        h.join().unwrap();
    }

    Ok(())
}
