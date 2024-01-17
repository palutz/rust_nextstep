use actix::prelude::*;
use actix::dev::{ MessageResponse, OneshotSender };

#[derive(Message)]
#[rtype(result = "Responses")]
enum Messages {
    Command {
    cmd_id : i32,
    payload : String,
},
    Event,
}

enum Responses {
    AllOk,
    Problem,
}

impl<A, M> MessageResponse<A, M> for Responses
where
    A: Actor,
    M: Message<Result = Responses>,
{
    fn handle(self, ctx: &mut A::Context, tx: Option<OneshotSender<M::Result>>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}

struct MyActor {
    count: usize,
}

impl Actor for MyActor {
    type Context = Context<Self>;
}

impl Handler<Messages> for MyActor {
    type Result = Responses;

    fn handle(&mut self, msg: Messages, _ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            Messages::Command { .. }  => Responses::AllOk,
            Messages::Event => Responses::Problem,
        }
    }
}


#[actix_rt::main]
async fn main() {
    // start new actor
    let addr = MyActor { count: 10 }.start();

    // send message and get future fo,
    let c = Messages::Command {
        cmd_id : 1,
        payload : "First cmd".to_string(),
    };
    //let res = addr.send(Ping(10)).await;
    let res = addr.send(c);

    match res.await {
        Ok(rsps) => match rsps {
                        Responses::AllOk => println!("Ok!!!"),
                        Responses::Problem => println!("ooopss"),
                }
        _ => todo!(),
    }
    // handle() returns tokio handle

    // stop system and exit
    System::current().stop();
}
