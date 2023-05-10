/// Metadata describing an \[Operation][google.longrunning.Operation\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadataV1 {
    /// Target of the operation - for example
    /// `projects/project-1/locations/region-1/functions/function-1`
    #[prost(string, tag = "1")]
    pub target: ::prost::alloc::string::String,
    /// Type of operation.
    #[prost(enumeration = "OperationType", tag = "2")]
    pub r#type: i32,
    /// The original request that started the operation.
    #[prost(message, optional, tag = "3")]
    pub request: ::core::option::Option<::prost_types::Any>,
    /// Version id of the function created or updated by an API call.
    /// This field is only populated for Create and Update operations.
    #[prost(int64, tag = "4")]
    pub version_id: i64,
    /// The last update timestamp of the operation.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The Cloud Build ID of the function created or updated by an API call.
    /// This field is only populated for Create and Update operations.
    #[prost(string, tag = "6")]
    pub build_id: ::prost::alloc::string::String,
    /// An identifier for Firebase function sources. Disclaimer: This field is only
    /// supported for Firebase function deployments.
    #[prost(string, tag = "7")]
    pub source_token: ::prost::alloc::string::String,
    /// The Cloud Build Name of the function deployment.
    /// This field is only populated for Create and Update operations.
    /// `projects/<project-number>/locations/<region>/builds/<build-id>`.
    #[prost(string, tag = "8")]
    pub build_name: ::prost::alloc::string::String,
}
/// A type of an operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationType {
    /// Unknown operation type.
    OperationUnspecified = 0,
    /// Triggered by CreateFunction call
    CreateFunction = 1,
    /// Triggered by UpdateFunction call
    UpdateFunction = 2,
    /// Triggered by DeleteFunction call.
    DeleteFunction = 3,
}
impl OperationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OperationType::OperationUnspecified => "OPERATION_UNSPECIFIED",
            OperationType::CreateFunction => "CREATE_FUNCTION",
            OperationType::UpdateFunction => "UPDATE_FUNCTION",
            OperationType::DeleteFunction => "DELETE_FUNCTION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPERATION_UNSPECIFIED" => Some(Self::OperationUnspecified),
            "CREATE_FUNCTION" => Some(Self::CreateFunction),
            "UPDATE_FUNCTION" => Some(Self::UpdateFunction),
            "DELETE_FUNCTION" => Some(Self::DeleteFunction),
            _ => None,
        }
    }
}
/// Describes a Cloud Function that contains user computation executed in
/// response to an event. It encapsulates function and triggers configurations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudFunction {
    /// A user-defined name of the function. Function names must be unique
    /// globally and match pattern `projects/*/locations/*/functions/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// User-provided description of a function.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Status of the function deployment.
    #[prost(enumeration = "CloudFunctionStatus", tag = "7")]
    pub status: i32,
    /// The name of the function (as defined in source code) that is executed.
    /// Defaults to the resource name suffix, if not specified. For
    /// backward compatibility, if function with given name is not found, the
    /// system tries to use the function named "function".
    /// For Node.js, this is the name of a function exported by the module
    /// as specified in `source_location`.
    #[prost(string, tag = "8")]
    pub entry_point: ::prost::alloc::string::String,
    /// The runtime in which to run the function. Required when deploying a new
    /// function, optional when updating an existing function. For a complete
    /// list of possible choices, see the
    /// [`gcloud` command
    /// reference](<https://cloud.google.com/sdk/gcloud/reference/functions/deploy#--runtime>).
    #[prost(string, tag = "19")]
    pub runtime: ::prost::alloc::string::String,
    /// The function execution timeout. Execution is considered failed and
    /// can be terminated if the function is not completed at the end of the
    /// timeout period. Defaults to 60 seconds.
    #[prost(message, optional, tag = "9")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
    /// The amount of memory in MB available for a function.
    /// Defaults to 256MB.
    #[prost(int32, tag = "10")]
    pub available_memory_mb: i32,
    /// The email of the function's service account. If empty, defaults to
    /// `{project_id}@appspot.gserviceaccount.com`.
    #[prost(string, tag = "11")]
    pub service_account_email: ::prost::alloc::string::String,
    /// Output only. The last update timestamp of a Cloud Function.
    #[prost(message, optional, tag = "12")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The version identifier of the Cloud Function. Each deployment
    /// attempt results in a new version of a function being created.
    #[prost(int64, tag = "14")]
    pub version_id: i64,
    /// Labels associated with this Cloud Function.
    #[prost(btree_map = "string, string", tag = "15")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Environment variables that shall be available during function execution.
    #[prost(btree_map = "string, string", tag = "17")]
    pub environment_variables: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Build environment variables that shall be available during build time.
    #[prost(btree_map = "string, string", tag = "28")]
    pub build_environment_variables: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The Serverless VPC Access connector that this cloud function can connect
    /// to. It can be either the fully qualified URI, or the short name of the
    /// connector resource. If the connector name is used, the connector must
    /// belong to the same project as the function. Otherwise, it must belong to a
    /// project within the same organization. The format of this field is either
    /// `projects/{project}/global/networks/{network}` or `{network}`, where
    /// `{project}` is a project id where the network is defined, and `{network}`
    /// is the short name of the network.
    ///
    /// This field is mutually exclusive with `vpc_connector` and will be replaced
    /// by it.
    ///
    /// See [the VPC documentation](<https://cloud.google.com/compute/docs/vpc>) for
    /// more information on connecting Cloud projects.
    #[prost(string, tag = "18")]
    pub network: ::prost::alloc::string::String,
    /// The limit on the maximum number of function instances that can coexist at a
    /// given time.
    ///
    /// In some cases, such as rapid traffic surges, Cloud Functions can for a
    /// short period of time create more instances than the specified max
    /// instances limit. If your function cannot tolerate this temporary behavior,
    /// you might want to factor in a safety margin and set a lower max instances
    /// value than your function can tolerate.
    ///
    /// See the [Max
    /// Instances](<https://cloud.google.com/functions/docs/max-instances>) Guide for
    /// more details.
    #[prost(int32, tag = "20")]
    pub max_instances: i32,
    /// A lower bound for the number function instances that can coexist at a
    /// given time.
    #[prost(int32, tag = "32")]
    pub min_instances: i32,
    /// The VPC Network Connector that this cloud function can connect to. It can
    /// be either the fully qualified URI, or the short name of the network
    /// connector resource. The format of this field is
    /// `projects/*/locations/*/connectors/*`
    ///
    /// This field is mutually exclusive with `network` field and will eventually
    /// replace it.
    ///
    /// See [the VPC documentation](<https://cloud.google.com/compute/docs/vpc>) for
    /// more information on connecting Cloud projects.
    #[prost(string, tag = "22")]
    pub vpc_connector: ::prost::alloc::string::String,
    /// The egress settings for the connector, controlling what traffic is diverted
    /// through it.
    #[prost(enumeration = "cloud_function::VpcConnectorEgressSettings", tag = "23")]
    pub vpc_connector_egress_settings: i32,
    /// The ingress settings for the function, controlling what traffic can reach
    /// it.
    #[prost(enumeration = "cloud_function::IngressSettings", tag = "24")]
    pub ingress_settings: i32,
    /// Resource name of a KMS crypto key (managed by the user) used to
    /// encrypt/decrypt function resources.
    ///
    /// It must match the pattern
    /// `projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}`.
    ///
    /// If specified, you must also provide an artifact registry repository using
    /// the `docker_repository` field that was created with the same KMS crypto
    /// key.
    ///
    /// The following service accounts need to be granted the role 'Cloud KMS
    /// CryptoKey Encrypter/Decrypter (roles/cloudkms.cryptoKeyEncrypterDecrypter)'
    /// on the Key/KeyRing/Project/Organization (least access preferred).
    ///
    /// 1. Google Cloud Functions service account
    ///     (service-{project_number}@gcf-admin-robot.iam.gserviceaccount.com) -
    ///     Required to protect the function's image.
    /// 2. Google Storage service account
    ///     (service-{project_number}@gs-project-accounts.iam.gserviceaccount.com) -
    ///     Required to protect the function's source code.
    ///     If this service account does not exist, deploying a function without a
    ///     KMS key or retrieving the service agent name provisions it. For more
    ///     information, see
    ///     <https://cloud.google.com/storage/docs/projects#service-agents> and
    ///     <https://cloud.google.com/storage/docs/getting-service-agent#gsutil.>
    ///
    /// Google Cloud Functions delegates access to service agents to protect
    /// function resources in internal projects that are not accessible by the
    /// end user.
    #[prost(string, tag = "25")]
    pub kms_key_name: ::prost::alloc::string::String,
    /// Name of the Cloud Build Custom Worker Pool that should be used to build the
    /// function. The format of this field is
    /// `projects/{project}/locations/{region}/workerPools/{workerPool}` where
    /// `{project}` and `{region}` are the project id and region respectively where
    /// the worker pool is defined and `{workerPool}` is the short name of the
    /// worker pool.
    ///
    /// If the project id is not the same as the function, then the Cloud
    /// Functions Service Agent
    /// (`service-<project_number>@gcf-admin-robot.iam.gserviceaccount.com`) must
    /// be granted the role Cloud Build Custom Workers Builder
    /// (`roles/cloudbuild.customworkers.builder`) in the project.
    #[prost(string, tag = "26")]
    pub build_worker_pool: ::prost::alloc::string::String,
    /// Output only. The Cloud Build ID of the latest successful deployment of the
    /// function.
    #[prost(string, tag = "27")]
    pub build_id: ::prost::alloc::string::String,
    /// Output only. The Cloud Build Name of the function deployment.
    /// `projects/<project-number>/locations/<region>/builds/<build-id>`.
    #[prost(string, tag = "33")]
    pub build_name: ::prost::alloc::string::String,
    /// Secret environment variables configuration.
    #[prost(message, repeated, tag = "29")]
    pub secret_environment_variables: ::prost::alloc::vec::Vec<SecretEnvVar>,
    /// Secret volumes configuration.
    #[prost(message, repeated, tag = "30")]
    pub secret_volumes: ::prost::alloc::vec::Vec<SecretVolume>,
    /// Input only. An identifier for Firebase function sources. Disclaimer: This
    /// field is only supported for Firebase function deployments.
    #[prost(string, tag = "31")]
    pub source_token: ::prost::alloc::string::String,
    /// User managed repository created in Artifact Registry optionally with a
    /// customer managed encryption key. If specified, deployments will use
    /// Artifact Registry. If unspecified and the deployment is eligible to use
    /// Artifact Registry, GCF will create and use a repository named
    /// 'gcf-artifacts' for every deployed region. This is the repository to which
    /// the function docker image is pushed after it is built by Cloud Build.
    ///
    /// It must match the pattern
    /// `projects/{project}/locations/{location}/repositories/{repository}`.
    ///
    /// Cross-project repositories are not supported.
    /// Cross-location repositories are not supported.
    /// Repository format must be 'DOCKER'.
    #[prost(string, tag = "34")]
    pub docker_repository: ::prost::alloc::string::String,
    /// Docker Registry to use for this deployment.
    ///
    /// If `docker_repository` field is specified, this field is automatically
    /// set as `ARTIFACT_REGISTRY`.
    /// If unspecified, it currently defaults to `CONTAINER_REGISTRY`.
    /// This field may be overridden by the backend for eligible deployments.
    #[prost(enumeration = "cloud_function::DockerRegistry", tag = "35")]
    pub docker_registry: i32,
    /// The location of the function source code.
    #[prost(oneof = "cloud_function::SourceCode", tags = "3, 4, 16")]
    pub source_code: ::core::option::Option<cloud_function::SourceCode>,
    /// An event that triggers the function.
    #[prost(oneof = "cloud_function::Trigger", tags = "5, 6")]
    pub trigger: ::core::option::Option<cloud_function::Trigger>,
}
/// Nested message and enum types in `CloudFunction`.
pub mod cloud_function {
    /// Available egress settings.
    ///
    /// This controls what traffic is diverted through the Serverless VPC Access
    /// connector resource. By default, PRIVATE_RANGES_ONLY is used.
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
    pub enum VpcConnectorEgressSettings {
        /// Unspecified.
        Unspecified = 0,
        /// Use the Serverless VPC Access connector only for private IP space from
        /// RFC1918.
        PrivateRangesOnly = 1,
        /// Force the use of Serverless VPC Access connector for all egress traffic
        /// from the function.
        AllTraffic = 2,
    }
    impl VpcConnectorEgressSettings {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                VpcConnectorEgressSettings::Unspecified => {
                    "VPC_CONNECTOR_EGRESS_SETTINGS_UNSPECIFIED"
                }
                VpcConnectorEgressSettings::PrivateRangesOnly => "PRIVATE_RANGES_ONLY",
                VpcConnectorEgressSettings::AllTraffic => "ALL_TRAFFIC",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "VPC_CONNECTOR_EGRESS_SETTINGS_UNSPECIFIED" => Some(Self::Unspecified),
                "PRIVATE_RANGES_ONLY" => Some(Self::PrivateRangesOnly),
                "ALL_TRAFFIC" => Some(Self::AllTraffic),
                _ => None,
            }
        }
    }
    /// Available ingress settings.
    ///
    /// This controls what traffic can reach the function.
    ///
    /// If unspecified, ALLOW_ALL is used.
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
    pub enum IngressSettings {
        /// Unspecified.
        Unspecified = 0,
        /// Allow HTTP traffic from public and private sources.
        AllowAll = 1,
        /// Allow HTTP traffic from only private VPC sources.
        AllowInternalOnly = 2,
        /// Allow HTTP traffic from private VPC sources and through GCLB.
        AllowInternalAndGclb = 3,
    }
    impl IngressSettings {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                IngressSettings::Unspecified => "INGRESS_SETTINGS_UNSPECIFIED",
                IngressSettings::AllowAll => "ALLOW_ALL",
                IngressSettings::AllowInternalOnly => "ALLOW_INTERNAL_ONLY",
                IngressSettings::AllowInternalAndGclb => "ALLOW_INTERNAL_AND_GCLB",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INGRESS_SETTINGS_UNSPECIFIED" => Some(Self::Unspecified),
                "ALLOW_ALL" => Some(Self::AllowAll),
                "ALLOW_INTERNAL_ONLY" => Some(Self::AllowInternalOnly),
                "ALLOW_INTERNAL_AND_GCLB" => Some(Self::AllowInternalAndGclb),
                _ => None,
            }
        }
    }
    /// Docker Registry to use for storing function Docker images.
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
    pub enum DockerRegistry {
        /// Unspecified.
        Unspecified = 0,
        /// Docker images are stored in multi-regional Container Registry
        /// repositories named `gcf`.
        ContainerRegistry = 1,
        /// Docker images are stored in regional Artifact Registry repositories.
        /// By default, Cloud Functions creates and uses repositories named
        /// `gcf-artifacts` in every region in which a function is deployed. But the
        /// repository to use can also be specified by the user by using the
        /// `docker_repository` field.
        ArtifactRegistry = 2,
    }
    impl DockerRegistry {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DockerRegistry::Unspecified => "DOCKER_REGISTRY_UNSPECIFIED",
                DockerRegistry::ContainerRegistry => "CONTAINER_REGISTRY",
                DockerRegistry::ArtifactRegistry => "ARTIFACT_REGISTRY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DOCKER_REGISTRY_UNSPECIFIED" => Some(Self::Unspecified),
                "CONTAINER_REGISTRY" => Some(Self::ContainerRegistry),
                "ARTIFACT_REGISTRY" => Some(Self::ArtifactRegistry),
                _ => None,
            }
        }
    }
    /// The location of the function source code.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SourceCode {
        /// The Google Cloud Storage URL, starting with `gs://`, pointing to the zip
        /// archive which contains the function.
        #[prost(string, tag = "3")]
        SourceArchiveUrl(::prost::alloc::string::String),
        /// **Beta Feature**
        ///
        /// The source repository where a function is hosted.
        #[prost(message, tag = "4")]
        SourceRepository(super::SourceRepository),
        /// The Google Cloud Storage-signed URL used for source uploading, generated
        /// by calling \[google.cloud.functions.v1.GenerateUploadUrl\].
        ///
        /// The signature is validated on write methods (Create, Update)
        /// The signature is stripped from the Function object on read methods (Get,
        /// List)
        #[prost(string, tag = "16")]
        SourceUploadUrl(::prost::alloc::string::String),
    }
    /// An event that triggers the function.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Trigger {
        /// An HTTPS endpoint type of source that can be triggered via URL.
        #[prost(message, tag = "5")]
        HttpsTrigger(super::HttpsTrigger),
        /// A source that fires events in response to a condition in another service.
        #[prost(message, tag = "6")]
        EventTrigger(super::EventTrigger),
    }
}
/// Describes SourceRepository, used to represent parameters related to
/// source repository where a function is hosted.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceRepository {
    /// The URL pointing to the hosted repository where the function is defined.
    /// There are supported Cloud Source Repository URLs in the following
    /// formats:
    ///
    /// To refer to a specific commit:
    /// `<https://source.developers.google.com/projects/*/repos/*/revisions/*/paths/*`>
    /// To refer to a moveable alias (branch):
    /// `<https://source.developers.google.com/projects/*/repos/*/moveable-aliases/*/paths/*`>
    /// In particular, to refer to HEAD use `master` moveable alias.
    /// To refer to a specific fixed alias (tag):
    /// `<https://source.developers.google.com/projects/*/repos/*/fixed-aliases/*/paths/*`>
    ///
    /// You can omit `paths/*` if you want to use the main directory.
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    /// Output only. The URL pointing to the hosted repository where the function
    /// were defined at the time of deployment. It always points to a specific
    /// commit in the format described above.
    #[prost(string, tag = "2")]
    pub deployed_url: ::prost::alloc::string::String,
}
/// Describes HttpsTrigger, could be used to connect web hooks to function.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpsTrigger {
    /// Output only. The deployed URL for the function.
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    /// The security level for the function.
    #[prost(enumeration = "https_trigger::SecurityLevel", tag = "2")]
    pub security_level: i32,
}
/// Nested message and enum types in `HttpsTrigger`.
pub mod https_trigger {
    /// Available security-level settings.
    ///
    /// This controls the methods to enforce security (HTTPS) on a URL.
    ///
    /// If unspecified, SECURE_OPTIONAL is used.
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
    pub enum SecurityLevel {
        /// Unspecified.
        Unspecified = 0,
        /// Requests for a URL that match this handler that do not use HTTPS are
        /// automatically redirected to the HTTPS URL with the same path. Query
        /// parameters are reserved for the redirect.
        SecureAlways = 1,
        /// Both HTTP and HTTPS requests with URLs that match the handler succeed
        /// without redirects. The application can examine the request to determine
        /// which protocol was used and respond accordingly.
        SecureOptional = 2,
    }
    impl SecurityLevel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SecurityLevel::Unspecified => "SECURITY_LEVEL_UNSPECIFIED",
                SecurityLevel::SecureAlways => "SECURE_ALWAYS",
                SecurityLevel::SecureOptional => "SECURE_OPTIONAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SECURITY_LEVEL_UNSPECIFIED" => Some(Self::Unspecified),
                "SECURE_ALWAYS" => Some(Self::SecureAlways),
                "SECURE_OPTIONAL" => Some(Self::SecureOptional),
                _ => None,
            }
        }
    }
}
/// Describes EventTrigger, used to request that events be sent from another
/// service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventTrigger {
    /// Required. The type of event to observe. For example:
    /// `providers/cloud.storage/eventTypes/object.change` and
    /// `providers/cloud.pubsub/eventTypes/topic.publish`.
    ///
    /// Event types match pattern `providers/*/eventTypes/*.*`.
    /// The pattern contains:
    ///
    /// 1. namespace: For example, `cloud.storage` and
    ///     `google.firebase.analytics`.
    /// 2. resource type: The type of resource on which event occurs. For
    ///     example, the Google Cloud Storage API includes the type `object`.
    /// 3. action: The action that generates the event. For example, action for
    ///     a Google Cloud Storage Object is 'change'.
    /// These parts are lowercase.
    #[prost(string, tag = "1")]
    pub event_type: ::prost::alloc::string::String,
    /// Required. The resource(s) from which to observe events, for example,
    /// `projects/_/buckets/myBucket`.
    ///
    /// Not all syntactically correct values are accepted by all services. For
    /// example:
    ///
    /// 1. The authorization model must support it. Google Cloud Functions
    ///     only allows EventTriggers to be deployed that observe resources in the
    ///     same project as the `CloudFunction`.
    /// 2. The resource type must match the pattern expected for an
    ///     `event_type`. For example, an `EventTrigger` that has an
    ///     `event_type` of "google.pubsub.topic.publish" should have a resource
    ///     that matches Google Cloud Pub/Sub topics.
    ///
    /// Additionally, some services may support short names when creating an
    /// `EventTrigger`. These are always returned in the normalized "long"
    /// format.
    ///
    /// See each *service's* documentation for supported formats.
    #[prost(string, tag = "2")]
    pub resource: ::prost::alloc::string::String,
    /// The hostname of the service that should be observed.
    ///
    /// If no string is provided, the default service implementing the API will
    /// be used. For example, `storage.googleapis.com` is the default for all
    /// event types in the `google.storage` namespace.
    #[prost(string, tag = "3")]
    pub service: ::prost::alloc::string::String,
    /// Specifies policy for failed executions.
    #[prost(message, optional, tag = "5")]
    pub failure_policy: ::core::option::Option<FailurePolicy>,
}
/// Describes the policy in case of function's execution failure.
/// If empty, then defaults to ignoring failures (i.e., not retrying them).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FailurePolicy {
    /// Defines the action taken in case of a function execution failure.
    #[prost(oneof = "failure_policy::Action", tags = "1")]
    pub action: ::core::option::Option<failure_policy::Action>,
}
/// Nested message and enum types in `FailurePolicy`.
pub mod failure_policy {
    /// Describes the retry policy in case of function's execution failure.
    /// A function execution is retried on any failure.
    /// A failed execution is retried up to 7 days with an exponential backoff
    /// (capped at 10 seconds).
    /// Retried execution is charged as any other execution.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Retry {}
    /// Defines the action taken in case of a function execution failure.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Action {
        /// If specified, the function is retried in case of a failure.
        #[prost(message, tag = "1")]
        Retry(Retry),
    }
}
/// Configuration for a secret environment variable. It has the information
/// necessary to fetch the secret value from Secret Manager and expose it as an
/// environment variable.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretEnvVar {
    /// Name of the environment variable.
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// Project identifier (preferrably project number but can also be the project
    /// ID) of the project that contains the secret. If not set, it is
    /// populated with the function's project, assuming that the secret exists in
    /// the same project as the function.
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
    /// Name of the secret in Secret Manager (not the full resource name).
    #[prost(string, tag = "3")]
    pub secret: ::prost::alloc::string::String,
    /// Version of the secret (version number or the string 'latest'). It is
    /// recommended to use a numeric version for secret environment variables as
    /// any updates to the secret value is not reflected until new instances start.
    #[prost(string, tag = "4")]
    pub version: ::prost::alloc::string::String,
}
/// Configuration for a secret volume. It has the information necessary to fetch
/// the secret value from Secret Manager and make it available as files mounted
/// at the requested paths within the application container. Secret value is not
/// a part of the configuration. Every file system read operation performs a
/// lookup in Secret Manager to retrieve the secret value.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretVolume {
    /// The path within the container to mount the secret volume. For example,
    /// setting the mount_path as `/etc/secrets` mounts the secret value files
    /// under the `/etc/secrets` directory. This directory is also completely
    /// shadowed and unavailable to mount any other secrets.
    ///
    /// Recommended mount paths: /etc/secrets
    /// Restricted mount paths: /cloudsql, /dev/log, /pod, /proc, /var/log
    #[prost(string, tag = "1")]
    pub mount_path: ::prost::alloc::string::String,
    /// Project identifier (preferrably project number but can also be the project
    /// ID) of the project that contains the secret. If not set, it is
    /// populated with the function's project, assuming that the secret exists in
    /// the same project as the function.
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
    /// Name of the secret in Secret Manager (not the full resource name).
    #[prost(string, tag = "3")]
    pub secret: ::prost::alloc::string::String,
    /// List of secret versions to mount for this secret. If empty, the `latest`
    /// version of the secret is made available in a file named after the
    /// secret under the mount point.
    #[prost(message, repeated, tag = "4")]
    pub versions: ::prost::alloc::vec::Vec<secret_volume::SecretVersion>,
}
/// Nested message and enum types in `SecretVolume`.
pub mod secret_volume {
    /// Configuration for a single version.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SecretVersion {
        /// Version of the secret (version number or the string 'latest'). It is
        /// preferable to use `latest` version with secret volumes as secret value
        /// changes are reflected immediately.
        #[prost(string, tag = "1")]
        pub version: ::prost::alloc::string::String,
        /// Relative path of the file under the mount path where the secret value for
        /// this version is fetched and made available. For example, setting the
        /// mount_path as '/etc/secrets' and path as `/secret_foo` mounts the
        /// secret value file at `/etc/secrets/secret_foo`.
        #[prost(string, tag = "2")]
        pub path: ::prost::alloc::string::String,
    }
}
/// Request for the `CreateFunction` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFunctionRequest {
    /// Required. The project and location in which the function should be created,
    /// specified in the format `projects/*/locations/*`
    #[prost(string, tag = "1")]
    pub location: ::prost::alloc::string::String,
    /// Required. Function to be created.
    #[prost(message, optional, tag = "2")]
    pub function: ::core::option::Option<CloudFunction>,
}
/// Request for the `UpdateFunction` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFunctionRequest {
    /// Required. New version of the function.
    #[prost(message, optional, tag = "1")]
    pub function: ::core::option::Option<CloudFunction>,
    /// Required. The list of fields in `CloudFunction` that have to be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request for the `GetFunction` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFunctionRequest {
    /// Required. The name of the function which details should be obtained.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for the `ListFunctions` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFunctionsRequest {
    /// The project and location from which the function should be listed,
    /// specified in the format `projects/*/locations/*`
    /// If you want to list functions in all locations, use "-" in place of a
    /// location. When listing functions in all locations, if one or more
    /// location(s) are unreachable, the response will contain functions from all
    /// reachable locations along with the names of any unreachable locations.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of functions to return per call.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The value returned by the last
    /// `ListFunctionsResponse`; indicates that
    /// this is a continuation of a prior `ListFunctions` call, and that the
    /// system should return the next page of data.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for the `ListFunctions` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFunctionsResponse {
    /// The functions that match the request.
    #[prost(message, repeated, tag = "1")]
    pub functions: ::prost::alloc::vec::Vec<CloudFunction>,
    /// If not empty, indicates that there may be more functions that match
    /// the request; this value should be passed in a new
    /// \[google.cloud.functions.v1.ListFunctionsRequest][google.cloud.functions.v1.ListFunctionsRequest\]
    /// to get more functions.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached. The response does not include any
    /// functions from these locations.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for the `DeleteFunction` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFunctionRequest {
    /// Required. The name of the function which should be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for the `CallFunction` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallFunctionRequest {
    /// Required. The name of the function to be called.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Input to be passed to the function.
    #[prost(string, tag = "2")]
    pub data: ::prost::alloc::string::String,
}
/// Response of `CallFunction` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallFunctionResponse {
    /// Execution id of function invocation.
    #[prost(string, tag = "1")]
    pub execution_id: ::prost::alloc::string::String,
    /// Result populated for successful execution of synchronous function. Will
    /// not be populated if function does not return a result through context.
    #[prost(string, tag = "2")]
    pub result: ::prost::alloc::string::String,
    /// Either system or user-function generated error. Set if execution
    /// was not successful.
    #[prost(string, tag = "3")]
    pub error: ::prost::alloc::string::String,
}
/// Request of `GenerateSourceUploadUrl` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateUploadUrlRequest {
    /// The project and location in which the Google Cloud Storage signed URL
    /// should be generated, specified in the format `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Resource name of a KMS crypto key (managed by the user) used to
    /// encrypt/decrypt function source code objects in staging Cloud Storage
    /// buckets. When you generate an upload url and upload your source code, it
    /// gets copied to a staging Cloud Storage bucket in an internal regional
    /// project. The source code is then copied to a versioned directory in the
    /// sources bucket in the consumer project during the function deployment.
    ///
    /// It must match the pattern
    /// `projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}`.
    ///
    /// The Google Cloud Functions service account
    /// (service-{project_number}@gcf-admin-robot.iam.gserviceaccount.com) must be
    /// granted the role 'Cloud KMS CryptoKey Encrypter/Decrypter
    /// (roles/cloudkms.cryptoKeyEncrypterDecrypter)' on the
    /// Key/KeyRing/Project/Organization (least access preferred). GCF will
    /// delegate access to the Google Storage service account in the internal
    /// project.
    #[prost(string, tag = "2")]
    pub kms_key_name: ::prost::alloc::string::String,
}
/// Response of `GenerateSourceUploadUrl` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateUploadUrlResponse {
    /// The generated Google Cloud Storage signed URL that should be used for a
    /// function source code upload. The uploaded file should be a zip archive
    /// which contains a function.
    #[prost(string, tag = "1")]
    pub upload_url: ::prost::alloc::string::String,
}
/// Request of `GenerateDownloadUrl` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateDownloadUrlRequest {
    /// The name of function for which source code Google Cloud Storage signed
    /// URL should be generated.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The optional version of function. If not set, default, current version
    /// is used.
    #[prost(uint64, tag = "2")]
    pub version_id: u64,
}
/// Response of `GenerateDownloadUrl` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateDownloadUrlResponse {
    /// The generated Google Cloud Storage signed URL that should be used for
    /// function source code download.
    #[prost(string, tag = "1")]
    pub download_url: ::prost::alloc::string::String,
}
/// Describes the current stage of a deployment.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CloudFunctionStatus {
    /// Not specified. Invalid state.
    Unspecified = 0,
    /// Function has been successfully deployed and is serving.
    Active = 1,
    /// Function deployment failed and the function isn’t serving.
    Offline = 2,
    /// Function is being created or updated.
    DeployInProgress = 3,
    /// Function is being deleted.
    DeleteInProgress = 4,
    /// Function deployment failed and the function serving state is undefined.
    /// The function should be updated or deleted to move it out of this state.
    Unknown = 5,
}
impl CloudFunctionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CloudFunctionStatus::Unspecified => "CLOUD_FUNCTION_STATUS_UNSPECIFIED",
            CloudFunctionStatus::Active => "ACTIVE",
            CloudFunctionStatus::Offline => "OFFLINE",
            CloudFunctionStatus::DeployInProgress => "DEPLOY_IN_PROGRESS",
            CloudFunctionStatus::DeleteInProgress => "DELETE_IN_PROGRESS",
            CloudFunctionStatus::Unknown => "UNKNOWN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CLOUD_FUNCTION_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "ACTIVE" => Some(Self::Active),
            "OFFLINE" => Some(Self::Offline),
            "DEPLOY_IN_PROGRESS" => Some(Self::DeployInProgress),
            "DELETE_IN_PROGRESS" => Some(Self::DeleteInProgress),
            "UNKNOWN" => Some(Self::Unknown),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod cloud_functions_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// A service that application uses to manipulate triggers and functions.
    #[derive(Debug, Clone)]
    pub struct CloudFunctionsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CloudFunctionsServiceClient<T>
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
        ) -> CloudFunctionsServiceClient<InterceptedService<T, F>>
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
            CloudFunctionsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Returns a list of functions that belong to the requested project.
        pub async fn list_functions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFunctionsRequest>,
        ) -> Result<tonic::Response<super::ListFunctionsResponse>, tonic::Status> {
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
                "/google.cloud.functions.v1.CloudFunctionsService/ListFunctions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns a function with the given name from the requested project.
        pub async fn get_function(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFunctionRequest>,
        ) -> Result<tonic::Response<super::CloudFunction>, tonic::Status> {
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
                "/google.cloud.functions.v1.CloudFunctionsService/GetFunction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new function. If a function with the given name already exists in
        /// the specified project, the long running operation returns an
        /// `ALREADY_EXISTS` error.
        pub async fn create_function(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFunctionRequest>,
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
                "/google.cloud.functions.v1.CloudFunctionsService/CreateFunction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates existing function.
        pub async fn update_function(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFunctionRequest>,
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
                "/google.cloud.functions.v1.CloudFunctionsService/UpdateFunction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a function with the given name from the specified project. If the
        /// given function is used by some trigger, the trigger is updated to
        /// remove this function.
        pub async fn delete_function(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteFunctionRequest>,
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
                "/google.cloud.functions.v1.CloudFunctionsService/DeleteFunction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Synchronously invokes a deployed Cloud Function. To be used for testing
        /// purposes as very limited traffic is allowed. For more information on
        /// the actual limits, refer to
        /// [Rate Limits](https://cloud.google.com/functions/quotas#rate_limits).
        pub async fn call_function(
            &mut self,
            request: impl tonic::IntoRequest<super::CallFunctionRequest>,
        ) -> Result<tonic::Response<super::CallFunctionResponse>, tonic::Status> {
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
                "/google.cloud.functions.v1.CloudFunctionsService/CallFunction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns a signed URL for uploading a function source code.
        /// For more information about the signed URL usage see:
        /// https://cloud.google.com/storage/docs/access-control/signed-urls.
        /// Once the function source code upload is complete, the used signed
        /// URL should be provided in CreateFunction or UpdateFunction request
        /// as a reference to the function source code.
        ///
        /// When uploading source code to the generated signed URL, please follow
        /// these restrictions:
        ///
        /// * Source file type should be a zip file.
        /// * Source file size should not exceed 100MB limit.
        /// * No credentials should be attached - the signed URLs provide access to the
        ///   target bucket using internal service identity; if credentials were
        ///   attached, the identity from the credentials would be used, but that
        ///   identity does not have permissions to upload files to the URL.
        ///
        /// When making an HTTP PUT request, these two headers must be specified:
        ///
        /// * `content-type: application/zip`
        /// * `x-goog-content-length-range: 0,104857600`
        ///
        /// And this header must NOT be specified:
        ///
        /// * `Authorization: Bearer YOUR_TOKEN`
        pub async fn generate_upload_url(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateUploadUrlRequest>,
        ) -> Result<tonic::Response<super::GenerateUploadUrlResponse>, tonic::Status> {
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
                "/google.cloud.functions.v1.CloudFunctionsService/GenerateUploadUrl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns a signed URL for downloading deployed function source code.
        /// The URL is only valid for a limited period and must be used within
        /// minutes after generation.
        /// For more information about the signed URL usage, see:
        /// https://cloud.google.com/storage/docs/access-control/signed-urls
        pub async fn generate_download_url(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateDownloadUrlRequest>,
        ) -> Result<tonic::Response<super::GenerateDownloadUrlResponse>, tonic::Status> {
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
                "/google.cloud.functions.v1.CloudFunctionsService/GenerateDownloadUrl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the IAM access control policy on the specified function.
        /// Replaces any existing policy.
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::SetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.functions.v1.CloudFunctionsService/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the IAM access control policy for a function.
        /// Returns an empty policy if the function exists and does not have a policy
        /// set.
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::GetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.functions.v1.CloudFunctionsService/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Tests the specified permissions against the IAM access control policy
        /// for a function.
        /// If the function does not exist, this returns an empty set of
        /// permissions, not a NOT_FOUND error.
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::iam::v1::TestIamPermissionsResponse,
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
                "/google.cloud.functions.v1.CloudFunctionsService/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
