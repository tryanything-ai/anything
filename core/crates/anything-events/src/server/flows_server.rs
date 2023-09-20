#![allow(dead_code)]

use crate::{
    generated::GetFlowsRequest,
    generated::{
        flows_server::Flows, CreateFlowRequest, CreateFlowResponse, GetFlowRequest,
        GetFlowResponse, GetFlowsResponse, UpdateFlowRequest, UpdateFlowResponse,
        UpdateFlowVersionRequest, UpdateFlowVersionResponse,
    },
    models::flow::Flow,
    repositories::flow_repo::FlowRepo,
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
    async fn get_flows(
        &self,
        request: Request<GetFlowsRequest>,
    ) -> Result<Response<GetFlowsResponse>, Status> {
        let _req = request.into_inner();
        // let flows = self.context.repositories.flow_repo.
        let flows = match self.context.repositories.flow_repo.get_flows().await {
            Ok(flows) => flows,
            Err(e) => {
                return Err(Status::internal(format!(
                    "Unable to get flows: {}",
                    e.to_string()
                )))
            }
        };

        let response = GetFlowsResponse {
            flows: flows.into_iter().map(|f| f.into()).collect(),
        };
        Ok(Response::new(response))
    }

    async fn get_flow(
        &self,
        request: Request<GetFlowRequest>,
    ) -> Result<Response<GetFlowResponse>, Status> {
        let req = request.into_inner();

        let flow_id = req.flow_id;

        if flow_id.is_empty() {
            return Err(Status::invalid_argument("No flow id provided"));
        }

        let flow = match self
            .context
            .repositories
            .flow_repo
            .get_flow_by_id(flow_id)
            .await
        {
            Ok(flow) => flow,
            Err(e) => {
                return Err(Status::internal(format!(
                    "Unable to get flow: {}",
                    e.to_string()
                )))
            }
        };

        Ok(Response::new(GetFlowResponse {
            flow: Some(flow.into()),
        }))
    }

    async fn create_flow(
        &self,
        _request: Request<CreateFlowRequest>,
    ) -> Result<Response<CreateFlowResponse>, Status> {
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
