/// A representation of an ExternalAccountKey used for [external account
/// binding](<https://tools.ietf.org/html/rfc8555#section-7.3.4>) within ACME.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalAccountKey {
    /// Output only. Resource name.
    /// projects/{project}/locations/{location}/externalAccountKeys/{key_id}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Key ID.
    /// It is generated by the PublicCertificateAuthorityService
    /// when the ExternalAccountKey is created
    #[prost(string, tag="2")]
    pub key_id: ::prost::alloc::string::String,
    /// Output only. Base64-URL-encoded HS256 key.
    /// It is generated by the PublicCertificateAuthorityService
    /// when the ExternalAccountKey is created
    #[prost(bytes="bytes", tag="3")]
    pub b64_mac_key: ::prost::bytes::Bytes,
}
/// Creates a new
/// \[ExternalAccountKey][google.cloud.security.publicca.v1beta1.ExternalAccountKey\]
/// in a given project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateExternalAccountKeyRequest {
    /// Required. The parent resource where this external_account_key will be
    /// created. Format:
    /// projects/\[project_id]/locations/[location\]/externalAccountKeys.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The external account key to create.
    #[prost(message, optional, tag="2")]
    pub external_account_key: ::core::option::Option<ExternalAccountKey>,
}
/// Generated client implementations.
pub mod public_certificate_authority_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Manages the resources required for ACME [external account
    /// binding](https://tools.ietf.org/html/rfc8555#section-7.3.4) for
    /// the public certificate authority service.
    #[derive(Debug, Clone)]
    pub struct PublicCertificateAuthorityServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> PublicCertificateAuthorityServiceClient<T>
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
        ) -> PublicCertificateAuthorityServiceClient<InterceptedService<T, F>>
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
            PublicCertificateAuthorityServiceClient::new(
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
        /// Creates a new
        /// [ExternalAccountKey][google.cloud.security.publicca.v1beta1.ExternalAccountKey]
        /// bound to the project.
        pub async fn create_external_account_key(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateExternalAccountKeyRequest>,
        ) -> Result<tonic::Response<super::ExternalAccountKey>, tonic::Status> {
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
                "/google.cloud.security.publicca.v1beta1.PublicCertificateAuthorityService/CreateExternalAccountKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
