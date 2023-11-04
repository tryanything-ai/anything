use anything_common::AnythingConfig;
use anything_runtime::{ExecuteConfig, Runner};
use anything_store::FileStore;
use ractor::{async_trait, Actor, ActorRef};

use crate::{processing::processor::Processor, CoordinatorActorResult};

use super::update_actor::UpdateActorMessage;

#[derive(Debug, Clone)]
pub enum FlowMessage {
    ExecuteFlow(anything_graph::Flow),
}

pub struct FlowActor;
pub struct FlowActorState {
    pub file_store: FileStore,
    pub runner: Runner,
    pub config: AnythingConfig,
    pub update_actor_ref: ActorRef<UpdateActorMessage>,
}

#[async_trait]
impl Actor for FlowActor {
    type Msg = FlowMessage;
    type State = FlowActorState;
    type Arguments = FlowActorState;

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> CoordinatorActorResult<Self::State> {
        Ok(args)
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> CoordinatorActorResult<()> {
        match message {
            FlowMessage::ExecuteFlow(flow) => {
                tracing::debug!("Execute flow");
                self.execute_flow(flow.clone(), state).await?;
            }
        }
        Ok(())
    }
}

impl FlowActor {
    async fn execute_flow(
        &self,
        flow: anything_graph::Flow,
        state: &<FlowActor as Actor>::State,
    ) -> CoordinatorActorResult<()> {
        let runner = state.runner.clone();
        let max_parallelism = state.config.execution_config().max_parallelism;
        let mut processor = Processor::new(runner, flow);
        processor.runtime_runner.load_plugins()?;

        let (results_tx, mut results_rx) = tokio::sync::mpsc::channel(1024);

        processor.run_graph(results_tx, max_parallelism).await?;

        while let Some(msg) = results_rx.recv().await {
            println!(
                "Got a result: {:?}",
                msg.1.lock().unwrap().get_result(msg.0.as_str())
            );
            // state.update_actor_ref.send_message(FlowMessage::StatusUpdate({
            //     flow_name: "thing",
            //     state_name: "node-2",
            //     payload: msg.1.clone()
            // }))
        }
        Ok(())
    }
}
