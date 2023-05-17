/// The request message for \[Locations.ListLocations][google.cloud.location.Locations.ListLocations\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLocationsRequest {
    /// The resource that owns the locations collection, if applicable.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The standard list filter.
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// The standard list page size.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The standard list page token.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for \[Locations.ListLocations][google.cloud.location.Locations.ListLocations\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLocationsResponse {
    /// A list of locations that matches the specified filter in the request.
    #[prost(message, repeated, tag = "1")]
    pub locations: ::prost::alloc::vec::Vec<Location>,
    /// The standard List next-page token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for \[Locations.GetLocation][google.cloud.location.Locations.GetLocation\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLocationRequest {
    /// Resource name for the location.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A resource that represents Google Cloud Platform location.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Location {
    /// Resource name for the location, which may vary between implementations.
    /// For example: `"projects/example-project/locations/us-east1"`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The canonical id for this location. For example: `"us-east1"`.
    #[prost(string, tag = "4")]
    pub location_id: ::prost::alloc::string::String,
    /// The friendly name for this location, typically a nearby city name.
    /// For example, "Tokyo".
    #[prost(string, tag = "5")]
    pub display_name: ::prost::alloc::string::String,
    /// Cross-service attributes for the location. For example
    ///
    ///      {"cloud.googleapis.com/region": "us-east1"}
    #[prost(btree_map = "string, string", tag = "2")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Service-specific metadata. For example the available capacity at the given
    /// location.
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<::prost_types::Any>,
}
/// Generated client implementations.
pub mod locations_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// An abstract interface that provides location-related information for
    /// a service. Service-specific metadata is provided through the
    /// [Location.metadata][google.cloud.location.Location.metadata] field.
    #[derive(Debug, Clone)]
    pub struct LocationsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> LocationsClient<T>
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
        ) -> LocationsClient<InterceptedService<T, F>>
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
            LocationsClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists information about the supported locations for this service.
        pub async fn list_locations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLocationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListLocationsResponse>,
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
                "/google.cloud.location.Locations/ListLocations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.location.Locations", "ListLocations"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets information about a location.
        pub async fn get_location(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLocationRequest>,
        ) -> std::result::Result<tonic::Response<super::Location>, tonic::Status> {
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
                "/google.cloud.location.Locations/GetLocation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.location.Locations", "GetLocation"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
