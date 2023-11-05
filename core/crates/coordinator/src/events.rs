use anything_graph::{Flow, Flowfile};
use anything_mq::MessageProtocol;
use anything_store::types::ChangeMessage;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum FlowPublisher {
    NewFlow(NewFlowPublisher),
    ExecuteFlow(ExecuteFlowPublisher),
}

impl MessageProtocol for FlowPublisher {
    fn name() -> &'static str {
        "flows"
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct NewFlowPublisher {
    pub flow: Flowfile,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ExecuteFlowPublisher {
    pub flow: Flow,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum InternalEventsPublisher {
    Shutdown,
    Ping,
}

impl MessageProtocol for InternalEventsPublisher {
    fn name() -> &'static str {
        "internal"
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum StoreChangesPublisher {
    ChangeMessage(ChangeMessage),
}

impl MessageProtocol for StoreChangesPublisher {
    fn name() -> &'static str {
        "store"
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct StringPublisher(pub String);

impl MessageProtocol for StringPublisher {
    fn name() -> &'static str {
        "string"
    }
}
