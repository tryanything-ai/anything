#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Variable {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Node {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub label: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub action: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "5")]
    pub dependencies: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag = "6")]
    pub variables: ::prost::alloc::vec::Vec<Variable>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Flow {
    #[prost(string, tag = "1")]
    pub flow_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub flow_name: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub active: bool,
    #[prost(message, repeated, tag = "5")]
    pub flow_versions: ::prost::alloc::vec::Vec<FlowVersion>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFlow {
    #[prost(string, tag = "2")]
    pub flow_name: ::prost::alloc::string::String,
    #[prost(bool, optional, tag = "3")]
    pub active: ::core::option::Option<bool>,
    #[prost(oneof = "create_flow::Version", tags = "1")]
    pub version: ::core::option::Option<create_flow::Version>,
}
/// Nested message and enum types in `CreateFlow`.
pub mod create_flow {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Version {
        #[prost(string, tag = "1")]
        VersionString(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlowVersion {
    #[prost(string, tag = "1")]
    pub version_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub flow_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub flow_definition: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub published: bool,
    #[prost(int64, tag = "7")]
    pub updated_at: i64,
    #[prost(oneof = "flow_version::Description", tags = "6")]
    pub description: ::core::option::Option<flow_version::Description>,
}
/// Nested message and enum types in `FlowVersion`.
pub mod flow_version {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Description {
        #[prost(string, tag = "6")]
        Present(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFlowVersion {
    #[prost(string, tag = "1")]
    pub flow_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub flow_definition: ::prost::alloc::string::String,
    #[prost(bool, optional, tag = "5")]
    pub published: ::core::option::Option<bool>,
    #[prost(oneof = "create_flow_version::Version", tags = "2")]
    pub version: ::core::option::Option<create_flow_version::Version>,
    #[prost(oneof = "create_flow_version::Description", tags = "4")]
    pub description: ::core::option::Option<create_flow_version::Description>,
}
/// Nested message and enum types in `CreateFlowVersion`.
pub mod create_flow_version {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Version {
        #[prost(string, tag = "2")]
        VersionString(::prost::alloc::string::String),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Description {
        #[prost(string, tag = "4")]
        Present(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFlow {
    #[prost(string, optional, tag = "1")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "2")]
    pub flow_name: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, tag = "4")]
    pub active: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFlowVersion {
    #[prost(string, optional, tag = "1")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub flow_definition: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "3")]
    pub published: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "4")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
/// Create a flow
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFlowRequest {
    #[prost(message, optional, tag = "1")]
    pub flow: ::core::option::Option<CreateFlow>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFlowResponse {
    #[prost(message, optional, tag = "1")]
    pub flow: ::core::option::Option<Flow>,
}
/// Get all flows
/// maybe add filtering, pagination
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFlowsRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFlowsResponse {
    #[prost(message, repeated, tag = "1")]
    pub flows: ::prost::alloc::vec::Vec<Flow>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFlowRequest {
    #[prost(string, tag = "1")]
    pub flow_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFlowResponse {
    #[prost(message, optional, tag = "1")]
    pub flow: ::core::option::Option<Flow>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFlowRequest {
    #[prost(string, tag = "1")]
    pub flow_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub update_flow: ::core::option::Option<UpdateFlow>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFlowResponse {
    #[prost(message, optional, tag = "1")]
    pub flow: ::core::option::Option<Flow>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFlowVersionRequest {
    #[prost(string, tag = "1")]
    pub flow_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub version_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub update_flow_version: ::core::option::Option<UpdateFlowVersion>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFlowVersionResponse {
    #[prost(message, optional, tag = "1")]
    pub flow_version: ::core::option::Option<FlowVersion>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishFlowRequest {
    #[prost(string, tag = "1")]
    pub flow_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishFlowResponse {
    #[prost(message, optional, tag = "1")]
    pub flow: ::core::option::Option<Flow>,
}
/// Generated client implementations.
pub mod flows_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct FlowsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl FlowsServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> FlowsServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> FlowsServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            FlowsServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn create_flow(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFlowRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateFlowResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flows.FlowsService/CreateFlow",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("flows.FlowsService", "CreateFlow"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_flows(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFlowsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetFlowsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flows.FlowsService/GetFlows",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("flows.FlowsService", "GetFlows"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_flow(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFlowRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetFlowResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flows.FlowsService/GetFlow",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("flows.FlowsService", "GetFlow"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_flow(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFlowRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateFlowResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flows.FlowsService/UpdateFlow",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("flows.FlowsService", "UpdateFlow"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_flow_version(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFlowVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateFlowVersionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flows.FlowsService/UpdateFlowVersion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("flows.FlowsService", "UpdateFlowVersion"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod flows_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with FlowsServiceServer.
    #[async_trait]
    pub trait FlowsService: Send + Sync + 'static {
        async fn create_flow(
            &self,
            request: tonic::Request<super::CreateFlowRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateFlowResponse>,
            tonic::Status,
        >;
        async fn get_flows(
            &self,
            request: tonic::Request<super::GetFlowsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetFlowsResponse>,
            tonic::Status,
        >;
        async fn get_flow(
            &self,
            request: tonic::Request<super::GetFlowRequest>,
        ) -> std::result::Result<tonic::Response<super::GetFlowResponse>, tonic::Status>;
        async fn update_flow(
            &self,
            request: tonic::Request<super::UpdateFlowRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateFlowResponse>,
            tonic::Status,
        >;
        async fn update_flow_version(
            &self,
            request: tonic::Request<super::UpdateFlowVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateFlowVersionResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct FlowsServiceServer<T: FlowsService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: FlowsService> FlowsServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for FlowsServiceServer<T>
    where
        T: FlowsService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/flows.FlowsService/CreateFlow" => {
                    #[allow(non_camel_case_types)]
                    struct CreateFlowSvc<T: FlowsService>(pub Arc<T>);
                    impl<
                        T: FlowsService,
                    > tonic::server::UnaryService<super::CreateFlowRequest>
                    for CreateFlowSvc<T> {
                        type Response = super::CreateFlowResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateFlowRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FlowsService>::create_flow(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateFlowSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/flows.FlowsService/GetFlows" => {
                    #[allow(non_camel_case_types)]
                    struct GetFlowsSvc<T: FlowsService>(pub Arc<T>);
                    impl<
                        T: FlowsService,
                    > tonic::server::UnaryService<super::GetFlowsRequest>
                    for GetFlowsSvc<T> {
                        type Response = super::GetFlowsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetFlowsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FlowsService>::get_flows(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetFlowsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/flows.FlowsService/GetFlow" => {
                    #[allow(non_camel_case_types)]
                    struct GetFlowSvc<T: FlowsService>(pub Arc<T>);
                    impl<
                        T: FlowsService,
                    > tonic::server::UnaryService<super::GetFlowRequest>
                    for GetFlowSvc<T> {
                        type Response = super::GetFlowResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetFlowRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FlowsService>::get_flow(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetFlowSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/flows.FlowsService/UpdateFlow" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateFlowSvc<T: FlowsService>(pub Arc<T>);
                    impl<
                        T: FlowsService,
                    > tonic::server::UnaryService<super::UpdateFlowRequest>
                    for UpdateFlowSvc<T> {
                        type Response = super::UpdateFlowResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateFlowRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FlowsService>::update_flow(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateFlowSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/flows.FlowsService/UpdateFlowVersion" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateFlowVersionSvc<T: FlowsService>(pub Arc<T>);
                    impl<
                        T: FlowsService,
                    > tonic::server::UnaryService<super::UpdateFlowVersionRequest>
                    for UpdateFlowVersionSvc<T> {
                        type Response = super::UpdateFlowVersionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateFlowVersionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FlowsService>::update_flow_version(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateFlowVersionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: FlowsService> Clone for FlowsServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: FlowsService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: FlowsService> tonic::server::NamedService for FlowsServiceServer<T> {
        const NAME: &'static str = "flows.FlowsService";
    }
}
