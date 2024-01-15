use tiny_tokio_actor::*;

// The event message you may want to publish to the system event bus.
#[derive(Clone, Debug)]
struct MyEventBus(String);

// Mark the struct as a system event message.
impl SystemEvent for MyEventBus {}

// The actor struct must be Send + Sync but need not be Clone
#[derive(Default)]
struct TestActor {
    counter: usize
}

// Mark the struct as an actor. Note that you can optionally override
// some default methods here like `timeout()` and `supervision_stragegy()`.
// See the [`Actor`] trait for details.
impl Actor<MyEventBus> for TestActor {}

// The message the actor will expect. It must derive Clone.
// Debug is not required.
#[derive(Clone, Debug)]
struct CommandMsg(String);

#[derive(Clone, Debug)]
struct EventMsg(String);

// Mark the message struct as an actor message. Note that we
// also define the response we expect back from this message.
// If no response is desired, just use `()`.
impl Message for CommandMsg {
    type Response = String;
}

impl Message for EventMsg {
    type Response = i32;
}

// Define the behaviour of the actor. Note that the `handle` method
// has a `String` return type because that is what we defined the
// Response to be of `CommandMsg`. As the method is async, we have
// to annotate the implementation with the `async_trait` macro (a
// re-export of the `async_trait` crate).
#[async_trait]
impl Handler<MyEventBus, CommandMsg> for TestActor {
    async fn handle(&mut self, msg: CommandMsg, ctx: &mut ActorContext<MyEventBus>) -> String {
        self.counter += 1;
        println!("Command received!!!! *****************");
        let act2 = ctx.system.create_actor("Trinity", TestActor { counter : self.counter }).await.unwrap();
        ctx.system.publish(MyEventBus(format!("message received by '{}, counter = {}'", ctx.path, &self.counter)));
        let rs = act2.ask(EventMsg("Hi Neo".to_string())).await.unwrap();
        println!(" ");
        for i in 1..100 {
            print!("{i}");
        }
        format!("Ping!\n Pong! r = {rs}")
    }
}

#[async_trait]
impl Handler<MyEventBus, EventMsg> for TestActor {
    async fn handle(&mut self, msg: EventMsg, ctx: &mut ActorContext<MyEventBus>) -> i32 {
        self.counter += 1;
        println!("Event received!!!! *****************");
        ctx.system.publish(MyEventBus(format!("message received by '{}, counter = {}'", ctx.path, &self.counter)));
        self.counter as i32
    }
}
// #[tokio::main]
#[tokio::main(flavor = "current_thread")]
pub async fn main() -> Result<(), ActorError> {

    // Create the actor
    let actor = TestActor { counter: 0 };
    // Create the message we will send
    let cmd = CommandMsg("hello world!".to_string());

    // Create the system event bus
    let bus = EventBus::<MyEventBus>::new(1000);
    // Create the actor system with the event bus
    let system = ActorSystem::new("TheMatrix", bus);
    // Launch the actor on the actor system
    let actor_ref = system.create_actor("Neo", actor).await?;

    // Listen for events on the system event bus
    let mut events: EventReceiver<MyEventBus> = system.events();
    tokio::spawn(async move {
        loop {
            print!("^");
            match events.recv().await {
                Ok(event) => {
                    println!("Received event! {:?} = {}", event, event.0)
                },
                Err(err) => println!("Error receivng event!!! {:?}", err)
            }
        }
    });

    // Wait a little for the actor to start up
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

    // Send the actor the message through an `ask` from which we will
    // get a response
    match actor_ref.ask(cmd).await {
        Ok(response) => {
            println!("Response: {}", response);
        },
        Err(_) => println!("ERRORRRRR!!!")
    }
    Ok(())
}
