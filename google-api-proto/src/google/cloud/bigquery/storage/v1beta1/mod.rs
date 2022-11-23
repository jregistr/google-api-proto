/// Table reference that includes just the 3 strings needed to identify a table.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableReference {
    /// The assigned project ID of the project.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// The ID of the dataset in the above project.
    #[prost(string, tag = "2")]
    pub dataset_id: ::prost::alloc::string::String,
    /// The ID of the table in the above dataset.
    #[prost(string, tag = "3")]
    pub table_id: ::prost::alloc::string::String,
}
/// All fields in this message optional.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableModifiers {
    /// The snapshot time of the table. If not set, interpreted as now.
    #[prost(message, optional, tag = "1")]
    pub snapshot_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Arrow schema.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArrowSchema {
    /// IPC serialized Arrow schema.
    #[prost(bytes = "bytes", tag = "1")]
    pub serialized_schema: ::prost::bytes::Bytes,
}
/// Arrow RecordBatch.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArrowRecordBatch {
    /// IPC serialized Arrow RecordBatch.
    #[prost(bytes = "bytes", tag = "1")]
    pub serialized_record_batch: ::prost::bytes::Bytes,
    /// The count of rows in the returning block.
    #[prost(int64, tag = "2")]
    pub row_count: i64,
}
/// Avro schema.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AvroSchema {
    /// Json serialized schema, as described at
    /// <https://avro.apache.org/docs/1.8.1/spec.html>
    #[prost(string, tag = "1")]
    pub schema: ::prost::alloc::string::String,
}
/// Avro rows.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AvroRows {
    /// Binary serialized rows in a block.
    #[prost(bytes = "bytes", tag = "1")]
    pub serialized_binary_rows: ::prost::bytes::Bytes,
    /// The count of rows in the returning block.
    #[prost(int64, tag = "2")]
    pub row_count: i64,
}
/// Options dictating how we read a table.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableReadOptions {
    /// Optional. Names of the fields in the table that should be read. If empty,
    /// all fields will be read. If the specified field is a nested field, all the
    /// sub-fields in the field will be selected. The output field order is
    /// unrelated to the order of fields in selected_fields.
    #[prost(string, repeated, tag = "1")]
    pub selected_fields: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. SQL text filtering statement, similar to a WHERE clause in
    /// a query. Aggregates are not supported.
    ///
    /// Examples: "int_field > 5"
    ///            "date_field = CAST('2014-9-27' as DATE)"
    ///            "nullable_field is not NULL"
    ///            "st_equals(geo_field, st_geofromtext("POINT(2, 2)"))"
    ///            "numeric_field BETWEEN 1.0 AND 5.0"
    #[prost(string, tag = "2")]
    pub row_restriction: ::prost::alloc::string::String,
}
/// Information about a single data stream within a read session.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stream {
    /// Name of the stream, in the form
    /// `projects/{project_id}/locations/{location}/streams/{stream_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Expresses a point within a given stream using an offset position.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamPosition {
    /// Identifier for a given Stream.
    #[prost(message, optional, tag = "1")]
    pub stream: ::core::option::Option<Stream>,
    /// Position in the stream.
    #[prost(int64, tag = "2")]
    pub offset: i64,
}
/// Information returned from a `CreateReadSession` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadSession {
    /// Unique identifier for the session, in the form
    /// `projects/{project_id}/locations/{location}/sessions/{session_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Time at which the session becomes invalid. After this time, subsequent
    /// requests to read this Session will return errors.
    #[prost(message, optional, tag = "2")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Streams associated with this session.
    #[prost(message, repeated, tag = "4")]
    pub streams: ::prost::alloc::vec::Vec<Stream>,
    /// Table that this ReadSession is reading from.
    #[prost(message, optional, tag = "7")]
    pub table_reference: ::core::option::Option<TableReference>,
    /// Any modifiers which are applied when reading from the specified table.
    #[prost(message, optional, tag = "8")]
    pub table_modifiers: ::core::option::Option<TableModifiers>,
    /// The strategy to use for distributing data among the streams.
    #[prost(enumeration = "ShardingStrategy", tag = "9")]
    pub sharding_strategy: i32,
    /// The schema for the read. If read_options.selected_fields is set, the
    /// schema may be different from the table schema as it will only contain
    /// the selected fields.
    #[prost(oneof = "read_session::Schema", tags = "5, 6")]
    pub schema: ::core::option::Option<read_session::Schema>,
}
/// Nested message and enum types in `ReadSession`.
pub mod read_session {
    /// The schema for the read. If read_options.selected_fields is set, the
    /// schema may be different from the table schema as it will only contain
    /// the selected fields.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Schema {
        /// Avro schema.
        #[prost(message, tag = "5")]
        AvroSchema(super::AvroSchema),
        /// Arrow schema.
        #[prost(message, tag = "6")]
        ArrowSchema(super::ArrowSchema),
    }
}
/// Creates a new read session, which may include additional options such as
/// requested parallelism, projection filters and constraints.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReadSessionRequest {
    /// Required. Reference to the table to read.
    #[prost(message, optional, tag = "1")]
    pub table_reference: ::core::option::Option<TableReference>,
    /// Required. String of the form `projects/{project_id}` indicating the
    /// project this ReadSession is associated with. This is the project that will
    /// be billed for usage.
    #[prost(string, tag = "6")]
    pub parent: ::prost::alloc::string::String,
    /// Any modifiers to the Table (e.g. snapshot timestamp).
    #[prost(message, optional, tag = "2")]
    pub table_modifiers: ::core::option::Option<TableModifiers>,
    /// Initial number of streams. If unset or 0, we will
    /// provide a value of streams so as to produce reasonable throughput. Must be
    /// non-negative. The number of streams may be lower than the requested number,
    /// depending on the amount parallelism that is reasonable for the table and
    /// the maximum amount of parallelism allowed by the system.
    ///
    /// Streams must be read starting from offset 0.
    #[prost(int32, tag = "3")]
    pub requested_streams: i32,
    /// Read options for this session (e.g. column selection, filters).
    #[prost(message, optional, tag = "4")]
    pub read_options: ::core::option::Option<TableReadOptions>,
    /// Data output format. Currently default to Avro.
    #[prost(enumeration = "DataFormat", tag = "5")]
    pub format: i32,
    /// The strategy to use for distributing data among multiple streams. Currently
    /// defaults to liquid sharding.
    #[prost(enumeration = "ShardingStrategy", tag = "7")]
    pub sharding_strategy: i32,
}
/// Requesting row data via `ReadRows` must provide Stream position information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRowsRequest {
    /// Required. Identifier of the position in the stream to start reading from.
    /// The offset requested must be less than the last row read from ReadRows.
    /// Requesting a larger offset is undefined.
    #[prost(message, optional, tag = "1")]
    pub read_position: ::core::option::Option<StreamPosition>,
}
/// Progress information for a given Stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamStatus {
    /// Number of estimated rows in the current stream. May change over time as
    /// different readers in the stream progress at rates which are relatively fast
    /// or slow.
    #[prost(int64, tag = "1")]
    pub estimated_row_count: i64,
    /// A value in the range [0.0, 1.0] that represents the fraction of rows
    /// assigned to this stream that have been processed by the server. In the
    /// presence of read filters, the server may process more rows than it returns,
    /// so this value reflects progress through the pre-filtering rows.
    ///
    /// This value is only populated for sessions created through the BALANCED
    /// sharding strategy.
    #[prost(float, tag = "2")]
    pub fraction_consumed: f32,
    /// Represents the progress of the current stream.
    #[prost(message, optional, tag = "4")]
    pub progress: ::core::option::Option<Progress>,
    /// Whether this stream can be split. For sessions that use the LIQUID sharding
    /// strategy, this value is always false. For BALANCED sessions, this value is
    /// false when enough data have been read such that no more splits are possible
    /// at that point or beyond. For small tables or streams that are the result of
    /// a chain of splits, this value may never be true.
    #[prost(bool, tag = "3")]
    pub is_splittable: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Progress {
    /// The fraction of rows assigned to the stream that have been processed by the
    /// server so far, not including the rows in the current response message.
    ///
    /// This value, along with `at_response_end`, can be used to interpolate the
    /// progress made as the rows in the message are being processed using the
    /// following formula: `at_response_start + (at_response_end -
    /// at_response_start) * rows_processed_from_response / rows_in_response`.
    ///
    /// Note that if a filter is provided, the `at_response_end` value of the
    /// previous response may not necessarily be equal to the `at_response_start`
    /// value of the current response.
    #[prost(float, tag = "1")]
    pub at_response_start: f32,
    /// Similar to `at_response_start`, except that this value includes the rows in
    /// the current response.
    #[prost(float, tag = "2")]
    pub at_response_end: f32,
}
/// Information on if the current connection is being throttled.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThrottleStatus {
    /// How much this connection is being throttled.
    /// 0 is no throttling, 100 is completely throttled.
    #[prost(int32, tag = "1")]
    pub throttle_percent: i32,
}
/// Response from calling `ReadRows` may include row data, progress and
/// throttling information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRowsResponse {
    /// Number of serialized rows in the rows block. This value is recorded here,
    /// in addition to the row_count values in the output-specific messages in
    /// `rows`, so that code which needs to record progress through the stream can
    /// do so in an output format-independent way.
    #[prost(int64, tag = "6")]
    pub row_count: i64,
    /// Estimated stream statistics.
    #[prost(message, optional, tag = "2")]
    pub status: ::core::option::Option<StreamStatus>,
    /// Throttling status. If unset, the latest response still describes
    /// the current throttling status.
    #[prost(message, optional, tag = "5")]
    pub throttle_status: ::core::option::Option<ThrottleStatus>,
    /// Row data is returned in format specified during session creation.
    #[prost(oneof = "read_rows_response::Rows", tags = "3, 4")]
    pub rows: ::core::option::Option<read_rows_response::Rows>,
}
/// Nested message and enum types in `ReadRowsResponse`.
pub mod read_rows_response {
    /// Row data is returned in format specified during session creation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Rows {
        /// Serialized row data in AVRO format.
        #[prost(message, tag = "3")]
        AvroRows(super::AvroRows),
        /// Serialized row data in Arrow RecordBatch format.
        #[prost(message, tag = "4")]
        ArrowRecordBatch(super::ArrowRecordBatch),
    }
}
/// Information needed to request additional streams for an established read
/// session.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateReadSessionStreamsRequest {
    /// Required. Must be a non-expired session obtained from a call to
    /// CreateReadSession. Only the name field needs to be set.
    #[prost(message, optional, tag = "1")]
    pub session: ::core::option::Option<ReadSession>,
    /// Required. Number of new streams requested. Must be positive.
    /// Number of added streams may be less than this, see CreateReadSessionRequest
    /// for more information.
    #[prost(int32, tag = "2")]
    pub requested_streams: i32,
}
/// The response from `BatchCreateReadSessionStreams` returns the stream
/// identifiers for the newly created streams.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateReadSessionStreamsResponse {
    /// Newly added streams.
    #[prost(message, repeated, tag = "1")]
    pub streams: ::prost::alloc::vec::Vec<Stream>,
}
/// Request information for invoking `FinalizeStream`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizeStreamRequest {
    /// Required. Stream to finalize.
    #[prost(message, optional, tag = "2")]
    pub stream: ::core::option::Option<Stream>,
}
/// Request information for `SplitReadStream`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplitReadStreamRequest {
    /// Required. Stream to split.
    #[prost(message, optional, tag = "1")]
    pub original_stream: ::core::option::Option<Stream>,
    /// A value in the range (0.0, 1.0) that specifies the fractional point at
    /// which the original stream should be split. The actual split point is
    /// evaluated on pre-filtered rows, so if a filter is provided, then there is
    /// no guarantee that the division of the rows between the new child streams
    /// will be proportional to this fractional value. Additionally, because the
    /// server-side unit for assigning data is collections of rows, this fraction
    /// will always map to to a data storage boundary on the server side.
    #[prost(float, tag = "2")]
    pub fraction: f32,
}
/// Response from `SplitReadStream`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplitReadStreamResponse {
    /// Primary stream, which contains the beginning portion of
    /// |original_stream|. An empty value indicates that the original stream can no
    /// longer be split.
    #[prost(message, optional, tag = "1")]
    pub primary_stream: ::core::option::Option<Stream>,
    /// Remainder stream, which contains the tail of |original_stream|. An empty
    /// value indicates that the original stream can no longer be split.
    #[prost(message, optional, tag = "2")]
    pub remainder_stream: ::core::option::Option<Stream>,
}
/// Data format for input or output data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DataFormat {
    /// Data format is unspecified.
    Unspecified = 0,
    /// Avro is a standard open source row based file format.
    /// See <https://avro.apache.org/> for more details.
    Avro = 1,
    Arrow = 3,
}
impl DataFormat {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DataFormat::Unspecified => "DATA_FORMAT_UNSPECIFIED",
            DataFormat::Avro => "AVRO",
            DataFormat::Arrow => "ARROW",
        }
    }
}
/// Strategy for distributing data among multiple streams in a read session.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ShardingStrategy {
    /// Same as LIQUID.
    Unspecified = 0,
    /// Assigns data to each stream based on the client's read rate. The faster the
    /// client reads from a stream, the more data is assigned to the stream. In
    /// this strategy, it's possible to read all data from a single stream even if
    /// there are other streams present.
    Liquid = 1,
    /// Assigns data to each stream such that roughly the same number of rows can
    /// be read from each stream. Because the server-side unit for assigning data
    /// is collections of rows, the API does not guarantee that each stream will
    /// return the same number or rows. Additionally, the limits are enforced based
    /// on the number of pre-filtering rows, so some filters can lead to lopsided
    /// assignments.
    Balanced = 2,
}
impl ShardingStrategy {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ShardingStrategy::Unspecified => "SHARDING_STRATEGY_UNSPECIFIED",
            ShardingStrategy::Liquid => "LIQUID",
            ShardingStrategy::Balanced => "BALANCED",
        }
    }
}
/// Generated client implementations.
pub mod big_query_storage_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// BigQuery storage API.
    ///
    /// The BigQuery storage API can be used to read data stored in BigQuery.
    #[derive(Debug, Clone)]
    pub struct BigQueryStorageClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BigQueryStorageClient<T>
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
        ) -> BigQueryStorageClient<InterceptedService<T, F>>
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
            BigQueryStorageClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a new read session. A read session divides the contents of a
        /// BigQuery table into one or more streams, which can then be used to read
        /// data from the table. The read session also specifies properties of the
        /// data to be read, such as a list of columns or a push-down filter describing
        /// the rows to be returned.
        ///
        /// A particular row can be read by at most one stream. When the caller has
        /// reached the end of each stream in the session, then all the data in the
        /// table has been read.
        ///
        /// Read sessions automatically expire 24 hours after they are created and do
        /// not require manual clean-up by the caller.
        pub async fn create_read_session(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateReadSessionRequest>,
        ) -> Result<tonic::Response<super::ReadSession>, tonic::Status> {
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
                "/google.cloud.bigquery.storage.v1beta1.BigQueryStorage/CreateReadSession",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Reads rows from the table in the format prescribed by the read session.
        /// Each response contains one or more table rows, up to a maximum of 10 MiB
        /// per response; read requests which attempt to read individual rows larger
        /// than this will fail.
        ///
        /// Each request also returns a set of stream statistics reflecting the
        /// estimated total number of rows in the read stream. This number is computed
        /// based on the total table size and the number of active streams in the read
        /// session, and may change as other streams continue to read data.
        pub async fn read_rows(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadRowsRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ReadRowsResponse>>,
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
                "/google.cloud.bigquery.storage.v1beta1.BigQueryStorage/ReadRows",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Creates additional streams for a ReadSession. This API can be used to
        /// dynamically adjust the parallelism of a batch processing task upwards by
        /// adding additional workers.
        pub async fn batch_create_read_session_streams(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchCreateReadSessionStreamsRequest>,
        ) -> Result<
            tonic::Response<super::BatchCreateReadSessionStreamsResponse>,
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
                "/google.cloud.bigquery.storage.v1beta1.BigQueryStorage/BatchCreateReadSessionStreams",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Triggers the graceful termination of a single stream in a ReadSession. This
        /// API can be used to dynamically adjust the parallelism of a batch processing
        /// task downwards without losing data.
        ///
        /// This API does not delete the stream -- it remains visible in the
        /// ReadSession, and any data processed by the stream is not released to other
        /// streams. However, no additional data will be assigned to the stream once
        /// this call completes. Callers must continue reading data on the stream until
        /// the end of the stream is reached so that data which has already been
        /// assigned to the stream will be processed.
        ///
        /// This method will return an error if there are no other live streams
        /// in the Session, or if SplitReadStream() has been called on the given
        /// Stream.
        pub async fn finalize_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::FinalizeStreamRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.bigquery.storage.v1beta1.BigQueryStorage/FinalizeStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Splits a given read stream into two Streams. These streams are referred to
        /// as the primary and the residual of the split. The original stream can still
        /// be read from in the same manner as before. Both of the returned streams can
        /// also be read from, and the total rows return by both child streams will be
        /// the same as the rows read from the original stream.
        ///
        /// Moreover, the two child streams will be allocated back to back in the
        /// original Stream. Concretely, it is guaranteed that for streams Original,
        /// Primary, and Residual, that Original[0-j] = Primary[0-j] and
        /// Original[j-n] = Residual[0-m] once the streams have been read to
        /// completion.
        ///
        /// This method is guaranteed to be idempotent.
        pub async fn split_read_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::SplitReadStreamRequest>,
        ) -> Result<tonic::Response<super::SplitReadStreamResponse>, tonic::Status> {
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
                "/google.cloud.bigquery.storage.v1beta1.BigQueryStorage/SplitReadStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
