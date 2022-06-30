/// Message describing ClientConnectorService object.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientConnectorService {
    /// Required. Name of resource. The name is ignored during creation.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. [Output only] Create time stamp.
    #[prost(message, optional, tag="2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. [Output only] Update time stamp.
    #[prost(message, optional, tag="3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. User-provided name.
    /// The display name should follow certain format.
    /// * Must be 6 to 30 characters in length.
    /// * Can only contain lowercase letters, numbers, and hyphens.
    /// * Must start with a letter.
    #[prost(string, tag="4")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. The details of the ingress settings.
    #[prost(message, optional, tag="6")]
    pub ingress: ::core::option::Option<client_connector_service::Ingress>,
    /// Required. The details of the egress settings.
    #[prost(message, optional, tag="7")]
    pub egress: ::core::option::Option<client_connector_service::Egress>,
    /// Output only. The operational state of the ClientConnectorService.
    #[prost(enumeration="client_connector_service::State", tag="8")]
    pub state: i32,
}
/// Nested message and enum types in `ClientConnectorService`.
pub mod client_connector_service {
    /// Settings of how to connect to the ClientGateway.
    /// One of the following options should be set.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Ingress {
        #[prost(oneof="ingress::IngressConfig", tags="1")]
        pub ingress_config: ::core::option::Option<ingress::IngressConfig>,
    }
    /// Nested message and enum types in `Ingress`.
    pub mod ingress {
        /// The basic ingress config for ClientGateways.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Config {
            /// Required. Immutable. The transport protocol used between the client and
            /// the server.
            #[prost(enumeration="config::TransportProtocol", tag="1")]
            pub transport_protocol: i32,
            /// Required. The settings used to configure basic ClientGateways.
            #[prost(message, repeated, tag="2")]
            pub destination_routes: ::prost::alloc::vec::Vec<config::DestinationRoute>,
        }
        /// Nested message and enum types in `Config`.
        pub mod config {
            /// The setting used to configure ClientGateways.
            /// It is adding routes to the client's routing table
            /// after the connection is established.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct DestinationRoute {
                /// Required. The network address of the subnet
                /// for which the packet is routed to the ClientGateway.
                #[prost(string, tag="1")]
                pub address: ::prost::alloc::string::String,
                /// Required. The network mask of the subnet
                /// for which the packet is routed to the ClientGateway.
                #[prost(string, tag="2")]
                pub netmask: ::prost::alloc::string::String,
            }
            /// The protocol used to connect to the server.
            #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
            #[repr(i32)]
            pub enum TransportProtocol {
                /// Default value. This value is unused.
                Unspecified = 0,
                /// TCP protocol.
                Tcp = 1,
            }
        }
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum IngressConfig {
            /// The basic ingress config for ClientGateways.
            #[prost(message, tag="1")]
            Config(Config),
        }
    }
    /// The details of the egress info. One of the following options should be set.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Egress {
        #[prost(oneof="egress::DestinationType", tags="1")]
        pub destination_type: ::core::option::Option<egress::DestinationType>,
    }
    /// Nested message and enum types in `Egress`.
    pub mod egress {
        /// The peered VPC owned by the consumer project.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PeeredVpc {
            /// Required. The name of the peered VPC owned by the consumer project.
            #[prost(string, tag="1")]
            pub network_vpc: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum DestinationType {
            /// A VPC from the consumer project.
            #[prost(message, tag="1")]
            PeeredVpc(PeeredVpc),
        }
    }
    /// Represents the different states of a ClientConnectorService.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// ClientConnectorService is being created.
        Creating = 1,
        /// ClientConnectorService is being updated.
        Updating = 2,
        /// ClientConnectorService is being deleted.
        Deleting = 3,
        /// ClientConnectorService is running.
        Running = 4,
        /// ClientConnectorService is down and may be restored in the future.
        /// This happens when CCFE sends ProjectState = OFF.
        Down = 5,
        /// ClientConnectorService encountered an error and is in an indeterministic
        /// state.
        Error = 6,
    }
}
/// Message for requesting list of ClientConnectorServices.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClientConnectorServicesRequest {
    /// Required. Parent value for ListClientConnectorServicesRequest.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server may return fewer items than
    /// requested. If unspecified, server will pick an appropriate default.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results the server should return.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filtering results.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Hint for how to order the results.
    #[prost(string, tag="5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Message for response to listing ClientConnectorServices.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClientConnectorServicesResponse {
    /// The list of ClientConnectorService.
    #[prost(message, repeated, tag="1")]
    pub client_connector_services: ::prost::alloc::vec::Vec<ClientConnectorService>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag="3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a ClientConnectorService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClientConnectorServiceRequest {
    /// Required. Name of the resource.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for creating a ClientConnectorService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateClientConnectorServiceRequest {
    /// Required. Value for parent.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. User-settable client connector service resource ID.
    ///  * Must start with a letter.
    ///  * Must contain between 4-63 characters from `/\[a-z][0-9\]-/`.
    ///  * Must end with a number or a letter.
    ///
    /// A random system generated name will be assigned
    /// if not specified by the user.
    #[prost(string, tag="2")]
    pub client_connector_service_id: ::prost::alloc::string::String,
    /// Required. The resource being created.
    #[prost(message, optional, tag="3")]
    pub client_connector_service: ::core::option::Option<ClientConnectorService>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag="4")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set, validates request by executing a dry-run which would not
    /// alter the resource in any way.
    #[prost(bool, tag="5")]
    pub validate_only: bool,
}
/// Message for updating a ClientConnectorService
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateClientConnectorServiceRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// ClientConnectorService resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    ///
    /// Mutable fields: display_name.
    #[prost(message, optional, tag="1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The resource being updated.
    #[prost(message, optional, tag="2")]
    pub client_connector_service: ::core::option::Option<ClientConnectorService>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag="3")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set, validates request by executing a dry-run which would not
    /// alter the resource in any way.
    #[prost(bool, tag="4")]
    pub validate_only: bool,
    /// Optional. If set as true, will create the resource if it is not found.
    #[prost(bool, tag="5")]
    pub allow_missing: bool,
}
/// Message for deleting a ClientConnectorService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteClientConnectorServiceRequest {
    /// Required. Name of the resource.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag="2")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set, validates request by executing a dry-run which would not
    /// alter the resource in any way.
    #[prost(bool, tag="3")]
    pub validate_only: bool,
}
/// Represents the metadata of the long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientConnectorServiceOperationMetadata {
    /// Output only. The time the operation was created.
    #[prost(message, optional, tag="1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the operation finished running.
    #[prost(message, optional, tag="2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Server-defined resource path for the target of the operation.
    #[prost(string, tag="3")]
    pub target: ::prost::alloc::string::String,
    /// Output only. Name of the verb executed by the operation.
    #[prost(string, tag="4")]
    pub verb: ::prost::alloc::string::String,
    /// Output only. Human-readable status of the operation, if any.
    #[prost(string, tag="5")]
    pub status_message: ::prost::alloc::string::String,
    /// Output only. Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have \[Operation.error][\] value with a
    /// \[google.rpc.Status.code][google.rpc.Status.code\] of 1, corresponding to
    /// `Code.CANCELLED`.
    #[prost(bool, tag="6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag="7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod client_connector_services_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// ## API Overview
    ///
    /// The `beyondcorp.googleapis.com` service implements the Google Cloud
    /// BeyondCorp API.
    ///
    /// ## Data Model
    ///
    /// The ClientConnectorServicesService exposes the following resources:
    ///
    /// * Client Connector Services, named as follows:
    ///   `projects/{project_id}/locations/{location_id}/client_connector_services/{client_connector_service_id}`.
    #[derive(Debug, Clone)]
    pub struct ClientConnectorServicesServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ClientConnectorServicesServiceClient<T>
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
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ClientConnectorServicesServiceClient<InterceptedService<T, F>>
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
            ClientConnectorServicesServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// Lists ClientConnectorServices in a given project and location.
        pub async fn list_client_connector_services(
            &mut self,
            request: impl tonic::IntoRequest<super::ListClientConnectorServicesRequest>,
        ) -> Result<
            tonic::Response<super::ListClientConnectorServicesResponse>,
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
                "/google.cloud.beyondcorp.clientconnectorservices.v1.ClientConnectorServicesService/ListClientConnectorServices",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single ClientConnectorService.
        pub async fn get_client_connector_service(
            &mut self,
            request: impl tonic::IntoRequest<super::GetClientConnectorServiceRequest>,
        ) -> Result<tonic::Response<super::ClientConnectorService>, tonic::Status> {
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
                "/google.cloud.beyondcorp.clientconnectorservices.v1.ClientConnectorServicesService/GetClientConnectorService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new ClientConnectorService in a given project and location.
        pub async fn create_client_connector_service(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateClientConnectorServiceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.beyondcorp.clientconnectorservices.v1.ClientConnectorServicesService/CreateClientConnectorService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the parameters of a single ClientConnectorService.
        pub async fn update_client_connector_service(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateClientConnectorServiceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.beyondcorp.clientconnectorservices.v1.ClientConnectorServicesService/UpdateClientConnectorService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single ClientConnectorService.
        pub async fn delete_client_connector_service(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteClientConnectorServiceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.beyondcorp.clientconnectorservices.v1.ClientConnectorServicesService/DeleteClientConnectorService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}