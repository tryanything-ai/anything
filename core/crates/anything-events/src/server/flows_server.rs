#![allow(dead_code)]

use crate::{
    generated::GetFlowsRequest,
    generated::{
        flows_service_server::FlowsService, ActivateFlowVersionRequest,
        ActivateFlowVersionResponse, CreateFlowRequest, CreateFlowResponse, GetFlowRequest,
        GetFlowResponse, GetFlowsResponse, UpdateFlowRequest, UpdateFlowResponse,
        UpdateFlowVersionRequest, UpdateFlowVersionResponse,
    },
    models::flow::Flow,
    repositories::flow_repo::FlowRepo,
    Context, CreateFlow,
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
impl FlowsService for FlowManager {
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
        request: Request<CreateFlowRequest>,
    ) -> Result<Response<CreateFlowResponse>, Status> {
        use crate::generated::flows::create_flow::Version;
        let req = request.into_inner();

        let flow = match req.flow {
            Some(f) => f,
            None => return Err(Status::invalid_argument("No flow provided")),
        };

        let version = flow.version;
        let flow_name = flow.flow_name;
        let active = flow.active;

        if flow_name.is_empty() {
            return Err(Status::invalid_argument("No flow name provided"));
        }

        let flow = match self
            .context
            .repositories
            .flow_repo
            .create_flow(CreateFlow {
                flow_name,
                version: match version {
                    Some(Version::VersionString(v)) => Some(v),
                    _ => Some("0.0.1".to_string()),
                },
                active,
            })
            .await
        {
            Ok(flow) => flow,
            Err(e) => {
                return Err(Status::internal(format!(
                    "Unable to create flow: {}",
                    e.to_string()
                )))
            }
        };

        Ok(Response::new(CreateFlowResponse {
            flow: Some(flow.into()),
        }))
    }

    async fn update_flow(
        &self,
        request: Request<UpdateFlowRequest>,
    ) -> Result<Response<UpdateFlowResponse>, Status> {
        let req = request.into_inner();
        let flow = match req.update_flow {
            Some(f) => f,
            None => return Err(Status::invalid_argument("No flow provided")),
        };
        let flow_id = req.flow_id;

        let flow_name = flow.flow_name;
        let version = Some(match flow.version {
            Some(v) => v.to_string(),
            None => "".to_string(),
        });

        let res = match self
            .context
            .repositories
            .flow_repo
            .update_flow(flow_id.clone(), crate::UpdateFlow { flow_name, version })
            .await
        {
            Ok(flow) => flow,
            Err(e) => {
                return Err(Status::internal(format!(
                    "Unable to update flow: {}",
                    e.to_string()
                )))
            }
        };

        Ok(Response::new(UpdateFlowResponse {
            flow: Some(res.into()),
        }))
    }

    async fn update_flow_version(
        &self,
        request: Request<UpdateFlowVersionRequest>,
    ) -> Result<Response<UpdateFlowVersionResponse>, Status> {
        let req = request.into_inner();

        let flow_id = req.flow_id;
        let version_id = req.version_id;
        let update_flow_version = match req.update_flow_version {
            Some(v) => v,
            None => return Err(Status::invalid_argument("No flow version provided")),
        };
        let version = update_flow_version.version;
        let flow_definition = update_flow_version.flow_definition;
        let published = update_flow_version.published;
        let description = update_flow_version.description;

        let res = match self
            .context
            .repositories
            .flow_repo
            .update_flow_version(
                flow_id,
                version_id,
                crate::UpdateFlowVersion {
                    version,
                    flow_definition,
                    published,
                    description,
                },
            )
            .await
        {
            Ok(flow) => flow,
            Err(e) => {
                return Err(Status::internal(format!(
                    "Unable to update flow version: {}",
                    e.to_string()
                )))
            }
        };

        Ok(Response::new(UpdateFlowVersionResponse {
            flow_version: Some(res.into()),
        }))
    }

    async fn activate_flow_version(
        &self,
        request: Request<ActivateFlowVersionRequest>,
    ) -> Result<Response<ActivateFlowVersionResponse>, Status> {
        let req = request.into_inner();

        let flow_id = req.flow_id;
        let version_id = req.version_id;

        let res = match self
            .context
            .repositories
            .flow_repo
            .activate_flow_version(flow_id, version_id)
            .await
        {
            Ok(flow) => flow,
            Err(e) => {
                return Err(Status::internal(format!(
                    "Unable to update flow version: {}",
                    e.to_string()
                )))
            }
        };

        Ok(Response::new(ActivateFlowVersionResponse {
            flow_version_id: res.into(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    use crate::{
        generated::{GetFlowsRequest, UpdateFlow},
        internal::test_helper::{get_test_context_from_pool, get_test_pool, TestFlowRepo},
    };

    #[tokio::test]
    async fn test_get_flows_returns_all_flows() -> Result<()> {
        let pool = get_test_pool().await.unwrap();
        let context = get_test_context_from_pool(&pool).await;
        let test = TestFlowRepo::new_with_pool(&context.pool);

        test.insert_create_flow(test.dummy_create_flow()).await?;
        let dummy_flow = test.dummy_create_flow();
        test.insert_create_flow(dummy_flow.clone()).await?;

        let flow_manager = FlowManager::new(&context, test.with_sender().await);

        let req = Request::new(GetFlowsRequest {});
        let res = flow_manager.get_flows(req).await;
        assert!(res.is_ok());
        let response = res.unwrap().into_inner();
        let flows = response.flows;
        assert_eq!(flows.len(), 2);

        let last = flows.last().unwrap();
        assert_eq!(last.flow_name, dummy_flow.flow_name);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_flows_returns_a_specific_flow_by_id() -> Result<()> {
        let pool = get_test_pool().await.unwrap();
        let context = get_test_context_from_pool(&pool).await;
        let test = TestFlowRepo::new_with_pool(&context.pool);

        test.insert_create_flow(test.dummy_create_flow()).await?;
        let dummy_flow = test.dummy_create_flow();
        let (final_flow_id, version_id) = test.insert_create_flow(dummy_flow.clone()).await?;
        let dummy_create_flow_version = test.dummy_create_flow_version(final_flow_id.clone());
        test.insert_create_flow_version(
            final_flow_id.clone(),
            version_id.clone(),
            dummy_create_flow_version.clone(),
        )
        .await?;

        let flow_manager = FlowManager::new(&context, test.with_sender().await);

        let req = Request::new(GetFlowRequest {
            flow_id: final_flow_id.clone(),
        });
        let res = flow_manager.get_flow(req).await;
        assert!(res.is_ok());
        let response = res.unwrap().into_inner();
        let flow = response.flow;
        assert!(flow.is_some());
        let flow = flow.unwrap();
        assert_eq!(flow.flow_name, dummy_flow.flow_name);

        Ok(())
    }

    #[tokio::test]
    async fn test_create_flow() -> Result<()> {
        let pool = get_test_pool().await.unwrap();
        let context = get_test_context_from_pool(&pool).await;
        let test = TestFlowRepo::new_with_pool(&context.pool);

        let dummy_flow = test.dummy_create_flow();
        let flow_manager = FlowManager::new(&context, test.with_sender().await);

        let req = Request::new(CreateFlowRequest {
            flow: Some(dummy_flow.clone().into()),
        });
        let res = flow_manager.create_flow(req).await;
        assert!(res.is_ok());
        let response = res.unwrap().into_inner();
        let flow = response.flow;
        assert!(flow.is_some());
        let flow = flow.unwrap();
        assert_eq!(flow.flow_name, dummy_flow.flow_name);

        let found = test.find_flow_by_id(flow.flow_id.clone()).await;
        assert!(found.is_ok());
        let found = found.unwrap();
        assert_eq!(found.flow_name, dummy_flow.flow_name);

        Ok(())
    }

    #[tokio::test]
    async fn test_update_flow() -> Result<()> {
        let pool = get_test_pool().await.unwrap();
        let context = get_test_context_from_pool(&pool).await;
        let test = TestFlowRepo::new_with_pool(&context.pool);

        test.insert_create_flow(test.dummy_create_flow()).await?;
        let dummy_flow = test.dummy_create_flow();
        let (final_flow_id, version_id) = test.insert_create_flow(dummy_flow.clone()).await?;
        let dummy_create_flow_version = test.dummy_create_flow_version(final_flow_id.clone());
        let out = test
            .insert_create_flow_version(
                final_flow_id.clone(),
                version_id.clone(),
                dummy_create_flow_version.clone(),
            )
            .await?;

        let flow_manager = FlowManager::new(&context, test.with_sender().await);

        let req = Request::new(UpdateFlowRequest {
            flow_id: final_flow_id.clone(),
            update_flow: Some(UpdateFlow {
                flow_name: "new name".to_string(),
                version: Some("0.0.2".to_string()),
            }),
        });
        let res = flow_manager.update_flow(req).await;
        assert!(res.is_ok());
        let response = res.unwrap().into_inner();
        let flow = response.flow;
        assert!(flow.is_some());
        let flow = flow.unwrap();
        assert_eq!(flow.flow_name, "new name".to_string());

        Ok(())
    }

    #[tokio::test]
    async fn test_update_flow_version() -> Result<()> {
        let pool = get_test_pool().await.unwrap();
        let context = get_test_context_from_pool(&pool).await;
        let test = TestFlowRepo::new_with_pool(&context.pool);

        test.insert_create_flow(test.dummy_create_flow()).await?;
        let dummy_flow = test.dummy_create_flow();
        let (final_flow_id, version_id) = test.insert_create_flow(dummy_flow.clone()).await?;
        let dummy_create_flow_version = test.dummy_create_flow_version(final_flow_id.clone());
        let vs_id = test
            .insert_create_flow_version(
                final_flow_id.clone(),
                version_id.clone(),
                dummy_create_flow_version.clone(),
            )
            .await?;

        let flow_manager = FlowManager::new(&context, test.with_sender().await);

        let found = test.get_flow_versions(final_flow_id.clone()).await?;

        let req = Request::new(UpdateFlowVersionRequest {
            flow_id: final_flow_id.clone(),
            version_id: vs_id.clone(),
            update_flow_version: Some(crate::generated::UpdateFlowVersion {
                flow_definition: Some(r#"{"name": "rad new flow"}"#.to_string()),
                version: None,
                published: Some(true),
                description: None,
            }),
        });
        let res = flow_manager.update_flow_version(req).await;
        assert!(res.is_ok());
        let response = res.unwrap().into_inner();
        let flow = response.flow_version;
        assert!(flow.is_some());
        let flow_version = flow.unwrap();
        assert_eq!(
            flow_version.flow_definition,
            r#"{"name":"rad new flow"}"#.to_string()
        );

        let original_version = found.first().unwrap();
        assert_eq!(flow_version.version, original_version.flow_version);
        assert_ne!(flow_version.published, original_version.published);

        Ok(())
    }
}
