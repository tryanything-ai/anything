use tokio::{
    select,
    sync::{
        mpsc::{self, Receiver, Sender},
        oneshot,
    },
};
use zmq::{Context, Message};

fn main() -> EvtResult<()> {
    let runtime = build_runtime().expect("unable to build runtime");

    // let (tx, mut rx) = mpsc::channel(32);
    let (backend_tx, mut backend_rx) = mpsc::channel::<Message>(32);
    let (manager_tx, mut manager_rx) = mpsc::channel::<Message>(32);

    let backend_tx2 = backend_tx.clone();
    let manager_tx2 = manager_tx.clone();

    let _fw_handle = init_logger();
    // runtime.block_on(async {

    // let _res = runtime.spawn(start_listener(tx));
    runtime.block_on(async move {
        let manager = tokio::spawn(async move {
            // Start receiving messages
            while let Some(cmd) = manager_rx.recv().await {
                println!("Got command: {:?}", cmd);
                let resp_msg = zmq::Message::from("World");
                backend_tx2.send(resp_msg).await.unwrap();
            }
        });

        let frontend_handle = tokio::spawn(async move {
            let context = zmq::Context::new();
            let responder = context.socket(zmq::REP).unwrap();

            assert!(responder.bind("tcp://*:5557").is_ok());

            loop {
                // let mut msg = zmq::Message::new();
                let msg = responder.recv_msg(0);
                match msg {
                    Ok(msg) => {
                        println!("Frontend received a message. Sending it to the manager");
                        manager_tx2.send(msg).await.unwrap()
                    }
                    Err(_) => break,
                }
                // responder.send("World", 0).unwrap();
                // tx.send(msg).await.unwrap();
            }
        });

        let backend_handle = tokio::spawn(async move {
            let ctx = zmq::Context::new();
            let requester = ctx.socket(zmq::REQ).unwrap();

            assert!(requester.connect("tcp://localhost:5557").is_ok());

            // let mut msg = zmq::Message::new();

            println!("Backend is sending a message");
            requester.send("Hello", 0).unwrap();

            loop {
                while let Some(msg) = backend_rx.recv().await {
                    println!("Got a message back: {:?}", msg.as_str().unwrap());
                }
            }
        });

        let _ = tokio::join!(frontend_handle, backend_handle, manager);
    });

    Ok(())
}

async fn spawn_listener(mut rx: Receiver<String>) -> EvtResult<()> {
    loop {
        select! {
            Some(msg) = rx.recv() => {
                println!("Received message: {:?}", msg);
                break;
            }
        }
    }
    Ok(())
}

async fn start_listener(tx: Sender<String>) -> EvtResult<()> {
    let context = Context::new();
    let frontend = context.socket(zmq::SUB).unwrap();
    frontend
        .connect("tcp://localhost:5557")
        .expect("unable to connect to frontend");

    let backend = context.socket(zmq::PUB).unwrap();
    backend
        .bind("tcp://*:5558")
        .expect("unable to bind backend socket");

    frontend.set_subscribe(b"").unwrap();
    let tx_clone = tx.clone();

    // let mut cache = HashMap::new();

    // loop {

    loop {
        let mut items = [
            frontend.as_poll_item(zmq::POLLIN),
            backend.as_poll_item(zmq::POLLIN),
        ];
        if zmq::poll(&mut items, 1000).is_err() {
            break; // Interrupted
        }
        println!("Checking for readability");
        let _ = tx_clone.send(true.to_string()).await;
        if items[0].is_readable() {
            let topic = frontend.recv_msg(0).unwrap();
            println!("Received topic: {:?}", topic);
            let current = frontend.recv_msg(0).unwrap();
            println!("Received current: {:?}", current);
        }

        // if items[1].is_readable() {
        //     let event = backend.recv_msg(0).unwrap();
        //     if event[0] == 1 {
        //         let topic = &event[1..];
        //         println!("Sending cached topic");
        //         if let Some(prev) = cache.get(topic) {
        //             backend.send(topic, zmq::SNDMORE).unwrap();
        //             backend.send(prev, 0).unwrap();
        //         }
        //     }
        // }
        // }
    }
    Ok(())
}
