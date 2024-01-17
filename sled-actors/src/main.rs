use tokio::sync::{oneshot, mpsc};
use tokio::time::{sleep, Duration };

struct MyActor {
    receiver: mpsc::Receiver<ActorMessage>,
    next_id: u32,
}
enum ActorMessage {
    GetUniqueId {
        respond_to: oneshot::Sender<u32>,
    },
    OtherMessage,
}

impl MyActor {
    fn new(receiver: mpsc::Receiver<ActorMessage>) -> Self {
        MyActor {
            receiver,
            next_id: 0,
        }
    }
    fn handle_message(&mut self, msg: ActorMessage) {
        match msg {
            ActorMessage::GetUniqueId { respond_to } => {
                self.next_id += 1;
                dbg!("Received msg {:?}", self.next_id);

                // The `let _ =` ignores any errors when sending.
                //
                // This can happen if the `select!` macro is used
                // to cancel waiting for the response.
                let _ = respond_to.send(self.next_id);
            },
            _ => { let _ = sleep(Duration::from_millis(500)); }
        }
    }

    async fn run(&mut self) {
        // if all the receivers are dropped then recv() will return None and this loop
        // will stop, the function will return and the actor shutdown
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg);
        }
    }
}

// async fn run_my_actor(mut actor: MyActor) {
//     // if all the receivers are dropped then recv() will return None and this loop
//     // will stop, the function will return and the actor shutdown
//     while let Some(msg) = actor.receiver.recv().await {
//         actor.handle_message(msg);
//     }
// }


#[derive(Clone)]
pub struct MyActorHandle {
    sender: mpsc::Sender<ActorMessage>,
}

impl MyActorHandle {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let mut actor = MyActor::new(receiver);
        tokio::spawn(async move { actor.run().await });

        Self { sender }
    }

    pub async fn get_unique_id(&self) -> u32 {
        let (send, recv) = oneshot::channel();
        let msg = ActorMessage::GetUniqueId {
            respond_to: send,
        };

        // Ignore send errors. If this send fails, so does the
        // recv.await below. There's no reason to check the
        // failure twice.
        let _ = self.sender.send(msg).await;
        recv.await.expect("Actor task has been killed")
    }

    pub async fn other_msg(&self) {
        let (_, recv) = oneshot::channel();
        let msg = ActorMessage::OtherMessage;

        let _ = self.sender.send(msg).await;
        recv.await.expect("Actor task has been killed")
    }
}

#[tokio::main]
async fn main() {
    let myh = MyActorHandle::new();

    for i  in 10..21 {
        let id = myh.get_unique_id();
        println!("Req {i} sent to actor");
        //println!("{i} Actor id = {}", id.await);
    }

}
