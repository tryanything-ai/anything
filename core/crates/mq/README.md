# MQ

A central hub for listening to messages in a loosely connected way. 

Although it only supports in-process messaging, it's written to be extensible. 

For example, it can be extended to allow messages to be sent over tcp, unix pipes, etc.

## Library usage

Starting the `mq` (affectionately `post_office`), call the `listen_and_serve()` function. 

```rust
let (mut post_office, _stop_tx, _join_handle) = mq::listen_and_serve().unwrap();
```

Using the `post_office` object, you can now connect a client:

```rust
let client = mq::Client::new_memory(&mut post_office).unwrap();
```

## Pub/Sub

In `mq` the bus is served via messaging types with prefixed message types with a heirarchical topic-matching pattern.

```rust
#[derive(Clone, Deserialize, Serialize)]
struct ExampleStartMessage(String);

impl PublishProtocol for ExampleStartMessage {
    fn prefix() -> &'static str {
        "start"
    }
}
```

Using this `ExampleStartMessage` struct, we can now subscribe to messages with a topic:

```rust
let mut subscription = client.subscribe::<ExampleStartMessage>("server").await?;
// Or deeper
let mut subscription = client.subscribe::<ExampleStartMessage>("server/email_server/start").await?;
// Or all topics
let mut subscription = client.subscribe::<ExampleStartMessage>("").await?;
```

Publishing messages are enabled through using the `publish()` function

```rust
let another_client = mq::Client::new_memory(&mut post_office).unwrap();
// Publish a message
another_client.publish("server", &ExampleStartMessage(String::from("This is my payload"))).await?;
```

Subscribers can now use the standard `recv()` function to receive messages:

```rust
while let Some((topic, message)) = subscription.recv().await {
    // ...
    // Do something with the message
}
```


## TODO

- [ ] Add unix socket pipes as a transport layer
- [ ] Add directory structure for queue memory recovery
- [ ] Add TCP pip transport layer support
- [ ] Determine if we want to support the `gen_server`-esque (of erlang fame) send and response?


## Credit where credit is due

This library took LOTS of inspiration from `t2_bus`. Due to constraints. 