// use crate::flows

use crate::{
    generated::{
        flows_server::Flows, CreateFlowRequest, CreateFlowResponse, GetFlowRequest,
        GetFlowResponse, GetFlowsResponse, PublishFlowRequest, PublishFlowResponse,
        UpdateFlowRequest, UpdateFlowResponse, UpdateFlowVersionRequest, UpdateFlowVersionResponse,
    },
    generated::{Flow, GetFlowsRequest},
    Context,
};
use postage::dispatch::Sender;
use std::sync::Arc;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct FlowManager {
    context: Arc<Context>,
    update_tx: Sender<Flow>,
}

impl FlowManager {
    pub fn new(context: &Context, update_tx: Sender<Flow>) -> Self {
        Self {
            context: Arc::new(context.clone()),
            update_tx,
        }
    }
}

#[tonic::async_trait]
impl Flows for FlowManager {
    async fn create_flow(
        &self,
        _request: Request<CreateFlowRequest>,
    ) -> Result<Response<CreateFlowResponse>, Status> {
        Err(Status::unimplemented("Not implemented"))
    }
    async fn get_flows(
        &self,
        _request: Request<GetFlowsRequest>,
    ) -> Result<Response<GetFlowsResponse>, Status> {
        Err(Status::unimplemented("Not implemented"))
    }

    async fn get_flow(
        &self,
        _request: Request<GetFlowRequest>,
    ) -> Result<Response<GetFlowResponse>, Status> {
        Err(Status::unimplemented("Not implemented"))
    }

    async fn update_flow(
        &self,
        _request: Request<UpdateFlowRequest>,
    ) -> Result<Response<UpdateFlowResponse>, Status> {
        Err(Status::unimplemented("Not implemented"))
    }

    async fn update_flow_version(
        &self,
        _request: Request<UpdateFlowVersionRequest>,
    ) -> Result<Response<UpdateFlowVersionResponse>, Status> {
        Err(Status::unimplemented("Not implemented"))
    }
}
