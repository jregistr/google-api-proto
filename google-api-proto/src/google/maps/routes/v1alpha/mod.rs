/// Generated client implementations.
pub mod routes_alpha_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// The Routes Preferred API.
    #[derive(Debug, Clone)]
    pub struct RoutesAlphaClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> RoutesAlphaClient<T>
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
        ) -> RoutesAlphaClient<InterceptedService<T, F>>
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
            RoutesAlphaClient::new(InterceptedService::new(inner, interceptor))
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
        /// Returns the primary route along with optional alternate routes, given a set
        /// of terminal and intermediate waypoints.
        ///
        /// **NOTE:** This method requires that you specify a response field mask in
        /// the input. You can provide the response field mask by using the URL
        /// parameter `$fields` or `fields`, or by using the HTTP/gRPC header
        /// `X-Goog-FieldMask` (see the [available URL parameters and
        /// headers](https://cloud.google.com/apis/docs/system-parameters). The value
        /// is a comma separated list of field paths. See this detailed documentation
        /// about [how to construct the field
        /// paths](https://github.com/protocolbuffers/protobuf/blob/master/src/google/protobuf/field_mask.proto).
        ///
        /// For example, in this method:
        ///
        /// * Field mask of all available fields (for manual inspection):
        ///   `X-Goog-FieldMask: *`
        /// * Field mask of Route-level duration, distance, and polyline (an example
        /// production setup):
        ///   `X-Goog-FieldMask:
        ///   routes.duration,routes.distanceMeters,routes.polyline.encodedPolyline`
        ///
        /// Google discourages the use of the wildcard (`*`) response field mask, or
        /// specifying the field mask at the top level (`routes`), because:
        ///
        /// * Selecting only the fields that you need helps our server save computation
        /// cycles, allowing us to return the result to you with a lower latency.
        /// * Selecting only the fields that you need in your production job ensures
        /// stable latency performance. We might add more response fields in the
        /// future, and those new fields might require extra computation time. If you
        /// select all fields, or if you select all fields at the top level, then you
        /// might experience performance degradation because any new field we add will
        /// be automatically included in the response.
        /// * Selecting only the fields that you need results in a smaller response
        /// size, and thus higher network throughput.
        pub async fn compute_routes(
            &mut self,
            request: impl tonic::IntoRequest<super::super::v1::ComputeRoutesRequest>,
        ) -> Result<
                tonic::Response<super::super::v1::ComputeRoutesResponse>,
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
                "/google.maps.routes.v1alpha.RoutesAlpha/ComputeRoutes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Takes in a list of origins and destinations and returns a stream containing
        /// route information for each combination of origin and destination.
        ///
        /// **NOTE:** This method requires that you specify a response field mask in
        /// the input. You can provide the response field mask by using the URL
        /// parameter `$fields` or `fields`, or by using the HTTP/gRPC header
        /// `X-Goog-FieldMask` (see the [available URL parameters and
        /// headers](https://cloud.google.com/apis/docs/system-parameters). The value
        /// is a comma separated list of field paths. See this detailed documentation
        /// about [how to construct the field
        /// paths](https://github.com/protocolbuffers/protobuf/blob/master/src/google/protobuf/field_mask.proto).
        ///
        /// For example, in this method:
        ///
        /// * Field mask of all available fields (for manual inspection):
        ///   `X-Goog-FieldMask: *`
        /// * Field mask of route durations, distances, element status, condition, and
        ///   element indices (an example production setup):
        ///   `X-Goog-FieldMask:
        ///   originIndex,destinationIndex,status,condition,distanceMeters,duration`
        ///
        /// It is critical that you include `status` in your field mask as otherwise
        /// all messages will appear to be OK. Google discourages the use of the
        /// wildcard (`*`) response field mask, because:
        ///
        /// * Selecting only the fields that you need helps our server save computation
        /// cycles, allowing us to return the result to you with a lower latency.
        /// * Selecting only the fields that you need in your production job ensures
        /// stable latency performance. We might add more response fields in the
        /// future, and those new fields might require extra computation time. If you
        /// select all fields, or if you select all fields at the top level, then you
        /// might experience performance degradation because any new field we add will
        /// be automatically included in the response.
        /// * Selecting only the fields that you need results in a smaller response
        /// size, and thus higher network throughput.
        pub async fn compute_route_matrix(
            &mut self,
            request: impl tonic::IntoRequest<super::super::v1::ComputeRouteMatrixRequest>,
        ) -> Result<
                tonic::Response<
                    tonic::codec::Streaming<super::super::v1::RouteMatrixElement>,
                >,
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
                "/google.maps.routes.v1alpha.RoutesAlpha/ComputeRouteMatrix",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Given a set of terminal and intermediate waypoints, and a route objective,
        /// computes the best route for the route objective. Also returns fastest route
        /// and shortest route as reference routes.
        ///
        /// **NOTE:** This method requires that you specify a response field mask in
        /// the input. You can provide the response field mask by using the URL
        /// parameter `$fields` or `fields`, or by using the HTTP/gRPC header
        /// `X-Goog-FieldMask` (see the [available URL parameters and
        /// headers](https://cloud.google.com/apis/docs/system-parameters). The value
        /// is a comma separated list of field paths. See this detailed documentation
        /// about [how to construct the field
        /// paths](https://github.com/protocolbuffers/protobuf/blob/master/src/google/protobuf/field_mask.proto).
        ///
        /// For example, in this method:
        ///
        /// * Field mask of all available fields (for manual inspection):
        ///   `X-Goog-FieldMask: *`
        /// * Field mask of route distances, durations, token and toll info:
        ///   `X-Goog-FieldMask:
        ///   routes.route.distanceMeters,routes.route.duration,routes.token,routes.route.travelAdvisory.tollInfo`
        ///
        /// Google discourages the use of the wildcard (`*`) response field mask, or
        /// specifying the field mask at the top level (`routes`), because:
        ///
        /// * Selecting only the fields that you need helps our server save computation
        /// cycles, allowing us to return the result to you with a lower latency.
        /// * Selecting only the fields that you need in your production job ensures
        /// stable latency performance. We might add more response fields in the
        /// future, and those new fields might require extra computation time. If you
        /// select all fields, or if you select all fields at the top level, then you
        /// might experience performance degradation because any new field we add will
        /// be automatically included in the response.
        /// * Selecting only the fields that you need results in a smaller response
        /// size, and thus higher network throughput.
        pub async fn compute_custom_routes(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::v1::ComputeCustomRoutesRequest,
            >,
        ) -> Result<
                tonic::Response<super::super::v1::ComputeCustomRoutesResponse>,
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
                "/google.maps.routes.v1alpha.RoutesAlpha/ComputeCustomRoutes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
