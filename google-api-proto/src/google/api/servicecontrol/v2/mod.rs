/// Request message for the Check method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckRequest {
    /// The service name as specified in its service configuration. For example,
    /// `"pubsub.googleapis.com"`.
    ///
    /// See
    /// \[google.api.Service\](<https://cloud.google.com/service-management/reference/rpc/google.api#google.api.Service>)
    /// for the definition of a service name.
    #[prost(string, tag="1")]
    pub service_name: ::prost::alloc::string::String,
    /// Specifies the version of the service configuration that should be used to
    /// process the request. Must not be empty. Set this field to 'latest' to
    /// specify using the latest configuration.
    #[prost(string, tag="2")]
    pub service_config_id: ::prost::alloc::string::String,
    /// Describes attributes about the operation being executed by the service.
    #[prost(message, optional, tag="3")]
    pub attributes: ::core::option::Option<super::super::super::rpc::context::AttributeContext>,
    /// Describes the resources and the policies applied to each resource.
    #[prost(message, repeated, tag="4")]
    pub resources: ::prost::alloc::vec::Vec<ResourceInfo>,
    /// Optional. Contains a comma-separated list of flags.
    #[prost(string, tag="5")]
    pub flags: ::prost::alloc::string::String,
}
/// Describes a resource referenced in the request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceInfo {
    /// The name of the resource referenced in the request.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The resource type in the format of "{service}/{kind}".
    #[prost(string, tag="2")]
    pub r#type: ::prost::alloc::string::String,
    /// The resource permission needed for this request.
    /// The format must be "{service}/{plural}.{verb}".
    #[prost(string, tag="3")]
    pub permission: ::prost::alloc::string::String,
    /// Optional. The identifier of the container of this resource. For Google
    /// Cloud APIs, the resource container must be one of the following formats:
    ///      - `projects/<project-id or project-number>`
    ///      - `folders/<folder-id>`
    ///      - `organizations/<organization-id>`
    /// For the policy enforcement on the container level (VPCSC and Location
    /// Policy check), this field takes precedence on the container extracted from
    /// name when presents.
    #[prost(string, tag="4")]
    pub container: ::prost::alloc::string::String,
    /// Optional. The location of the resource. The value must be a valid zone,
    /// region or multiregion. For example: "europe-west4" or
    /// "northamerica-northeast1-a"
    #[prost(string, tag="5")]
    pub location: ::prost::alloc::string::String,
}
/// Response message for the Check method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckResponse {
    /// Operation is allowed when this field is not set. Any non-'OK' status
    /// indicates a denial; \[google.rpc.Status.details][google.rpc.Status.details\]
    /// would contain additional details about the denial.
    #[prost(message, optional, tag="1")]
    pub status: ::core::option::Option<super::super::super::rpc::Status>,
    /// Returns a set of request contexts generated from the `CheckRequest`.
    #[prost(btree_map="string, string", tag="2")]
    pub headers: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Request message for the Report method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportRequest {
    /// The service name as specified in its service configuration. For example,
    /// `"pubsub.googleapis.com"`.
    ///
    /// See
    /// \[google.api.Service\](<https://cloud.google.com/service-management/reference/rpc/google.api#google.api.Service>)
    /// for the definition of a service name.
    #[prost(string, tag="1")]
    pub service_name: ::prost::alloc::string::String,
    /// Specifies the version of the service configuration that should be used to
    /// process the request. Must not be empty. Set this field to 'latest' to
    /// specify using the latest configuration.
    #[prost(string, tag="2")]
    pub service_config_id: ::prost::alloc::string::String,
    /// Describes the list of operations to be reported. Each operation is
    /// represented as an AttributeContext, and contains all attributes around an
    /// API access.
    #[prost(message, repeated, tag="3")]
    pub operations: ::prost::alloc::vec::Vec<super::super::super::rpc::context::AttributeContext>,
}
/// Response message for the Report method.
/// If the request contains any invalid data, the server returns an RPC error.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportResponse {
}
/// Generated client implementations.
pub mod service_controller_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// [Service Control API
    /// v2](https://cloud.google.com/service-infrastructure/docs/service-control/access-control)
    ///
    /// Private Preview. This feature is only available for approved services.
    ///
    /// This API provides admission control and telemetry reporting for services
    /// that are integrated with [Service
    /// Infrastructure](https://cloud.google.com/service-infrastructure).
    #[derive(Debug, Clone)]
    pub struct ServiceControllerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ServiceControllerClient<T>
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
        ) -> ServiceControllerClient<InterceptedService<T, F>>
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
            ServiceControllerClient::new(InterceptedService::new(inner, interceptor))
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
        /// Private Preview. This feature is only available for approved services.
        ///
        /// This method provides admission control for services that are integrated
        /// with [Service
        /// Infrastructure](https://cloud.google.com/service-infrastructure). It checks
        /// whether an operation should be allowed based on the service configuration
        /// and relevant policies. It must be called before the operation is executed.
        /// For more information, see
        /// [Admission
        /// Control](https://cloud.google.com/service-infrastructure/docs/admission-control).
        ///
        /// NOTE: The admission control has an expected policy propagation delay of
        /// 60s. The caller **must** not depend on the most recent policy changes.
        ///
        /// NOTE: The admission control has a hard limit of 1 referenced resources
        /// per call. If an operation refers to more than 1 resources, the caller
        /// must call the Check method multiple times.
        ///
        /// This method requires the `servicemanagement.services.check` permission
        /// on the specified service. For more information, see
        /// [Service Control API Access
        /// Control](https://cloud.google.com/service-infrastructure/docs/service-control/access-control).
        pub async fn check(
            &mut self,
            request: impl tonic::IntoRequest<super::CheckRequest>,
        ) -> Result<tonic::Response<super::CheckResponse>, tonic::Status> {
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
                "/google.api.servicecontrol.v2.ServiceController/Check",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Private Preview. This feature is only available for approved services.
        ///
        /// This method provides telemetry reporting for services that are integrated
        /// with [Service
        /// Infrastructure](https://cloud.google.com/service-infrastructure). It
        /// reports a list of operations that have occurred on a service. It must be
        /// called after the operations have been executed. For more information, see
        /// [Telemetry
        /// Reporting](https://cloud.google.com/service-infrastructure/docs/telemetry-reporting).
        ///
        /// NOTE: The telemetry reporting has a hard limit of 1000 operations and 1MB
        /// per Report call. It is recommended to have no more than 100 operations per
        /// call.
        ///
        /// This method requires the `servicemanagement.services.report` permission
        /// on the specified service. For more information, see
        /// [Service Control API Access
        /// Control](https://cloud.google.com/service-infrastructure/docs/service-control/access-control).
        pub async fn report(
            &mut self,
            request: impl tonic::IntoRequest<super::ReportRequest>,
        ) -> Result<tonic::Response<super::ReportResponse>, tonic::Status> {
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
                "/google.api.servicecontrol.v2.ServiceController/Report",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
