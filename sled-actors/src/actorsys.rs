use tiny_tokio_actor::*;
use crate::commander::{ CommandActor, CommandMsg };
use crate::eventbus::MyEventBus;


pub(crate) struct MyActorSystem<T>
where T : SystemEvent
{
    bus       : EventBus<T>,
    actsys    : ActorSystem<T>,
}

impl<T> MyActorSystem<T>
where T : SystemEvent
{
    fn new(name: &str) -> Self {
        let b = EventBus::<T>::new(1000);
        MyActorSystem {
            bus : b.clone(),
            actsys : ActorSystem::new(name,b),
        }
    }

    // Add an actor to this system - Sync method
    pub async fn add_actor<A>(&self, name : &str, actor : A) -> Result<ActorRef<T, A>, ActorError>
    where
        A : Actor<T>
    {
        self.actsys.create_actor(name, actor).await
    }
}


pub async fn actor_system() {
    // Create the actor
    let actor = CommandActor { counter: 0 };
    // let actor2 = TestActor { counter: 0 };
    // Create the message we will send
    let cmd = CommandMsg("hello world!".to_string());

    let myactors = MyActorSystem::<MyEventBus>::new("TheMatrix");
    // Create the system event bus
    //let bus = EventBus::<MyEventBus>::new(1000);
    // Create the actor system with the event bus
   // let system = ActorSystem::new("TheMatrix", bus);
    // Launch the actor on the actor system
    //let actor_ref = system.create_actor("Neo", actor).await.unwrap();
    let actor_ref = myactors.add_actor("Neo", actor).await.unwrap();
    //let act2_ref = system.create_actor("Trinity", actor2).await?;
        //let act2 = ctx.system.create_actor("Trinity", TestActor { counter : self.counter }).await.unwrap();

    // Listen for events on the system event bus
    // let mut events: EventReceiver<MyEventBus> = system.events();
    let mut events: EventReceiver<MyEventBus> = myactors.actsys.events();
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
    let response = actor_ref.ask(cmd).await.unwrap();
    println!("Response: {}", response);
}
