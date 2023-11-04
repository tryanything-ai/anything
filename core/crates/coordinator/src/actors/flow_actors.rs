use anything_common::{loop_with_timeout_or_message, AnythingConfig};
use anything_runtime::Runner;
use anything_store::FileStore;
use ractor::{async_trait, cast, Actor, ActorRef};

use crate::{
    processing::processor::{Processor, ProcessorMessage},
    CoordinatorActorResult,
};

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

        let (results_tx, results_rx) = tokio::sync::mpsc::channel(1024);

        processor.run_graph(results_tx, max_parallelism).await?;

        let update_actor_ref = state.update_actor_ref.clone();

        // Wait a maximum of 15 seconds for results to come in
        let _ = tokio::spawn(async move {
            loop_with_timeout_or_message(
                std::time::Duration::from_secs(15),
                results_rx,
                |msg: ProcessorMessage| {
                    tracing::debug!("Got a result: {:#?}", msg);
                    // Do we want to keep other messages?
                    match msg {
                        ProcessorMessage::FlowTaskFinishedSuccessfully(task_name, task_result) => {
                            tracing::debug!("Got a task result: {:#?}", task_result);
                            cast!(
                                update_actor_ref,
                                UpdateActorMessage::FlowLifecycle(
                                    ProcessorMessage::FlowTaskFinishedSuccessfully(
                                        task_name,
                                        task_result
                                    )
                                )
                            )
                            .unwrap();
                            false
                        }
                        ProcessorMessage::FlowTaskFinishedWithError(
                            task_name,
                            status,
                            flow_result,
                        ) => {
                            tracing::debug!("Got a flow result: {:#?}", flow_result);
                            cast!(
                                update_actor_ref,
                                UpdateActorMessage::FlowLifecycle(
                                    ProcessorMessage::FlowTaskFinishedWithError(
                                        task_name,
                                        status,
                                        flow_result
                                    )
                                )
                            )
                            .unwrap();
                            false
                        }
                        _ => true,
                    }
                },
            )
            .await
            .unwrap();
        });

        Ok(())
    }
}
