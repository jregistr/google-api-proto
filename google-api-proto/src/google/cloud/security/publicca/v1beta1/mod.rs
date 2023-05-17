/// A representation of an ExternalAccountKey used for [external account
/// binding](<https://tools.ietf.org/html/rfc8555#section-7.3.4>) within ACME.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalAccountKey {
    /// Output only. Resource name.
    /// projects/{project}/locations/{location}/externalAccountKeys/{key_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Key ID.
    /// It is generated by the PublicCertificateAuthorityService
    /// when the ExternalAccountKey is created
    #[prost(string, tag = "2")]
    pub key_id: ::prost::alloc::string::String,
    /// Output only. Base64-URL-encoded HS256 key.
    /// It is generated by the PublicCertificateAuthorityService
    /// when the ExternalAccountKey is created
    #[prost(bytes = "bytes", tag = "3")]
    pub b64_mac_key: ::prost::bytes::Bytes,
}
/// Creates a new \[ExternalAccountKey][google.cloud.security.publicca.v1beta1.ExternalAccountKey\] in a given project.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateExternalAccountKeyRequest {
    /// Required. The parent resource where this external_account_key will be created.
    /// Format: projects/\[project_id]/locations/[location\].
    /// At present only the "global" location is supported.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The external account key to create. This field only exists to future-proof
    /// the API. At present, all fields in ExternalAccountKey are output only and
    /// all values are ignored. For the purpose of the
    /// CreateExternalAccountKeyRequest, set it to a default/empty value.
    #[prost(message, optional, tag = "2")]
    pub external_account_key: ::core::option::Option<ExternalAccountKey>,
}
/// Generated client implementations.
pub mod public_certificate_authority_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
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
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
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
        /// Creates a new [ExternalAccountKey][google.cloud.security.publicca.v1beta1.ExternalAccountKey] bound to the project.
        pub async fn create_external_account_key(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateExternalAccountKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExternalAccountKey>,
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
                "/google.cloud.security.publicca.v1beta1.PublicCertificateAuthorityService/CreateExternalAccountKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.security.publicca.v1beta1.PublicCertificateAuthorityService",
                        "CreateExternalAccountKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
