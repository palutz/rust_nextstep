use crate::eventbus::MyEventBus;
use tiny_tokio_actor::*;


// The actor struct must be Send + Sync but need not be Clone
#[derive(Default)]
pub(crate) struct EventActor {
    counter: usize
}

// Mark the struct as an actor. Note that you can optionally override
// some default methods here like `timeout()` and `supervision_stragegy()`.
// See the [`Actor`] trait for details.
impl Actor<MyEventBus> for EventActor {}

#[derive(Clone, Debug)]
pub(crate) struct EventMsg(pub(crate) String);

// Mark the message struct as an actor message. Note that we
// also define the response we expect back from this message.
// If no response is desired, just use `()`.
impl Message for EventMsg {
    type Response = i32;
}

// Define the behaviour of the actor. Note that the `handle` method
// has a `String` return type because that is what we defined the
// Response to be of `CommandMsg`. As the method is async, we have
// to annotate the implementation with the `async_trait` macro (a
// re-export of the `async_trait` crate).
#[async_trait]
impl Handler<MyEventBus, EventMsg> for EventActor {
    async fn handle(&mut self, msg: EventMsg, ctx: &mut ActorContext<MyEventBus>) -> i32 {
        self.counter += 1;
        println!("Event received!!!! *****************");
        ctx.system.publish(MyEventBus(format!("message {:?} received by '{}, counter = {}'", msg, ctx.path, &self.counter)));
        self.counter as i32
    }
}
