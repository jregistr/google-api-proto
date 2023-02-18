/// The descriptor for a gstreamer buffer payload.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GstreamerBufferDescriptor {
    /// The caps string of the payload.
    #[prost(string, tag = "1")]
    pub caps_string: ::prost::alloc::string::String,
    /// Whether the buffer is a key frame.
    #[prost(bool, tag = "2")]
    pub is_key_frame: bool,
    /// PTS of the frame.
    #[prost(message, optional, tag = "3")]
    pub pts_time: ::core::option::Option<::prost_types::Timestamp>,
    /// DTS of the frame.
    #[prost(message, optional, tag = "4")]
    pub dts_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Duration of the frame.
    #[prost(message, optional, tag = "5")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
}
/// The descriptor for a raw image.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawImageDescriptor {
    /// Raw image format. Its possible values are: "srgb".
    #[prost(string, tag = "1")]
    pub format: ::prost::alloc::string::String,
    /// The height of the image.
    #[prost(int32, tag = "2")]
    pub height: i32,
    /// The width of the image.
    #[prost(int32, tag = "3")]
    pub width: i32,
}
/// The message that represents the data type of a packet.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PacketType {
    /// The type class of the packet. Its possible values are:
    /// "gst", "protobuf", and "string".
    #[prost(string, tag = "1")]
    pub type_class: ::prost::alloc::string::String,
    /// The type descriptor.
    #[prost(message, optional, tag = "2")]
    pub type_descriptor: ::core::option::Option<packet_type::TypeDescriptor>,
}
/// Nested message and enum types in `PacketType`.
pub mod packet_type {
    /// The message that fully specifies the type of the packet.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TypeDescriptor {
        /// The type of the packet. Its possible values is codec dependent.
        ///
        /// The fully qualified type name is always the concatenation of the
        /// value in `type_class` together with the value in `type`, separated by a
        /// '/'.
        ///
        /// Note that specific codecs can define their own type hierarchy, and so the
        /// type string here can in fact be separated by multiple '/'s of its own.
        ///
        /// Please see the open source SDK for specific codec documentation.
        #[prost(string, tag = "1")]
        pub r#type: ::prost::alloc::string::String,
        /// Detailed information about the type.
        ///
        /// It is non-empty only for specific type class codecs. Needed only when the
        /// type string alone is not enough to disambiguate the specific type.
        #[prost(oneof = "type_descriptor::TypeDetails", tags = "2, 3")]
        pub type_details: ::core::option::Option<type_descriptor::TypeDetails>,
    }
    /// Nested message and enum types in `TypeDescriptor`.
    pub mod type_descriptor {
        /// Detailed information about the type.
        ///
        /// It is non-empty only for specific type class codecs. Needed only when the
        /// type string alone is not enough to disambiguate the specific type.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum TypeDetails {
            /// GstreamerBufferDescriptor is the descriptor for gstreamer buffer type.
            #[prost(message, tag = "2")]
            GstreamerBufferDescriptor(super::super::GstreamerBufferDescriptor),
            /// RawImageDescriptor is the descriptor for the raw image type.
            #[prost(message, tag = "3")]
            RawImageDescriptor(super::super::RawImageDescriptor),
        }
    }
}
/// The message that represents server metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerMetadata {
    /// The offset position for the packet in its stream.
    #[prost(int64, tag = "1")]
    pub offset: i64,
    /// The timestamp at which the stream server receives this packet. This is
    /// based on the local clock of on the server side. It is guaranteed to be
    /// monotonically increasing for the packets within each session; however
    /// this timestamp is not comparable across packets sent to the same stream
    /// different sessions. Session here refers to one individual gRPC streaming
    /// request to the stream server.
    #[prost(message, optional, tag = "2")]
    pub ingest_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// The message that represents series metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeriesMetadata {
    /// Series name. It's in the format of
    /// "projects/{project}/locations/{location}/clusters/{cluster}/series/{stream}".
    #[prost(string, tag = "1")]
    pub series: ::prost::alloc::string::String,
}
/// The message that represents packet header.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PacketHeader {
    /// Input only. The capture time of the packet.
    #[prost(message, optional, tag = "1")]
    pub capture_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Input only. Immutable. The type of the payload.
    #[prost(message, optional, tag = "2")]
    pub r#type: ::core::option::Option<PacketType>,
    /// Input only. This field is for users to attach user managed metadata.
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<::prost_types::Struct>,
    /// Output only. Metadata that the server appends to each packet before sending
    /// it to receivers. You don't need to set a value for this field when sending
    /// packets.
    #[prost(message, optional, tag = "4")]
    pub server_metadata: ::core::option::Option<ServerMetadata>,
    /// Input only. Immutable. Metadata that the server needs to know where to
    /// write the packets to. It's only required for the first packet.
    #[prost(message, optional, tag = "5")]
    pub series_metadata: ::core::option::Option<SeriesMetadata>,
    /// Immutable. Packet flag set. SDK will set the flag automatically.
    #[prost(int32, tag = "6")]
    pub flags: i32,
    /// Immutable. Header string for tracing across services. It should be set when the packet
    /// is first arrived in the stream server.
    ///
    /// The input format is a lowercase hex string:
    ///    - version_id: 1 byte, currently must be zero - hex encoded (2 characters)
    ///    - trace_id: 16 bytes (opaque blob) - hex encoded (32 characters)
    ///    - span_id: 8 bytes (opaque blob) - hex encoded (16 characters)
    ///    - trace_options: 1 byte (LSB means tracing enabled) - hex encoded (2
    ///    characters)
    /// Example: "00-404142434445464748494a4b4c4d4e4f-6162636465666768-01"
    ///            v  trace_id                         span_id          options
    #[prost(string, tag = "7")]
    pub trace_context: ::prost::alloc::string::String,
}
/// The quanta of datum that the series accepts.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Packet {
    /// The packet header.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<PacketHeader>,
    /// The payload of the packet.
    #[prost(bytes = "bytes", tag = "2")]
    pub payload: ::prost::bytes::Bytes,
}
/// Request message for ReceiveEvents.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceiveEventsRequest {
    #[prost(oneof = "receive_events_request::Request", tags = "1, 2")]
    pub request: ::core::option::Option<receive_events_request::Request>,
}
/// Nested message and enum types in `ReceiveEventsRequest`.
pub mod receive_events_request {
    /// SetupRequest is the first message sent to the service to setup the RPC
    /// connection.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetupRequest {
        /// The cluster name.
        #[prost(string, tag = "1")]
        pub cluster: ::prost::alloc::string::String,
        /// The stream name. The service will return the events for the given stream.
        #[prost(string, tag = "2")]
        pub stream: ::prost::alloc::string::String,
        /// A name for the receiver to self-identify.
        ///
        /// This is used to keep track of a receiver's read progress.
        #[prost(string, tag = "3")]
        pub receiver: ::prost::alloc::string::String,
        /// Controller mode configuration for receiving events from the server.
        #[prost(message, optional, tag = "4")]
        pub controlled_mode: ::core::option::Option<super::ControlledMode>,
        /// The maximum duration of server silence before the client determines the
        /// server unreachable.
        ///
        /// The client must either receive an `Event` update or a heart beat message
        /// before this duration expires; otherwise, the client will automatically
        /// cancel the current connection and retry.
        #[prost(message, optional, tag = "5")]
        pub heartbeat_interval: ::core::option::Option<::prost_types::Duration>,
        /// The grace period after which a `writes_done_request` is issued, that a
        /// `WritesDone` is expected from the client.
        ///
        /// The server is free to cancel the RPC should this expire.
        ///
        /// A system default will be chosen if unset.
        #[prost(message, optional, tag = "6")]
        pub writes_done_grace_period: ::core::option::Option<::prost_types::Duration>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        /// The setup request to setup the RPC connection.
        #[prost(message, tag = "1")]
        SetupRequest(SetupRequest),
        /// This request checkpoints the consumer's read progress.
        #[prost(message, tag = "2")]
        CommitRequest(super::CommitRequest),
    }
}
/// The event update message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventUpdate {
    /// The name of the stream that the event is attached to.
    #[prost(string, tag = "1")]
    pub stream: ::prost::alloc::string::String,
    /// The name of the event.
    #[prost(string, tag = "2")]
    pub event: ::prost::alloc::string::String,
    /// The name of the series.
    #[prost(string, tag = "3")]
    pub series: ::prost::alloc::string::String,
    /// The timestamp when the Event update happens.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The offset of the message that will be used to acknowledge of the message
    /// receiving.
    #[prost(int64, tag = "5")]
    pub offset: i64,
}
/// Control message for a ReceiveEventsResponse.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceiveEventsControlResponse {
    /// Possible control messages.
    #[prost(oneof = "receive_events_control_response::Control", tags = "1, 2")]
    pub control: ::core::option::Option<receive_events_control_response::Control>,
}
/// Nested message and enum types in `ReceiveEventsControlResponse`.
pub mod receive_events_control_response {
    /// Possible control messages.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Control {
        /// A server heartbeat.
        #[prost(bool, tag = "1")]
        Heartbeat(bool),
        /// A request to the receiver to complete any final writes followed by a
        /// `WritesDone`; e.g. issue any final `CommitRequest`s.
        ///
        /// May be ignored if `WritesDone` has already been issued at any point
        /// prior to receiving this message.
        ///
        /// If `WritesDone` does not get issued, then the server will forcefully
        /// cancel the connection, and the receiver will likely receive an
        /// uninformative after `Read` returns `false` and `Finish` is called.
        #[prost(bool, tag = "2")]
        WritesDoneRequest(bool),
    }
}
/// Response message for the ReceiveEvents.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceiveEventsResponse {
    /// Possible response types.
    #[prost(oneof = "receive_events_response::Response", tags = "1, 2")]
    pub response: ::core::option::Option<receive_events_response::Response>,
}
/// Nested message and enum types in `ReceiveEventsResponse`.
pub mod receive_events_response {
    /// Possible response types.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        /// The event update message.
        #[prost(message, tag = "1")]
        EventUpdate(super::EventUpdate),
        /// A control message from the server.
        #[prost(message, tag = "2")]
        Control(super::ReceiveEventsControlResponse),
    }
}
/// The lease message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Lease {
    /// The lease id.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The series name.
    #[prost(string, tag = "2")]
    pub series: ::prost::alloc::string::String,
    /// The owner name.
    #[prost(string, tag = "3")]
    pub owner: ::prost::alloc::string::String,
    /// The lease expire time.
    #[prost(message, optional, tag = "4")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The lease type.
    #[prost(enumeration = "LeaseType", tag = "5")]
    pub lease_type: i32,
}
/// Request message for acquiring a lease.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcquireLeaseRequest {
    /// The series name.
    #[prost(string, tag = "1")]
    pub series: ::prost::alloc::string::String,
    /// The owner name.
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// The lease term.
    #[prost(message, optional, tag = "3")]
    pub term: ::core::option::Option<::prost_types::Duration>,
    /// The lease type.
    #[prost(enumeration = "LeaseType", tag = "4")]
    pub lease_type: i32,
}
/// Request message for renewing a lease.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenewLeaseRequest {
    /// Lease id.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Series name.
    #[prost(string, tag = "2")]
    pub series: ::prost::alloc::string::String,
    /// Lease owner.
    #[prost(string, tag = "3")]
    pub owner: ::prost::alloc::string::String,
    /// Lease term.
    #[prost(message, optional, tag = "4")]
    pub term: ::core::option::Option<::prost_types::Duration>,
}
/// Request message for releasing lease.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReleaseLeaseRequest {
    /// Lease id.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Series name.
    #[prost(string, tag = "2")]
    pub series: ::prost::alloc::string::String,
    /// Lease owner.
    #[prost(string, tag = "3")]
    pub owner: ::prost::alloc::string::String,
}
/// Response message for release lease.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReleaseLeaseResponse {}
/// RequestMetadata is the metadata message for the request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestMetadata {
    /// Stream name.
    #[prost(string, tag = "1")]
    pub stream: ::prost::alloc::string::String,
    /// Evevt name.
    #[prost(string, tag = "2")]
    pub event: ::prost::alloc::string::String,
    /// Series name.
    #[prost(string, tag = "3")]
    pub series: ::prost::alloc::string::String,
    /// Lease id.
    #[prost(string, tag = "4")]
    pub lease_id: ::prost::alloc::string::String,
    /// Owner name.
    #[prost(string, tag = "5")]
    pub owner: ::prost::alloc::string::String,
    /// Lease term specifies how long the client wants the session to be maintained
    /// by the server after the client leaves. If the lease term is not set, the
    /// server will release the session immediately and the client cannot reconnect
    /// to the same session later.
    #[prost(message, optional, tag = "6")]
    pub lease_term: ::core::option::Option<::prost_types::Duration>,
}
/// Request message for sending packets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendPacketsRequest {
    #[prost(oneof = "send_packets_request::Request", tags = "1, 2")]
    pub request: ::core::option::Option<send_packets_request::Request>,
}
/// Nested message and enum types in `SendPacketsRequest`.
pub mod send_packets_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        /// Packets sent over the streaming rpc.
        #[prost(message, tag = "1")]
        Packet(super::Packet),
        /// The first message of the streaming rpc including the request metadata.
        #[prost(message, tag = "2")]
        Metadata(super::RequestMetadata),
    }
}
/// Response message for sending packets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendPacketsResponse {}
/// Request message for receiving packets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceivePacketsRequest {
    /// Possible request types from the client.
    #[prost(oneof = "receive_packets_request::Request", tags = "6, 7")]
    pub request: ::core::option::Option<receive_packets_request::Request>,
}
/// Nested message and enum types in `ReceivePacketsRequest`.
pub mod receive_packets_request {
    /// The message specifying the initial settings for the ReceivePackets session.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetupRequest {
        /// The configurations that specify where packets are retrieved.
        #[prost(message, optional, tag = "1")]
        pub metadata: ::core::option::Option<super::RequestMetadata>,
        /// A name for the receiver to self-identify.
        ///
        /// This is used to keep track of a receiver's read progress.
        #[prost(string, tag = "2")]
        pub receiver: ::prost::alloc::string::String,
        /// The maximum duration of server silence before the client determines the
        /// server unreachable.
        ///
        /// The client must either receive a `Packet` or a heart beat message before
        /// this duration expires; otherwise, the client will automatically cancel
        /// the current connection and retry.
        #[prost(message, optional, tag = "5")]
        pub heartbeat_interval: ::core::option::Option<::prost_types::Duration>,
        /// The grace period after which a `writes_done_request` is issued, that a
        /// `WritesDone` is expected from the client.
        ///
        /// The server is free to cancel the RPC should this expire.
        ///
        /// A system default will be chosen if unset.
        #[prost(message, optional, tag = "6")]
        pub writes_done_grace_period: ::core::option::Option<::prost_types::Duration>,
        /// The mode in which the consumer reads messages.
        #[prost(oneof = "setup_request::ConsumerMode", tags = "3, 4")]
        pub consumer_mode: ::core::option::Option<setup_request::ConsumerMode>,
    }
    /// Nested message and enum types in `SetupRequest`.
    pub mod setup_request {
        /// The mode in which the consumer reads messages.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ConsumerMode {
            /// Options for configuring eager mode.
            #[prost(message, tag = "3")]
            EagerReceiveMode(super::super::EagerMode),
            /// Options for configuring controlled mode.
            #[prost(message, tag = "4")]
            ControlledReceiveMode(super::super::ControlledMode),
        }
    }
    /// Possible request types from the client.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        /// The request to setup the initial state of session.
        ///
        /// The client must send and only send this as the first message.
        #[prost(message, tag = "6")]
        SetupRequest(SetupRequest),
        /// This request checkpoints the consumer's read progress.
        #[prost(message, tag = "7")]
        CommitRequest(super::CommitRequest),
    }
}
/// Control message for a ReceivePacketsResponse.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceivePacketsControlResponse {
    /// Possible control messages.
    #[prost(oneof = "receive_packets_control_response::Control", tags = "1, 2")]
    pub control: ::core::option::Option<receive_packets_control_response::Control>,
}
/// Nested message and enum types in `ReceivePacketsControlResponse`.
pub mod receive_packets_control_response {
    /// Possible control messages.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Control {
        /// A server heartbeat.
        #[prost(bool, tag = "1")]
        Heartbeat(bool),
        /// A request to the receiver to complete any final writes followed by a
        /// `WritesDone`; e.g. issue any final `CommitRequest`s.
        ///
        /// May be ignored if `WritesDone` has already been issued at any point
        /// prior to receiving this message.
        ///
        /// If `WritesDone` does not get issued, then the server will forcefully
        /// cancel the connection, and the receiver will likely receive an
        /// uninformative after `Read` returns `false` and `Finish` is called.
        #[prost(bool, tag = "2")]
        WritesDoneRequest(bool),
    }
}
/// Response message from ReceivePackets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceivePacketsResponse {
    /// Possible response types.
    #[prost(oneof = "receive_packets_response::Response", tags = "1, 3")]
    pub response: ::core::option::Option<receive_packets_response::Response>,
}
/// Nested message and enum types in `ReceivePacketsResponse`.
pub mod receive_packets_response {
    /// Possible response types.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        /// A genuine data payload originating from the sender.
        #[prost(message, tag = "1")]
        Packet(super::Packet),
        /// A control message from the server.
        #[prost(message, tag = "3")]
        Control(super::ReceivePacketsControlResponse),
    }
}
/// The options for receiver under the eager mode.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EagerMode {}
/// The options for receiver under the controlled mode.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ControlledMode {
    /// This is the logical starting point to fallback upon should the
    /// specified starting offset be unavailable.
    ///
    /// This can be one of the following values:
    ///
    /// "begin": This will read from the earliest available message.
    ///
    /// "end": This will read only future messages.
    #[prost(string, tag = "2")]
    pub fallback_starting_offset: ::prost::alloc::string::String,
    /// This is the offset from which to start receiveing.
    #[prost(oneof = "controlled_mode::StartingOffset", tags = "1")]
    pub starting_offset: ::core::option::Option<controlled_mode::StartingOffset>,
}
/// Nested message and enum types in `ControlledMode`.
pub mod controlled_mode {
    /// This is the offset from which to start receiveing.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StartingOffset {
        /// This can be set to the following logical starting points:
        ///
        /// "begin": This will read from the earliest available message.
        ///
        /// "most-recent": This will read from the latest available message.
        ///
        /// "end": This will read only future messages.
        ///
        /// "stored": This will resume reads one past the last committed offset.
        ///            It is the only option that resumes progress; all others
        ///            jump unilaterally.
        #[prost(string, tag = "1")]
        StartingLogicalOffset(::prost::alloc::string::String),
    }
}
/// The message for explicitly committing the read progress.
///
/// This may only be used when `ReceivePacketsControlledMode` is set in the
/// initial setup request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitRequest {
    /// The offset to commit.
    #[prost(int64, tag = "1")]
    pub offset: i64,
}
/// The lease type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LeaseType {
    /// Lease type unspecified.
    Unspecified = 0,
    /// Lease for stream reader.
    Reader = 1,
    /// Lease for stream writer.
    Writer = 2,
}
impl LeaseType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LeaseType::Unspecified => "LEASE_TYPE_UNSPECIFIED",
            LeaseType::Reader => "LEASE_TYPE_READER",
            LeaseType::Writer => "LEASE_TYPE_WRITER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LEASE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "LEASE_TYPE_READER" => Some(Self::Reader),
            "LEASE_TYPE_WRITER" => Some(Self::Writer),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod streaming_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Streaming service for receiving and sending packets.
    #[derive(Debug, Clone)]
    pub struct StreamingServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> StreamingServiceClient<T>
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
        ) -> StreamingServiceClient<InterceptedService<T, F>>
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
            StreamingServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Send packets to the series.
        pub async fn send_packets(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::SendPacketsRequest,
            >,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::SendPacketsResponse>>,
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
                "/google.cloud.visionai.v1.StreamingService/SendPackets",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        /// Receive packets from the series.
        pub async fn receive_packets(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::ReceivePacketsRequest,
            >,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ReceivePacketsResponse>>,
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
                "/google.cloud.visionai.v1.StreamingService/ReceivePackets",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        /// Receive events given the stream name.
        pub async fn receive_events(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::ReceiveEventsRequest,
            >,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ReceiveEventsResponse>>,
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
                "/google.cloud.visionai.v1.StreamingService/ReceiveEvents",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        /// AcquireLease acquires a lease.
        pub async fn acquire_lease(
            &mut self,
            request: impl tonic::IntoRequest<super::AcquireLeaseRequest>,
        ) -> Result<tonic::Response<super::Lease>, tonic::Status> {
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
                "/google.cloud.visionai.v1.StreamingService/AcquireLease",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// RenewLease renews a lease.
        pub async fn renew_lease(
            &mut self,
            request: impl tonic::IntoRequest<super::RenewLeaseRequest>,
        ) -> Result<tonic::Response<super::Lease>, tonic::Status> {
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
                "/google.cloud.visionai.v1.StreamingService/RenewLease",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// RleaseLease releases a lease.
        pub async fn release_lease(
            &mut self,
            request: impl tonic::IntoRequest<super::ReleaseLeaseRequest>,
        ) -> Result<tonic::Response<super::ReleaseLeaseResponse>, tonic::Status> {
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
                "/google.cloud.visionai.v1.StreamingService/ReleaseLease",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Message describing the Cluster object.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cluster {
    /// Output only. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The create timestamp.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The update timestamp.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels as key value pairs
    #[prost(btree_map = "string, string", tag = "4")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Annotations to allow clients to store small amounts of arbitrary data.
    #[prost(btree_map = "string, string", tag = "5")]
    pub annotations: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. The DNS name of the data plane service
    #[prost(string, tag = "6")]
    pub dataplane_service_endpoint: ::prost::alloc::string::String,
    /// Output only. The current state of the cluster.
    #[prost(enumeration = "cluster::State", tag = "7")]
    pub state: i32,
    /// Output only. The private service connection service target name.
    #[prost(string, tag = "8")]
    pub psc_target: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Cluster`.
pub mod cluster {
    /// The current state of the cluster.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum State {
        /// Not set.
        Unspecified = 0,
        /// The PROVISIONING state indicates the cluster is being created.
        Provisioning = 1,
        /// The RUNNING state indicates the cluster has been created and is fully
        /// usable.
        Running = 2,
        /// The STOPPING state indicates the cluster is being deleted.
        Stopping = 3,
        /// The ERROR state indicates the cluster is unusable. It will be
        /// automatically deleted.
        Error = 4,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Provisioning => "PROVISIONING",
                State::Running => "RUNNING",
                State::Stopping => "STOPPING",
                State::Error => "ERROR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "PROVISIONING" => Some(Self::Provisioning),
                "RUNNING" => Some(Self::Running),
                "STOPPING" => Some(Self::Stopping),
                "ERROR" => Some(Self::Error),
                _ => None,
            }
        }
    }
}
/// Represents the metadata of the long-running operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Output only. The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Output only. Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Output only. Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_message: ::prost::alloc::string::String,
    /// Output only. Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have \[Operation.error][\] value with a \[google.rpc.Status.code][google.rpc.Status.code\] of 1,
    /// corresponding to `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// The Google Cloud Storage location for the input content.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsSource {
    /// Required. References to a Google Cloud Storage paths.
    #[prost(string, repeated, tag = "1")]
    pub uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Output format for Personal Protective Equipment Detection Operator.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersonalProtectiveEquipmentDetectionOutput {
    /// Current timestamp.
    #[prost(message, optional, tag = "1")]
    pub current_time: ::core::option::Option<::prost_types::Timestamp>,
    /// A list of DetectedPersons.
    #[prost(message, repeated, tag = "2")]
    pub detected_persons: ::prost::alloc::vec::Vec<
        personal_protective_equipment_detection_output::DetectedPerson,
    >,
}
/// Nested message and enum types in `PersonalProtectiveEquipmentDetectionOutput`.
pub mod personal_protective_equipment_detection_output {
    /// The entity info for annotations from person detection prediction result.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PersonEntity {
        /// Entity id.
        #[prost(int64, tag = "1")]
        pub person_entity_id: i64,
    }
    /// The entity info for annotations from PPE detection prediction result.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PpeEntity {
        /// Label id.
        #[prost(int64, tag = "1")]
        pub ppe_label_id: i64,
        /// Human readable string of the label (Examples: helmet, glove, mask).
        #[prost(string, tag = "2")]
        pub ppe_label_string: ::prost::alloc::string::String,
        /// Human readable string of the super category label (Examples: head_cover,
        /// hands_cover, face_cover).
        #[prost(string, tag = "3")]
        pub ppe_supercategory_label_string: ::prost::alloc::string::String,
        /// Entity id.
        #[prost(int64, tag = "4")]
        pub ppe_entity_id: i64,
    }
    /// Bounding Box in the normalized coordinates.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NormalizedBoundingBox {
        /// Min in x coordinate.
        #[prost(float, tag = "1")]
        pub xmin: f32,
        /// Min in y coordinate.
        #[prost(float, tag = "2")]
        pub ymin: f32,
        /// Width of the bounding box.
        #[prost(float, tag = "3")]
        pub width: f32,
        /// Height of the bounding box.
        #[prost(float, tag = "4")]
        pub height: f32,
    }
    /// PersonIdentified box contains the location and the entity info of the
    /// person.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PersonIdentifiedBox {
        /// An unique id for this box.
        #[prost(int64, tag = "1")]
        pub box_id: i64,
        /// Bounding Box in the normalized coordinates.
        #[prost(message, optional, tag = "2")]
        pub normalized_bounding_box: ::core::option::Option<NormalizedBoundingBox>,
        /// Confidence score associated with this box.
        #[prost(float, tag = "3")]
        pub confidence_score: f32,
        /// Person entity info.
        #[prost(message, optional, tag = "4")]
        pub person_entity: ::core::option::Option<PersonEntity>,
    }
    /// PPEIdentified box contains the location and the entity info of the PPE.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PpeIdentifiedBox {
        /// An unique id for this box.
        #[prost(int64, tag = "1")]
        pub box_id: i64,
        /// Bounding Box in the normalized coordinates.
        #[prost(message, optional, tag = "2")]
        pub normalized_bounding_box: ::core::option::Option<NormalizedBoundingBox>,
        /// Confidence score associated with this box.
        #[prost(float, tag = "3")]
        pub confidence_score: f32,
        /// PPE entity info.
        #[prost(message, optional, tag = "4")]
        pub ppe_entity: ::core::option::Option<PpeEntity>,
    }
    /// Detected Person contains the detected person and their associated
    /// ppes and their protecting information.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DetectedPerson {
        /// The id of detected person.
        #[prost(int64, tag = "1")]
        pub person_id: i64,
        /// The info of detected person identified box.
        #[prost(message, optional, tag = "2")]
        pub detected_person_identified_box: ::core::option::Option<PersonIdentifiedBox>,
        /// The info of detected person associated ppe identified boxes.
        #[prost(message, repeated, tag = "3")]
        pub detected_ppe_identified_boxes: ::prost::alloc::vec::Vec<PpeIdentifiedBox>,
        /// Coverage score for each body part.
        /// Coverage score for face.
        #[prost(float, optional, tag = "4")]
        pub face_coverage_score: ::core::option::Option<f32>,
        /// Coverage score for eyes.
        #[prost(float, optional, tag = "5")]
        pub eyes_coverage_score: ::core::option::Option<f32>,
        /// Coverage score for head.
        #[prost(float, optional, tag = "6")]
        pub head_coverage_score: ::core::option::Option<f32>,
        /// Coverage score for hands.
        #[prost(float, optional, tag = "7")]
        pub hands_coverage_score: ::core::option::Option<f32>,
        /// Coverage score for body.
        #[prost(float, optional, tag = "8")]
        pub body_coverage_score: ::core::option::Option<f32>,
        /// Coverage score for feet.
        #[prost(float, optional, tag = "9")]
        pub feet_coverage_score: ::core::option::Option<f32>,
    }
}
/// Prediction output format for Generic Object Detection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectDetectionPredictionResult {
    /// Current timestamp.
    #[prost(message, optional, tag = "1")]
    pub current_time: ::core::option::Option<::prost_types::Timestamp>,
    /// A list of identified boxes.
    #[prost(message, repeated, tag = "2")]
    pub identified_boxes: ::prost::alloc::vec::Vec<
        object_detection_prediction_result::IdentifiedBox,
    >,
}
/// Nested message and enum types in `ObjectDetectionPredictionResult`.
pub mod object_detection_prediction_result {
    /// The entity info for annotations from object detection prediction result.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Entity {
        /// Label id.
        #[prost(int64, tag = "1")]
        pub label_id: i64,
        /// Human readable string of the label.
        #[prost(string, tag = "2")]
        pub label_string: ::prost::alloc::string::String,
    }
    /// Identified box contains location and the entity of the object.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IdentifiedBox {
        /// An unique id for this box.
        #[prost(int64, tag = "1")]
        pub box_id: i64,
        /// Bounding Box in the normalized coordinates.
        #[prost(message, optional, tag = "2")]
        pub normalized_bounding_box: ::core::option::Option<
            identified_box::NormalizedBoundingBox,
        >,
        /// Confidence score associated with this box.
        #[prost(float, tag = "3")]
        pub confidence_score: f32,
        /// Entity of this box.
        #[prost(message, optional, tag = "4")]
        pub entity: ::core::option::Option<Entity>,
    }
    /// Nested message and enum types in `IdentifiedBox`.
    pub mod identified_box {
        /// Bounding Box in the normalized coordinates.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct NormalizedBoundingBox {
            /// Min in x coordinate.
            #[prost(float, tag = "1")]
            pub xmin: f32,
            /// Min in y coordinate.
            #[prost(float, tag = "2")]
            pub ymin: f32,
            /// Width of the bounding box.
            #[prost(float, tag = "3")]
            pub width: f32,
            /// Height of the bounding box.
            #[prost(float, tag = "4")]
            pub height: f32,
        }
    }
}
/// Prediction output format for Image Object Detection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageObjectDetectionPredictionResult {
    /// The resource IDs of the AnnotationSpecs that had been identified, ordered
    /// by the confidence score descendingly. It is the id segment instead of full
    /// resource name.
    #[prost(int64, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<i64>,
    /// The display names of the AnnotationSpecs that had been identified, order
    /// matches the IDs.
    #[prost(string, repeated, tag = "2")]
    pub display_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The Model's confidences in correctness of the predicted IDs, higher value
    /// means higher confidence. Order matches the Ids.
    #[prost(float, repeated, tag = "3")]
    pub confidences: ::prost::alloc::vec::Vec<f32>,
    /// Bounding boxes, i.e. the rectangles over the image, that pinpoint
    /// the found AnnotationSpecs. Given in order that matches the IDs. Each
    /// bounding box is an array of 4 numbers `xMin`, `xMax`, `yMin`, and
    /// `yMax`, which represent the extremal coordinates of the box. They are
    /// relative to the image size, and the point 0,0 is in the top left
    /// of the image.
    #[prost(message, repeated, tag = "4")]
    pub bboxes: ::prost::alloc::vec::Vec<::prost_types::ListValue>,
}
/// Prediction output format for Image and Text Classification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassificationPredictionResult {
    /// The resource IDs of the AnnotationSpecs that had been identified.
    #[prost(int64, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<i64>,
    /// The display names of the AnnotationSpecs that had been identified, order
    /// matches the IDs.
    #[prost(string, repeated, tag = "2")]
    pub display_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The Model's confidences in correctness of the predicted IDs, higher value
    /// means higher confidence. Order matches the Ids.
    #[prost(float, repeated, tag = "3")]
    pub confidences: ::prost::alloc::vec::Vec<f32>,
}
/// Prediction output format for Image Segmentation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageSegmentationPredictionResult {
    /// A PNG image where each pixel in the mask represents the category in which
    /// the pixel in the original image was predicted to belong to. The size of
    /// this image will be the same as the original image. The mapping between the
    /// AnntoationSpec and the color can be found in model's metadata. The model
    /// will choose the most likely category and if none of the categories reach
    /// the confidence threshold, the pixel will be marked as background.
    #[prost(string, tag = "1")]
    pub category_mask: ::prost::alloc::string::String,
    /// A one channel image which is encoded as an 8bit lossless PNG. The size of
    /// the image will be the same as the original image. For a specific pixel,
    /// darker color means less confidence in correctness of the cateogry in the
    /// categoryMask for the corresponding pixel. Black means no confidence and
    /// white means complete confidence.
    #[prost(string, tag = "2")]
    pub confidence_mask: ::prost::alloc::string::String,
}
/// Prediction output format for Video Action Recognition.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoActionRecognitionPredictionResult {
    /// The beginning, inclusive, of the video's time segment in which the
    /// actions have been identified.
    #[prost(message, optional, tag = "1")]
    pub segment_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The end, inclusive, of the video's time segment in which the actions have
    /// been identified. Particularly, if the end is the same as the start, it
    /// means the identification happens on a specific video frame.
    #[prost(message, optional, tag = "2")]
    pub segment_end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// All of the actions identified in the time range.
    #[prost(message, repeated, tag = "3")]
    pub actions: ::prost::alloc::vec::Vec<
        video_action_recognition_prediction_result::IdentifiedAction,
    >,
}
/// Nested message and enum types in `VideoActionRecognitionPredictionResult`.
pub mod video_action_recognition_prediction_result {
    /// Each IdentifiedAction is one particular identification of an action
    /// specified with the AnnotationSpec id, display_name and the associated
    /// confidence score.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IdentifiedAction {
        /// The resource ID of the AnnotationSpec that had been identified.
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// The display name of the AnnotationSpec that had been identified.
        #[prost(string, tag = "2")]
        pub display_name: ::prost::alloc::string::String,
        /// The Model's confidence in correction of this identification, higher
        /// value means higher confidence.
        #[prost(float, tag = "3")]
        pub confidence: f32,
    }
}
/// Prediction output format for Video Object Tracking.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoObjectTrackingPredictionResult {
    /// The beginning, inclusive, of the video's time segment in which the
    /// current identifications happens.
    #[prost(message, optional, tag = "1")]
    pub segment_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The end, inclusive, of the video's time segment in which the current
    /// identifications happen. Particularly, if the end is the same as the start,
    /// it means the identifications happen on a specific video frame.
    #[prost(message, optional, tag = "2")]
    pub segment_end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// All of the objects detected in the specified time range.
    #[prost(message, repeated, tag = "3")]
    pub objects: ::prost::alloc::vec::Vec<
        video_object_tracking_prediction_result::DetectedObject,
    >,
}
/// Nested message and enum types in `VideoObjectTrackingPredictionResult`.
pub mod video_object_tracking_prediction_result {
    /// Boundingbox for detected object. I.e. the rectangle over the video frame
    /// pinpointing the found AnnotationSpec. The coordinates are relative to the
    /// frame size, and the point 0,0 is in the top left of the frame.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BoundingBox {
        /// The leftmost coordinate of the bounding box.
        #[prost(float, tag = "1")]
        pub x_min: f32,
        /// The rightmost coordinate of the bounding box.
        #[prost(float, tag = "2")]
        pub x_max: f32,
        /// The topmost coordinate of the bounding box.
        #[prost(float, tag = "3")]
        pub y_min: f32,
        /// The bottommost coordinate of the bounding box.
        #[prost(float, tag = "4")]
        pub y_max: f32,
    }
    /// Each DetectedObject is one particular identification of an object
    /// specified with the AnnotationSpec id and display_name, the bounding box,
    /// the associated confidence score and the corresponding track_id.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DetectedObject {
        /// The resource ID of the AnnotationSpec that had been identified.
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// The display name of the AnnotationSpec that had been identified.
        #[prost(string, tag = "2")]
        pub display_name: ::prost::alloc::string::String,
        /// Boundingbox.
        #[prost(message, optional, tag = "3")]
        pub bounding_box: ::core::option::Option<BoundingBox>,
        /// The Model's confidence in correction of this identification, higher
        /// value means higher confidence.
        #[prost(float, tag = "4")]
        pub confidence: f32,
        /// The same object may be identified on muitiple frames which are typical
        /// adjacent. The set of frames where a particular object has been detected
        /// form a track. This track_id can be used to trace down all frames for an
        /// detected object.
        #[prost(int64, tag = "5")]
        pub track_id: i64,
    }
}
/// Prediction output format for Video Classification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoClassificationPredictionResult {
    /// The beginning, inclusive, of the video's time segment in which the
    /// classifications have been identified.
    #[prost(message, optional, tag = "1")]
    pub segment_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The end, inclusive, of the video's time segment in which the
    /// classifications have been identified. Particularly, if the end is the same
    /// as the start, it means the identification happens on a specific video
    /// frame.
    #[prost(message, optional, tag = "2")]
    pub segment_end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// All of the classifications identified in the time range.
    #[prost(message, repeated, tag = "3")]
    pub classifications: ::prost::alloc::vec::Vec<
        video_classification_prediction_result::IdentifiedClassification,
    >,
}
/// Nested message and enum types in `VideoClassificationPredictionResult`.
pub mod video_classification_prediction_result {
    /// Each IdentifiedClassification is one particular identification of an
    /// classification specified with the AnnotationSpec id and display_name,
    /// and the associated confidence score.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IdentifiedClassification {
        /// The resource ID of the AnnotationSpec that had been identified.
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// The display name of the AnnotationSpec that had been identified.
        #[prost(string, tag = "2")]
        pub display_name: ::prost::alloc::string::String,
        /// The Model's confidence in correction of this identification, higher
        /// value means higher confidence.
        #[prost(float, tag = "3")]
        pub confidence: f32,
    }
}
/// The prediction result proto for occupancy counting.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OccupancyCountingPredictionResult {
    /// Current timestamp.
    #[prost(message, optional, tag = "1")]
    pub current_time: ::core::option::Option<::prost_types::Timestamp>,
    /// A list of identified boxes.
    #[prost(message, repeated, tag = "2")]
    pub identified_boxes: ::prost::alloc::vec::Vec<
        occupancy_counting_prediction_result::IdentifiedBox,
    >,
    /// Detection statistics.
    #[prost(message, optional, tag = "3")]
    pub stats: ::core::option::Option<occupancy_counting_prediction_result::Stats>,
    /// Track related information. All the tracks that are live at this timestamp.
    /// It only exists if tracking is enabled.
    #[prost(message, repeated, tag = "4")]
    pub track_info: ::prost::alloc::vec::Vec<
        occupancy_counting_prediction_result::TrackInfo,
    >,
    /// Dwell time related information. All the tracks that are live in a given
    /// zone with a start and end dwell time timestamp
    #[prost(message, repeated, tag = "5")]
    pub dwell_time_info: ::prost::alloc::vec::Vec<
        occupancy_counting_prediction_result::DwellTimeInfo,
    >,
}
/// Nested message and enum types in `OccupancyCountingPredictionResult`.
pub mod occupancy_counting_prediction_result {
    /// The entity info for annotations from occupancy counting operator.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Entity {
        /// Label id.
        #[prost(int64, tag = "1")]
        pub label_id: i64,
        /// Human readable string of the label.
        #[prost(string, tag = "2")]
        pub label_string: ::prost::alloc::string::String,
    }
    /// Identified box contains location and the entity of the object.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IdentifiedBox {
        /// An unique id for this box.
        #[prost(int64, tag = "1")]
        pub box_id: i64,
        /// Bounding Box in the normalized coordinates.
        #[prost(message, optional, tag = "2")]
        pub normalized_bounding_box: ::core::option::Option<
            identified_box::NormalizedBoundingBox,
        >,
        /// Confidence score associated with this box.
        #[prost(float, tag = "3")]
        pub score: f32,
        /// Entity of this box.
        #[prost(message, optional, tag = "4")]
        pub entity: ::core::option::Option<Entity>,
        /// An unique id to identify a track. It should be consistent across frames.
        /// It only exists if tracking is enabled.
        #[prost(int64, tag = "5")]
        pub track_id: i64,
    }
    /// Nested message and enum types in `IdentifiedBox`.
    pub mod identified_box {
        /// Bounding Box in the normalized coordinates.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct NormalizedBoundingBox {
            /// Min in x coordinate.
            #[prost(float, tag = "1")]
            pub xmin: f32,
            /// Min in y coordinate.
            #[prost(float, tag = "2")]
            pub ymin: f32,
            /// Width of the bounding box.
            #[prost(float, tag = "3")]
            pub width: f32,
            /// Height of the bounding box.
            #[prost(float, tag = "4")]
            pub height: f32,
        }
    }
    /// The statistics info for annotations from occupancy counting operator.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Stats {
        /// Counts of the full frame.
        #[prost(message, repeated, tag = "1")]
        pub full_frame_count: ::prost::alloc::vec::Vec<stats::ObjectCount>,
        /// Crossing line counts.
        #[prost(message, repeated, tag = "2")]
        pub crossing_line_counts: ::prost::alloc::vec::Vec<stats::CrossingLineCount>,
        /// Active zone counts.
        #[prost(message, repeated, tag = "3")]
        pub active_zone_counts: ::prost::alloc::vec::Vec<stats::ActiveZoneCount>,
    }
    /// Nested message and enum types in `Stats`.
    pub mod stats {
        /// The object info and instant count for annotations from occupancy counting
        /// operator.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ObjectCount {
            /// Entity of this object.
            #[prost(message, optional, tag = "1")]
            pub entity: ::core::option::Option<super::Entity>,
            /// Count of the object.
            #[prost(int32, tag = "2")]
            pub count: i32,
        }
        /// The object info and accumulated count for annotations from occupancy
        /// counting operator.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct AccumulatedObjectCount {
            /// The start time of the accumulated count.
            #[prost(message, optional, tag = "1")]
            pub start_time: ::core::option::Option<::prost_types::Timestamp>,
            /// The object count for the accumulated count.
            #[prost(message, optional, tag = "2")]
            pub object_count: ::core::option::Option<ObjectCount>,
        }
        /// Message for Crossing line count.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CrossingLineCount {
            /// Line annotation from the user.
            #[prost(message, optional, tag = "1")]
            pub annotation: ::core::option::Option<super::super::StreamAnnotation>,
            /// The direction that follows the right hand rule.
            #[prost(message, repeated, tag = "2")]
            pub positive_direction_counts: ::prost::alloc::vec::Vec<ObjectCount>,
            /// The direction that is opposite to the right hand rule.
            #[prost(message, repeated, tag = "3")]
            pub negative_direction_counts: ::prost::alloc::vec::Vec<ObjectCount>,
            /// The accumulated positive count.
            #[prost(message, repeated, tag = "4")]
            pub accumulated_positive_direction_counts: ::prost::alloc::vec::Vec<
                AccumulatedObjectCount,
            >,
            /// The accumulated negative count.
            #[prost(message, repeated, tag = "5")]
            pub accumulated_negative_direction_counts: ::prost::alloc::vec::Vec<
                AccumulatedObjectCount,
            >,
        }
        /// Message for the active zone count.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ActiveZoneCount {
            /// Active zone annotation from the user.
            #[prost(message, optional, tag = "1")]
            pub annotation: ::core::option::Option<super::super::StreamAnnotation>,
            /// Counts in the zone.
            #[prost(message, repeated, tag = "2")]
            pub counts: ::prost::alloc::vec::Vec<ObjectCount>,
        }
    }
    /// The track info for annotations from occupancy counting operator.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TrackInfo {
        /// An unique id to identify a track. It should be consistent across frames.
        #[prost(string, tag = "1")]
        pub track_id: ::prost::alloc::string::String,
        /// Start timestamp of this track.
        #[prost(message, optional, tag = "2")]
        pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    }
    /// The dwell time info for annotations from occupancy counting operator.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DwellTimeInfo {
        /// An unique id to identify a track. It should be consistent across frames.
        #[prost(string, tag = "1")]
        pub track_id: ::prost::alloc::string::String,
        /// The unique id for the zone in which the object is dwelling/waiting.
        #[prost(string, tag = "2")]
        pub zone_id: ::prost::alloc::string::String,
        /// The beginning time when a dwelling object has been identified in a zone.
        #[prost(message, optional, tag = "3")]
        pub dwell_start_time: ::core::option::Option<::prost_types::Timestamp>,
        /// The end time when a dwelling object has exited in a zone.
        #[prost(message, optional, tag = "4")]
        pub dwell_end_time: ::core::option::Option<::prost_types::Timestamp>,
    }
}
/// message about annotations about Vision AI stream resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamAnnotation {
    /// ID of the annotation. It must be unique when used in the certain context.
    /// For example, all the annotations to one input streams of a Vision AI
    /// application.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// User-friendly name for the annotation.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// The Vision AI stream resource name.
    #[prost(string, tag = "3")]
    pub source_stream: ::prost::alloc::string::String,
    /// The actual type of Annotation.
    #[prost(enumeration = "StreamAnnotationType", tag = "4")]
    pub r#type: i32,
    #[prost(oneof = "stream_annotation::AnnotationPayload", tags = "5, 6")]
    pub annotation_payload: ::core::option::Option<stream_annotation::AnnotationPayload>,
}
/// Nested message and enum types in `StreamAnnotation`.
pub mod stream_annotation {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AnnotationPayload {
        /// Annotation for type ACTIVE_ZONE
        #[prost(message, tag = "5")]
        ActiveZone(super::NormalizedPolygon),
        /// Annotation for type CROSSING_LINE
        #[prost(message, tag = "6")]
        CrossingLine(super::NormalizedPolyline),
    }
}
/// A wrapper of repeated StreamAnnotation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamAnnotations {
    /// Multiple annotations.
    #[prost(message, repeated, tag = "1")]
    pub stream_annotations: ::prost::alloc::vec::Vec<StreamAnnotation>,
}
/// Normalized Polygon.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NormalizedPolygon {
    /// The bounding polygon normalized vertices. Top left corner of the image
    /// will be [0, 0].
    #[prost(message, repeated, tag = "1")]
    pub normalized_vertices: ::prost::alloc::vec::Vec<NormalizedVertex>,
}
/// Normalized Pplyline, which represents a curve consisting of connected
/// straight-line segments.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NormalizedPolyline {
    /// A sequence of vertices connected by straight lines.
    #[prost(message, repeated, tag = "1")]
    pub normalized_vertices: ::prost::alloc::vec::Vec<NormalizedVertex>,
}
/// A vertex represents a 2D point in the image.
/// NOTE: the normalized vertex coordinates are relative to the original image
/// and range from 0 to 1.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NormalizedVertex {
    /// X coordinate.
    #[prost(float, tag = "1")]
    pub x: f32,
    /// Y coordinate.
    #[prost(float, tag = "2")]
    pub y: f32,
}
/// Message of essential metadata of App Platform.
/// This message is usually attached to a certain processor output annotation for
/// customer to identify the source of the data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppPlatformMetadata {
    /// The application resource name.
    #[prost(string, tag = "1")]
    pub application: ::prost::alloc::string::String,
    /// The instance resource id. Instance is the nested resource of application
    /// under collection 'instances'.
    #[prost(string, tag = "2")]
    pub instance_id: ::prost::alloc::string::String,
    /// The node name of the application graph.
    #[prost(string, tag = "3")]
    pub node: ::prost::alloc::string::String,
    /// The referred processor resource name of the application node.
    #[prost(string, tag = "4")]
    pub processor: ::prost::alloc::string::String,
}
/// For any cloud function based customer processing logic, customer's cloud
/// function is expected to receive AppPlatformCloudFunctionRequest as request
/// and send back AppPlatformCloudFunctionResponse as response.
/// Message of request from AppPlatform to Cloud Function.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppPlatformCloudFunctionRequest {
    /// The metadata of the AppPlatform for customer to identify the source of the
    /// payload.
    #[prost(message, optional, tag = "1")]
    pub app_platform_metadata: ::core::option::Option<AppPlatformMetadata>,
    /// The actual annotations to be processed by the customized Cloud Function.
    #[prost(message, repeated, tag = "2")]
    pub annotations: ::prost::alloc::vec::Vec<
        app_platform_cloud_function_request::StructedInputAnnotation,
    >,
}
/// Nested message and enum types in `AppPlatformCloudFunctionRequest`.
pub mod app_platform_cloud_function_request {
    /// A general annotation message that uses struct format to represent different
    /// concrete annotation protobufs.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StructedInputAnnotation {
        /// The ingestion time of the current annotation.
        #[prost(int64, tag = "1")]
        pub ingestion_time_micros: i64,
        /// The struct format of the actual annotation.
        #[prost(message, optional, tag = "2")]
        pub annotation: ::core::option::Option<::prost_types::Struct>,
    }
}
/// Message of the response from customer's Cloud Function to AppPlatform.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppPlatformCloudFunctionResponse {
    /// The modified annotations that is returned back to AppPlatform.
    /// If the annotations fields are empty, then those annotations will be dropped
    /// by AppPlatform.
    #[prost(message, repeated, tag = "2")]
    pub annotations: ::prost::alloc::vec::Vec<
        app_platform_cloud_function_response::StructedOutputAnnotation,
    >,
    /// If set to true, AppPlatform will use original annotations instead of
    /// dropping them, even if it is empty in the annotations filed.
    #[prost(bool, tag = "3")]
    pub annotation_passthrough: bool,
    /// The event notifications that is returned back to AppPlatform. Typically it
    /// will then be configured to be consumed/forwared to a operator that handles
    /// events, such as Pub/Sub operator.
    #[prost(message, repeated, tag = "4")]
    pub events: ::prost::alloc::vec::Vec<AppPlatformEventBody>,
}
/// Nested message and enum types in `AppPlatformCloudFunctionResponse`.
pub mod app_platform_cloud_function_response {
    /// A general annotation message that uses struct format to represent different
    /// concrete annotation protobufs.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StructedOutputAnnotation {
        /// The struct format of the actual annotation.
        #[prost(message, optional, tag = "1")]
        pub annotation: ::core::option::Option<::prost_types::Struct>,
    }
}
/// Message of content of appPlatform event
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppPlatformEventBody {
    /// Human readable string of the event like "There are more than 6 people in
    /// the scene". or "Shelf is empty!".
    #[prost(string, tag = "1")]
    pub event_message: ::prost::alloc::string::String,
    /// For the case of Pub/Sub, it will be stored in the message attributes.
    /// ​​pubsub.proto
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<::prost_types::Struct>,
    /// User defined Event Id, used to classify event, within a delivery interval,
    /// events from the same application instance with the same id will be
    /// de-duplicated & only first one will be sent out. Empty event_id will be
    /// treated as "".
    #[prost(string, tag = "3")]
    pub event_id: ::prost::alloc::string::String,
}
/// Enum describing all possible types of a stream annotation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StreamAnnotationType {
    /// Type UNSPECIFIED.
    Unspecified = 0,
    /// active_zone annotation defines a polygon on top of the content from an
    /// image/video based stream, following processing will only focus on the
    /// content inside the active zone.
    ActiveZone = 1,
    /// crossing_line annotation defines a polyline on top of the content from an
    /// image/video based Vision AI stream, events happening across the line will
    /// be captured. For example, the counts of people who goes acroos the line
    /// in Occupancy Analytic Processor.
    CrossingLine = 2,
}
impl StreamAnnotationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StreamAnnotationType::Unspecified => "STREAM_ANNOTATION_TYPE_UNSPECIFIED",
            StreamAnnotationType::ActiveZone => "STREAM_ANNOTATION_TYPE_ACTIVE_ZONE",
            StreamAnnotationType::CrossingLine => "STREAM_ANNOTATION_TYPE_CROSSING_LINE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STREAM_ANNOTATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "STREAM_ANNOTATION_TYPE_ACTIVE_ZONE" => Some(Self::ActiveZone),
            "STREAM_ANNOTATION_TYPE_CROSSING_LINE" => Some(Self::CrossingLine),
            _ => None,
        }
    }
}
/// Message for DeleteApplicationInstance Response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteApplicationInstancesResponse {}
/// Message for CreateApplicationInstance Response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateApplicationInstancesResponse {}
/// Message for UpdateApplicationInstances Response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateApplicationInstancesResponse {}
/// Message for adding stream input to an Application.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateApplicationInstancesRequest {
    /// Required. the name of the application to retrieve.
    /// Format:
    /// "projects/{project}/locations/{location}/applications/{application}"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The resources being created.
    #[prost(message, repeated, tag = "2")]
    pub application_instances: ::prost::alloc::vec::Vec<ApplicationInstance>,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for removing stream input from an Application.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteApplicationInstancesRequest {
    /// Required. the name of the application to retrieve.
    /// Format:
    /// "projects/{project}/locations/{location}/applications/{application}"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Id of the requesting object.
    #[prost(string, repeated, tag = "2")]
    pub instance_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// RPC Request Messages.
/// Message for DeployApplication Response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployApplicationResponse {}
/// Message for UndeployApplication Response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeployApplicationResponse {}
/// Message for RemoveApplicationStreamInput Response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveApplicationStreamInputResponse {}
/// Message for AddApplicationStreamInput Response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddApplicationStreamInputResponse {}
/// Message for AddApplicationStreamInput Response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateApplicationStreamInputResponse {}
/// Message for requesting list of Applications.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListApplicationsRequest {
    /// Required. Parent value for ListApplicationsRequest.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Hint for how to order the results.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Message for response to listing Applications.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListApplicationsResponse {
    /// The list of Application.
    #[prost(message, repeated, tag = "1")]
    pub applications: ::prost::alloc::vec::Vec<Application>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a Application.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApplicationRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for creating a Application.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateApplicationRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Id of the requesting object.
    #[prost(string, tag = "2")]
    pub application_id: ::prost::alloc::string::String,
    /// Required. The resource being created.
    #[prost(message, optional, tag = "3")]
    pub application: ::core::option::Option<Application>,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for updating an Application.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateApplicationRequest {
    /// Optional. Field mask is used to specify the fields to be overwritten in the
    /// Application resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The resource being updated.
    #[prost(message, optional, tag = "2")]
    pub application: ::core::option::Option<Application>,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for deleting an Application.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteApplicationRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set to true, any instances and drafts from this application will also be
    /// deleted. (Otherwise, the request will only work if the application has no
    /// instances and drafts.)
    #[prost(bool, tag = "3")]
    pub force: bool,
}
/// Message for deploying an Application.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployApplicationRequest {
    /// Required. the name of the application to retrieve.
    /// Format:
    /// "projects/{project}/locations/{location}/applications/{application}"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If set, validate the request and preview the application graph, but do not
    /// actually deploy it.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. Whether or not to enable monitoring for the application on deployment.
    #[prost(bool, tag = "4")]
    pub enable_monitoring: bool,
}
/// Message for undeploying an Application.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeployApplicationRequest {
    /// Required. the name of the application to retrieve.
    /// Format:
    /// "projects/{project}/locations/{location}/applications/{application}"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message about a single stream input config.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplicationStreamInput {
    #[prost(message, optional, tag = "1")]
    pub stream_with_annotation: ::core::option::Option<StreamWithAnnotation>,
}
/// Message for adding stream input to an Application.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddApplicationStreamInputRequest {
    /// Required. the name of the application to retrieve.
    /// Format:
    /// "projects/{project}/locations/{location}/applications/{application}"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The stream inputs to add, the stream resource name is the key of each
    /// StreamInput, and it must be unique within each application.
    #[prost(message, repeated, tag = "2")]
    pub application_stream_inputs: ::prost::alloc::vec::Vec<ApplicationStreamInput>,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for updating stream input to an Application.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateApplicationStreamInputRequest {
    /// Required. the name of the application to retrieve.
    /// Format:
    /// "projects/{project}/locations/{location}/applications/{application}"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The stream inputs to update, the stream resource name is the key of each
    /// StreamInput, and it must be unique within each application.
    #[prost(message, repeated, tag = "2")]
    pub application_stream_inputs: ::prost::alloc::vec::Vec<ApplicationStreamInput>,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
    /// If true, UpdateApplicationStreamInput will insert stream input to
    /// application even if the target stream is not included in the application.
    #[prost(bool, tag = "4")]
    pub allow_missing: bool,
}
/// Message for removing stream input from an Application.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveApplicationStreamInputRequest {
    /// Required. the name of the application to retrieve.
    /// Format:
    /// "projects/{project}/locations/{location}/applications/{application}"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The target stream to remove.
    #[prost(message, repeated, tag = "2")]
    pub target_stream_inputs: ::prost::alloc::vec::Vec<
        remove_application_stream_input_request::TargetStreamInput,
    >,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `RemoveApplicationStreamInputRequest`.
