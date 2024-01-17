// eventbus -- BAD NAMING
//
//
use tiny_tokio_actor::SystemEvent;

// The event message you may want to publish to the system event bus.
#[derive(Clone, Debug)]
pub(crate) struct MyEventBus(pub String);

// Mark the struct as a system event message.
impl SystemEvent for MyEventBus {}
