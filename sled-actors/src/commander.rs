use tiny_tokio_actor::*;
use crate::eventbus::MyEventBus;

// The message the actor will expect. It must derive Clone.
// Debug is not required.
#[derive(Clone, Debug)]
pub(crate) struct CommandMsg(pub String);

// Mark the message struct as an actor message. Note that we
// also define the response we expect back from this message.
// If no response is desired, just use `()`.
impl Message for CommandMsg {
    type Response = String;
}

// The actor struct must be Send + Sync but need not be Clone
#[derive(Default)]
pub(crate) struct CommandActor {
    pub counter: usize
}

// Mark the struct as an actor. Note that you can optionally override
// some default methods here like `timeout()` and `supervision_stragegy()`.
// See the [`Actor`] trait for details.
impl Actor<MyEventBus> for CommandActor {}


// Define the behaviour of the actor. Note that the `handle` method
// has a `String` return type because that is what we defined the
// Response to be of `CommandMsg`. As the method is async, we have
// to annotate the implementation with the `async_trait` macro (a
// re-export of the `async_trait` crate).
#[async_trait]
impl Handler<MyEventBus, CommandMsg> for CommandActor {
    async fn handle(&mut self, msg: CommandMsg, ctx: &mut ActorContext<MyEventBus>) -> String {
        self.counter += 1;
        println!("Command received!!!! *****************");
        //let path = ActorPath::from("/user") / "Trinity";
        let act2 = ctx.system.get_or_create_actor("Trinity", || { println!("Creating actor2"); CommandActor { counter : self.counter }}).await.unwrap();
        ctx.system.publish(MyEventBus(format!("message {:?} received by '{}, counter = {}'", msg, ctx.path, &self.counter)));
        // let rs = act2.ask(EventMsg("Hi Neo".to_string())).await.unwrap();
        // println!(" ");
        // for i in 1..100 {
        //     print!("{i}");
        // }
        // format!("Ping!\n Pong! r = {rs}")
        format!("Ping!\n Pong! r ")
    }
}
