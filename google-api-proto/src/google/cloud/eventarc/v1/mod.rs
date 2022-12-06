/// A representation of the Provider resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Provider {
    /// Output only. In `projects/{project}/locations/{location}/providers/{provider_id}`
    /// format.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Human friendly name for the Provider. For example "Cloud Storage".
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. Event types for this provider.
    #[prost(message, repeated, tag = "3")]
    pub event_types: ::prost::alloc::vec::Vec<EventType>,
}
/// A representation of the event type resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventType {
    /// Output only. The full name of the event type (for example,
    /// "google.cloud.storage.object.v1.finalized"). In the form of
    /// {provider-specific-prefix}.{resource}.{version}.{verb}. Types MUST be
    /// versioned and event schemas are guaranteed to remain backward compatible
    /// within one version. Note that event type versions and API versions do not
    /// need to match.
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// Output only. Human friendly description of what the event type is about.
    /// For example "Bucket created in Cloud Storage".
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Filtering attributes for the event type.
    #[prost(message, repeated, tag = "3")]
    pub filtering_attributes: ::prost::alloc::vec::Vec<FilteringAttribute>,
    /// Output only. URI for the event schema.
    /// For example
    /// "<https://github.com/googleapis/google-cloudevents/blob/master/proto/google/events/cloud/storage/v1/events.proto">
    #[prost(string, tag = "4")]
    pub event_schema_uri: ::prost::alloc::string::String,
}
/// A representation of the FilteringAttribute resource.
/// Filtering attributes are per event type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilteringAttribute {
    /// Output only. Attribute used for filtering the event type.
    #[prost(string, tag = "1")]
    pub attribute: ::prost::alloc::string::String,
    /// Output only. Description of the purpose of the attribute.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Output only. If true, the triggers for this provider should always specify a filter
    /// on these attributes. Trigger creation will fail otherwise.
    #[prost(bool, tag = "3")]
    pub required: bool,
    /// Output only. If true, the attribute accepts matching expressions in the Eventarc
    /// PathPattern format.
    #[prost(bool, tag = "4")]
    pub path_pattern_supported: bool,
}
/// A representation of the Channel resource.
/// A Channel is a resource on which event providers publish their events.
/// The published events are delivered through the transport associated with the
/// channel. Note that a channel is associated with exactly one event provider.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Channel {
    /// Required. The resource name of the channel. Must be unique within the
    /// location on the project and must be in
    /// `projects/{project}/locations/{location}/channels/{channel_id}` format.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Server assigned unique identifier for the channel. The value
    /// is a UUID4 string and guaranteed to remain unchanged until the resource is
    /// deleted.
    #[prost(string, tag = "2")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. The creation time.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last-modified time.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The name of the event provider (e.g. Eventarc SaaS partner) associated
    /// with the channel. This provider will be granted permissions to publish
    /// events to the channel. Format:
    /// `projects/{project}/locations/{location}/providers/{provider_id}`.
    #[prost(string, tag = "7")]
    pub provider: ::prost::alloc::string::String,
    /// Output only. The state of a Channel.
    #[prost(enumeration = "channel::State", tag = "9")]
    pub state: i32,
    /// Output only. The activation token for the channel. The token must be used
    /// by the provider to register the channel for publishing.
    #[prost(string, tag = "10")]
    pub activation_token: ::prost::alloc::string::String,
    /// Optional. Resource name of a KMS crypto key (managed by the user) used to
    /// encrypt/decrypt their event data.
    ///
    /// It must match the pattern
    /// `projects/*/locations/*/keyRings/*/cryptoKeys/*`.
    #[prost(string, tag = "11")]
    pub crypto_key_name: ::prost::alloc::string::String,
    #[prost(oneof = "channel::Transport", tags = "8")]
    pub transport: ::core::option::Option<channel::Transport>,
}
/// Nested message and enum types in `Channel`.
pub mod channel {
    /// State lists all the possible states of a Channel
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
        /// Default value. This value is unused.
        Unspecified = 0,
        /// The PENDING state indicates that a Channel has been created successfully
        /// and there is a new activation token available for the subscriber to use
        /// to convey the Channel to the provider in order to create a Connection.
        Pending = 1,
        /// The ACTIVE state indicates that a Channel has been successfully
        /// connected with the event provider.
        /// An ACTIVE Channel is ready to receive and route events from the
        /// event provider.
        Active = 2,
        /// The INACTIVE state indicates that the Channel cannot receive events
        /// permanently. There are two possible cases this state can happen:
        ///
        /// 1. The SaaS provider disconnected from this Channel.
        /// 2. The Channel activation token has expired but the SaaS provider
        ///     wasn't connected.
        ///
        /// To re-establish a Connection with a provider, the subscriber
        /// should create a new Channel and give it to the provider.
        Inactive = 3,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Pending => "PENDING",
                State::Active => "ACTIVE",
                State::Inactive => "INACTIVE",
            }
        }
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Transport {
        /// Output only. The name of the Pub/Sub topic created and managed by
        /// Eventarc system as a transport for the event delivery. Format:
        /// `projects/{project}/topics/{topic_id}`.
        #[prost(string, tag = "8")]
        PubsubTopic(::prost::alloc::string::String),
    }
}
/// A representation of the ChannelConnection resource.
/// A ChannelConnection is a resource which event providers create during the
/// activation process to establish a connection between the provider and the
/// subscriber channel.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelConnection {
    /// Required. The name of the connection.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Server assigned ID of the resource.
    /// The server guarantees uniqueness and immutability until deleted.
    #[prost(string, tag = "2")]
    pub uid: ::prost::alloc::string::String,
    /// Required. The name of the connected subscriber Channel.
    /// This is a weak reference to avoid cross project and cross accounts
    /// references. This must be in
    /// `projects/{project}/location/{location}/channels/{channel_id}` format.
    #[prost(string, tag = "5")]
    pub channel: ::prost::alloc::string::String,
    /// Output only. The creation time.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last-modified time.
    #[prost(message, optional, tag = "7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Input only. Activation token for the channel. The token will be used
    /// during the creation of ChannelConnection to bind the channel with the
    /// provider project. This field will not be stored in the provider resource.
    #[prost(string, tag = "8")]
    pub activation_token: ::prost::alloc::string::String,
}
/// A GoogleChannelConfig is a resource that stores the custom settings
/// respected by Eventarc first-party triggers in the matching region.
/// Once configured, first-party event data will be protected
/// using the specified custom managed encryption key instead of Google-managed
/// encryption keys.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleChannelConfig {
    /// Required. The resource name of the config. Must be in the format of,
    /// `projects/{project}/locations/{location}/googleChannelConfig`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The last-modified time.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Resource name of a KMS crypto key (managed by the user) used to
    /// encrypt/decrypt their event data.
    ///
    /// It must match the pattern
    /// `projects/*/locations/*/keyRings/*/cryptoKeys/*`.
    #[prost(string, tag = "7")]
    pub crypto_key_name: ::prost::alloc::string::String,
}
/// A representation of the trigger resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trigger {
    /// Required. The resource name of the trigger. Must be unique within the location of the
    /// project and must be in
    /// `projects/{project}/locations/{location}/triggers/{trigger}` format.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Server-assigned unique identifier for the trigger. The value is a UUID4
    /// string and guaranteed to remain unchanged until the resource is deleted.
    #[prost(string, tag = "2")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. The creation time.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last-modified time.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. Unordered list. The list of filters that applies to event attributes. Only events that
    /// match all the provided filters are sent to the destination.
    #[prost(message, repeated, tag = "8")]
    pub event_filters: ::prost::alloc::vec::Vec<EventFilter>,
    /// Optional. The IAM service account email associated with the trigger. The
    /// service account represents the identity of the trigger.
    ///
    /// The principal who calls this API must have the `iam.serviceAccounts.actAs`
    /// permission in the service account. See
    /// <https://cloud.google.com/iam/docs/understanding-service-accounts?hl=en#sa_common>
    /// for more information.
    ///
    /// For Cloud Run destinations, this service account is used to generate
    /// identity tokens when invoking the service. See
    /// <https://cloud.google.com/run/docs/triggering/pubsub-push#create-service-account>
    /// for information on how to invoke authenticated Cloud Run services.
    /// To create Audit Log triggers, the service account should also
    /// have the `roles/eventarc.eventReceiver` IAM role.
    #[prost(string, tag = "9")]
    pub service_account: ::prost::alloc::string::String,
    /// Required. Destination specifies where the events should be sent to.
    #[prost(message, optional, tag = "10")]
    pub destination: ::core::option::Option<Destination>,
    /// Optional. To deliver messages, Eventarc might use other GCP
    /// products as a transport intermediary. This field contains a reference to
    /// that transport intermediary. This information can be used for debugging
    /// purposes.
    #[prost(message, optional, tag = "11")]
    pub transport: ::core::option::Option<Transport>,
    /// Optional. User labels attached to the triggers that can be used to group resources.
    #[prost(btree_map = "string, string", tag = "12")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. The name of the channel associated with the trigger in
    /// `projects/{project}/locations/{location}/channels/{channel}` format.
    /// You must provide a channel to receive events from Eventarc SaaS partners.
    #[prost(string, tag = "13")]
    pub channel: ::prost::alloc::string::String,
    /// Output only. The reason(s) why a trigger is in FAILED state.
    #[prost(btree_map = "string, message", tag = "15")]
    pub conditions: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        StateCondition,
    >,
    /// Output only. This checksum is computed by the server based on the value of other
    /// fields, and might be sent only on create requests to ensure that the
    /// client has an up-to-date value before proceeding.
    #[prost(string, tag = "99")]
    pub etag: ::prost::alloc::string::String,
}
/// Filters events based on exact matches on the CloudEvents attributes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventFilter {
    /// Required. The name of a CloudEvents attribute. Currently, only a subset of attributes
    /// are supported for filtering.
    ///
    /// All triggers MUST provide a filter for the 'type' attribute.
    #[prost(string, tag = "1")]
    pub attribute: ::prost::alloc::string::String,
    /// Required. The value for the attribute.
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
    /// Optional. The operator used for matching the events with the value of the
    /// filter. If not specified, only events that have an exact key-value pair
    /// specified in the filter are matched. The only allowed value is
    /// `match-path-pattern`.
    #[prost(string, tag = "3")]
    pub operator: ::prost::alloc::string::String,
}
/// A condition that is part of the trigger state computation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StateCondition {
    /// The canonical code of the condition.
    #[prost(enumeration = "super::super::super::rpc::Code", tag = "1")]
    pub code: i32,
    /// Human-readable message.
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
/// Represents a target of an invocation over HTTP.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Destination {
    #[prost(oneof = "destination::Descriptor", tags = "1, 2, 3, 4")]
    pub descriptor: ::core::option::Option<destination::Descriptor>,
}
/// Nested message and enum types in `Destination`.
pub mod destination {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Descriptor {
        /// Cloud Run fully-managed resource that receives the events. The resource
        /// should be in the same project as the trigger.
        #[prost(message, tag = "1")]
        CloudRun(super::CloudRun),
        /// The Cloud Function resource name. Only Cloud Functions V2 is supported.
        /// Format: `projects/{project}/locations/{location}/functions/{function}`
        #[prost(string, tag = "2")]
        CloudFunction(::prost::alloc::string::String),
        /// A GKE service capable of receiving events. The service should be running
        /// in the same project as the trigger.
        #[prost(message, tag = "3")]
        Gke(super::Gke),
        /// The resource name of the Workflow whose Executions are triggered by
        /// the events. The Workflow resource should be deployed in the same project
        /// as the trigger.
        /// Format: `projects/{project}/locations/{location}/workflows/{workflow}`
        #[prost(string, tag = "4")]
        Workflow(::prost::alloc::string::String),
    }
}
/// Represents the transport intermediaries created for the trigger to
/// deliver events.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transport {
    #[prost(oneof = "transport::Intermediary", tags = "1")]
    pub intermediary: ::core::option::Option<transport::Intermediary>,
}
/// Nested message and enum types in `Transport`.
pub mod transport {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Intermediary {
        /// The Pub/Sub topic and subscription used by Eventarc as a transport
        /// intermediary.
        #[prost(message, tag = "1")]
        Pubsub(super::Pubsub),
    }
}
/// Represents a Cloud Run destination.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudRun {
    /// Required. The name of the Cloud Run service being addressed. See
    /// <https://cloud.google.com/run/docs/reference/rest/v1/namespaces.services.>
    ///
    /// Only services located in the same project as the trigger object
    /// can be addressed.
    #[prost(string, tag = "1")]
    pub service: ::prost::alloc::string::String,
    /// Optional. The relative path on the Cloud Run service the events should be sent to.
    ///
    /// The value must conform to the definition of a URI path segment (section 3.3
    /// of RFC2396). Examples: "/route", "route", "route/subroute".
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
    /// Required. The region the Cloud Run service is deployed in.
    #[prost(string, tag = "3")]
    pub region: ::prost::alloc::string::String,
}
/// Represents a GKE destination.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Gke {
    /// Required. The name of the cluster the GKE service is running in. The cluster must be
    /// running in the same project as the trigger being created.
    #[prost(string, tag = "1")]
    pub cluster: ::prost::alloc::string::String,
    /// Required. The name of the Google Compute Engine in which the cluster resides, which
    /// can either be compute zone (for example, us-central1-a) for the zonal
    /// clusters or region (for example, us-central1) for regional clusters.
    #[prost(string, tag = "2")]
    pub location: ::prost::alloc::string::String,
    /// Required. The namespace the GKE service is running in.
    #[prost(string, tag = "3")]
    pub namespace: ::prost::alloc::string::String,
    /// Required. Name of the GKE service.
    #[prost(string, tag = "4")]
    pub service: ::prost::alloc::string::String,
    /// Optional. The relative path on the GKE service the events should be sent to.
    ///
    /// The value must conform to the definition of a URI path segment (section 3.3
    /// of RFC2396). Examples: "/route", "route", "route/subroute".
    #[prost(string, tag = "5")]
    pub path: ::prost::alloc::string::String,
}
/// Represents a Pub/Sub transport.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pubsub {
    /// Optional. The name of the Pub/Sub topic created and managed by Eventarc as
    /// a transport for the event delivery. Format:
    /// `projects/{PROJECT_ID}/topics/{TOPIC_NAME}`.
    ///
    /// You can set an existing topic for triggers of the type
    /// `google.cloud.pubsub.topic.v1.messagePublished`. The topic you provide
    /// here is not deleted by Eventarc at trigger deletion.
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
    /// Output only. The name of the Pub/Sub subscription created and managed by Eventarc
    /// as a transport for the event delivery. Format:
    /// `projects/{PROJECT_ID}/subscriptions/{SUBSCRIPTION_NAME}`.
    #[prost(string, tag = "2")]
    pub subscription: ::prost::alloc::string::String,
}
/// The request message for the GetTrigger method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTriggerRequest {
    /// Required. The name of the trigger to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for the ListTriggers method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTriggersRequest {
    /// Required. The parent collection to list triggers on.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of triggers to return on each page.
    ///
    /// Note: The service may send fewer.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The page token; provide the value from the `next_page_token` field in a
    /// previous `ListTriggers` call to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListTriggers` must match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// The sorting order of the resources returned. Value should be a
    /// comma-separated list of fields. The default sorting order is ascending. To
    /// specify descending order for a field, append a `desc` suffix; for example:
    /// `name desc, trigger_id`.
    #[prost(string, tag = "4")]
    pub order_by: ::prost::alloc::string::String,
    /// Filter field. Used to filter the Triggers to be listed. Possible filters
    /// are described in <https://google.aip.dev/160.> For example, using
    /// "?filter=destination:gke" would list only Triggers with a gke destination.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
}
/// The response message for the `ListTriggers` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTriggersResponse {
    /// The requested triggers, up to the number specified in `page_size`.
    #[prost(message, repeated, tag = "1")]
    pub triggers: ::prost::alloc::vec::Vec<Trigger>,
    /// A page token that can be sent to `ListTriggers` to request the next page.
    /// If this is empty, then there are no more pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Unreachable resources, if any.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The request message for the CreateTrigger method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTriggerRequest {
    /// Required. The parent collection in which to add this trigger.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The trigger to create.
    #[prost(message, optional, tag = "2")]
    pub trigger: ::core::option::Option<Trigger>,
    /// Required. The user-provided ID to be assigned to the trigger.
    #[prost(string, tag = "3")]
    pub trigger_id: ::prost::alloc::string::String,
    /// Required. If set, validate the request and preview the review, but do not
    /// post it.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// The request message for the UpdateTrigger method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTriggerRequest {
    /// The trigger to be updated.
    #[prost(message, optional, tag = "1")]
    pub trigger: ::core::option::Option<Trigger>,
    /// The fields to be updated; only fields explicitly provided are updated.
    /// If no field mask is provided, all provided fields in the request are
    /// updated. To update all fields, provide a field mask of "*".
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// If set to true, and the trigger is not found, a new trigger will be
    /// created. In this situation, `update_mask` is ignored.
    #[prost(bool, tag = "3")]
    pub allow_missing: bool,
    /// Required. If set, validate the request and preview the review, but do not
    /// post it.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// The request message for the DeleteTrigger method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTriggerRequest {
    /// Required. The name of the trigger to be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If provided, the trigger will only be deleted if the etag matches the
    /// current etag on the resource.
    #[prost(string, tag = "2")]
    pub etag: ::prost::alloc::string::String,
    /// If set to true, and the trigger is not found, the request will succeed
    /// but no action will be taken on the server.
    #[prost(bool, tag = "3")]
    pub allow_missing: bool,
    /// Required. If set, validate the request and preview the review, but do not
    /// post it.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// The request message for the GetChannel method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChannelRequest {
    /// Required. The name of the channel to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for the ListChannels method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChannelsRequest {
    /// Required. The parent collection to list channels on.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of channels to return on each page.
    ///
    /// Note: The service may send fewer.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The page token; provide the value from the `next_page_token` field in a
    /// previous `ListChannels` call to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListChannels` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// The sorting order of the resources returned. Value should be a
    /// comma-separated list of fields. The default sorting order is ascending. To
    /// specify descending order for a field, append a `desc` suffix; for example:
    /// `name desc, channel_id`.
    #[prost(string, tag = "4")]
    pub order_by: ::prost::alloc::string::String,
}
/// The response message for the `ListChannels` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChannelsResponse {
    /// The requested channels, up to the number specified in `page_size`.
    #[prost(message, repeated, tag = "1")]
    pub channels: ::prost::alloc::vec::Vec<Channel>,
    /// A page token that can be sent to `ListChannels` to request the next page.
    /// If this is empty, then there are no more pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Unreachable resources, if any.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The request message for the CreateChannel method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateChannelRequest {
    /// Required. The parent collection in which to add this channel.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The channel to create.
    #[prost(message, optional, tag = "2")]
    pub channel: ::core::option::Option<Channel>,
    /// Required. The user-provided ID to be assigned to the channel.
    #[prost(string, tag = "3")]
    pub channel_id: ::prost::alloc::string::String,
    /// Required. If set, validate the request and preview the review, but do not
    /// post it.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// The request message for the UpdateChannel method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateChannelRequest {
    /// The channel to be updated.
    #[prost(message, optional, tag = "1")]
    pub channel: ::core::option::Option<Channel>,
    /// The fields to be updated; only fields explicitly provided are updated.
    /// If no field mask is provided, all provided fields in the request are
    /// updated. To update all fields, provide a field mask of "*".
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. If set, validate the request and preview the review, but do not
    /// post it.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// The request message for the DeleteChannel method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteChannelRequest {
    /// Required. The name of the channel to be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. If set, validate the request and preview the review, but do not
    /// post it.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
}
/// The request message for the GetProvider method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProviderRequest {
    /// Required. The name of the provider to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for the ListProviders method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProvidersRequest {
    /// Required. The parent of the provider to get.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of providers to return on each page.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The page token; provide the value from the `next_page_token` field in a
    /// previous `ListProviders` call to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListProviders` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// The sorting order of the resources returned. Value should be a
    /// comma-separated list of fields. The default sorting oder is ascending. To
    /// specify descending order for a field, append a `desc` suffix; for example:
    /// `name desc, _id`.
    #[prost(string, tag = "4")]
    pub order_by: ::prost::alloc::string::String,
    /// The filter field that the list request will filter on.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
}
/// The response message for the `ListProviders` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProvidersResponse {
    /// The requested providers, up to the number specified in `page_size`.
    #[prost(message, repeated, tag = "1")]
    pub providers: ::prost::alloc::vec::Vec<Provider>,
    /// A page token that can be sent to `ListProviders` to request the next page.
    /// If this is empty, then there are no more pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Unreachable resources, if any.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The request message for the GetChannelConnection method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChannelConnectionRequest {
    /// Required. The name of the channel connection to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for the ListChannelConnections method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChannelConnectionsRequest {
    /// Required. The parent collection from which to list channel connections.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of channel connections to return on each page.
    ///
    /// Note: The service may send fewer responses.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The page token; provide the value from the `next_page_token` field in a
    /// previous `ListChannelConnections` call to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListChannelConnetions`
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for the `ListChannelConnections` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChannelConnectionsResponse {
    /// The requested channel connections, up to the number specified in
    /// `page_size`.
    #[prost(message, repeated, tag = "1")]
    pub channel_connections: ::prost::alloc::vec::Vec<ChannelConnection>,
    /// A page token that can be sent to `ListChannelConnections` to request the
    /// next page.
    /// If this is empty, then there are no more pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Unreachable resources, if any.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The request message for the CreateChannelConnection method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateChannelConnectionRequest {
    /// Required. The parent collection in which to add this channel connection.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Channel connection to create.
    #[prost(message, optional, tag = "2")]
    pub channel_connection: ::core::option::Option<ChannelConnection>,
    /// Required. The user-provided ID to be assigned to the channel connection.
    #[prost(string, tag = "3")]
    pub channel_connection_id: ::prost::alloc::string::String,
}
/// The request message for the DeleteChannelConnection method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteChannelConnectionRequest {
    /// Required. The name of the channel connection to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for the UpdateGoogleChannelConfig method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGoogleChannelConfigRequest {
    /// Required. The config to be updated.
    #[prost(message, optional, tag = "1")]
    pub google_channel_config: ::core::option::Option<GoogleChannelConfig>,
    /// The fields to be updated; only fields explicitly provided are updated.
    /// If no field mask is provided, all provided fields in the request are
    /// updated. To update all fields, provide a field mask of "*".
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request message for the GetGoogleChannelConfig method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGoogleChannelConfigRequest {
    /// Required. The name of the config to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Represents the metadata of the long-running operation.
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
/// Generated client implementations.
pub mod eventarc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Eventarc allows users to subscribe to various events that are provided by
    /// Google Cloud services and forward them to supported destinations.
    #[derive(Debug, Clone)]
    pub struct EventarcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> EventarcClient<T>
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
        ) -> EventarcClient<InterceptedService<T, F>>
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
            EventarcClient::new(InterceptedService::new(inner, interceptor))
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
        /// Get a single trigger.
        pub async fn get_trigger(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTriggerRequest>,
        ) -> Result<tonic::Response<super::Trigger>, tonic::Status> {
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
                "/google.cloud.eventarc.v1.Eventarc/GetTrigger",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List triggers.
        pub async fn list_triggers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTriggersRequest>,
        ) -> Result<tonic::Response<super::ListTriggersResponse>, tonic::Status> {
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
                "/google.cloud.eventarc.v1.Eventarc/ListTriggers",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Create a new trigger in a particular project and location.
        pub async fn create_trigger(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTriggerRequest>,
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
                "/google.cloud.eventarc.v1.Eventarc/CreateTrigger",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Update a single trigger.
        pub async fn update_trigger(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTriggerRequest>,
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
                "/google.cloud.eventarc.v1.Eventarc/UpdateTrigger",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Delete a single trigger.
        pub async fn delete_trigger(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTriggerRequest>,
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
                "/google.cloud.eventarc.v1.Eventarc/DeleteTrigger",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get a single Channel.
        pub async fn get_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::GetChannelRequest>,
        ) -> Result<tonic::Response<super::Channel>, tonic::Status> {
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
                "/google.cloud.eventarc.v1.Eventarc/GetChannel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List channels.
        pub async fn list_channels(
            &mut self,
            request: impl tonic::IntoRequest<super::ListChannelsRequest>,
        ) -> Result<tonic::Response<super::ListChannelsResponse>, tonic::Status> {
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
                "/google.cloud.eventarc.v1.Eventarc/ListChannels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Create a new channel in a particular project and location.
        pub async fn create_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateChannelRequest>,
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
                "/google.cloud.eventarc.v1.Eventarc/CreateChannel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Update a single channel.
        pub async fn update_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateChannelRequest>,
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
                "/google.cloud.eventarc.v1.Eventarc/UpdateChannel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Delete a single channel.
        pub async fn delete_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteChannelRequest>,
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
                "/google.cloud.eventarc.v1.Eventarc/DeleteChannel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get a single Provider.
        pub async fn get_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProviderRequest>,
        ) -> Result<tonic::Response<super::Provider>, tonic::Status> {
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
                "/google.cloud.eventarc.v1.Eventarc/GetProvider",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List providers.
        pub async fn list_providers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProvidersRequest>,
        ) -> Result<tonic::Response<super::ListProvidersResponse>, tonic::Status> {
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
                "/google.cloud.eventarc.v1.Eventarc/ListProviders",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get a single ChannelConnection.
        pub async fn get_channel_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::GetChannelConnectionRequest>,
        ) -> Result<tonic::Response<super::ChannelConnection>, tonic::Status> {
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
                "/google.cloud.eventarc.v1.Eventarc/GetChannelConnection",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List channel connections.
        pub async fn list_channel_connections(
            &mut self,
            request: impl tonic::IntoRequest<super::ListChannelConnectionsRequest>,
        ) -> Result<
            tonic::Response<super::ListChannelConnectionsResponse>,
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
                "/google.cloud.eventarc.v1.Eventarc/ListChannelConnections",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Create a new ChannelConnection in a particular project and location.
        pub async fn create_channel_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateChannelConnectionRequest>,
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
                "/google.cloud.eventarc.v1.Eventarc/CreateChannelConnection",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Delete a single ChannelConnection.
        pub async fn delete_channel_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteChannelConnectionRequest>,
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
                "/google.cloud.eventarc.v1.Eventarc/DeleteChannelConnection",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get a GoogleChannelConfig
        pub async fn get_google_channel_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGoogleChannelConfigRequest>,
        ) -> Result<tonic::Response<super::GoogleChannelConfig>, tonic::Status> {
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
                "/google.cloud.eventarc.v1.Eventarc/GetGoogleChannelConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Update a single GoogleChannelConfig
        pub async fn update_google_channel_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateGoogleChannelConfigRequest>,
        ) -> Result<tonic::Response<super::GoogleChannelConfig>, tonic::Status> {
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
                "/google.cloud.eventarc.v1.Eventarc/UpdateGoogleChannelConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