pub mod remove_application_stream_input_request {
    /// Message about target streamInput to remove.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TargetStreamInput {
        #[prost(string, tag = "1")]
        pub stream: ::prost::alloc::string::String,
    }
}
/// Message for requesting list of Instances.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesRequest {
    /// Required. Parent value for ListInstancesRequest.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Hint for how to order the results.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Message for response to listing Instances.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesResponse {
    /// The list of Instance.
    #[prost(message, repeated, tag = "1")]
    pub instances: ::prost::alloc::vec::Vec<Instance>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a Instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for requesting list of Drafts.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDraftsRequest {
    /// Required. Parent value for ListDraftsRequest.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Hint for how to order the results.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Message for response to listing Drafts.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDraftsResponse {
    /// The list of Draft.
    #[prost(message, repeated, tag = "1")]
    pub drafts: ::prost::alloc::vec::Vec<Draft>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a Draft.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDraftRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for creating a Draft.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDraftRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Id of the requesting object.
    #[prost(string, tag = "2")]
    pub draft_id: ::prost::alloc::string::String,
    /// Required. The resource being created.
    #[prost(message, optional, tag = "3")]
    pub draft: ::core::option::Option<Draft>,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for updating an Draft.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDraftRequest {
    /// Optional. Field mask is used to specify the fields to be overwritten in the
    /// Draft resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The resource being updated.
    #[prost(message, optional, tag = "2")]
    pub draft: ::core::option::Option<Draft>,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
    /// If true, UpdateDraftRequest will create one resource if the target resource
    /// doesn't exist, this time, the field_mask will be ignored.
    #[prost(bool, tag = "4")]
    pub allow_missing: bool,
}
/// Message for updating an ApplicationInstance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateApplicationInstancesRequest {
    /// Required. the name of the application to retrieve.
    /// Format:
    /// "projects/{project}/locations/{location}/applications/{application}"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub application_instances: ::prost::alloc::vec::Vec<
        update_application_instances_request::UpdateApplicationInstance,
    >,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
    /// If true, Update Request will create one resource if the target resource
    /// doesn't exist, this time, the field_mask will be ignored.
    #[prost(bool, tag = "4")]
    pub allow_missing: bool,
}
/// Nested message and enum types in `UpdateApplicationInstancesRequest`.
pub mod update_application_instances_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdateApplicationInstance {
        /// Optional. Field mask is used to specify the fields to be overwritten in the
        /// Draft resource by the update.
        /// The fields specified in the update_mask are relative to the resource, not
        /// the full request. A field will be overwritten if it is in the mask. If
        /// the user does not provide a mask then all fields will be overwritten.
        #[prost(message, optional, tag = "1")]
        pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
        /// Required. The resource being updated.
        #[prost(message, optional, tag = "2")]
        pub instance: ::core::option::Option<super::Instance>,
        /// Required. The id of the instance.
        #[prost(string, tag = "3")]
        pub instance_id: ::prost::alloc::string::String,
    }
}
/// Message for deleting an Draft.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDraftRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for requesting list of Processors.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProcessorsRequest {
    /// Required. Parent value for ListProcessorsRequest.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Hint for how to order the results.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Message for response to listing Processors.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProcessorsResponse {
    /// The list of Processor.
    #[prost(message, repeated, tag = "1")]
    pub processors: ::prost::alloc::vec::Vec<Processor>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request Message for listing Prebuilt Processors.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPrebuiltProcessorsRequest {
    /// Required. Parent path.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
}
/// Response Message for listing Prebuilt Processors.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPrebuiltProcessorsResponse {
    /// The list of Processor.
    #[prost(message, repeated, tag = "1")]
    pub processors: ::prost::alloc::vec::Vec<Processor>,
}
/// Message for getting a Processor.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProcessorRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for creating a Processor.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProcessorRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Id of the requesting object.
    #[prost(string, tag = "2")]
    pub processor_id: ::prost::alloc::string::String,
    /// Required. The resource being created.
    #[prost(message, optional, tag = "3")]
    pub processor: ::core::option::Option<Processor>,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for updating a Processor.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProcessorRequest {
    /// Optional. Field mask is used to specify the fields to be overwritten in the
    /// Processor resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The resource being updated.
    #[prost(message, optional, tag = "2")]
    pub processor: ::core::option::Option<Processor>,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for deleting a Processor.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteProcessorRequest {
    /// Required. Name of the resource
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message describing Application object
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Application {
    /// name of resource
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. [Output only] Create timestamp
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. [Output only] Update timestamp
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels as key value pairs
    #[prost(btree_map = "string, string", tag = "4")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Required. A user friendly display name for the solution.
    #[prost(string, tag = "5")]
    pub display_name: ::prost::alloc::string::String,
    /// A description for this application.
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
    /// Application graph configuration.
    #[prost(message, optional, tag = "7")]
    pub application_configs: ::core::option::Option<ApplicationConfigs>,
    /// Output only. Application graph runtime info. Only exists when application state equals
    /// to DEPLOYED.
    #[prost(message, optional, tag = "8")]
    pub runtime_info: ::core::option::Option<application::ApplicationRuntimeInfo>,
    /// Output only. State of the application.
    #[prost(enumeration = "application::State", tag = "9")]
    pub state: i32,
}
/// Nested message and enum types in `Application`.
pub mod application {
    /// Message storing the runtime information of the application.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ApplicationRuntimeInfo {
        /// Timestamp when the engine be deployed
        #[prost(message, optional, tag = "1")]
        pub deploy_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Globally created resources like warehouse dataschemas.
        #[prost(message, repeated, tag = "3")]
        pub global_output_resources: ::prost::alloc::vec::Vec<
            application_runtime_info::GlobalOutputResource,
        >,
        /// Monitoring-related configuration for this application.
        #[prost(message, optional, tag = "4")]
        pub monitoring_config: ::core::option::Option<
            application_runtime_info::MonitoringConfig,
        >,
    }
    /// Nested message and enum types in `ApplicationRuntimeInfo`.
    pub mod application_runtime_info {
        /// Message about output resources from application.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct GlobalOutputResource {
            /// The full resource name of the outputted resources.
            #[prost(string, tag = "1")]
            pub output_resource: ::prost::alloc::string::String,
            /// The name of graph node who produces the output resource name.
            /// For example:
            /// output_resource:
            /// /projects/123/locations/us-central1/corpora/my-corpus/dataSchemas/my-schema
            /// producer_node: occupancy-count
            #[prost(string, tag = "2")]
            pub producer_node: ::prost::alloc::string::String,
            /// The key of the output resource, it has to be unique within the same
            /// producer node. One producer node can output several output resources,
            /// the key can be used to match corresponding output resources.
            #[prost(string, tag = "3")]
            pub key: ::prost::alloc::string::String,
        }
        /// Monitoring-related configuration for an application.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MonitoringConfig {
            /// Whether this application has monitoring enabled.
            #[prost(bool, tag = "1")]
            pub enabled: bool,
        }
    }
    /// State of the Application
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum State {
        /// The default value. This value is used if the state is omitted.
        Unspecified = 0,
        /// State CREATED.
        Created = 1,
        /// State DEPLOYING.
        Deploying = 2,
        /// State DEPLOYED.
        Deployed = 3,
        /// State UNDEPLOYING.
        Undeploying = 4,
        /// State DELETED.
        Deleted = 5,
        /// State ERROR.
        Error = 6,
        /// State CREATING.
        Creating = 7,
        /// State Updating.
        Updating = 8,
        /// State Deleting.
        Deleting = 9,
        /// State Fixing.
        Fixing = 10,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Created => "CREATED",
                State::Deploying => "DEPLOYING",
                State::Deployed => "DEPLOYED",
                State::Undeploying => "UNDEPLOYING",
                State::Deleted => "DELETED",
                State::Error => "ERROR",
                State::Creating => "CREATING",
                State::Updating => "UPDATING",
                State::Deleting => "DELETING",
                State::Fixing => "FIXING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATED" => Some(Self::Created),
                "DEPLOYING" => Some(Self::Deploying),
                "DEPLOYED" => Some(Self::Deployed),
                "UNDEPLOYING" => Some(Self::Undeploying),
                "DELETED" => Some(Self::Deleted),
                "ERROR" => Some(Self::Error),
                "CREATING" => Some(Self::Creating),
                "UPDATING" => Some(Self::Updating),
                "DELETING" => Some(Self::Deleting),
                "FIXING" => Some(Self::Fixing),
                _ => None,
            }
        }
    }
}
/// Message storing the graph of the application.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplicationConfigs {
    /// A list of nodes  in the application graph.
    #[prost(message, repeated, tag = "1")]
    pub nodes: ::prost::alloc::vec::Vec<Node>,
    /// Event-related configuration for this application.
    #[prost(message, optional, tag = "3")]
    pub event_delivery_config: ::core::option::Option<
        application_configs::EventDeliveryConfig,
    >,
}
/// Nested message and enum types in `ApplicationConfigs`.
pub mod application_configs {
    /// message storing the config for event delivery
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EventDeliveryConfig {
        /// The delivery channel for the event notification, only pub/sub topic is
        /// supported now.
        /// Example channel:
        /// \[//pubsub.googleapis.com/projects/visionai-testing-stable/topics/test-topic\]
        #[prost(string, tag = "1")]
        pub channel: ::prost::alloc::string::String,
        /// The expected delivery interval for the same event. The same event won't
        /// be notified multiple times during this internal event that it is
        /// happening multiple times during the period of time.The same event is
        /// identified by <event_id, app_platform_metadata>.
        #[prost(message, optional, tag = "2")]
        pub minimal_delivery_interval: ::core::option::Option<::prost_types::Duration>,
    }
}
/// Message describing node object.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Node {
    /// Required. A unique name for the node.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A user friendly display name for the node.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Node config.
    #[prost(message, optional, tag = "3")]
    pub node_config: ::core::option::Option<ProcessorConfig>,
    /// Processor name refer to the chosen processor resource.
    #[prost(string, tag = "4")]
    pub processor: ::prost::alloc::string::String,
    /// Parent node. Input node should not have parent node. For V1 Alpha1/Beta
    /// only media warehouse node can have multiple parents, other types of nodes
    /// will only have one parent.
    #[prost(message, repeated, tag = "5")]
    pub parents: ::prost::alloc::vec::Vec<node::InputEdge>,
    #[prost(oneof = "node::StreamOutputConfig", tags = "6")]
    pub stream_output_config: ::core::option::Option<node::StreamOutputConfig>,
}
/// Nested message and enum types in `Node`.
pub mod node {
    /// Message describing one edge pointing into a node.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InputEdge {
        /// The name of the parent node.
        #[prost(string, tag = "1")]
        pub parent_node: ::prost::alloc::string::String,
        /// The connected output artifact of the parent node.
        /// It can be omitted if target processor only has 1 output artifact.
        #[prost(string, tag = "2")]
        pub parent_output_channel: ::prost::alloc::string::String,
        /// The connected input channel of the current node's processor.
        /// It can be omitted if target processor only has 1 input channel.
        #[prost(string, tag = "3")]
        pub connected_input_channel: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StreamOutputConfig {
        /// By default, the output of the node will only be available to downstream
        /// nodes. To consume the direct output from the application node, the output
        /// must be sent to Vision AI Streams at first.
        ///
        /// By setting output_all_output_channels_to_stream to true, App Platform
        /// will automatically send all the outputs of the current node to Vision AI
        /// Stream resources (one stream per output channel). The output stream
        /// resource will be created by App Platform automatically during deployment
        /// and deleted after application un-deployment.
        /// Note that this config applies to all the Application Instances.
        ///
        /// The output stream can be override at instance level by
        /// configuring the `output_resources` section of Instance resource.
        /// `producer_node` should be current node, `output_resource_binding` should
        /// be the output channel name (or leave it blank if there is only 1 output
        /// channel of the processor) and `output_resource` should be the target
        /// output stream.
        #[prost(bool, tag = "6")]
        OutputAllOutputChannelsToStream(bool),
    }
}
/// Message describing Draft object
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Draft {
    /// name of resource
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. [Output only] Create timestamp
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. [Output only] Create timestamp
    #[prost(message, optional, tag = "7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels as key value pairs
    #[prost(btree_map = "string, string", tag = "3")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Required. A user friendly display name for the solution.
    #[prost(string, tag = "4")]
    pub display_name: ::prost::alloc::string::String,
    /// A description for this application.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// The draft application configs which haven't been updated to an application.
    #[prost(message, optional, tag = "6")]
    pub draft_application_configs: ::core::option::Option<ApplicationConfigs>,
}
/// Message describing Instance object
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instance {
    /// Output only. name of resource
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. [Output only] Create timestamp
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. [Output only] Update timestamp
    #[prost(message, optional, tag = "8")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels as key value pairs
    #[prost(btree_map = "string, string", tag = "3")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Required. A user friendly display name for the solution.
    #[prost(string, tag = "4")]
    pub display_name: ::prost::alloc::string::String,
    /// A description for this application.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// The input resources for the current application instance.
    /// For example:
    /// input_resources:
    /// visionai.googleapis.com/v1/projects/123/locations/us-central1/clusters/456/streams/stream-a
    #[prost(message, repeated, tag = "6")]
    pub input_resources: ::prost::alloc::vec::Vec<instance::InputResource>,
    /// All the output resources associated to one application instance.
    #[prost(message, repeated, tag = "7")]
    pub output_resources: ::prost::alloc::vec::Vec<instance::OutputResource>,
    /// State of the instance.
    #[prost(enumeration = "instance::State", tag = "9")]
    pub state: i32,
}
/// Nested message and enum types in `Instance`.
pub mod instance {
    /// Message of input resource used in one application instance.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InputResource {
        /// The name of graph node who receives the input resource.
        /// For example:
        /// input_resource:
        /// visionai.googleapis.com/v1/projects/123/locations/us-central1/clusters/456/streams/input-stream-a
        /// consumer_node: stream-input
        #[prost(string, tag = "2")]
        pub consumer_node: ::prost::alloc::string::String,
        /// The specific input resource binding which will consume the current Input
        /// Resource, can be ignored is there is only 1 input binding.
        #[prost(string, tag = "3")]
        pub input_resource_binding: ::prost::alloc::string::String,
        /// Contains resource annotations.
        #[prost(message, optional, tag = "5")]
        pub annotations: ::core::option::Option<super::ResourceAnnotations>,
        /// Required. Specify the input to the application instance.
        #[prost(oneof = "input_resource::InputResourceInformation", tags = "1, 4")]
        pub input_resource_information: ::core::option::Option<
            input_resource::InputResourceInformation,
        >,
    }
    /// Nested message and enum types in `InputResource`.
    pub mod input_resource {
        /// Required. Specify the input to the application instance.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum InputResourceInformation {
            /// The direct input resource name.
            #[prost(string, tag = "1")]
            InputResource(::prost::alloc::string::String),
            /// If the input resource is VisionAI Stream, the associated annotations
            /// can be specified using annotated_stream instead.
            #[prost(message, tag = "4")]
            AnnotatedStream(super::super::StreamWithAnnotation),
        }
    }
    /// Message of output resource used in one application instance.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OutputResource {
        /// The output resource name for the current application instance.
        #[prost(string, tag = "1")]
        pub output_resource: ::prost::alloc::string::String,
        /// The name of graph node who produces the output resource name.
        /// For example:
        /// output_resource:
        /// /projects/123/locations/us-central1/clusters/456/streams/output-application-789-stream-a-occupancy-counting
        /// producer_node: occupancy-counting
        #[prost(string, tag = "2")]
        pub producer_node: ::prost::alloc::string::String,
        /// The specific output resource binding which produces the current
        /// OutputResource.
        #[prost(string, tag = "4")]
        pub output_resource_binding: ::prost::alloc::string::String,
        /// Output only. Whether the output resource is temporary which means the resource is
        /// generated during the deployment of the application.
        /// Temporary resource will be deleted during the undeployment of the
        /// application.
        #[prost(bool, tag = "3")]
        pub is_temporary: bool,
        /// Output only. Whether the output resource is created automatically by the Vision AI App
        /// Platform.
        #[prost(bool, tag = "5")]
        pub autogen: bool,
    }
    /// State of the Instance
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum State {
        /// The default value. This value is used if the state is omitted.
        Unspecified = 0,
        /// State CREATING.
        Creating = 1,
        /// State CREATED.
        Created = 2,
        /// State DEPLOYING.
        Deploying = 3,
        /// State DEPLOYED.
        Deployed = 4,
        /// State UNDEPLOYING.
        Undeploying = 5,
        /// State DELETED.
        Deleted = 6,
        /// State ERROR.
        Error = 7,
        /// State Updating
        Updating = 8,
        /// State Deleting.
        Deleting = 9,
        /// State Fixing.
        Fixing = 10,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Creating => "CREATING",
                State::Created => "CREATED",
                State::Deploying => "DEPLOYING",
                State::Deployed => "DEPLOYED",
                State::Undeploying => "UNDEPLOYING",
                State::Deleted => "DELETED",
                State::Error => "ERROR",
                State::Updating => "UPDATING",
                State::Deleting => "DELETING",
                State::Fixing => "FIXING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "CREATED" => Some(Self::Created),
                "DEPLOYING" => Some(Self::Deploying),
                "DEPLOYED" => Some(Self::Deployed),
                "UNDEPLOYING" => Some(Self::Undeploying),
                "DELETED" => Some(Self::Deleted),
                "ERROR" => Some(Self::Error),
                "UPDATING" => Some(Self::Updating),
                "DELETING" => Some(Self::Deleting),
                "FIXING" => Some(Self::Fixing),
                _ => None,
            }
        }
    }
}
/// Message for creating a Instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplicationInstance {
    /// Required. Id of the requesting object.
    #[prost(string, tag = "1")]
    pub instance_id: ::prost::alloc::string::String,
    /// Required. The resource being created.
    #[prost(message, optional, tag = "2")]
    pub instance: ::core::option::Option<Instance>,
}
/// Message describing Processor object.
/// Next ID: 18
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Processor {
    /// name of resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. [Output only] Create timestamp.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. [Output only] Update timestamp.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels as key value pairs.
    #[prost(btree_map = "string, string", tag = "4")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Required. A user friendly display name for the processor.
    #[prost(string, tag = "5")]
    pub display_name: ::prost::alloc::string::String,
    /// Illustrative sentences for describing the functionality of the processor.
    #[prost(string, tag = "10")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Processor Type.
    #[prost(enumeration = "processor::ProcessorType", tag = "6")]
    pub processor_type: i32,
    /// Model Type.
    #[prost(enumeration = "ModelType", tag = "13")]
    pub model_type: i32,
    /// Source info for customer created processor.
    #[prost(message, optional, tag = "7")]
    pub custom_processor_source_info: ::core::option::Option<CustomProcessorSourceInfo>,
    /// Output only. State of the Processor.
    #[prost(enumeration = "processor::ProcessorState", tag = "8")]
    pub state: i32,
    /// Output only. [Output only] The input / output specifications of a processor, each type
    /// of processor has fixed input / output specs which cannot be altered by
    /// customer.
    #[prost(message, optional, tag = "11")]
    pub processor_io_spec: ::core::option::Option<ProcessorIoSpec>,
    /// Output only. The corresponding configuration can be used in the Application to customize
    /// the behavior of the processor.
    #[prost(string, tag = "14")]
    pub configuration_typeurl: ::prost::alloc::string::String,
    #[prost(
        enumeration = "StreamAnnotationType",
        repeated,
        packed = "false",
        tag = "15"
    )]
    pub supported_annotation_types: ::prost::alloc::vec::Vec<i32>,
    /// Indicates if the processor supports post processing.
    #[prost(bool, tag = "17")]
    pub supports_post_processing: bool,
}
/// Nested message and enum types in `Processor`.
pub mod processor {
    /// Type
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ProcessorType {
        /// Processor Type UNSPECIFIED.
        Unspecified = 0,
        /// Processor Type PRETRAINED.
        /// Pretrained processor is developed by Vision AI App Platform with
        /// state-of-the-art vision data processing functionality, like occupancy
        /// counting or person blur. Pretrained processor is usually publicly
        /// available.
        Pretrained = 1,
        /// Processor Type CUSTOM.
        /// Custom processors are specialized processors which are either uploaded by
        /// customers or imported from other GCP platform (for example Vertex AI).
        /// Custom processor is only visible to the creator.
        Custom = 2,
        /// Processor Type CONNECTOR.
        /// Connector processors are special processors which perform I/O for the
        /// application, they do not processing the data but either deliver the data
        /// to other processors or receive data from other processors.
        Connector = 3,
    }
    impl ProcessorType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ProcessorType::Unspecified => "PROCESSOR_TYPE_UNSPECIFIED",
                ProcessorType::Pretrained => "PRETRAINED",
                ProcessorType::Custom => "CUSTOM",
                ProcessorType::Connector => "CONNECTOR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PROCESSOR_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "PRETRAINED" => Some(Self::Pretrained),
                "CUSTOM" => Some(Self::Custom),
                "CONNECTOR" => Some(Self::Connector),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ProcessorState {
        /// Unspecified Processor state.
        Unspecified = 0,
        /// Processor is being created (not ready for use).
        Creating = 1,
        /// Processor is and ready for use.
        Active = 2,
        /// Processor is being deleted (not ready for use).
        Deleting = 3,
        /// Processor deleted or creation failed .
        Failed = 4,
    }
    impl ProcessorState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ProcessorState::Unspecified => "PROCESSOR_STATE_UNSPECIFIED",
                ProcessorState::Creating => "CREATING",
                ProcessorState::Active => "ACTIVE",
                ProcessorState::Deleting => "DELETING",
                ProcessorState::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PROCESSOR_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "ACTIVE" => Some(Self::Active),
                "DELETING" => Some(Self::Deleting),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
/// Message describing the input / output specifications of a processor.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessorIoSpec {
    /// For processors with input_channel_specs, the processor must be explicitly
    /// connected to another processor.
    #[prost(message, repeated, tag = "3")]
    pub graph_input_channel_specs: ::prost::alloc::vec::Vec<
        processor_io_spec::GraphInputChannelSpec,
    >,
    /// The output artifact specifications for the current processor.
    #[prost(message, repeated, tag = "4")]
    pub graph_output_channel_specs: ::prost::alloc::vec::Vec<
        processor_io_spec::GraphOutputChannelSpec,
    >,
    /// The input resource that needs to be fed from the application instance.
    #[prost(message, repeated, tag = "5")]
    pub instance_resource_input_binding_specs: ::prost::alloc::vec::Vec<
        processor_io_spec::InstanceResourceInputBindingSpec,
    >,
    /// The output resource that the processor will generate per instance.
    /// Other than the explicitly listed output bindings here, all the processors'
    /// GraphOutputChannels can be binded to stream resource. The bind name then is
    /// the same as the GraphOutputChannel's name.
    #[prost(message, repeated, tag = "6")]
    pub instance_resource_output_binding_specs: ::prost::alloc::vec::Vec<
        processor_io_spec::InstanceResourceOutputBindingSpec,
    >,
}
/// Nested message and enum types in `ProcessorIOSpec`.
pub mod processor_io_spec {
    /// Message for input channel specification.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GraphInputChannelSpec {
        /// The name of the current input channel.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// The data types of the current input channel.
        /// When this field has more than 1 value, it means this input channel can be
        /// connected to either of these different data types.
        #[prost(enumeration = "DataType", tag = "2")]
        pub data_type: i32,
        /// If specified, only those detailed data types can be connected to the
        /// processor. For example, jpeg stream for MEDIA, or PredictionResult proto
        /// for PROTO type. If unspecified, then any proto is accepted.
        #[prost(string, repeated, tag = "5")]
        pub accepted_data_type_uris: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        /// Whether the current input channel is required by the processor.
        /// For example, for a processor with required video input and optional audio
        /// input, if video input is missing, the application will be rejected while
        /// the audio input can be missing as long as the video input exists.
        #[prost(bool, tag = "3")]
        pub required: bool,
        /// How many input edges can be connected to this input channel. 0 means
        /// unlimited.
        #[prost(int64, tag = "4")]
        pub max_connection_allowed: i64,
    }
    /// Message for output channel specification.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GraphOutputChannelSpec {
        /// The name of the current output channel.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// The data type of the current output channel.
        #[prost(enumeration = "DataType", tag = "2")]
        pub data_type: i32,
        #[prost(string, tag = "3")]
        pub data_type_uri: ::prost::alloc::string::String,
    }
    /// Message for instance resource channel specification.
    /// External resources are virtual nodes which are not expressed in the
    /// application graph. Each processor expresses its out-graph spec, so customer
    /// is able to override the external source or destinations to the
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InstanceResourceInputBindingSpec {
        /// Name of the input binding, unique within the processor.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(
            oneof = "instance_resource_input_binding_spec::ResourceType",
            tags = "2, 3"
        )]
        pub resource_type: ::core::option::Option<
            instance_resource_input_binding_spec::ResourceType,
        >,
    }
    /// Nested message and enum types in `InstanceResourceInputBindingSpec`.
    pub mod instance_resource_input_binding_spec {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ResourceType {
            /// The configuration proto that includes the Googleapis resources. I.e.
            /// type.googleapis.com/google.cloud.vision.v1.StreamWithAnnotation
            #[prost(string, tag = "2")]
            ConfigTypeUri(::prost::alloc::string::String),
            /// The direct type url of Googleapis resource. i.e.
            /// type.googleapis.com/google.cloud.vision.v1.Asset
            #[prost(string, tag = "3")]
            ResourceTypeUri(::prost::alloc::string::String),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InstanceResourceOutputBindingSpec {
        /// Name of the output binding, unique within the processor.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// The resource type uri of the acceptable output resource.
        #[prost(string, tag = "2")]
        pub resource_type_uri: ::prost::alloc::string::String,
        /// Whether the output resource needs to be explicitly set in the instance.
        /// If it is false, the processor will automatically generate it if required.
        #[prost(bool, tag = "3")]
        pub explicit: bool,
    }
    /// High level data types supported by the processor.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum DataType {
        /// The default value of DataType.
        Unspecified = 0,
        /// Video data type like H264.
        Video = 1,
        /// Protobuf data type, usually used for general data blob.
        Proto = 2,
    }
    impl DataType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DataType::Unspecified => "DATA_TYPE_UNSPECIFIED",
                DataType::Video => "VIDEO",
                DataType::Proto => "PROTO",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DATA_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "VIDEO" => Some(Self::Video),
                "PROTO" => Some(Self::Proto),
                _ => None,
            }
        }
    }
}
/// Describes the source info for a custom processor.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomProcessorSourceInfo {
    /// The original product which holds the custom processor's functionality.
    #[prost(enumeration = "custom_processor_source_info::SourceType", tag = "1")]
    pub source_type: i32,
    /// Output only. Additional info related to the imported custom processor.
    /// Data is filled in by app platform during the processor creation.
    #[prost(btree_map = "string, string", tag = "4")]
    pub additional_info: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Model schema files which specifies the signature of the model.
    /// For VERTEX_CUSTOM models, instances schema is required.
    /// If instances schema is not specified during the processor creation,
    /// VisionAI Platform will try to get it from Vertex, if it doesn't exist, the
    /// creation will fail.
    #[prost(message, optional, tag = "5")]
    pub model_schema: ::core::option::Option<custom_processor_source_info::ModelSchema>,
    /// The path where App Platform loads the artifacts for the custom processor.
    #[prost(oneof = "custom_processor_source_info::ArtifactPath", tags = "2")]
    pub artifact_path: ::core::option::Option<
        custom_processor_source_info::ArtifactPath,
    >,
}
/// Nested message and enum types in `CustomProcessorSourceInfo`.
pub mod custom_processor_source_info {
    /// The schema is defined as an OpenAPI 3.0.2 [Schema
    /// Object](<https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.2.md#schemaObject>).
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ModelSchema {
        /// Cloud Storage location to a YAML file that defines the format of a single
        /// instance used in prediction and explanation requests.
        #[prost(message, optional, tag = "1")]
        pub instances_schema: ::core::option::Option<super::GcsSource>,
        /// Cloud Storage location to a YAML file that defines the prediction and
        /// explanation parameters.
        #[prost(message, optional, tag = "2")]
        pub parameters_schema: ::core::option::Option<super::GcsSource>,
        /// Cloud Storage location to a YAML file that defines the format of a single
        /// prediction or explanation.
        #[prost(message, optional, tag = "3")]
        pub predictions_schema: ::core::option::Option<super::GcsSource>,
    }
    /// Source type of the imported custom processor.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum SourceType {
        /// Source type unspecified.
        Unspecified = 0,
        /// Custom processors coming from Vertex AutoML product.
        VertexAutoml = 1,
        /// Custom processors coming from general custom models from Vertex.
        VertexCustom = 2,
    }
    impl SourceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SourceType::Unspecified => "SOURCE_TYPE_UNSPECIFIED",
                SourceType::VertexAutoml => "VERTEX_AUTOML",
                SourceType::VertexCustom => "VERTEX_CUSTOM",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SOURCE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "VERTEX_AUTOML" => Some(Self::VertexAutoml),
                "VERTEX_CUSTOM" => Some(Self::VertexCustom),
                _ => None,
            }
        }
    }
    /// The path where App Platform loads the artifacts for the custom processor.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ArtifactPath {
        /// The resource name original model hosted in the vertex AI platform.
        #[prost(string, tag = "2")]
        VertexModel(::prost::alloc::string::String),
    }
}
/// Next ID: 24
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessorConfig {
    #[prost(
        oneof = "processor_config::ProcessorConfig",
        tags = "9, 20, 10, 11, 12, 15, 13, 14, 17, 18, 19, 22"
    )]
    pub processor_config: ::core::option::Option<processor_config::ProcessorConfig>,
}
/// Nested message and enum types in `ProcessorConfig`.
pub mod processor_config {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ProcessorConfig {
        /// Configs of stream input processor.
        #[prost(message, tag = "9")]
        VideoStreamInputConfig(super::VideoStreamInputConfig),
        /// Config of AI-enabled input devices.
        #[prost(message, tag = "20")]
        AiEnabledDevicesInputConfig(super::AiEnabledDevicesInputConfig),
        /// Configs of media warehouse processor.
        #[prost(message, tag = "10")]
        MediaWarehouseConfig(super::MediaWarehouseConfig),
        /// Configs of person blur processor.
        #[prost(message, tag = "11")]
        PersonBlurConfig(super::PersonBlurConfig),
        /// Configs of occupancy count processor.
        #[prost(message, tag = "12")]
        OccupancyCountConfig(super::OccupancyCountConfig),
        /// Configs of Person Vehicle Detection processor.
        #[prost(message, tag = "15")]
        PersonVehicleDetectionConfig(super::PersonVehicleDetectionConfig),
        /// Configs of Vertex AutoML vision processor.
        #[prost(message, tag = "13")]
        VertexAutomlVisionConfig(super::VertexAutoMlVisionConfig),
        /// Configs of Vertex AutoML video processor.
        #[prost(message, tag = "14")]
        VertexAutomlVideoConfig(super::VertexAutoMlVideoConfig),
        /// Configs of Vertex Custom processor.
        #[prost(message, tag = "17")]
        VertexCustomConfig(super::VertexCustomConfig),
        /// Configs of General Object Detection processor.
        #[prost(message, tag = "18")]
        GeneralObjectDetectionConfig(super::GeneralObjectDetectionConfig),
        /// Configs of BigQuery processor.
        #[prost(message, tag = "19")]
        BigQueryConfig(super::BigQueryConfig),
        /// Configs of personal_protective_equipment_detection_config
        #[prost(message, tag = "22")]
        PersonalProtectiveEquipmentDetectionConfig(
            super::PersonalProtectiveEquipmentDetectionConfig,
        ),
    }
}
/// Message describing Vision AI stream with application specific annotations.
/// All the StreamAnnotation object inside this message MUST have unique id.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamWithAnnotation {
    /// Vision AI Stream resource name.
    #[prost(string, tag = "1")]
    pub stream: ::prost::alloc::string::String,
    /// Annotations that will be applied to the whole application.
    #[prost(message, repeated, tag = "2")]
    pub application_annotations: ::prost::alloc::vec::Vec<StreamAnnotation>,
    /// Annotations that will be applied to the specific node of the application.
    /// If the same type of the annotations is applied to both application and
    /// node, the node annotation will be added in addition to the global
    /// application one.
    /// For example, if there is one active zone annotation for the whole
    /// application and one active zone annotation for the Occupancy Analytic
    /// processor, then the Occupancy Analytic processor will have two active zones
    /// defined.
    #[prost(message, repeated, tag = "3")]
    pub node_annotations: ::prost::alloc::vec::Vec<
        stream_with_annotation::NodeAnnotation,
    >,
}
/// Nested message and enum types in `StreamWithAnnotation`.
pub mod stream_with_annotation {
    /// Message describing annotations specific to application node.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NodeAnnotation {
        /// The node name of the application graph.
        #[prost(string, tag = "1")]
        pub node: ::prost::alloc::string::String,
        /// The node specific stream annotations.
        #[prost(message, repeated, tag = "2")]
        pub annotations: ::prost::alloc::vec::Vec<super::StreamAnnotation>,
    }
}
/// Message describing annotations specific to application node.
/// This message is a duplication of StreamWithAnnotation.NodeAnnotation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplicationNodeAnnotation {
    /// The node name of the application graph.
    #[prost(string, tag = "1")]
    pub node: ::prost::alloc::string::String,
    /// The node specific stream annotations.
    #[prost(message, repeated, tag = "2")]
    pub annotations: ::prost::alloc::vec::Vec<StreamAnnotation>,
}
/// Message describing general annotation for resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceAnnotations {
    /// Annotations that will be applied to the whole application.
    #[prost(message, repeated, tag = "1")]
    pub application_annotations: ::prost::alloc::vec::Vec<StreamAnnotation>,
    /// Annotations that will be applied to the specific node of the application.
    /// If the same type of the annotations is applied to both application and
    /// node, the node annotation will be added in addition to the global
    /// application one.
    /// For example, if there is one active zone annotation for the whole
    /// application and one active zone annotation for the Occupancy Analytic
    /// processor, then the Occupancy Analytic processor will have two active zones
    /// defined.
    #[prost(message, repeated, tag = "2")]
    pub node_annotations: ::prost::alloc::vec::Vec<ApplicationNodeAnnotation>,
}
/// Message describing Video Stream Input Config.
/// This message should only be used as a placeholder for builtin:stream-input
/// processor, actual stream binding should be specified using corresponding
/// API.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoStreamInputConfig {
    #[deprecated]
    #[prost(string, repeated, tag = "1")]
    pub streams: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[deprecated]
    #[prost(message, repeated, tag = "2")]
    pub streams_with_annotation: ::prost::alloc::vec::Vec<StreamWithAnnotation>,
}
/// Message describing AI-enabled Devices Input Config.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AiEnabledDevicesInputConfig {}
/// Message describing MediaWarehouseConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaWarehouseConfig {
    /// Resource name of the Media Warehouse corpus.
    /// Format:
    /// projects/${project_id}/locations/${location_id}/corpora/${corpus_id}
    #[prost(string, tag = "1")]
    pub corpus: ::prost::alloc::string::String,
    /// Deprecated.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub region: ::prost::alloc::string::String,
    /// The duration for which all media assets, associated metadata, and search
    /// documents can exist.
    #[prost(message, optional, tag = "3")]
    pub ttl: ::core::option::Option<::prost_types::Duration>,
}
/// Message describing FaceBlurConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersonBlurConfig {
    /// Person blur type.
    #[prost(enumeration = "person_blur_config::PersonBlurType", tag = "1")]
    pub person_blur_type: i32,
    /// Whether only blur faces other than the whole object in the processor.
    #[prost(bool, tag = "2")]
    pub faces_only: bool,
}
/// Nested message and enum types in `PersonBlurConfig`.
pub mod person_blur_config {
    /// Type of Person Blur
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PersonBlurType {
        /// PersonBlur Type UNSPECIFIED.
        Unspecified = 0,
        /// FaceBlur Type full occlusion.
        FullOcculusion = 1,
        /// FaceBlur Type blur filter.
        BlurFilter = 2,
    }
    impl PersonBlurType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PersonBlurType::Unspecified => "PERSON_BLUR_TYPE_UNSPECIFIED",
                PersonBlurType::FullOcculusion => "FULL_OCCULUSION",
                PersonBlurType::BlurFilter => "BLUR_FILTER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PERSON_BLUR_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "FULL_OCCULUSION" => Some(Self::FullOcculusion),
                "BLUR_FILTER" => Some(Self::BlurFilter),
                _ => None,
            }
        }
    }
}
/// Message describing OccupancyCountConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OccupancyCountConfig {
    /// Whether to count the appearances of people, output counts have 'people' as
    /// the key.
    #[prost(bool, tag = "1")]
    pub enable_people_counting: bool,
    /// Whether to count the appearances of vehicles, output counts will have
    /// 'vehicle' as the key.
    #[prost(bool, tag = "2")]
    pub enable_vehicle_counting: bool,
    /// Whether to track each invidual object's loitering time inside the scene or
    /// specific zone.
    #[prost(bool, tag = "3")]
    pub enable_dwelling_time_tracking: bool,
}
/// Message describing PersonVehicleDetectionConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersonVehicleDetectionConfig {
    /// At least one of enable_people_counting and enable_vehicle_counting fields
    /// must be set to true.
    /// Whether to count the appearances of people, output counts have 'people' as
    /// the key.
    #[prost(bool, tag = "1")]
    pub enable_people_counting: bool,
    /// Whether to count the appearances of vehicles, output counts will have
    /// 'vehicle' as the key.
    #[prost(bool, tag = "2")]
    pub enable_vehicle_counting: bool,
}
/// Message describing PersonalProtectiveEquipmentDetectionConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersonalProtectiveEquipmentDetectionConfig {
    /// Whether to enable face coverage detection.
    #[prost(bool, tag = "1")]
    pub enable_face_coverage_detection: bool,
    /// Whether to enable head coverage detection.
    #[prost(bool, tag = "2")]
    pub enable_head_coverage_detection: bool,
    /// Whether to enable hands coverage detection.
    #[prost(bool, tag = "3")]
    pub enable_hands_coverage_detection: bool,
}
/// Message of configurations for General Object Detection processor.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeneralObjectDetectionConfig {}
/// Message of configurations for BigQuery processor.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQueryConfig {
    /// BigQuery table resource for Vision AI Platform to ingest annotations to.
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    /// Data Schema
    /// By default, Vision AI Application will try to write annotations to the
    /// target BigQuery table using the following schema:
    ///
    /// ingestion_time: TIMESTAMP, the ingestion time of the original data.
    ///
    /// application: STRING, name of the application which produces the annotation.
    ///
    /// instance: STRING, Id of the instance which produces the annotation.
    ///
    /// node: STRING, name of the application graph node which produces the
    /// annotation.
    ///
    /// annotation: STRING or JSON, the actual annotation protobuf will be
    /// converted to json string with bytes field as 64 encoded string. It can be
    /// written to both String or Json type column.
    ///
    /// To forward annotation data to an existing BigQuery table, customer needs to
    /// make sure the compatibility of the schema.
    /// The map maps application node name to its corresponding cloud function
    /// endpoint to transform the annotations directly to the
    /// google.cloud.bigquery.storage.v1.AppendRowsRequest (only avro_rows or
    /// proto_rows should be set). If configured, annotations produced by
    /// corresponding application node will sent to the Cloud Function at first
    /// before be forwarded to BigQuery.
    ///
    /// If the default table schema doesn't fit, customer is able to transform the
    /// annotation output from Vision AI Application to arbitrary BigQuery table
    /// schema with CloudFunction.
    /// * The cloud function will receive AppPlatformCloudFunctionRequest where
    /// the annotations field will be the json format of Vision AI annotation.
    /// * The cloud function should return AppPlatformCloudFunctionResponse with
    /// AppendRowsRequest stored in the annotations field.
    /// * To drop the annotation, simply clear the annotations field in the
    /// returned AppPlatformCloudFunctionResponse.
    #[prost(btree_map = "string, string", tag = "2")]
    pub cloud_function_mapping: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// If true, App Platform will create the BigQuery DataSet and the
    /// BigQuery Table with default schema if the specified table doesn't exist.
    /// This doesn't work if any cloud function customized schema is specified
    /// since the system doesn't know your desired schema.
    /// JSON column will be used in the default table created by App Platform.
    #[prost(bool, tag = "3")]
    pub create_default_table_if_not_exists: bool,
}
/// Message of configurations of Vertex AutoML Vision Processors.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VertexAutoMlVisionConfig {
    /// Only entities with higher score than the threshold will be returned.
    /// Value 0.0 means to return all the detected entities.
    #[prost(float, tag = "1")]
    pub confidence_threshold: f32,
    /// At most this many predictions will be returned per output frame.
    /// Value 0 means to return all the detected entities.
    #[prost(int32, tag = "2")]
    pub max_predictions: i32,
}
/// Message describing VertexAutoMLVideoConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VertexAutoMlVideoConfig {
    /// Only entities with higher score than the threshold will be returned.
    /// Value 0.0 means returns all the detected entities.
    #[prost(float, tag = "1")]
    pub confidence_threshold: f32,
    /// Labels specified in this field won't be returned.
    #[prost(string, repeated, tag = "2")]
    pub blocked_labels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// At most this many predictions will be returned per output frame.
    /// Value 0 means to return all the detected entities.
    #[prost(int32, tag = "3")]
    pub max_predictions: i32,
    /// Only Bounding Box whose size is larger than this limit will be returned.
    /// Object Tracking only.
    /// Value 0.0 means to return all the detected entities.
    #[prost(float, tag = "4")]
    pub bounding_box_size_limit: f32,
}
/// Message describing VertexCustomConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VertexCustomConfig {
    /// The max prediction frame per second. This attribute sets how fast the
    /// operator sends prediction requests to Vertex AI endpoint. Default value is
    /// 0, which means there is no max prediction fps limit. The operator sends
    /// prediction requests at input fps.
    #[prost(int32, tag = "1")]
    pub max_prediction_fps: i32,
    /// A description of resources that are dedicated to the DeployedModel, and
    /// that need a higher degree of manual configuration.
    #[prost(message, optional, tag = "2")]
    pub dedicated_resources: ::core::option::Option<DedicatedResources>,
    /// If not empty, the prediction result will be sent to the specified cloud
    /// function for post processing.
    /// * The cloud function will receive AppPlatformCloudFunctionRequest where
    /// the annotations field will be the json format of proto PredictResponse.
    /// * The cloud function should return AppPlatformCloudFunctionResponse with
    /// PredictResponse stored in the annotations field.
    /// * To drop the prediction output, simply clear the payload field in the
    /// returned AppPlatformCloudFunctionResponse.
    #[prost(string, tag = "3")]
    pub post_processing_cloud_function: ::prost::alloc::string::String,
    /// If true, the prediction request received by custom model will also contain
    /// metadata with the following schema:
    /// 'appPlatformMetadata': {
    ///        'ingestionTime': DOUBLE; (UNIX timestamp)
    ///        'application': STRING;
    ///        'instanceId': STRING;
    ///        'node': STRING;
    ///        'processor': STRING;
    ///   }
    #[prost(bool, tag = "4")]
    pub attach_application_metadata: bool,
}
/// Specification of a single machine.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MachineSpec {
    /// Immutable. The type of the machine.
    ///
    /// See the [list of machine types supported for
    /// prediction](<https://cloud.google.com/vertex-ai/docs/predictions/configure-compute#machine-types>)
    ///
    /// See the [list of machine types supported for custom
    /// training](<https://cloud.google.com/vertex-ai/docs/training/configure-compute#machine-types>).
    ///
    /// For \[DeployedModel][\] this field is optional, and the default
    /// value is `n1-standard-2`. For \[BatchPredictionJob][\] or as part of
    /// \[WorkerPoolSpec][\] this field is required.
    #[prost(string, tag = "1")]
    pub machine_type: ::prost::alloc::string::String,
    /// Immutable. The type of accelerator(s) that may be attached to the machine as per
    /// \[accelerator_count][google.cloud.visionai.v1.MachineSpec.accelerator_count\].
    #[prost(enumeration = "AcceleratorType", tag = "2")]
    pub accelerator_type: i32,
    /// The number of accelerators to attach to the machine.
    #[prost(int32, tag = "3")]
    pub accelerator_count: i32,
}
/// The metric specification that defines the target resource utilization
/// (CPU utilization, accelerator's duty cycle, and so on) for calculating the
/// desired replica count.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoscalingMetricSpec {
    /// Required. The resource metric name.
    /// Supported metrics:
    ///
    /// * For Online Prediction:
    /// * `aiplatform.googleapis.com/prediction/online/accelerator/duty_cycle`
    /// * `aiplatform.googleapis.com/prediction/online/cpu/utilization`
    #[prost(string, tag = "1")]
    pub metric_name: ::prost::alloc::string::String,
    /// The target resource utilization in percentage (1% - 100%) for the given
    /// metric; once the real usage deviates from the target by a certain
    /// percentage, the machine replicas change. The default value is 60
    /// (representing 60%) if not provided.
    #[prost(int32, tag = "2")]
    pub target: i32,
}
/// A description of resources that are dedicated to a DeployedModel, and
/// that need a higher degree of manual configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DedicatedResources {
    /// Required. Immutable. The specification of a single machine used by the prediction.
    #[prost(message, optional, tag = "1")]
    pub machine_spec: ::core::option::Option<MachineSpec>,
    /// Required. Immutable. The minimum number of machine replicas this DeployedModel will be always
    /// deployed on. This value must be greater than or equal to 1.
    ///
    /// If traffic against the DeployedModel increases, it may dynamically be
    /// deployed onto more replicas, and as traffic decreases, some of these extra
    /// replicas may be freed.
    #[prost(int32, tag = "2")]
    pub min_replica_count: i32,
    /// Immutable. The maximum number of replicas this DeployedModel may be deployed on when
    /// the traffic against it increases. If the requested value is too large,
    /// the deployment will error, but if deployment succeeds then the ability
    /// to scale the model to that many replicas is guaranteed (barring service
    /// outages). If traffic against the DeployedModel increases beyond what its
    /// replicas at maximum may handle, a portion of the traffic will be dropped.
    /// If this value is not provided, will use \[min_replica_count][google.cloud.visionai.v1.DedicatedResources.min_replica_count\] as the
    /// default value.
    ///
    /// The value of this field impacts the charge against Vertex CPU and GPU
    /// quotas. Specifically, you will be charged for max_replica_count *
    /// number of cores in the selected machine type) and (max_replica_count *
    /// number of GPUs per replica in the selected machine type).
    #[prost(int32, tag = "3")]
    pub max_replica_count: i32,
    /// Immutable. The metric specifications that overrides a resource
    /// utilization metric (CPU utilization, accelerator's duty cycle, and so on)
    /// target value (default to 60 if not set). At most one entry is allowed per
    /// metric.
    ///
    /// If \[machine_spec.accelerator_count][google.cloud.visionai.v1.MachineSpec.accelerator_count\] is
    /// above 0, the autoscaling will be based on both CPU utilization and
    /// accelerator's duty cycle metrics and scale up when either metrics exceeds
    /// its target value while scale down if both metrics are under their target
    /// value. The default target value is 60 for both metrics.
    ///
    /// If \[machine_spec.accelerator_count][google.cloud.visionai.v1.MachineSpec.accelerator_count\] is
    /// 0, the autoscaling will be based on CPU utilization metric only with
    /// default target value 60 if not explicitly set.
    ///
    /// For example, in the case of Online Prediction, if you want to override
    /// target CPU utilization to 80, you should set
    /// \[autoscaling_metric_specs.metric_name][google.cloud.visionai.v1.AutoscalingMetricSpec.metric_name\]
    /// to `aiplatform.googleapis.com/prediction/online/cpu/utilization` and
    /// \[autoscaling_metric_specs.target][google.cloud.visionai.v1.AutoscalingMetricSpec.target\] to `80`.
    #[prost(message, repeated, tag = "4")]
    pub autoscaling_metric_specs: ::prost::alloc::vec::Vec<AutoscalingMetricSpec>,
}
/// All the supported model types in Vision AI App Platform.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ModelType {
    /// Processor Type UNSPECIFIED.
    Unspecified = 0,
    /// Model Type Image Classification.
    ImageClassification = 1,
    /// Model Type Object Detection.
    ObjectDetection = 2,
    /// Model Type Video Classification.
    VideoClassification = 3,
    /// Model Type Object Tracking.
    VideoObjectTracking = 4,
    /// Model Type Action Recognition.
    VideoActionRecognition = 5,
    /// Model Type Occupancy Counting.
    OccupancyCounting = 6,
    /// Model Type Person Blur.
    PersonBlur = 7,
    /// Model Type Vertex Custom.
    VertexCustom = 8,
}
impl ModelType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ModelType::Unspecified => "MODEL_TYPE_UNSPECIFIED",
            ModelType::ImageClassification => "IMAGE_CLASSIFICATION",
            ModelType::ObjectDetection => "OBJECT_DETECTION",
            ModelType::VideoClassification => "VIDEO_CLASSIFICATION",
            ModelType::VideoObjectTracking => "VIDEO_OBJECT_TRACKING",
            ModelType::VideoActionRecognition => "VIDEO_ACTION_RECOGNITION",
            ModelType::OccupancyCounting => "OCCUPANCY_COUNTING",
            ModelType::PersonBlur => "PERSON_BLUR",
            ModelType::VertexCustom => "VERTEX_CUSTOM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MODEL_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "IMAGE_CLASSIFICATION" => Some(Self::ImageClassification),
            "OBJECT_DETECTION" => Some(Self::ObjectDetection),
            "VIDEO_CLASSIFICATION" => Some(Self::VideoClassification),
            "VIDEO_OBJECT_TRACKING" => Some(Self::VideoObjectTracking),
            "VIDEO_ACTION_RECOGNITION" => Some(Self::VideoActionRecognition),
            "OCCUPANCY_COUNTING" => Some(Self::OccupancyCounting),
            "PERSON_BLUR" => Some(Self::PersonBlur),
            "VERTEX_CUSTOM" => Some(Self::VertexCustom),
            _ => None,
        }
    }
}
/// Represents a hardware accelerator type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AcceleratorType {
    /// Unspecified accelerator type, which means no accelerator.
    Unspecified = 0,
    /// Nvidia Tesla K80 GPU.
    NvidiaTeslaK80 = 1,
    /// Nvidia Tesla P100 GPU.
    NvidiaTeslaP100 = 2,
    /// Nvidia Tesla V100 GPU.
    NvidiaTeslaV100 = 3,
    /// Nvidia Tesla P4 GPU.
    NvidiaTeslaP4 = 4,
    /// Nvidia Tesla T4 GPU.
    NvidiaTeslaT4 = 5,
    /// Nvidia Tesla A100 GPU.
    NvidiaTeslaA100 = 8,
    /// TPU v2.
    TpuV2 = 6,
    /// TPU v3.
    TpuV3 = 7,
}
impl AcceleratorType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AcceleratorType::Unspecified => "ACCELERATOR_TYPE_UNSPECIFIED",
            AcceleratorType::NvidiaTeslaK80 => "NVIDIA_TESLA_K80",
            AcceleratorType::NvidiaTeslaP100 => "NVIDIA_TESLA_P100",
            AcceleratorType::NvidiaTeslaV100 => "NVIDIA_TESLA_V100",
            AcceleratorType::NvidiaTeslaP4 => "NVIDIA_TESLA_P4",
            AcceleratorType::NvidiaTeslaT4 => "NVIDIA_TESLA_T4",
            AcceleratorType::NvidiaTeslaA100 => "NVIDIA_TESLA_A100",
            AcceleratorType::TpuV2 => "TPU_V2",
            AcceleratorType::TpuV3 => "TPU_V3",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACCELERATOR_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "NVIDIA_TESLA_K80" => Some(Self::NvidiaTeslaK80),
            "NVIDIA_TESLA_P100" => Some(Self::NvidiaTeslaP100),
            "NVIDIA_TESLA_V100" => Some(Self::NvidiaTeslaV100),
            "NVIDIA_TESLA_P4" => Some(Self::NvidiaTeslaP4),
            "NVIDIA_TESLA_T4" => Some(Self::NvidiaTeslaT4),
            "NVIDIA_TESLA_A100" => Some(Self::NvidiaTeslaA100),
            "TPU_V2" => Some(Self::TpuV2),
            "TPU_V3" => Some(Self::TpuV3),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod app_platform_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service describing handlers for resources
    #[derive(Debug, Clone)]
    pub struct AppPlatformClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AppPlatformClient<T>
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
        ) -> AppPlatformClient<InterceptedService<T, F>>
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
            AppPlatformClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists Applications in a given project and location.
        pub async fn list_applications(
            &mut self,
            request: impl tonic::IntoRequest<super::ListApplicationsRequest>,
        ) -> Result<tonic::Response<super::ListApplicationsResponse>, tonic::Status> {
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
                "/google.cloud.visionai.v1.AppPlatform/ListApplications",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single Application.
        pub async fn get_application(
            &mut self,
            request: impl tonic::IntoRequest<super::GetApplicationRequest>,
        ) -> Result<tonic::Response<super::Application>, tonic::Status> {
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
                "/google.cloud.visionai.v1.AppPlatform/GetApplication",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new Application in a given project and location.
        pub async fn create_application(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateApplicationRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.AppPlatform/CreateApplication",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the parameters of a single Application.
        pub async fn update_application(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateApplicationRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.AppPlatform/UpdateApplication",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single Application.
        pub async fn delete_application(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteApplicationRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.AppPlatform/DeleteApplication",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deploys a single Application.
        pub async fn deploy_application(
            &mut self,
            request: impl tonic::IntoRequest<super::DeployApplicationRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.AppPlatform/DeployApplication",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Undeploys a single Application.
        pub async fn undeploy_application(
            &mut self,
            request: impl tonic::IntoRequest<super::UndeployApplicationRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.AppPlatform/UndeployApplication",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Adds target stream input to the Application.
        /// If the Application is deployed, the corresponding new Application instance
        /// will be created. If the stream has already been in the Application, the RPC
        /// will fail.
        pub async fn add_application_stream_input(
            &mut self,
            request: impl tonic::IntoRequest<super::AddApplicationStreamInputRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.AppPlatform/AddApplicationStreamInput",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Remove target stream input to the Application, if the Application is
        /// deployed, the corresponding instance based will be deleted. If the stream
        /// is not in the Application, the RPC will fail.
        pub async fn remove_application_stream_input(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveApplicationStreamInputRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.AppPlatform/RemoveApplicationStreamInput",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Update target stream input to the Application, if the Application is
        /// deployed, the corresponding instance based will be deployed. For
        /// CreateOrUpdate behavior, set allow_missing to true.
        pub async fn update_application_stream_input(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateApplicationStreamInputRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.AppPlatform/UpdateApplicationStreamInput",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists Instances in a given project and location.
        pub async fn list_instances(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInstancesRequest>,
        ) -> Result<tonic::Response<super::ListInstancesResponse>, tonic::Status> {
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
                "/google.cloud.visionai.v1.AppPlatform/ListInstances",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single Instance.
        pub async fn get_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInstanceRequest>,
        ) -> Result<tonic::Response<super::Instance>, tonic::Status> {
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
                "/google.cloud.visionai.v1.AppPlatform/GetInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Adds target stream input to the Application.
        /// If the Application is deployed, the corresponding new Application instance
        /// will be created. If the stream has already been in the Application, the RPC
        /// will fail.
        pub async fn create_application_instances(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateApplicationInstancesRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.AppPlatform/CreateApplicationInstances",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Remove target stream input to the Application, if the Application is
        /// deployed, the corresponding instance based will be deleted. If the stream
        /// is not in the Application, the RPC will fail.
        pub async fn delete_application_instances(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteApplicationInstancesRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.AppPlatform/DeleteApplicationInstances",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Adds target stream input to the Application.
        /// If the Application is deployed, the corresponding new Application instance
        /// will be created. If the stream has already been in the Application, the RPC
        /// will fail.
        pub async fn update_application_instances(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateApplicationInstancesRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.AppPlatform/UpdateApplicationInstances",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists Drafts in a given project and location.
        pub async fn list_drafts(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDraftsRequest>,
        ) -> Result<tonic::Response<super::ListDraftsResponse>, tonic::Status> {
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
                "/google.cloud.visionai.v1.AppPlatform/ListDrafts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single Draft.
        pub async fn get_draft(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDraftRequest>,
        ) -> Result<tonic::Response<super::Draft>, tonic::Status> {
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
                "/google.cloud.visionai.v1.AppPlatform/GetDraft",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new Draft in a given project and location.
        pub async fn create_draft(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDraftRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.AppPlatform/CreateDraft",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the parameters of a single Draft.
        pub async fn update_draft(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDraftRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.AppPlatform/UpdateDraft",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single Draft.
        pub async fn delete_draft(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDraftRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.AppPlatform/DeleteDraft",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists Processors in a given project and location.
        pub async fn list_processors(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProcessorsRequest>,
        ) -> Result<tonic::Response<super::ListProcessorsResponse>, tonic::Status> {
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
                "/google.cloud.visionai.v1.AppPlatform/ListProcessors",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ListPrebuiltProcessors is a custom pass-through verb that Lists Prebuilt
        /// Processors.
        pub async fn list_prebuilt_processors(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPrebuiltProcessorsRequest>,
        ) -> Result<
            tonic::Response<super::ListPrebuiltProcessorsResponse>,
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
                "/google.cloud.visionai.v1.AppPlatform/ListPrebuiltProcessors",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single Processor.
        pub async fn get_processor(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProcessorRequest>,
        ) -> Result<tonic::Response<super::Processor>, tonic::Status> {
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
                "/google.cloud.visionai.v1.AppPlatform/GetProcessor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new Processor in a given project and location.
        pub async fn create_processor(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateProcessorRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.AppPlatform/CreateProcessor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the parameters of a single Processor.
        pub async fn update_processor(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateProcessorRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.AppPlatform/UpdateProcessor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single Processor.
        pub async fn delete_processor(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteProcessorRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.AppPlatform/DeleteProcessor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Message describing the Stream object. The Stream and the Event resources are
/// many to many; i.e., each Stream resource can associate to many Event
/// resources and each Event resource can associate to many Stream resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stream {
    /// Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The create timestamp.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The update timestamp.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels as key value pairs.
    #[prost(btree_map = "string, string", tag = "4")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Annotations to allow clients to store small amounts of arbitrary data.
    #[prost(btree_map = "string, string", tag = "5")]
    pub annotations: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The display name for the stream resource.
    #[prost(string, tag = "6")]
    pub display_name: ::prost::alloc::string::String,
    /// Whether to enable the HLS playback service on this stream.
    #[prost(bool, tag = "7")]
    pub enable_hls_playback: bool,
    /// The name of the media warehouse asset for long term storage of stream data.
    /// Format: projects/${p_id}/locations/${l_id}/corpora/${c_id}/assets/${a_id}
    /// Remain empty if the media warehouse storage is not needed for the stream.
    #[prost(string, tag = "8")]
    pub media_warehouse_asset: ::prost::alloc::string::String,
}
/// Message describing the Event object.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    /// Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The create timestamp.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The update timestamp.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels as key value pairs.
    #[prost(btree_map = "string, string", tag = "4")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Annotations to allow clients to store small amounts of arbitrary data.
    #[prost(btree_map = "string, string", tag = "5")]
    pub annotations: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The clock used for joining streams.
    #[prost(enumeration = "event::Clock", tag = "6")]
    pub alignment_clock: i32,
    /// Grace period for cleaning up the event. This is the time the controller
    /// waits for before deleting the event. During this period, if there is any
    /// active channel on the event. The deletion of the event after grace_period
    /// will be ignored.
    #[prost(message, optional, tag = "7")]
    pub grace_period: ::core::option::Option<::prost_types::Duration>,
}
/// Nested message and enum types in `Event`.
pub mod event {
    /// Clock that will be used for joining streams.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Clock {
        /// Clock is not specified.
        Unspecified = 0,
        /// Use the timestamp when the data is captured. Clients need to sync the
        /// clock.
        Capture = 1,
        /// Use the timestamp when the data is received.
        Ingest = 2,
    }
    impl Clock {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Clock::Unspecified => "CLOCK_UNSPECIFIED",
                Clock::Capture => "CAPTURE",
                Clock::Ingest => "INGEST",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CLOCK_UNSPECIFIED" => Some(Self::Unspecified),
                "CAPTURE" => Some(Self::Capture),
                "INGEST" => Some(Self::Ingest),
                _ => None,
            }
        }
    }
}
/// Message describing the Series object.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Series {
    /// Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The create timestamp.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The update timestamp.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels as key value pairs.
    #[prost(btree_map = "string, string", tag = "4")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Annotations to allow clients to store small amounts of arbitrary data.
    #[prost(btree_map = "string, string", tag = "5")]
    pub annotations: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Required. Stream that is associated with this series.
    #[prost(string, tag = "6")]
    pub stream: ::prost::alloc::string::String,
    /// Required. Event that is associated with this series.
    #[prost(string, tag = "7")]
    pub event: ::prost::alloc::string::String,
}
/// Message describing the Channel object.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Channel {
    /// Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The create timestamp.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The update timestamp.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels as key value pairs.
    #[prost(btree_map = "string, string", tag = "4")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Annotations to allow clients to store small amounts of arbitrary data.
    #[prost(btree_map = "string, string", tag = "5")]
    pub annotations: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Required. Stream that is associated with this series.
    #[prost(string, tag = "6")]
    pub stream: ::prost::alloc::string::String,
    /// Required. Event that is associated with this series.
    #[prost(string, tag = "7")]
    pub event: ::prost::alloc::string::String,
}
/// Message for requesting list of Clusters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClustersRequest {
    /// Required. Parent value for ListClustersRequest.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Hint for how to order the results.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Message for response to listing Clusters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClustersResponse {
    /// The list of Cluster.
    #[prost(message, repeated, tag = "1")]
    pub clusters: ::prost::alloc::vec::Vec<Cluster>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a Cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClusterRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for creating a Cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateClusterRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Id of the requesting object.
    #[prost(string, tag = "2")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Required. The resource being created.
    #[prost(message, optional, tag = "3")]
    pub cluster: ::core::option::Option<Cluster>,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for updating a Cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateClusterRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// Cluster resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The resource being updated
    #[prost(message, optional, tag = "2")]
    pub cluster: ::core::option::Option<Cluster>,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for deleting a Cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteClusterRequest {
    /// Required. Name of the resource
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for requesting list of Streams.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStreamsRequest {
    /// Required. Parent value for ListStreamsRequest.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Hint for how to order the results.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Message for response to listing Streams.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStreamsResponse {
    /// The list of Stream.
    #[prost(message, repeated, tag = "1")]
    pub streams: ::prost::alloc::vec::Vec<Stream>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a Stream.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStreamRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for creating a Stream.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateStreamRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Id of the requesting object.
    #[prost(string, tag = "2")]
    pub stream_id: ::prost::alloc::string::String,
    /// Required. The resource being created.
    #[prost(message, optional, tag = "3")]
    pub stream: ::core::option::Option<Stream>,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for updating a Stream.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateStreamRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// Stream resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The resource being updated.
    #[prost(message, optional, tag = "2")]
    pub stream: ::core::option::Option<Stream>,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for deleting a Stream.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteStreamRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for getting the thumbnail of a Stream.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStreamThumbnailRequest {
    /// Required. The name of the stream for to get the thumbnail from.
    #[prost(string, tag = "1")]
    pub stream: ::prost::alloc::string::String,
    /// Required. The name of the GCS object to store the thumbnail image.
    #[prost(string, tag = "2")]
    pub gcs_object_name: ::prost::alloc::string::String,
    /// Optional. The name of the event. If unspecified, the thumbnail will be retrieved from
    /// the latest event.
    #[prost(string, tag = "3")]
    pub event: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify the requests. Specify a unique request
    /// ID so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for the response of GetStreamThumbnail. The empty response message
/// indicates the thumbnail image has been uploaded to GCS successfully.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStreamThumbnailResponse {}
/// Request message for getting the auth token to access the stream HLS contents.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateStreamHlsTokenRequest {
    /// Required. The name of the stream.
    #[prost(string, tag = "1")]
    pub stream: ::prost::alloc::string::String,
}
/// Response message for GenerateStreamHlsToken.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateStreamHlsTokenResponse {
    /// The generated JWT token.
    ///
    /// The caller should insert this token to the authorization header of the HTTP
    /// requests to get the HLS playlist manifest and the video chunks.
    /// eg: curl -H "Authorization: Bearer $TOKEN"
    ///      <https://domain.com/test-stream.playback/master.m3u8>
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    /// The expiration time of the token.
    #[prost(message, optional, tag = "2")]
    pub expiration_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Message for requesting list of Events.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEventsRequest {
    /// Required. Parent value for ListEventsRequest.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Hint for how to order the results.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Message for response to listing Events.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEventsResponse {
    /// The list of Event.
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<Event>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a Event.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEventRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for creating a Event.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEventRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Id of the requesting object.
    #[prost(string, tag = "2")]
    pub event_id: ::prost::alloc::string::String,
    /// Required. The resource being created.
    #[prost(message, optional, tag = "3")]
    pub event: ::core::option::Option<Event>,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for updating a Event.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEventRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// Event resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The resource being updated.
    #[prost(message, optional, tag = "2")]
    pub event: ::core::option::Option<Event>,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for deleting a Event.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEventRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for requesting list of Series.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSeriesRequest {
    /// Required. Parent value for ListSeriesRequest.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Hint for how to order the results.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Message for response to listing Series.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSeriesResponse {
    /// The list of Series.
    #[prost(message, repeated, tag = "1")]
    pub series: ::prost::alloc::vec::Vec<Series>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a Series.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSeriesRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for creating a Series.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSeriesRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Id of the requesting object.
    #[prost(string, tag = "2")]
    pub series_id: ::prost::alloc::string::String,
    /// Required. The resource being created.
    #[prost(message, optional, tag = "3")]
    pub series: ::core::option::Option<Series>,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for updating a Series.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSeriesRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the Series
    /// resource by the update. The fields specified in the update_mask are
    /// relative to the resource, not the full request. A field will be overwritten
    /// if it is in the mask. If the user does not provide a mask then all fields
    /// will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The resource being updated
    #[prost(message, optional, tag = "2")]
    pub series: ::core::option::Option<Series>,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for deleting a Series.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSeriesRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for materializing a channel.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaterializeChannelRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Id of the channel.
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
    /// Required. The resource being created.
    #[prost(message, optional, tag = "3")]
    pub channel: ::core::option::Option<Channel>,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod streams_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service describing handlers for resources.
    /// Vision API and Vision AI API are two independent APIs developed by the same
    /// team. Vision API is for people to annotate their image while Vision AI is an
    /// e2e solution for customer to build their own computer vision application.
    #[derive(Debug, Clone)]
    pub struct StreamsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> StreamsServiceClient<T>
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
        ) -> StreamsServiceClient<InterceptedService<T, F>>
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
            StreamsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists Clusters in a given project and location.
        pub async fn list_clusters(
            &mut self,
            request: impl tonic::IntoRequest<super::ListClustersRequest>,
        ) -> Result<tonic::Response<super::ListClustersResponse>, tonic::Status> {
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
                "/google.cloud.visionai.v1.StreamsService/ListClusters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single Cluster.
        pub async fn get_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::GetClusterRequest>,
        ) -> Result<tonic::Response<super::Cluster>, tonic::Status> {
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
                "/google.cloud.visionai.v1.StreamsService/GetCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new Cluster in a given project and location.
        pub async fn create_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateClusterRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.StreamsService/CreateCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the parameters of a single Cluster.
        pub async fn update_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateClusterRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.StreamsService/UpdateCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single Cluster.
        pub async fn delete_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteClusterRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.StreamsService/DeleteCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists Streams in a given project and location.
        pub async fn list_streams(
            &mut self,
            request: impl tonic::IntoRequest<super::ListStreamsRequest>,
        ) -> Result<tonic::Response<super::ListStreamsResponse>, tonic::Status> {
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
                "/google.cloud.visionai.v1.StreamsService/ListStreams",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single Stream.
        pub async fn get_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::GetStreamRequest>,
        ) -> Result<tonic::Response<super::Stream>, tonic::Status> {
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
                "/google.cloud.visionai.v1.StreamsService/GetStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new Stream in a given project and location.
        pub async fn create_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateStreamRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.StreamsService/CreateStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the parameters of a single Stream.
        pub async fn update_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateStreamRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.StreamsService/UpdateStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single Stream.
        pub async fn delete_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteStreamRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.StreamsService/DeleteStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the thumbnail (image snapshot) of a single Stream.
        pub async fn get_stream_thumbnail(
            &mut self,
            request: impl tonic::IntoRequest<super::GetStreamThumbnailRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.StreamsService/GetStreamThumbnail",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Generate the JWT auth token required to get the stream HLS contents.
        pub async fn generate_stream_hls_token(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateStreamHlsTokenRequest>,
        ) -> Result<
            tonic::Response<super::GenerateStreamHlsTokenResponse>,
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
                "/google.cloud.visionai.v1.StreamsService/GenerateStreamHlsToken",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists Events in a given project and location.
        pub async fn list_events(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEventsRequest>,
        ) -> Result<tonic::Response<super::ListEventsResponse>, tonic::Status> {
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
                "/google.cloud.visionai.v1.StreamsService/ListEvents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single Event.
        pub async fn get_event(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEventRequest>,
        ) -> Result<tonic::Response<super::Event>, tonic::Status> {
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
                "/google.cloud.visionai.v1.StreamsService/GetEvent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new Event in a given project and location.
        pub async fn create_event(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEventRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.StreamsService/CreateEvent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the parameters of a single Event.
        pub async fn update_event(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEventRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.StreamsService/UpdateEvent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single Event.
        pub async fn delete_event(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEventRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.StreamsService/DeleteEvent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists Series in a given project and location.
        pub async fn list_series(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSeriesRequest>,
        ) -> Result<tonic::Response<super::ListSeriesResponse>, tonic::Status> {
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
                "/google.cloud.visionai.v1.StreamsService/ListSeries",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single Series.
        pub async fn get_series(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSeriesRequest>,
        ) -> Result<tonic::Response<super::Series>, tonic::Status> {
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
                "/google.cloud.visionai.v1.StreamsService/GetSeries",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new Series in a given project and location.
        pub async fn create_series(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSeriesRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.StreamsService/CreateSeries",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the parameters of a single Event.
        pub async fn update_series(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSeriesRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.StreamsService/UpdateSeries",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single Series.
        pub async fn delete_series(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSeriesRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.StreamsService/DeleteSeries",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Materialize a channel.
        pub async fn materialize_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::MaterializeChannelRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.StreamsService/MaterializeChannel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Represents an actual value of an operator attribute.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttributeValue {
    /// Attribute value.
    #[prost(oneof = "attribute_value::Value", tags = "1, 2, 3, 4")]
    pub value: ::core::option::Option<attribute_value::Value>,
}
/// Nested message and enum types in `AttributeValue`.
pub mod attribute_value {
    /// Attribute value.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// int.
        #[prost(int64, tag = "1")]
        I(i64),
        /// float.
        #[prost(float, tag = "2")]
        F(f32),
        /// bool.
        #[prost(bool, tag = "3")]
        B(bool),
        /// string.
        #[prost(bytes, tag = "4")]
        S(::prost::bytes::Bytes),
    }
}
/// Defines an Analyzer.
///
/// An analyzer processes data from its input streams using the logic defined in
/// the Operator that it represents. Of course, it produces data for the output
/// streams declared in the Operator.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzerDefinition {
    /// The name of this analyzer.
    ///
    /// Tentatively \[a-z][a-z0-9]*(_[a-z0-9\]+)*.
    #[prost(string, tag = "1")]
    pub analyzer: ::prost::alloc::string::String,
    /// The name of the operator that this analyzer runs.
    ///
    /// Must match the name of a supported operator.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// Input streams.
    #[prost(message, repeated, tag = "3")]
    pub inputs: ::prost::alloc::vec::Vec<analyzer_definition::StreamInput>,
    /// The attribute values that this analyzer applies to the operator.
    ///
    /// Supply a mapping between the attribute names and the actual value you wish
    /// to apply. If an attribute name is omitted, then it will take a
    /// preconfigured default value.
    #[prost(btree_map = "string, message", tag = "4")]
    pub attrs: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        AttributeValue,
    >,
    /// Debug options.
    #[prost(message, optional, tag = "5")]
    pub debug_options: ::core::option::Option<analyzer_definition::DebugOptions>,
}
/// Nested message and enum types in `AnalyzerDefinition`.
pub mod analyzer_definition {
    /// The inputs to this analyzer.
    ///
    /// We accept input name references of the following form:
    /// <analyzer-name>:<output-argument-name>
    ///
    /// Example:
    ///
    /// Suppose you had an operator named "SomeOp" that has 2 output
    /// arguments, the first of which is named "foo" and the second of which is
    /// named "bar", and an operator named "MyOp" that accepts 2 inputs.
    ///
    /// Also suppose that there is an analyzer named "some-analyzer" that is
    /// running "SomeOp" and another analyzer named "my-analyzer" running "MyOp".
    ///
    /// To indicate that "my-analyzer" is to consume "some-analyzer"'s "foo"
    /// output as its first input and "some-analyzer"'s "bar" output as its
    /// second input, you can set this field to the following:
    /// input = ["some-analyzer:foo", "some-analyzer:bar"]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StreamInput {
        /// The name of the stream input (as discussed above).
        #[prost(string, tag = "1")]
        pub input: ::prost::alloc::string::String,
    }
    /// Options available for debugging purposes only.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DebugOptions {
        /// Environment variables.
        #[prost(btree_map = "string, string", tag = "1")]
        pub environment_variables: ::prost::alloc::collections::BTreeMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
    }
}
/// Defines a full analysis.
///
/// This is a description of the overall live analytics pipeline.
/// You may think of this as an edge list representation of a multigraph.
///
/// This may be directly authored by a human in protobuf textformat, or it may be
/// generated by a programming API (perhaps Python or JavaScript depending on
/// context).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalysisDefinition {
    /// Analyzer definitions.
    #[prost(message, repeated, tag = "1")]
    pub analyzers: ::prost::alloc::vec::Vec<AnalyzerDefinition>,
}
/// Message describing the Analysis object.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Analysis {
    /// The name of resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The create timestamp.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The update timestamp.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels as key value pairs.
    #[prost(btree_map = "string, string", tag = "4")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The definition of the analysis.
    #[prost(message, optional, tag = "5")]
    pub analysis_definition: ::core::option::Option<AnalysisDefinition>,
    /// Map from the input parameter in the definition to the real stream.
    /// E.g., suppose you had a stream source operator named "input-0" and you try
    /// to receive from the real stream "stream-0". You can add the following
    /// mapping: [input-0: stream-0].
    #[prost(btree_map = "string, string", tag = "6")]
    pub input_streams_mapping: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Map from the output parameter in the definition to the real stream.
    /// E.g., suppose you had a stream sink operator named "output-0" and you try
    /// to send to the real stream "stream-0". You can add the following
    /// mapping: [output-0: stream-0].
    #[prost(btree_map = "string, string", tag = "7")]
    pub output_streams_mapping: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Request message for CreateAssetRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAssetRequest {
    /// Required. The parent resource where this asset will be created.
    /// Format: projects/*/locations/*/corpora/*
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The asset to create.
    #[prost(message, optional, tag = "2")]
    pub asset: ::core::option::Option<Asset>,
    /// Optional. The ID to use for the asset, which will become the final component of
    /// the asset's resource name if user choose to specify. Otherwise, asset id
    /// will be generated by system.
    ///
    /// This value should be up to 63 characters, and valid characters
    /// are /\[a-z][0-9\]-/. The first character must be a letter, the last could be
    /// a letter or a number.
    #[prost(string, optional, tag = "3")]
    pub asset_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// Request message for GetAsset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAssetRequest {
    /// Required. The name of the asset to retrieve.
    /// Format:
    /// projects/{project_number}/locations/{location}/corpora/{corpus}/assets/{asset}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListAssets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAssetsRequest {
    /// Required. The parent, which owns this collection of assets.
    /// Format:
    /// projects/{project_number}/locations/{location}/corpora/{corpus}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of assets to return. The service may return fewer than
    /// this value.
    /// If unspecified, at most 50 assets will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListAssets` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListAssets` must match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListAssets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAssetsResponse {
    /// The assets from the specified corpus.
    #[prost(message, repeated, tag = "1")]
    pub assets: ::prost::alloc::vec::Vec<Asset>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Response message for UpdateAsset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAssetRequest {
    /// Required. The asset to update.
    ///
    /// The asset's `name` field is used to identify the asset to be updated.
    /// Format:
    /// projects/{project_number}/locations/{location}/corpora/{corpus}/assets/{asset}
    #[prost(message, optional, tag = "1")]
    pub asset: ::core::option::Option<Asset>,
    /// The list of fields to be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for DeleteAsset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAssetRequest {
    /// Required. The name of the asset to delete.
    /// Format:
    /// projects/{project_number}/locations/{location}/corpora/{corpus}/assets/{asset}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// An asset is a resource in corpus. It represents a media object inside corpus,
/// contains metadata and another resource annotation. Different feature could be
/// applied to the asset to generate annotations. User could specified annotation
/// related to the target asset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Asset {
    /// Resource name of the asset.
    /// Form:
    /// `projects/{project_number}/locations/{location_id}/corpora/{corpus_id}/assets/{asset_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The duration for which all media assets, associated metadata, and search
    /// documents can exist. If not set, then it will using the default ttl in the
    /// parent corpus resource.
    #[prost(message, optional, tag = "2")]
    pub ttl: ::core::option::Option<::prost_types::Duration>,
}
/// Request message of CreateCorpus API.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCorpusRequest {
    /// Required. Form: `projects/{project_number}/locations/{location_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The corpus to be created.
    #[prost(message, optional, tag = "2")]
    pub corpus: ::core::option::Option<Corpus>,
}
/// Metadata for CreateCorpus API.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCorpusMetadata {}
/// Corpus is a set of video contents for management. Within a corpus, videos
/// share the same data schema. Search is also restricted within a single corpus.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Corpus {
    /// Resource name of the corpus.
    /// Form:
    /// `projects/{project_number}/locations/{location_id}/corpora/{corpus_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The corpus name to shown in the UI. The name can be up to 32 characters
    /// long.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. Description of the corpus. Can be up to 25000 characters long.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Required. The default TTL value for all assets under the corpus without a asset level
    /// user-defined TTL with a maximum of 10 years. This is required for all
    /// corpora.
    #[prost(message, optional, tag = "5")]
    pub default_ttl: ::core::option::Option<::prost_types::Duration>,
}
/// Request message for GetCorpus.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCorpusRequest {
    /// Required. The resource name of the corpus to retrieve.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateCorpus.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCorpusRequest {
    /// Required. The corpus which replaces the resource on the server.
    #[prost(message, optional, tag = "1")]
    pub corpus: ::core::option::Option<Corpus>,
    /// The list of fields to be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for ListCorpora.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCorporaRequest {
    /// Required. The resource name of the project from which to list corpora.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. API may return fewer results than requested.
    /// If negative, INVALID_ARGUMENT error will be returned.
    /// If unspecified or 0, API will pick a default size, which is 10.
    /// If the requested page size is larger than the maximum size, API will pick
    /// use the maximum size, which is 20.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results for the server to return.
    /// Typically obtained via \[ListCorpora.next_page_token][\] of the previous
    /// \[Warehouse.ListCorpora][google.cloud.visionai.v1.Warehouse.ListCorpora\] call.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListCorpora.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCorporaResponse {
    /// The corpora in the project.
    #[prost(message, repeated, tag = "1")]
    pub corpora: ::prost::alloc::vec::Vec<Corpus>,
    /// A token to retrieve next page of results.
    /// Pass to \[ListCorporaRequest.page_token][google.cloud.visionai.v1.ListCorporaRequest.page_token\] to obtain that page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for DeleteCorpus.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCorpusRequest {
    /// Required. The resource name of the corpus to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for CreateDataSchema.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDataSchemaRequest {
    /// Required. The parent resource where this data schema will be created.
    /// Format: projects/*/locations/*/corpora/*
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The data schema to create.
    #[prost(message, optional, tag = "2")]
    pub data_schema: ::core::option::Option<DataSchema>,
}
/// Data schema indicates how the user specified annotation is interpreted in the
/// system.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataSchema {
    /// Resource name of the data schema in the form of:
    /// `projects/{project_number}/locations/{location}/corpora/{corpus}/dataSchemas/{data_schema}`
    /// where {data_schema} part should be the same as the `key` field below.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The key of this data schema. This key should be matching the key of user
    /// specified annotation and unique inside corpus. This value can be up to
    /// 63 characters, and valid characters are /\[a-z][0-9\]-/. The first character
    /// must be a letter, the last could be a letter or a number.
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    /// The schema details mapping to the key.
    #[prost(message, optional, tag = "3")]
    pub schema_details: ::core::option::Option<DataSchemaDetails>,
}
/// Data schema details indicates the data type and the data struct corresponding
/// to the key of user specified annotation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataSchemaDetails {
    /// Type of the annotation.
    #[prost(enumeration = "data_schema_details::DataType", tag = "1")]
    pub r#type: i32,
    /// Config for protobuf any type.
    #[prost(message, optional, tag = "6")]
    pub proto_any_config: ::core::option::Option<data_schema_details::ProtoAnyConfig>,
    /// The granularity associated with this DataSchema.
    #[prost(enumeration = "data_schema_details::Granularity", tag = "5")]
    pub granularity: i32,
    /// The search strategy to be applied on the `key` above.
    #[prost(message, optional, tag = "7")]
    pub search_strategy: ::core::option::Option<data_schema_details::SearchStrategy>,
}
/// Nested message and enum types in `DataSchemaDetails`.
pub mod data_schema_details {
    /// The configuration for `PROTO_ANY` data type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProtoAnyConfig {
        /// The type URI of the proto message.
        #[prost(string, tag = "1")]
        pub type_uri: ::prost::alloc::string::String,
    }
    /// The search strategy for annotations value of the `key`.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SearchStrategy {
        /// The type of search strategy to be applied on the `key` above.
        /// The allowed `search_strategy_type` is different for different data types,
        /// which is documented in the DataSchemaDetails.DataType. Specifying
        /// unsupported `search_strategy_type` for data types will result in
        /// INVALID_ARGUMENT error.
        #[prost(enumeration = "search_strategy::SearchStrategyType", tag = "1")]
        pub search_strategy_type: i32,
    }
    /// Nested message and enum types in `SearchStrategy`.
    pub mod search_strategy {
        /// The types of search strategies to be applied on the annotation key.
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum SearchStrategyType {
            /// Annotatation values of the `key` above will not be searchable.
            NoSearch = 0,
            /// When searching with `key`, the value must be exactly as the annotation
            /// value that has been ingested.
            ExactSearch = 1,
            /// When searching with `key`, Warehouse will perform broad search based on
            /// semantic of the annotation value.
            SmartSearch = 2,
        }
        impl SearchStrategyType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    SearchStrategyType::NoSearch => "NO_SEARCH",
                    SearchStrategyType::ExactSearch => "EXACT_SEARCH",
                    SearchStrategyType::SmartSearch => "SMART_SEARCH",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "NO_SEARCH" => Some(Self::NoSearch),
                    "EXACT_SEARCH" => Some(Self::ExactSearch),
                    "SMART_SEARCH" => Some(Self::SmartSearch),
                    _ => None,
                }
            }
        }
    }
    /// Data type of the annotation.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum DataType {
        /// Unspecified type.
        Unspecified = 0,
        /// Integer type.
        /// Allowed search strategies:
        /// - DataSchema.SearchStrategy.NO_SEARCH,
        /// - DataSchema.SearchStrategy.EXACT_SEARCH.
        ///    Supports query by IntRangeArray.
        Integer = 1,
        /// Float type.
        /// Allowed search strategies:
        /// - DataSchema.SearchStrategy.NO_SEARCH,
        /// - DataSchema.SearchStrategy.EXACT_SEARCH.
        ///    Supports query by FloatRangeArray.
        Float = 2,
        /// String type.
        /// Allowed search strategies:
        /// - DataSchema.SearchStrategy.NO_SEARCH,
        /// - DataSchema.SearchStrategy.EXACT_SEARCH,
        /// - DataSchema.SearchStrategy.SMART_SEARCH.
        String = 3,
        /// Supported formats:
        /// %Y-%m-%dT%H:%M:%E*S%E*z (absl::RFC3339_full)
        /// %Y-%m-%dT%H:%M:%E*S
        /// %Y-%m-%dT%H:%M%E*z
        /// %Y-%m-%dT%H:%M
        /// %Y-%m-%dT%H%E*z
        /// %Y-%m-%dT%H
        /// %Y-%m-%d%E*z
        /// %Y-%m-%d
        /// %Y-%m
        /// %Y
        /// Allowed search strategies:
        /// - DataSchema.SearchStrategy.NO_SEARCH,
        /// - DataSchema.SearchStrategy.EXACT_SEARCH.
        ///    Supports query by DateTimeRangeArray.
        Datetime = 5,
        /// Geo coordinate type.
        /// Allowed search strategies:
        /// - DataSchema.SearchStrategy.NO_SEARCH,
        /// - DataSchema.SearchStrategy.EXACT_SEARCH.
        ///    Supports query by GeoLocationArray.
        GeoCoordinate = 7,
        /// Type to pass any proto as available in annotations.proto. Only use
        /// internally.
        /// Available proto types and its corresponding search behavior:
        /// - ImageObjectDetectionPredictionResult, allows SMART_SEARCH on
        ///    display_names and NO_SEARCH.
        /// - ClassificationPredictionResult, allows SMART_SEARCH on display_names
        ///    and NO_SEARCH.
        /// - ImageSegmentationPredictionResult, allows NO_SEARCH.
        /// - VideoActionRecognitionPredictionResult, allows SMART_SEARCH on
        ///    display_name and NO_SEARCH.
        /// - VideoObjectTrackingPredictionResult, allows SMART_SEARCH on
        ///    display_name and NO_SEARCH.
        /// - VideoClassificationPredictionResult, allows SMART_SEARCH on
        ///    display_name and NO_SEARCH.
        /// - OccupancyCountingPredictionResult, allows EXACT_SEARCH on
        ///    stats.full_frame_count.count and NO_SEARCH.
        /// - ObjectDetectionPredictionResult, allows SMART_SEARCH on
        ///    identified_boxes.entity.label_string and NO_SEARCH.
        ProtoAny = 8,
        /// Boolean type.
        /// Allowed search strategies:
        /// - DataSchema.SearchStrategy.NO_SEARCH,
        /// - DataSchema.SearchStrategy.EXACT_SEARCH.
        Boolean = 9,
    }
    impl DataType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DataType::Unspecified => "DATA_TYPE_UNSPECIFIED",
                DataType::Integer => "INTEGER",
                DataType::Float => "FLOAT",
                DataType::String => "STRING",
                DataType::Datetime => "DATETIME",
                DataType::GeoCoordinate => "GEO_COORDINATE",
                DataType::ProtoAny => "PROTO_ANY",
                DataType::Boolean => "BOOLEAN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DATA_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "INTEGER" => Some(Self::Integer),
                "FLOAT" => Some(Self::Float),
                "STRING" => Some(Self::String),
                "DATETIME" => Some(Self::Datetime),
                "GEO_COORDINATE" => Some(Self::GeoCoordinate),
                "PROTO_ANY" => Some(Self::ProtoAny),
                "BOOLEAN" => Some(Self::Boolean),
                _ => None,
            }
        }
    }
    /// The granularity of annotations under this DataSchema.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Granularity {
        /// Unspecified granularity.
        Unspecified = 0,
        /// Asset-level granularity (annotations must not contain partition info).
        AssetLevel = 1,
        /// Partition-level granularity (annotations must contain partition info).
        PartitionLevel = 2,
    }
    impl Granularity {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Granularity::Unspecified => "GRANULARITY_UNSPECIFIED",
                Granularity::AssetLevel => "GRANULARITY_ASSET_LEVEL",
                Granularity::PartitionLevel => "GRANULARITY_PARTITION_LEVEL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "GRANULARITY_UNSPECIFIED" => Some(Self::Unspecified),
                "GRANULARITY_ASSET_LEVEL" => Some(Self::AssetLevel),
                "GRANULARITY_PARTITION_LEVEL" => Some(Self::PartitionLevel),
                _ => None,
            }
        }
    }
}
/// Request message for UpdateDataSchema.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDataSchemaRequest {
    /// Required. The data schema's `name` field is used to identify the data schema to be
    /// updated. Format:
    /// projects/{project_number}/locations/{location}/corpora/{corpus}/dataSchemas/{data_schema}
    #[prost(message, optional, tag = "1")]
    pub data_schema: ::core::option::Option<DataSchema>,
    /// The list of fields to be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for GetDataSchema.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataSchemaRequest {
    /// Required. The name of the data schema to retrieve.
    /// Format:
    /// projects/{project_number}/locations/{location_id}/corpora/{corpus_id}/dataSchemas/{data_schema_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for DeleteDataSchema.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDataSchemaRequest {
    /// Required. The name of the data schema to delete.
    /// Format:
    /// projects/{project_number}/locations/{location_id}/corpora/{corpus_id}/dataSchemas/{data_schema_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListDataSchemas.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataSchemasRequest {
    /// Required. The parent, which owns this collection of data schemas.
    /// Format:
    /// projects/{project_number}/locations/{location_id}/corpora/{corpus_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of data schemas to return. The service may return fewer
    /// than this value. If unspecified, at most 50 data schemas will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListDataSchemas` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListDataSchemas` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListDataSchemas.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataSchemasResponse {
    /// The data schemas from the specified corpus.
    #[prost(message, repeated, tag = "1")]
    pub data_schemas: ::prost::alloc::vec::Vec<DataSchema>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for CreateAnnotation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAnnotationRequest {
    /// Required. The parent resource where this annotation will be created.
    /// Format: projects/*/locations/*/corpora/*/assets/*
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The annotation to create.
    #[prost(message, optional, tag = "2")]
    pub annotation: ::core::option::Option<Annotation>,
    /// Optional. The ID to use for the annotation, which will become the final component of
    /// the annotation's resource name if user choose to specify. Otherwise,
    /// annotation id will be generated by system.
    ///
    /// This value should be up to 63 characters, and valid characters
    /// are /\[a-z][0-9\]-/. The first character must be a letter, the last could be
    /// a letter or a number.
    #[prost(string, optional, tag = "3")]
    pub annotation_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// An annotation is a resource in asset. It represents a key-value mapping of
/// content in asset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Annotation {
    /// Resource name of the annotation.
    /// Form:
    /// `projects/{project_number}/locations/{location}/corpora/{corpus}/assets/{asset}/annotations/{annotation}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// User provided annotation.
    #[prost(message, optional, tag = "2")]
    pub user_specified_annotation: ::core::option::Option<UserSpecifiedAnnotation>,
}
/// Annotation provided by users.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserSpecifiedAnnotation {
    /// Required. Key of the annotation. The key must be set with type by CreateDataSchema.
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// Value of the annotation. The value must be able to convert
    /// to the type according to the data schema.
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<AnnotationValue>,
    /// Partition information in time and space for the sub-asset level annotation.
    #[prost(message, optional, tag = "3")]
    pub partition: ::core::option::Option<Partition>,
}
/// Location Coordinate Representation
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoCoordinate {
    /// Latitude Coordinate. Degrees [-90 .. 90]
    #[prost(double, tag = "1")]
    pub latitude: f64,
    /// Longitude Coordinate. Degrees [-180 .. 180]
    #[prost(double, tag = "2")]
    pub longitude: f64,
}
/// Value of annotation, including all types available in data schema.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotationValue {
    #[prost(oneof = "annotation_value::Value", tags = "1, 2, 3, 5, 7, 8, 9, 10")]
    pub value: ::core::option::Option<annotation_value::Value>,
}
/// Nested message and enum types in `AnnotationValue`.
pub mod annotation_value {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// Value of int type annotation.
        #[prost(int64, tag = "1")]
        IntValue(i64),
        /// Value of float type annotation.
        #[prost(float, tag = "2")]
        FloatValue(f32),
        /// Value of string type annotation.
        #[prost(string, tag = "3")]
        StrValue(::prost::alloc::string::String),
        /// Value of date time type annotation.
        #[prost(string, tag = "5")]
        DatetimeValue(::prost::alloc::string::String),
        /// Value of geo coordinate type annotation.
        #[prost(message, tag = "7")]
        GeoCoordinate(super::GeoCoordinate),
        /// Value of any proto value.
        #[prost(message, tag = "8")]
        ProtoAnyValue(::prost_types::Any),
        /// Value of boolean type annotation.
        #[prost(bool, tag = "9")]
        BoolValue(bool),
        /// Value of customized struct annotation.
        #[prost(message, tag = "10")]
        CustomizedStructDataValue(::prost_types::Struct),
    }
}
/// Request message for GetAnnotation API.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAnnotationsRequest {
    /// The parent, which owns this collection of annotations.
    /// Format:
    /// projects/{project_number}/locations/{location}/corpora/{corpus}/assets/{asset}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of annotations to return. The service may return fewer
    /// than this value. If unspecified, at most 50 annotations will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListAnnotations` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListAnnotations` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// The filter applied to the returned list.
    /// We only support filtering for the following fields:
    /// `partition.temporal_partition.start_time`,
    /// `partition.temporal_partition.end_time`, and `key`.
    /// Timestamps are specified in the RFC-3339 format, and only one restriction
    /// may be applied per field, joined by conjunctions.
    /// Format:
    /// "partition.temporal_partition.start_time > "2012-04-21T11:30:00-04:00" AND
    /// partition.temporal_partition.end_time < "2012-04-22T11:30:00-04:00" AND
    /// key = "example_key""
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Request message for ListAnnotations API.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAnnotationsResponse {
    /// The annotations from the specified asset.
    #[prost(message, repeated, tag = "1")]
    pub annotations: ::prost::alloc::vec::Vec<Annotation>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetAnnotation API.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAnnotationRequest {
    /// Required. The name of the annotation to retrieve.
    /// Format:
    /// projects/{project_number}/locations/{location}/corpora/{corpus}/assets/{asset}/annotations/{annotation}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateAnnotation API.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAnnotationRequest {
    /// Required. The annotation to update.
    /// The annotation's `name` field is used to identify the annotation to be
    /// updated. Format:
    /// projects/{project_number}/locations/{location}/corpora/{corpus}/assets/{asset}/annotations/{annotation}
    #[prost(message, optional, tag = "1")]
    pub annotation: ::core::option::Option<Annotation>,
    /// The list of fields to be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for DeleteAnnotation API.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAnnotationRequest {
    /// Required. The name of the annotation to delete.
    /// Format:
    /// projects/{project_number}/locations/{location}/corpora/{corpus}/assets/{asset}/annotations/{annotation}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for CreateSearchConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSearchConfigRequest {
    /// Required. The parent resource where this search configuration will be created.
    /// Format: projects/*/locations/*/corpora/*
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The search config to create.
    #[prost(message, optional, tag = "2")]
    pub search_config: ::core::option::Option<SearchConfig>,
    /// Required. ID to use for the new search config. Will become the final component of the
    /// SearchConfig's resource name. This value should be up to 63 characters, and
    /// valid characters are /\[a-z][0-9\]-_/. The first character must be a letter,
    /// the last could be a letter or a number.
    #[prost(string, tag = "3")]
    pub search_config_id: ::prost::alloc::string::String,
}
/// Request message for UpdateSearchConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSearchConfigRequest {
    /// Required. The search configuration to update.
    ///
    /// The search configuration's `name` field is used to identify the resource to
    /// be updated. Format:
    /// projects/{project_number}/locations/{location}/corpora/{corpus}/searchConfigs/{search_config}
    #[prost(message, optional, tag = "1")]
    pub search_config: ::core::option::Option<SearchConfig>,
    /// The list of fields to be updated. If left unset, all field paths will be
    /// updated/overwritten.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for GetSearchConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSearchConfigRequest {
    /// Required. The name of the search configuration to retrieve.
    /// Format:
    /// projects/{project_number}/locations/{location}/corpora/{corpus}/searchConfigs/{search_config}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for DeleteSearchConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSearchConfigRequest {
    /// Required. The name of the search configuration to delete.
    /// Format:
    /// projects/{project_number}/locations/{location}/corpora/{corpus}/searchConfigs/{search_config}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListSearchConfigs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSearchConfigsRequest {
    /// Required. The parent, which owns this collection of search configurations.
    /// Format:
    /// projects/{project_number}/locations/{location}/corpora/{corpus}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of search configurations to return. The service may
    /// return fewer than this value. If unspecified, a page size of 50 will be
    /// used. The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListSearchConfigs` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `ListSearchConfigs` must match the call that provided the page
    /// token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListSearchConfigs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSearchConfigsResponse {
    /// The search configurations from the specified corpus.
    #[prost(message, repeated, tag = "1")]
    pub search_configs: ::prost::alloc::vec::Vec<SearchConfig>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// SearchConfig stores different properties that will affect search
/// behaviors and search results.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchConfig {
    /// Resource name of the search configuration.
    /// For CustomSearchCriteria, search_config would be the search
    /// operator name. For Facets, search_config would be the facet
    /// dimension name.
    /// Form:
    /// `projects/{project_number}/locations/{location}/corpora/{corpus}/searchConfigs/{search_config}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Establishes a FacetDimension and associated specifications.
    #[prost(message, optional, tag = "2")]
    pub facet_property: ::core::option::Option<FacetProperty>,
    /// Creates a mapping between a custom SearchCriteria and one or more UGA keys.
    #[prost(message, optional, tag = "3")]
    pub search_criteria_property: ::core::option::Option<SearchCriteriaProperty>,
}
/// Central configuration for a facet.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FacetProperty {
    /// Name of the facets, which are the dimensions users want to use to refine
    /// search results. `mapped_fields` will match UserSpecifiedDataSchema keys.
    ///
    /// For example, user can add a bunch of UGAs with the same key, such as
    /// player:adam, player:bob, player:charles. When multiple mapped_fields are
    /// specified, will merge their value together as final facet value. E.g.
    /// home_team: a, home_team:b, away_team:a, away_team:c, when facet_field =
    /// [home_team, away_team], facet_value will be [a, b, c].
    ///
    /// UNLESS this is a 1:1 facet dimension (mapped_fields.size() == 1) AND the
    /// mapped_field equals the parent SearchConfig.name, the parent must
    /// also contain a SearchCriteriaProperty that maps to the same fields.
    /// mapped_fields must not be empty.
    #[prost(string, repeated, tag = "1")]
    pub mapped_fields: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Display name of the facet. To be used by UI for facet rendering.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Maximum number of unique bucket to return for one facet. Bucket number can
    /// be large for high-cardinality facet such as "player". We only return top-n
    /// most related ones to user. If it's <= 0, the server will decide the
    /// appropriate result_size.
    #[prost(int64, tag = "3")]
    pub result_size: i64,
    /// Facet bucket type e.g. value, range.
    #[prost(enumeration = "FacetBucketType", tag = "4")]
    pub bucket_type: i32,
    #[prost(oneof = "facet_property::RangeFacetConfig", tags = "5, 6, 7")]
    pub range_facet_config: ::core::option::Option<facet_property::RangeFacetConfig>,
}
/// Nested message and enum types in `FacetProperty`.
pub mod facet_property {
    /// If bucket type is FIXED_RANGE, specify how values are bucketized. Use
    /// FixedRangeBucketSpec when you want to create multiple buckets with equal
    /// granularities. Using integer bucket value as an example, when
    /// bucket_start = 0, bucket_granularity = 10, bucket_count = 5, this facet
    /// will be aggregated via the following buckets:
    /// [-inf, 0), [0, 10), [10, 20), [20, 30), [30, inf).
    /// Notably, bucket_count <= 1 is an invalid spec.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FixedRangeBucketSpec {
        /// Lower bound of the bucket. NOTE: Only integer type is currently supported
        /// for this field.
        #[prost(message, optional, tag = "1")]
        pub bucket_start: ::core::option::Option<super::FacetValue>,
        /// Bucket granularity. NOTE: Only integer type is currently supported for
        /// this field.
        #[prost(message, optional, tag = "2")]
        pub bucket_granularity: ::core::option::Option<super::FacetValue>,
        /// Total number of buckets.
        #[prost(int32, tag = "3")]
        pub bucket_count: i32,
    }
    /// If bucket type is CUSTOM_RANGE, specify how values are bucketized. Use
    /// integer bucket value as an example, when the endpoints are 0, 10, 100, and
    /// 1000, we will generate the following facets:
    /// [-inf, 0), [0, 10), [10, 100), [100, 1000), [1000, inf).
    /// Notably:
    /// - endpoints must be listed in ascending order. Otherwise, the SearchConfig
    ///    API will reject the facet config.
    /// - < 1 endpoints is an invalid spec.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CustomRangeBucketSpec {
        /// Currently, only integer type is supported for this field.
        #[prost(message, repeated, tag = "1")]
        pub endpoints: ::prost::alloc::vec::Vec<super::FacetValue>,
    }
    /// If bucket type is DATE, specify how date values are bucketized.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DateTimeBucketSpec {
        /// Granularity of date type facet.
        #[prost(enumeration = "date_time_bucket_spec::Granularity", tag = "1")]
        pub granularity: i32,
    }
    /// Nested message and enum types in `DateTimeBucketSpec`.
    pub mod date_time_bucket_spec {
        /// Granularity enum for the datetime bucket.
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum Granularity {
            /// Unspecified granularity.
            Unspecified = 0,
            /// Granularity is year.
            Year = 1,
            /// Granularity is month.
            Month = 2,
            /// Granularity is day.
            Day = 3,
        }
        impl Granularity {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Granularity::Unspecified => "GRANULARITY_UNSPECIFIED",
                    Granularity::Year => "YEAR",
                    Granularity::Month => "MONTH",
                    Granularity::Day => "DAY",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "GRANULARITY_UNSPECIFIED" => Some(Self::Unspecified),
                    "YEAR" => Some(Self::Year),
                    "MONTH" => Some(Self::Month),
                    "DAY" => Some(Self::Day),
                    _ => None,
                }
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RangeFacetConfig {
        /// Fixed range facet bucket config.
        #[prost(message, tag = "5")]
        FixedRangeBucketSpec(FixedRangeBucketSpec),
        /// Custom range facet bucket config.
        #[prost(message, tag = "6")]
        CustomRangeBucketSpec(CustomRangeBucketSpec),
        /// Datetime range facet bucket config.
        #[prost(message, tag = "7")]
        DatetimeBucketSpec(DateTimeBucketSpec),
    }
}
/// Central configuration for custom search criteria.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchCriteriaProperty {
    /// Each mapped_field corresponds to a UGA key. To understand how this property
    /// works, take the following example. In the SearchConfig table, the
    /// user adds this entry:
    ///    search_config {
    ///      name: "person"
    ///      search_criteria_property {
    ///        mapped_fields: "player"
    ///        mapped_fields: "coach"
    ///      }
    ///    }
    ///
    /// Now, when a user issues a query like:
    ///    criteria {
    ///      field: "person"
    ///      text_array {
    ///        txt_values: "Tom Brady"
    ///        txt_values: "Bill Belichick"
    ///      }
    ///    }
    ///
    /// MWH search will return search documents where (player=Tom Brady ||
    /// coach=Tom Brady || player=Bill Belichick || coach=Bill Belichick).
    #[prost(string, repeated, tag = "1")]
    pub mapped_fields: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Definition of a single value with generic type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FacetValue {
    #[prost(oneof = "facet_value::Value", tags = "1, 2, 3")]
    pub value: ::core::option::Option<facet_value::Value>,
}
/// Nested message and enum types in `FacetValue`.
pub mod facet_value {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// String type value.
        #[prost(string, tag = "1")]
        StringValue(::prost::alloc::string::String),
        /// Integer type value.
        #[prost(int64, tag = "2")]
        IntegerValue(i64),
        /// Datetime type value.
        #[prost(message, tag = "3")]
        DatetimeValue(super::super::super::super::r#type::DateTime),
    }
}
/// Holds the facet value, selections state, and metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FacetBucket {
    /// Whether one facet bucket is selected. This field represents user's facet
    /// selection. It is set by frontend in SearchVideosRequest.
    #[prost(bool, tag = "3")]
    pub selected: bool,
    /// Bucket associated with a facet. For example, bucket of facet “team”
    /// can be "49ers", "patriots", etc; bucket of facet "player" can be "tom
    /// brady", "drew brees", etc.
    #[prost(oneof = "facet_bucket::BucketValue", tags = "2, 4")]
    pub bucket_value: ::core::option::Option<facet_bucket::BucketValue>,
}
/// Nested message and enum types in `FacetBucket`.
pub mod facet_bucket {
    /// The range of values [start, end) for which faceting is applied.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Range {
        /// Start of the range. Non-existence indicates some bound (e.g. -inf).
        #[prost(message, optional, tag = "1")]
        pub start: ::core::option::Option<super::FacetValue>,
        /// End of the range. Non-existence indicates some bound (e.g. inf).
        #[prost(message, optional, tag = "2")]
        pub end: ::core::option::Option<super::FacetValue>,
    }
    /// Bucket associated with a facet. For example, bucket of facet “team”
    /// can be "49ers", "patriots", etc; bucket of facet "player" can be "tom
    /// brady", "drew brees", etc.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum BucketValue {
        /// Singular value.
        #[prost(message, tag = "2")]
        Value(super::FacetValue),
        /// Range value.
        #[prost(message, tag = "4")]
        Range(Range),
    }
}
/// A group of facet buckets to be passed back and forth between backend &
/// frontend.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FacetGroup {
    /// Unique id of the facet group.
    #[prost(string, tag = "1")]
    pub facet_id: ::prost::alloc::string::String,
    /// Display name of the facet. To be used by UI for facet rendering.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Buckets associated with the facet. E.g. for "Team" facet, the bucket
    /// can be 49ers, patriots, etc.
    #[prost(message, repeated, tag = "3")]
    pub buckets: ::prost::alloc::vec::Vec<FacetBucket>,
    /// Facet bucket type.
    #[prost(enumeration = "FacetBucketType", tag = "4")]
    pub bucket_type: i32,
    /// If true, return query matched annotations for this facet group's selection.
    /// This option is only applicable for facets based on partition level
    /// annotations. It supports the following facet values:
    ///   - INTEGER
    ///   - STRING (DataSchema.SearchStrategy.EXACT_SEARCH only)
    #[prost(bool, tag = "5")]
    pub fetch_matched_annotations: bool,
}
/// Request message for IngestAsset API.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngestAssetRequest {
    #[prost(oneof = "ingest_asset_request::StreamingRequest", tags = "1, 2")]
    pub streaming_request: ::core::option::Option<
        ingest_asset_request::StreamingRequest,
    >,
}
/// Nested message and enum types in `IngestAssetRequest`.
pub mod ingest_asset_request {
    /// Configuration for the data.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Config {
        /// Required. The resource name of the asset that the ingested data belongs to.
        #[prost(string, tag = "1")]
        pub asset: ::prost::alloc::string::String,
        #[prost(oneof = "config::DataType", tags = "2")]
        pub data_type: ::core::option::Option<config::DataType>,
    }
    /// Nested message and enum types in `Config`.
    pub mod config {
        /// Type information for video data.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct VideoType {
            /// Container format of the video data.
            #[prost(enumeration = "video_type::ContainerFormat", tag = "1")]
            pub container_format: i32,
        }
        /// Nested message and enum types in `VideoType`.
        pub mod video_type {
            /// Container format of the video.
            #[derive(
                Clone,
                Copy,
                Debug,
                PartialEq,
                Eq,
                Hash,
                PartialOrd,
                Ord,
                ::prost::Enumeration
            )]
            #[repr(i32)]
            pub enum ContainerFormat {
                /// The default type, not supposed to be used.
                Unspecified = 0,
                /// Mp4 container format.
                Mp4 = 1,
            }
            impl ContainerFormat {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        ContainerFormat::Unspecified => "CONTAINER_FORMAT_UNSPECIFIED",
                        ContainerFormat::Mp4 => "CONTAINER_FORMAT_MP4",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "CONTAINER_FORMAT_UNSPECIFIED" => Some(Self::Unspecified),
                        "CONTAINER_FORMAT_MP4" => Some(Self::Mp4),
                        _ => None,
                    }
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum DataType {
            /// Type information for video data.
            #[prost(message, tag = "2")]
            VideoType(VideoType),
        }
    }
    /// Contains the data and the corresponding time range this data is for.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TimeIndexedData {
        /// Data to be ingested.
        #[prost(bytes = "bytes", tag = "1")]
        pub data: ::prost::bytes::Bytes,
        /// Time range of the data.
        #[prost(message, optional, tag = "2")]
        pub temporal_partition: ::core::option::Option<
            super::partition::TemporalPartition,
        >,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StreamingRequest {
        /// Provides information for the data and the asset resource name that the
        /// data belongs to. The first `IngestAssetRequest` message must only contain
        /// a `Config` message.
        #[prost(message, tag = "1")]
        Config(Config),
        /// Data to be ingested.
        #[prost(message, tag = "2")]
        TimeIndexedData(TimeIndexedData),
    }
}
/// Response message for IngestAsset API.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngestAssetResponse {
    /// Time range of the data that has been successfully ingested.
    #[prost(message, optional, tag = "1")]
    pub successfully_ingested_partition: ::core::option::Option<
        partition::TemporalPartition,
    >,
}
/// Request message for ClipAsset API.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClipAssetRequest {
    /// Required. The resource name of the asset to request clips for.
    /// Form:
    /// 'projects/{project_number}/locations/{location_id}/corpora/{corpus_id}/assets/{asset_id}'
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The time range to request clips for.
    #[prost(message, optional, tag = "2")]
    pub temporal_partition: ::core::option::Option<partition::TemporalPartition>,
}
/// Response message for ClipAsset API.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClipAssetResponse {
    /// A list of signed uris to download the video clips that cover the requested
    /// time range ordered by time.
    #[prost(message, repeated, tag = "1")]
    pub time_indexed_uris: ::prost::alloc::vec::Vec<clip_asset_response::TimeIndexedUri>,
}
/// Nested message and enum types in `ClipAssetResponse`.
pub mod clip_asset_response {
    /// Signed uri with corresponding time range.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TimeIndexedUri {
        /// Time range of the video that the uri is for.
        #[prost(message, optional, tag = "1")]
        pub temporal_partition: ::core::option::Option<
            super::partition::TemporalPartition,
        >,
        /// Signed uri to download the video clip.
        #[prost(string, tag = "2")]
        pub uri: ::prost::alloc::string::String,
    }
}
/// Request message for GenerateHlsUri API.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateHlsUriRequest {
    /// Required. The resource name of the asset to request clips for.
    /// Form:
    /// 'projects/{project_number}/locations/{location_id}/corpora/{corpus_id}/assets/{asset_id}'
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The time range to request clips for.
    #[prost(message, repeated, tag = "2")]
    pub temporal_partitions: ::prost::alloc::vec::Vec<partition::TemporalPartition>,
}
/// Response message for GenerateHlsUri API.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateHlsUriResponse {
    /// A signed uri to download the HLS manifest corresponding to the requested
    /// times.
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    /// A list of temporal partitions of the content returned in the order they
    /// appear in the stream.
    #[prost(message, repeated, tag = "2")]
    pub temporal_partitions: ::prost::alloc::vec::Vec<partition::TemporalPartition>,
}
/// Request message for SearchAssets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAssetsRequest {
    /// Required. The parent corpus to search.
    /// Form: `projects/{project_id}/locations/{location_id}/corpora/{corpus_id}'
    #[prost(string, tag = "1")]
    pub corpus: ::prost::alloc::string::String,
    /// The number of results to be returned in this page. If it's 0, the server
    /// will decide the appropriate page_size.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The continuation token to fetch the next page. If empty, it means it is
    /// fetching the first page.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Time ranges that matching video content must fall within. If no ranges are
    /// provided, there will be no time restriction. This field is treated just
    /// like the criteria below, but defined separately for convenience as it is
    /// used frequently. Note that if the end_time is in the future, it will be
    /// clamped to the time the request was received.
    #[prost(message, optional, tag = "5")]
    pub content_time_ranges: ::core::option::Option<DateTimeRangeArray>,
    /// Criteria applied to search results.
    #[prost(message, repeated, tag = "4")]
    pub criteria: ::prost::alloc::vec::Vec<Criteria>,
    /// Stores most recent facet selection state. Only facet groups with user's
    /// selection will be presented here. Selection state is either selected or
    /// unselected. Only selected facet buckets will be used as search criteria.
    #[prost(message, repeated, tag = "6")]
    pub facet_selections: ::prost::alloc::vec::Vec<FacetGroup>,
    /// A list of annotation keys to specify the annotations to be retrieved and
    /// returned with each search result.
    /// Annotation granularity must be GRANULARITY_ASSET_LEVEL and its search
    /// strategy must not be NO_SEARCH.
    #[prost(string, repeated, tag = "8")]
    pub result_annotation_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The metadata for DeleteAsset API that embeds in
/// \[metadata][google.longrunning.Operation.metadata\] field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAssetMetadata {}
/// Stores the criteria-annotation matching results for each search result item.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotationMatchingResult {
    /// The criteria used for matching. It can be an input search criteria or a
    /// criteria converted from a facet selection.
    #[prost(message, optional, tag = "1")]
    pub criteria: ::core::option::Option<Criteria>,
    /// Matched annotations for the criteria.
    #[prost(message, repeated, tag = "2")]
    pub matched_annotations: ::prost::alloc::vec::Vec<Annotation>,
    /// Status of the match result. Possible values:
    /// FAILED_PRECONDITION - the criteria is not eligible for match.
    /// OK - matching is performed.
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<super::super::super::rpc::Status>,
}
/// Search result contains asset name and corresponding time ranges.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchResultItem {
    /// The resource name of the asset.
    /// Form:
    /// 'projects/{project_number}/locations/{location_id}/corpora/{corpus_id}/assets/{asset_id}'
    #[prost(string, tag = "1")]
    pub asset: ::prost::alloc::string::String,
    /// The matched asset segments.
    /// Deprecated: please use singular `segment` field.
    #[deprecated]
    #[prost(message, repeated, tag = "2")]
    pub segments: ::prost::alloc::vec::Vec<partition::TemporalPartition>,
    /// The matched asset segment.
    #[prost(message, optional, tag = "5")]
    pub segment: ::core::option::Option<partition::TemporalPartition>,
    /// Search result annotations specified by result_annotation_keys in search
    /// request.
    #[prost(message, repeated, tag = "3")]
    pub requested_annotations: ::prost::alloc::vec::Vec<Annotation>,
    /// Criteria or facet-selection based annotation matching results associated to
    /// this search result item. Only contains results for criteria or
    /// facet_selections with fetch_matched_annotations=true.
    #[prost(message, repeated, tag = "4")]
    pub annotation_matching_results: ::prost::alloc::vec::Vec<AnnotationMatchingResult>,
}
/// Response message for SearchAssets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAssetsResponse {
    /// Returned search results.
    #[prost(message, repeated, tag = "1")]
    pub search_result_items: ::prost::alloc::vec::Vec<SearchResultItem>,
    /// The next-page continuation token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Facet search results of a given query, which contains user's
    /// already-selected facet values and updated facet search results.
    #[prost(message, repeated, tag = "3")]
    pub facet_results: ::prost::alloc::vec::Vec<FacetGroup>,
}
/// Integer range type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntRange {
    /// Start of the int range.
    #[prost(int64, optional, tag = "1")]
    pub start: ::core::option::Option<i64>,
    /// End of the int range.
    #[prost(int64, optional, tag = "2")]
    pub end: ::core::option::Option<i64>,
}
/// Float range type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloatRange {
    /// Start of the float range.
    #[prost(float, optional, tag = "1")]
    pub start: ::core::option::Option<f32>,
    /// End of the float range.
    #[prost(float, optional, tag = "2")]
    pub end: ::core::option::Option<f32>,
}
/// A list of string-type values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringArray {
    /// String type values.
    #[prost(string, repeated, tag = "1")]
    pub txt_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A list of integer range values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntRangeArray {
    /// Int range values.
    #[prost(message, repeated, tag = "1")]
    pub int_ranges: ::prost::alloc::vec::Vec<IntRange>,
}
/// A list of float range values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloatRangeArray {
    /// Float range values.
    #[prost(message, repeated, tag = "1")]
    pub float_ranges: ::prost::alloc::vec::Vec<FloatRange>,
}
/// Datetime range type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateTimeRange {
    /// Start date time.
    #[prost(message, optional, tag = "1")]
    pub start: ::core::option::Option<super::super::super::r#type::DateTime>,
    /// End data time.
    #[prost(message, optional, tag = "2")]
    pub end: ::core::option::Option<super::super::super::r#type::DateTime>,
}
/// A list of datetime range values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateTimeRangeArray {
    /// Date time ranges.
    #[prost(message, repeated, tag = "1")]
    pub date_time_ranges: ::prost::alloc::vec::Vec<DateTimeRange>,
}
/// Representation of a circle area.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CircleArea {
    /// Latitude of circle area's center. Degrees [-90 .. 90]
    #[prost(double, tag = "1")]
    pub latitude: f64,
    /// Longitude of circle area's center. Degrees [-180 .. 180]
    #[prost(double, tag = "2")]
    pub longitude: f64,
    /// Radius of the circle area in meters.
    #[prost(double, tag = "3")]
    pub radius_meter: f64,
}
/// A list of locations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoLocationArray {
    /// A list of circle areas.
    #[prost(message, repeated, tag = "1")]
    pub circle_areas: ::prost::alloc::vec::Vec<CircleArea>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoolValue {
    #[prost(bool, tag = "1")]
    pub value: bool,
}
/// Filter criteria applied to current search results.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Criteria {
    /// The UGA field or ML field to apply filtering criteria.
    #[prost(string, tag = "1")]
    pub field: ::prost::alloc::string::String,
    /// If true, return query matched annotations for this criteria.
    /// This option is only applicable for partition level annotations and supports
    /// the following data types:
    ///   - INTEGER
    ///   - FLOAT
    ///   - STRING (DataSchema.SearchStrategy.EXACT_SEARCH only)
    ///   - BOOLEAN
    #[prost(bool, tag = "8")]
    pub fetch_matched_annotations: bool,
    #[prost(oneof = "criteria::Value", tags = "2, 3, 4, 5, 6, 7")]
    pub value: ::core::option::Option<criteria::Value>,
}
/// Nested message and enum types in `Criteria`.
pub mod criteria {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// The text values associated with the field.
        #[prost(message, tag = "2")]
        TextArray(super::StringArray),
        /// The integer ranges associated with the field.
        #[prost(message, tag = "3")]
        IntRangeArray(super::IntRangeArray),
        /// The float ranges associated with the field.
        #[prost(message, tag = "4")]
        FloatRangeArray(super::FloatRangeArray),
        /// The datetime ranges associated with the field.
        #[prost(message, tag = "5")]
        DateTimeRangeArray(super::DateTimeRangeArray),
        /// Geo Location array.
        #[prost(message, tag = "6")]
        GeoLocationArray(super::GeoLocationArray),
        /// A Boolean value.
        #[prost(message, tag = "7")]
        BoolValue(super::BoolValue),
    }
}
/// Partition to specify the partition in time and space for sub-asset level
/// annotation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Partition {
    /// Partition of asset in time.
    #[prost(message, optional, tag = "1")]
    pub temporal_partition: ::core::option::Option<partition::TemporalPartition>,
    /// Partition of asset in space.
    #[prost(message, optional, tag = "2")]
    pub spatial_partition: ::core::option::Option<partition::SpatialPartition>,
}
/// Nested message and enum types in `Partition`.
pub mod partition {
    /// Partition of asset in UTC Epoch time.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TemporalPartition {
        /// Start time of the partition.
        #[prost(message, optional, tag = "1")]
        pub start_time: ::core::option::Option<::prost_types::Timestamp>,
        /// End time of the partition.
        #[prost(message, optional, tag = "2")]
        pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    }
    /// Partition of asset in space.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SpatialPartition {
        /// The minimum x coordinate value.
        #[prost(int64, optional, tag = "1")]
        pub x_min: ::core::option::Option<i64>,
        /// The minimum y coordinate value.
        #[prost(int64, optional, tag = "2")]
        pub y_min: ::core::option::Option<i64>,
        /// The maximum x coordinate value.
        #[prost(int64, optional, tag = "3")]
        pub x_max: ::core::option::Option<i64>,
        /// The maximum y coordinate value.
        #[prost(int64, optional, tag = "4")]
        pub y_max: ::core::option::Option<i64>,
    }
}
/// Different types for a facet bucket.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FacetBucketType {
    /// Unspecified type.
    Unspecified = 0,
    /// Value type.
    Value = 1,
    /// Datetime type.
    Datetime = 2,
    /// Fixed Range type.
    FixedRange = 3,
    /// Custom Range type.
    CustomRange = 4,
}
impl FacetBucketType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FacetBucketType::Unspecified => "FACET_BUCKET_TYPE_UNSPECIFIED",
            FacetBucketType::Value => "FACET_BUCKET_TYPE_VALUE",
            FacetBucketType::Datetime => "FACET_BUCKET_TYPE_DATETIME",
            FacetBucketType::FixedRange => "FACET_BUCKET_TYPE_FIXED_RANGE",
            FacetBucketType::CustomRange => "FACET_BUCKET_TYPE_CUSTOM_RANGE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FACET_BUCKET_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "FACET_BUCKET_TYPE_VALUE" => Some(Self::Value),
            "FACET_BUCKET_TYPE_DATETIME" => Some(Self::Datetime),
            "FACET_BUCKET_TYPE_FIXED_RANGE" => Some(Self::FixedRange),
            "FACET_BUCKET_TYPE_CUSTOM_RANGE" => Some(Self::CustomRange),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod warehouse_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service that manages media content + metadata for streaming.
    #[derive(Debug, Clone)]
    pub struct WarehouseClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> WarehouseClient<T>
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
        ) -> WarehouseClient<InterceptedService<T, F>>
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
            WarehouseClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates an asset inside corpus.
        pub async fn create_asset(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAssetRequest>,
        ) -> Result<tonic::Response<super::Asset>, tonic::Status> {
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
                "/google.cloud.visionai.v1.Warehouse/CreateAsset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an asset inside corpus.
        pub async fn update_asset(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAssetRequest>,
        ) -> Result<tonic::Response<super::Asset>, tonic::Status> {
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
                "/google.cloud.visionai.v1.Warehouse/UpdateAsset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Reads an asset inside corpus.
        pub async fn get_asset(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAssetRequest>,
        ) -> Result<tonic::Response<super::Asset>, tonic::Status> {
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
                "/google.cloud.visionai.v1.Warehouse/GetAsset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists an list of assets inside corpus.
        pub async fn list_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAssetsRequest>,
        ) -> Result<tonic::Response<super::ListAssetsResponse>, tonic::Status> {
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
                "/google.cloud.visionai.v1.Warehouse/ListAssets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes asset inside corpus.
        pub async fn delete_asset(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAssetRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.Warehouse/DeleteAsset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a corpus inside a project.
        pub async fn create_corpus(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCorpusRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.Warehouse/CreateCorpus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets corpus details inside a project.
        pub async fn get_corpus(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCorpusRequest>,
        ) -> Result<tonic::Response<super::Corpus>, tonic::Status> {
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
                "/google.cloud.visionai.v1.Warehouse/GetCorpus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a corpus in a project.
        pub async fn update_corpus(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCorpusRequest>,
        ) -> Result<tonic::Response<super::Corpus>, tonic::Status> {
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
                "/google.cloud.visionai.v1.Warehouse/UpdateCorpus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all corpora in a project.
        pub async fn list_corpora(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCorporaRequest>,
        ) -> Result<tonic::Response<super::ListCorporaResponse>, tonic::Status> {
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
                "/google.cloud.visionai.v1.Warehouse/ListCorpora",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a corpus only if its empty.
        /// Returns empty response.
        pub async fn delete_corpus(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCorpusRequest>,
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
                "/google.cloud.visionai.v1.Warehouse/DeleteCorpus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates data schema inside corpus.
        pub async fn create_data_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDataSchemaRequest>,
        ) -> Result<tonic::Response<super::DataSchema>, tonic::Status> {
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
                "/google.cloud.visionai.v1.Warehouse/CreateDataSchema",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates data schema inside corpus.
        pub async fn update_data_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDataSchemaRequest>,
        ) -> Result<tonic::Response<super::DataSchema>, tonic::Status> {
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
                "/google.cloud.visionai.v1.Warehouse/UpdateDataSchema",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets data schema inside corpus.
        pub async fn get_data_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDataSchemaRequest>,
        ) -> Result<tonic::Response<super::DataSchema>, tonic::Status> {
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
                "/google.cloud.visionai.v1.Warehouse/GetDataSchema",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes data schema inside corpus.
        pub async fn delete_data_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDataSchemaRequest>,
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
                "/google.cloud.visionai.v1.Warehouse/DeleteDataSchema",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists a list of data schemas inside corpus.
        pub async fn list_data_schemas(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDataSchemasRequest>,
        ) -> Result<tonic::Response<super::ListDataSchemasResponse>, tonic::Status> {
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
                "/google.cloud.visionai.v1.Warehouse/ListDataSchemas",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates annotation inside asset.
        pub async fn create_annotation(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAnnotationRequest>,
        ) -> Result<tonic::Response<super::Annotation>, tonic::Status> {
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
                "/google.cloud.visionai.v1.Warehouse/CreateAnnotation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Reads annotation inside asset.
        pub async fn get_annotation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAnnotationRequest>,
        ) -> Result<tonic::Response<super::Annotation>, tonic::Status> {
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
                "/google.cloud.visionai.v1.Warehouse/GetAnnotation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists a list of annotations inside asset.
        pub async fn list_annotations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAnnotationsRequest>,
        ) -> Result<tonic::Response<super::ListAnnotationsResponse>, tonic::Status> {
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
                "/google.cloud.visionai.v1.Warehouse/ListAnnotations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates annotation inside asset.
        pub async fn update_annotation(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAnnotationRequest>,
        ) -> Result<tonic::Response<super::Annotation>, tonic::Status> {
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
                "/google.cloud.visionai.v1.Warehouse/UpdateAnnotation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes annotation inside asset.
        pub async fn delete_annotation(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAnnotationRequest>,
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
                "/google.cloud.visionai.v1.Warehouse/DeleteAnnotation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Ingests data for the asset. It is not allowed to ingest a data chunk which
        /// is already expired according to TTL.
        /// This method is only available via the gRPC API (not HTTP since
        /// bi-directional streaming is not supported via HTTP).
        pub async fn ingest_asset(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::IngestAssetRequest,
            >,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::IngestAssetResponse>>,
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
                "/google.cloud.visionai.v1.Warehouse/IngestAsset",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        /// Generates clips for downloading. The api takes in a time range, and
        /// generates a clip of the first content available after start_time and
        /// before end_time, which may overflow beyond these bounds.
        /// Returned clips are truncated if the total size of the clips are larger
        /// than 100MB.
        pub async fn clip_asset(
            &mut self,
            request: impl tonic::IntoRequest<super::ClipAssetRequest>,
        ) -> Result<tonic::Response<super::ClipAssetResponse>, tonic::Status> {
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
                "/google.cloud.visionai.v1.Warehouse/ClipAsset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Generates a uri for an HLS manifest. The api takes in a collection of time
        /// ranges, and generates a URI for an HLS manifest that covers all the
        /// requested time ranges.
        pub async fn generate_hls_uri(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateHlsUriRequest>,
        ) -> Result<tonic::Response<super::GenerateHlsUriResponse>, tonic::Status> {
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
                "/google.cloud.visionai.v1.Warehouse/GenerateHlsUri",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a search configuration inside a corpus.
        ///
        /// Please follow the rules below to create a valid CreateSearchConfigRequest.
        /// --- General Rules ---
        /// 1. Request.search_config_id must not be associated with an existing
        ///    SearchConfig.
        /// 2. Request must contain at least one non-empty search_criteria_property or
        ///    facet_property.
        /// 3. mapped_fields must not be empty, and must map to existing UGA keys.
        /// 4. All mapped_fields must be of the same type.
        /// 5. All mapped_fields must share the same granularity.
        /// 6. All mapped_fields must share the same semantic SearchConfig match
        ///    options.
        /// For property-specific rules, please reference the comments for
        /// FacetProperty and SearchCriteriaProperty.
        pub async fn create_search_config(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSearchConfigRequest>,
        ) -> Result<tonic::Response<super::SearchConfig>, tonic::Status> {
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
                "/google.cloud.visionai.v1.Warehouse/CreateSearchConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a search configuration inside a corpus.
        ///
        /// Please follow the rules below to create a valid UpdateSearchConfigRequest.
        /// --- General Rules ---
        /// 1. Request.search_configuration.name must already exist.
        /// 2. Request must contain at least one non-empty search_criteria_property or
        /// facet_property.
        /// 3. mapped_fields must not be empty, and must map to existing UGA keys.
        /// 4. All mapped_fields must be of the same type.
        /// 5. All mapped_fields must share the same granularity.
        /// 6. All mapped_fields must share the same semantic SearchConfig match
        ///    options.
        /// For property-specific rules, please reference the comments for
        /// FacetProperty and SearchCriteriaProperty.
        pub async fn update_search_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSearchConfigRequest>,
        ) -> Result<tonic::Response<super::SearchConfig>, tonic::Status> {
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
                "/google.cloud.visionai.v1.Warehouse/UpdateSearchConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a search configuration inside a corpus.
        pub async fn get_search_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSearchConfigRequest>,
        ) -> Result<tonic::Response<super::SearchConfig>, tonic::Status> {
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
                "/google.cloud.visionai.v1.Warehouse/GetSearchConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a search configuration inside a corpus.
        ///
        /// For a DeleteSearchConfigRequest to be valid,
        /// Request.search_configuration.name must already exist.
        pub async fn delete_search_config(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSearchConfigRequest>,
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
                "/google.cloud.visionai.v1.Warehouse/DeleteSearchConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all search configurations inside a corpus.
        pub async fn list_search_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSearchConfigsRequest>,
        ) -> Result<tonic::Response<super::ListSearchConfigsResponse>, tonic::Status> {
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
                "/google.cloud.visionai.v1.Warehouse/ListSearchConfigs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Search media asset.
        pub async fn search_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchAssetsRequest>,
        ) -> Result<tonic::Response<super::SearchAssetsResponse>, tonic::Status> {
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
                "/google.cloud.visionai.v1.Warehouse/SearchAssets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Message for requesting list of Analyses
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAnalysesRequest {
    /// Required. Parent value for ListAnalysesRequest
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Hint for how to order the results
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Message for response to listing Analyses
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAnalysesResponse {
    /// The list of Analysis
    #[prost(message, repeated, tag = "1")]
    pub analyses: ::prost::alloc::vec::Vec<Analysis>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting an Analysis.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAnalysisRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for creating an Analysis.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAnalysisRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Id of the requesting object.
    #[prost(string, tag = "2")]
    pub analysis_id: ::prost::alloc::string::String,
    /// Required. The resource being created.
    #[prost(message, optional, tag = "3")]
    pub analysis: ::core::option::Option<Analysis>,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for updating an Analysis.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAnalysisRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// Analysis resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The resource being updated.
    #[prost(message, optional, tag = "2")]
    pub analysis: ::core::option::Option<Analysis>,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for deleting an Analysis.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAnalysisRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod live_video_analytics_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service describing handlers for resources. The service enables clients to run
    /// Live Video Analytics (LVA) on the streaming inputs.
    #[derive(Debug, Clone)]
    pub struct LiveVideoAnalyticsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> LiveVideoAnalyticsClient<T>
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
        ) -> LiveVideoAnalyticsClient<InterceptedService<T, F>>
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
            LiveVideoAnalyticsClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists Analyses in a given project and location.
        pub async fn list_analyses(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAnalysesRequest>,
        ) -> Result<tonic::Response<super::ListAnalysesResponse>, tonic::Status> {
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
                "/google.cloud.visionai.v1.LiveVideoAnalytics/ListAnalyses",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single Analysis.
        pub async fn get_analysis(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAnalysisRequest>,
        ) -> Result<tonic::Response<super::Analysis>, tonic::Status> {
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
                "/google.cloud.visionai.v1.LiveVideoAnalytics/GetAnalysis",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new Analysis in a given project and location.
        pub async fn create_analysis(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAnalysisRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.LiveVideoAnalytics/CreateAnalysis",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the parameters of a single Analysis.
        pub async fn update_analysis(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAnalysisRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.LiveVideoAnalytics/UpdateAnalysis",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single Analysis.
        pub async fn delete_analysis(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAnalysisRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.visionai.v1.LiveVideoAnalytics/DeleteAnalysis",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
