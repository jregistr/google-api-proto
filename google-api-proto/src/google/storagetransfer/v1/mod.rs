/// Google service account
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleServiceAccount {
    /// Email address of the service account.
    #[prost(string, tag = "1")]
    pub account_email: ::prost::alloc::string::String,
    /// Unique identifier for the service account.
    #[prost(string, tag = "2")]
    pub subject_id: ::prost::alloc::string::String,
}
/// AWS access key (see
/// [AWS Security
/// Credentials](<https://docs.aws.amazon.com/general/latest/gr/aws-security-credentials.html>)).
///
/// For information on our data retention policy for user credentials, see
/// [User credentials](/storage-transfer/docs/data-retention#user-credentials).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsAccessKey {
    /// Required. AWS access key ID.
    #[prost(string, tag = "1")]
    pub access_key_id: ::prost::alloc::string::String,
    /// Required. AWS secret access key. This field is not returned in RPC
    /// responses.
    #[prost(string, tag = "2")]
    pub secret_access_key: ::prost::alloc::string::String,
}
/// Azure credentials
///
/// For information on our data retention policy for user credentials, see
/// [User credentials](/storage-transfer/docs/data-retention#user-credentials).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureCredentials {
    /// Required. Azure shared access signature (SAS).
    ///
    /// For more information about SAS, see
    /// [Grant limited access to Azure Storage resources using shared access
    /// signatures
    /// (SAS)](<https://docs.microsoft.com/en-us/azure/storage/common/storage-sas-overview>).
    #[prost(string, tag = "2")]
    pub sas_token: ::prost::alloc::string::String,
}
/// Conditions that determine which objects are transferred. Applies only
/// to Cloud Data Sources such as S3, Azure, and Cloud Storage.
///
/// The "last modification time" refers to the time of the
/// last change to the object's content or metadata — specifically, this is
/// the `updated` property of Cloud Storage objects, the `LastModified` field
/// of S3 objects, and the `Last-Modified` header of Azure blobs.
///
/// Transfers with a \[PosixFilesystem][google.storagetransfer.v1.PosixFilesystem\] source or destination don't support
/// `ObjectConditions`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectConditions {
    /// Ensures that objects are not transferred until a specific minimum time
    /// has elapsed after the "last modification time". When a
    /// \[TransferOperation][google.storagetransfer.v1.TransferOperation\] begins, objects with a "last modification time" are
    /// transferred only if the elapsed time between the
    /// \[start_time][google.storagetransfer.v1.TransferOperation.start_time\] of the `TransferOperation`
    /// and the "last modification time" of the object is equal to or
    /// greater than the value of min_time_elapsed_since_last_modification`.
    /// Objects that do not have a "last modification time" are also transferred.
    #[prost(message, optional, tag = "1")]
    pub min_time_elapsed_since_last_modification: ::core::option::Option<
        ::prost_types::Duration,
    >,
    /// Ensures that objects are not transferred if a specific maximum time
    /// has elapsed since the "last modification time".
    /// When a \[TransferOperation][google.storagetransfer.v1.TransferOperation\] begins, objects with a
    /// "last modification time" are transferred only if the elapsed time
    /// between the \[start_time][google.storagetransfer.v1.TransferOperation.start_time\] of the
    /// `TransferOperation`and the "last modification time" of the object
    ///   is less than the value of max_time_elapsed_since_last_modification`.
    /// Objects that do not have a "last modification time" are also transferred.
    #[prost(message, optional, tag = "2")]
    pub max_time_elapsed_since_last_modification: ::core::option::Option<
        ::prost_types::Duration,
    >,
    /// If you specify `include_prefixes`, Storage Transfer Service uses the items
    /// in the `include_prefixes` array to determine which objects to include in a
    /// transfer. Objects must start with one of the matching `include_prefixes`
    /// for inclusion in the transfer. If \[exclude_prefixes][google.storagetransfer.v1.ObjectConditions.exclude_prefixes\] is specified,
    /// objects must not start with any of the `exclude_prefixes` specified for
    /// inclusion in the transfer.
    ///
    /// The following are requirements of `include_prefixes`:
    ///
    ///    * Each include-prefix can contain any sequence of Unicode characters, to
    ///      a max length of 1024 bytes when UTF8-encoded, and must not contain
    ///      Carriage Return or Line Feed characters.  Wildcard matching and regular
    ///      expression matching are not supported.
    ///
    ///    * Each include-prefix must omit the leading slash. For example, to
    ///      include the object `s3://my-aws-bucket/logs/y=2015/requests.gz`,
    ///      specify the include-prefix as `logs/y=2015/requests.gz`.
    ///
    ///    * None of the include-prefix values can be empty, if specified.
    ///
    ///    * Each include-prefix must include a distinct portion of the object
    ///      namespace. No include-prefix may be a prefix of another
    ///      include-prefix.
    ///
    /// The max size of `include_prefixes` is 1000.
    ///
    /// For more information, see [Filtering objects from
    /// transfers](/storage-transfer/docs/filtering-objects-from-transfers).
    #[prost(string, repeated, tag = "3")]
    pub include_prefixes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// If you specify `exclude_prefixes`, Storage Transfer Service uses the items
    /// in the `exclude_prefixes` array to determine which objects to exclude from
    /// a transfer. Objects must not start with one of the matching
    /// `exclude_prefixes` for inclusion in a transfer.
    ///
    /// The following are requirements of `exclude_prefixes`:
    ///
    ///    * Each exclude-prefix can contain any sequence of Unicode characters, to
    ///      a max length of 1024 bytes when UTF8-encoded, and must not contain
    ///      Carriage Return or Line Feed characters.  Wildcard matching and regular
    ///      expression matching are not supported.
    ///
    ///    * Each exclude-prefix must omit the leading slash. For example, to
    ///      exclude the object `s3://my-aws-bucket/logs/y=2015/requests.gz`,
    ///      specify the exclude-prefix as `logs/y=2015/requests.gz`.
    ///
    ///    * None of the exclude-prefix values can be empty, if specified.
    ///
    ///    * Each exclude-prefix must exclude a distinct portion of the object
    ///      namespace. No exclude-prefix may be a prefix of another
    ///      exclude-prefix.
    ///
    ///    * If \[include_prefixes][google.storagetransfer.v1.ObjectConditions.include_prefixes\] is specified, then each exclude-prefix must
    ///    start with the value of a path explicitly included by `include_prefixes`.
    ///
    /// The max size of `exclude_prefixes` is 1000.
    ///
    /// For more information, see [Filtering objects from
    /// transfers](/storage-transfer/docs/filtering-objects-from-transfers).
    #[prost(string, repeated, tag = "4")]
    pub exclude_prefixes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// If specified, only objects with a "last modification time" on or after
    /// this timestamp and objects that don't have a "last modification time" are
    /// transferred.
    ///
    /// The `last_modified_since` and `last_modified_before` fields can be used
    /// together for chunked data processing. For example, consider a script that
    /// processes each day's worth of data at a time. For that you'd set each
    /// of the fields as follows:
    ///
    /// *  `last_modified_since` to the start of the day
    ///
    /// *  `last_modified_before` to the end of the day
    #[prost(message, optional, tag = "5")]
    pub last_modified_since: ::core::option::Option<::prost_types::Timestamp>,
    /// If specified, only objects with a "last modification time" before this
    /// timestamp and objects that don't have a "last modification time" are
    /// transferred.
    #[prost(message, optional, tag = "6")]
    pub last_modified_before: ::core::option::Option<::prost_types::Timestamp>,
}
/// In a GcsData resource, an object's name is the Cloud Storage object's
/// name and its "last modification time" refers to the object's `updated`
/// property of Cloud Storage objects, which changes when the content or the
/// metadata of the object is updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsData {
    /// Required. Cloud Storage bucket name. Must meet
    /// [Bucket Name Requirements](/storage/docs/naming#requirements).
    #[prost(string, tag = "1")]
    pub bucket_name: ::prost::alloc::string::String,
    /// Root path to transfer objects.
    ///
    /// Must be an empty string or full path name that ends with a '/'. This field
    /// is treated as an object prefix. As such, it should generally not begin with
    /// a '/'.
    ///
    /// The root path value must meet
    /// [Object Name Requirements](/storage/docs/naming#objectnames).
    #[prost(string, tag = "3")]
    pub path: ::prost::alloc::string::String,
}
/// An AwsS3Data resource can be a data source, but not a data sink.
/// In an AwsS3Data resource, an object's name is the S3 object's key name.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsS3Data {
    /// Required. S3 Bucket name (see
    /// [Creating a
    /// bucket](<https://docs.aws.amazon.com/AmazonS3/latest/dev/create-bucket-get-location-example.html>)).
    #[prost(string, tag = "1")]
    pub bucket_name: ::prost::alloc::string::String,
    /// Input only. AWS access key used to sign the API requests to the AWS S3 bucket.
    /// Permissions on the bucket must be granted to the access ID of the AWS
    /// access key.
    ///
    /// For information on our data retention policy for user credentials, see
    /// [User credentials](/storage-transfer/docs/data-retention#user-credentials).
    #[prost(message, optional, tag = "2")]
    pub aws_access_key: ::core::option::Option<AwsAccessKey>,
    /// Root path to transfer objects.
    ///
    /// Must be an empty string or full path name that ends with a '/'. This field
    /// is treated as an object prefix. As such, it should generally not begin with
    /// a '/'.
    #[prost(string, tag = "3")]
    pub path: ::prost::alloc::string::String,
    /// The Amazon Resource Name (ARN) of the role to support temporary
    /// credentials via `AssumeRoleWithWebIdentity`. For more information about
    /// ARNs, see [IAM
    /// ARNs](<https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns>).
    ///
    /// When a role ARN is provided, Transfer Service fetches temporary
    /// credentials for the session using a `AssumeRoleWithWebIdentity` call for
    /// the provided role using the \[GoogleServiceAccount][google.storagetransfer.v1.GoogleServiceAccount\] for this project.
    #[prost(string, tag = "4")]
    pub role_arn: ::prost::alloc::string::String,
}
/// An AzureBlobStorageData resource can be a data source, but not a data sink.
/// An AzureBlobStorageData resource represents one Azure container. The storage
/// account determines the [Azure
/// endpoint](<https://docs.microsoft.com/en-us/azure/storage/common/storage-create-storage-account#storage-account-endpoints>).
/// In an AzureBlobStorageData resource, a blobs's name is the [Azure Blob
/// Storage blob's key
/// name](<https://docs.microsoft.com/en-us/rest/api/storageservices/naming-and-referencing-containers--blobs--and-metadata#blob-names>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureBlobStorageData {
    /// Required. The name of the Azure Storage account.
    #[prost(string, tag = "1")]
    pub storage_account: ::prost::alloc::string::String,
    /// Required. Input only. Credentials used to authenticate API requests to Azure.
    ///
    /// For information on our data retention policy for user credentials, see
    /// [User credentials](/storage-transfer/docs/data-retention#user-credentials).
    #[prost(message, optional, tag = "2")]
    pub azure_credentials: ::core::option::Option<AzureCredentials>,
    /// Required. The container to transfer from the Azure Storage account.
    #[prost(string, tag = "4")]
    pub container: ::prost::alloc::string::String,
    /// Root path to transfer objects.
    ///
    /// Must be an empty string or full path name that ends with a '/'. This field
    /// is treated as an object prefix. As such, it should generally not begin with
    /// a '/'.
    #[prost(string, tag = "5")]
    pub path: ::prost::alloc::string::String,
}
/// An HttpData resource specifies a list of objects on the web to be transferred
/// over HTTP.  The information of the objects to be transferred is contained in
/// a file referenced by a URL. The first line in the file must be
/// `"TsvHttpData-1.0"`, which specifies the format of the file.  Subsequent
/// lines specify the information of the list of objects, one object per list
/// entry. Each entry has the following tab-delimited fields:
///
/// * **HTTP URL** — The location of the object.
///
/// * **Length** — The size of the object in bytes.
///
/// * **MD5** — The base64-encoded MD5 hash of the object.
///
/// For an example of a valid TSV file, see
/// [Transferring data from
/// URLs](<https://cloud.google.com/storage-transfer/docs/create-url-list>).
///
/// When transferring data based on a URL list, keep the following in mind:
///
/// * When an object located at `http(s)://hostname:port/<URL-path>` is
/// transferred to a data sink, the name of the object at the data sink is
/// `<hostname>/<URL-path>`.
///
/// * If the specified size of an object does not match the actual size of the
/// object fetched, the object is not transferred.
///
/// * If the specified MD5 does not match the MD5 computed from the transferred
/// bytes, the object transfer fails.
///
/// * Ensure that each URL you specify is publicly accessible. For
/// example, in Cloud Storage you can
/// [share an object publicly]
/// (/storage/docs/cloud-console#_sharingdata) and get a link to it.
///
/// * Storage Transfer Service obeys `robots.txt` rules and requires the source
/// HTTP server to support `Range` requests and to return a `Content-Length`
/// header in each response.
///
/// * \[ObjectConditions][google.storagetransfer.v1.ObjectConditions\] have no effect when filtering objects to transfer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpData {
    /// Required. The URL that points to the file that stores the object list
    /// entries. This file must allow public access.  Currently, only URLs with
    /// HTTP and HTTPS schemes are supported.
    #[prost(string, tag = "1")]
    pub list_url: ::prost::alloc::string::String,
}
/// A POSIX filesystem resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PosixFilesystem {
    /// Root directory path to the filesystem.
    #[prost(string, tag = "1")]
    pub root_directory: ::prost::alloc::string::String,
}
/// An AwsS3CompatibleData resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsS3CompatibleData {
    /// Required. Specifies the name of the bucket.
    #[prost(string, tag = "1")]
    pub bucket_name: ::prost::alloc::string::String,
    /// Specifies the root path to transfer objects.
    ///
    /// Must be an empty string or full path name that ends with a '/'. This
    /// field is treated as an object prefix. As such, it should generally not
    /// begin with a '/'.
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
    /// Required. Specifies the endpoint of the storage service.
    #[prost(string, tag = "3")]
    pub endpoint: ::prost::alloc::string::String,
    /// Specifies the region to sign requests with. This can be left blank if
    /// requests should be signed with an empty region.
    #[prost(string, tag = "5")]
    pub region: ::prost::alloc::string::String,
    /// Specifies the metadata of the S3 compatible data provider. Each provider
    /// may contain some attributes that do not apply to all S3-compatible data
    /// providers. When not specified, S3CompatibleMetadata is used by default.
    #[prost(oneof = "aws_s3_compatible_data::DataProvider", tags = "4")]
    pub data_provider: ::core::option::Option<aws_s3_compatible_data::DataProvider>,
}
/// Nested message and enum types in `AwsS3CompatibleData`.
pub mod aws_s3_compatible_data {
    /// Specifies the metadata of the S3 compatible data provider. Each provider
    /// may contain some attributes that do not apply to all S3-compatible data
    /// providers. When not specified, S3CompatibleMetadata is used by default.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DataProvider {
        /// A S3 compatible metadata.
        #[prost(message, tag = "4")]
        S3Metadata(super::S3CompatibleMetadata),
    }
}
/// S3CompatibleMetadata contains the metadata fields that apply to the basic
/// types of S3-compatible data providers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S3CompatibleMetadata {
    /// Specifies the authentication and authorization method used by the storage
    /// service. When not specified, Transfer Service will attempt to determine
    /// right auth method to use.
    #[prost(enumeration = "s3_compatible_metadata::AuthMethod", tag = "1")]
    pub auth_method: i32,
    /// Specifies the API request model used to call the storage service. When not
    /// specified, the default value of RequestModel
    /// REQUEST_MODEL_VIRTUAL_HOSTED_STYLE is used.
    #[prost(enumeration = "s3_compatible_metadata::RequestModel", tag = "2")]
    pub request_model: i32,
    /// Specifies the network protocol of the agent. When not specified, the
    /// default value of NetworkProtocol NETWORK_PROTOCOL_HTTPS is used.
    #[prost(enumeration = "s3_compatible_metadata::NetworkProtocol", tag = "3")]
    pub protocol: i32,
    /// The Listing API to use for discovering objects. When not specified,
    /// Transfer Service will attempt to determine the right API to use.
    #[prost(enumeration = "s3_compatible_metadata::ListApi", tag = "4")]
    pub list_api: i32,
}
/// Nested message and enum types in `S3CompatibleMetadata`.
pub mod s3_compatible_metadata {
    /// The authentication and authorization method used by the storage service.
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
    pub enum AuthMethod {
        /// AuthMethod is not specified.
        Unspecified = 0,
        /// Auth requests with AWS SigV4.
        AwsSignatureV4 = 1,
        /// Auth requests with AWS SigV2.
        AwsSignatureV2 = 2,
    }
    impl AuthMethod {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AuthMethod::Unspecified => "AUTH_METHOD_UNSPECIFIED",
                AuthMethod::AwsSignatureV4 => "AUTH_METHOD_AWS_SIGNATURE_V4",
                AuthMethod::AwsSignatureV2 => "AUTH_METHOD_AWS_SIGNATURE_V2",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "AUTH_METHOD_UNSPECIFIED" => Some(Self::Unspecified),
                "AUTH_METHOD_AWS_SIGNATURE_V4" => Some(Self::AwsSignatureV4),
                "AUTH_METHOD_AWS_SIGNATURE_V2" => Some(Self::AwsSignatureV2),
                _ => None,
            }
        }
    }
    /// The request model of the API.
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
    pub enum RequestModel {
        /// RequestModel is not specified.
        Unspecified = 0,
        /// Perform requests using Virtual Hosted Style.
        /// Example: <https://bucket-name.s3.region.amazonaws.com/key-name>
        VirtualHostedStyle = 1,
        /// Perform requests using Path Style.
        /// Example: <https://s3.region.amazonaws.com/bucket-name/key-name>
        PathStyle = 2,
    }
    impl RequestModel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RequestModel::Unspecified => "REQUEST_MODEL_UNSPECIFIED",
                RequestModel::VirtualHostedStyle => "REQUEST_MODEL_VIRTUAL_HOSTED_STYLE",
                RequestModel::PathStyle => "REQUEST_MODEL_PATH_STYLE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REQUEST_MODEL_UNSPECIFIED" => Some(Self::Unspecified),
                "REQUEST_MODEL_VIRTUAL_HOSTED_STYLE" => Some(Self::VirtualHostedStyle),
                "REQUEST_MODEL_PATH_STYLE" => Some(Self::PathStyle),
                _ => None,
            }
        }
    }
    /// The agent network protocol to access the storage service.
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
    pub enum NetworkProtocol {
        /// NetworkProtocol is not specified.
        Unspecified = 0,
        /// Perform requests using HTTPS.
        Https = 1,
        /// Not recommended: This sends data in clear-text. This is only
        /// appropriate within a closed network or for publicly available data.
        /// Perform requests using HTTP.
        Http = 2,
    }
    impl NetworkProtocol {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                NetworkProtocol::Unspecified => "NETWORK_PROTOCOL_UNSPECIFIED",
                NetworkProtocol::Https => "NETWORK_PROTOCOL_HTTPS",
                NetworkProtocol::Http => "NETWORK_PROTOCOL_HTTP",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NETWORK_PROTOCOL_UNSPECIFIED" => Some(Self::Unspecified),
                "NETWORK_PROTOCOL_HTTPS" => Some(Self::Https),
                "NETWORK_PROTOCOL_HTTP" => Some(Self::Http),
                _ => None,
            }
        }
    }
    /// The Listing API to use for discovering objects.
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
    pub enum ListApi {
        /// ListApi is not specified.
        Unspecified = 0,
        /// Perform listing using ListObjectsV2 API.
        ListObjectsV2 = 1,
        /// Legacy ListObjects API.
        ListObjects = 2,
    }
    impl ListApi {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListApi::Unspecified => "LIST_API_UNSPECIFIED",
                ListApi::ListObjectsV2 => "LIST_OBJECTS_V2",
                ListApi::ListObjects => "LIST_OBJECTS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LIST_API_UNSPECIFIED" => Some(Self::Unspecified),
                "LIST_OBJECTS_V2" => Some(Self::ListObjectsV2),
                "LIST_OBJECTS" => Some(Self::ListObjects),
                _ => None,
            }
        }
    }
}
/// Represents an On-Premises Agent pool.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AgentPool {
    /// Required. Specifies a unique string that identifies the agent pool.
    ///
    /// Format: `projects/{project_id}/agentPools/{agent_pool_id}`
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// Specifies the client-specified AgentPool description.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. Specifies the state of the AgentPool.
    #[prost(enumeration = "agent_pool::State", tag = "4")]
    pub state: i32,
    /// Specifies the bandwidth limit details. If this field is unspecified, the
    /// default value is set as 'No Limit'.
    #[prost(message, optional, tag = "5")]
    pub bandwidth_limit: ::core::option::Option<agent_pool::BandwidthLimit>,
}
/// Nested message and enum types in `AgentPool`.
pub mod agent_pool {
    /// Specifies a bandwidth limit for an agent pool.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BandwidthLimit {
        /// Bandwidth rate in megabytes per second, distributed across all the agents
        /// in the pool.
        #[prost(int64, tag = "1")]
        pub limit_mbps: i64,
    }
    /// The state of an AgentPool.
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
        /// This is an initialization state. During this stage, the resources such as
        /// Pub/Sub topics are allocated for the AgentPool.
        Creating = 1,
        /// Determines that the AgentPool is created for use. At this state, Agents
        /// can join the AgentPool and participate in the transfer jobs in that pool.
        Created = 2,
        /// Determines that the AgentPool deletion has been initiated, and all the
        /// resources are scheduled to be cleaned up and freed.
        Deleting = 3,
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
                State::Deleting => "DELETING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "CREATED" => Some(Self::Created),
                "DELETING" => Some(Self::Deleting),
                _ => None,
            }
        }
    }
}
/// TransferOptions define the actions to be performed on objects in a transfer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferOptions {
    /// When to overwrite objects that already exist in the sink. The default is
    /// that only objects that are different from the source are ovewritten. If
    /// true, all objects in the sink whose name matches an object in the source
    /// are overwritten with the source object.
    #[prost(bool, tag = "1")]
    pub overwrite_objects_already_existing_in_sink: bool,
    /// Whether objects that exist only in the sink should be deleted.
    ///
    /// **Note:** This option and \[delete_objects_from_source_after_transfer][google.storagetransfer.v1.TransferOptions.delete_objects_from_source_after_transfer\] are
    /// mutually exclusive.
    #[prost(bool, tag = "2")]
    pub delete_objects_unique_in_sink: bool,
    /// Whether objects should be deleted from the source after they are
    /// transferred to the sink.
    ///
    /// **Note:** This option and \[delete_objects_unique_in_sink][google.storagetransfer.v1.TransferOptions.delete_objects_unique_in_sink\] are mutually
    /// exclusive.
    #[prost(bool, tag = "3")]
    pub delete_objects_from_source_after_transfer: bool,
    /// When to overwrite objects that already exist in the sink. If not set,
    /// overwrite behavior is determined by
    /// \[overwrite_objects_already_existing_in_sink][google.storagetransfer.v1.TransferOptions.overwrite_objects_already_existing_in_sink\].
    #[prost(enumeration = "transfer_options::OverwriteWhen", tag = "4")]
    pub overwrite_when: i32,
    /// Represents the selected metadata options for a transfer job.
    #[prost(message, optional, tag = "5")]
    pub metadata_options: ::core::option::Option<MetadataOptions>,
}
/// Nested message and enum types in `TransferOptions`.
pub mod transfer_options {
    /// Specifies when to overwrite an object in the sink when an object with
    /// matching name is found in the source.
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
    pub enum OverwriteWhen {
        /// Overwrite behavior is unspecified.
        Unspecified = 0,
        /// Overwrites destination objects with the source objects, only if the
        /// objects have the same name but different HTTP ETags or checksum values.
        Different = 1,
        /// Never overwrites a destination object if a source object has the
        /// same name. In this case, the source object is not transferred.
        Never = 2,
        /// Always overwrite the destination object with the source object, even if
        /// the HTTP Etags or checksum values are the same.
        Always = 3,
    }
    impl OverwriteWhen {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                OverwriteWhen::Unspecified => "OVERWRITE_WHEN_UNSPECIFIED",
                OverwriteWhen::Different => "DIFFERENT",
                OverwriteWhen::Never => "NEVER",
                OverwriteWhen::Always => "ALWAYS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OVERWRITE_WHEN_UNSPECIFIED" => Some(Self::Unspecified),
                "DIFFERENT" => Some(Self::Different),
                "NEVER" => Some(Self::Never),
                "ALWAYS" => Some(Self::Always),
                _ => None,
            }
        }
    }
}
/// Configuration for running a transfer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferSpec {
    /// Only objects that satisfy these object conditions are included in the set
    /// of data source and data sink objects.  Object conditions based on
    /// objects' "last modification time" do not exclude objects in a data sink.
    #[prost(message, optional, tag = "5")]
    pub object_conditions: ::core::option::Option<ObjectConditions>,
    /// If the option
    /// \[delete_objects_unique_in_sink][google.storagetransfer.v1.TransferOptions.delete_objects_unique_in_sink\]
    /// is `true` and time-based object conditions such as 'last modification time'
    /// are specified, the request fails with an
    /// \[INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT\] error.
    #[prost(message, optional, tag = "6")]
    pub transfer_options: ::core::option::Option<TransferOptions>,
    /// A manifest file provides a list of objects to be transferred from the data
    /// source. This field points to the location of the manifest file.
    /// Otherwise, the entire source bucket is used. ObjectConditions still apply.
    #[prost(message, optional, tag = "15")]
    pub transfer_manifest: ::core::option::Option<TransferManifest>,
    /// Specifies the agent pool name associated with the posix data source. When
    /// unspecified, the default name is used.
    #[prost(string, tag = "17")]
    pub source_agent_pool_name: ::prost::alloc::string::String,
    /// Specifies the agent pool name associated with the posix data sink. When
    /// unspecified, the default name is used.
    #[prost(string, tag = "18")]
    pub sink_agent_pool_name: ::prost::alloc::string::String,
    /// The write sink for the data.
    #[prost(oneof = "transfer_spec::DataSink", tags = "4, 13")]
    pub data_sink: ::core::option::Option<transfer_spec::DataSink>,
    /// The read source of the data.
    #[prost(oneof = "transfer_spec::DataSource", tags = "1, 2, 3, 14, 8, 19")]
    pub data_source: ::core::option::Option<transfer_spec::DataSource>,
    /// Represents a supported data container type which is required for transfer
    /// jobs which needs a data source, a data sink and an intermediate location to
    /// transfer data through. This is validated on TransferJob creation.
    #[prost(oneof = "transfer_spec::IntermediateDataLocation", tags = "16")]
    pub intermediate_data_location: ::core::option::Option<
        transfer_spec::IntermediateDataLocation,
    >,
}
/// Nested message and enum types in `TransferSpec`.
pub mod transfer_spec {
    /// The write sink for the data.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DataSink {
        /// A Cloud Storage data sink.
        #[prost(message, tag = "4")]
        GcsDataSink(super::GcsData),
        /// A POSIX Filesystem data sink.
        #[prost(message, tag = "13")]
        PosixDataSink(super::PosixFilesystem),
    }
    /// The read source of the data.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DataSource {
        /// A Cloud Storage data source.
        #[prost(message, tag = "1")]
        GcsDataSource(super::GcsData),
        /// An AWS S3 data source.
        #[prost(message, tag = "2")]
        AwsS3DataSource(super::AwsS3Data),
        /// An HTTP URL data source.
        #[prost(message, tag = "3")]
        HttpDataSource(super::HttpData),
        /// A POSIX Filesystem data source.
        #[prost(message, tag = "14")]
        PosixDataSource(super::PosixFilesystem),
        /// An Azure Blob Storage data source.
        #[prost(message, tag = "8")]
        AzureBlobStorageDataSource(super::AzureBlobStorageData),
        /// An AWS S3 compatible data source.
        #[prost(message, tag = "19")]
        AwsS3CompatibleDataSource(super::AwsS3CompatibleData),
    }
    /// Represents a supported data container type which is required for transfer
    /// jobs which needs a data source, a data sink and an intermediate location to
    /// transfer data through. This is validated on TransferJob creation.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum IntermediateDataLocation {
        /// Cloud Storage intermediate data location.
        #[prost(message, tag = "16")]
        GcsIntermediateDataLocation(super::GcsData),
    }
}
/// Specifies the metadata options for running a transfer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetadataOptions {
    /// Specifies how symlinks should be handled by the transfer. By default,
    /// symlinks are not preserved. Only applicable to transfers involving
    /// POSIX file systems, and ignored for other transfers.
    #[prost(enumeration = "metadata_options::Symlink", tag = "1")]
    pub symlink: i32,
    /// Specifies how each file's mode attribute should be handled by the transfer.
    /// By default, mode is not preserved. Only applicable to transfers involving
    /// POSIX file systems, and ignored for other transfers.
    #[prost(enumeration = "metadata_options::Mode", tag = "2")]
    pub mode: i32,
    /// Specifies how each file's POSIX group ID (GID) attribute should be handled
    /// by the transfer. By default, GID is not preserved. Only applicable to
    /// transfers involving POSIX file systems, and ignored for other transfers.
    #[prost(enumeration = "metadata_options::Gid", tag = "3")]
    pub gid: i32,
    /// Specifies how each file's POSIX user ID (UID) attribute should be handled
    /// by the transfer. By default, UID is not preserved. Only applicable to
    /// transfers involving POSIX file systems, and ignored for other transfers.
    #[prost(enumeration = "metadata_options::Uid", tag = "4")]
    pub uid: i32,
    /// Specifies how each object's ACLs should be preserved for transfers between
    /// Google Cloud Storage buckets. If unspecified, the default behavior is the
    /// same as ACL_DESTINATION_BUCKET_DEFAULT.
    #[prost(enumeration = "metadata_options::Acl", tag = "5")]
    pub acl: i32,
    /// Specifies the storage class to set on objects being transferred to Google
    /// Cloud Storage buckets.  If unspecified, the default behavior is the same as
    /// \[STORAGE_CLASS_DESTINATION_BUCKET_DEFAULT][google.storagetransfer.v1.MetadataOptions.StorageClass.STORAGE_CLASS_DESTINATION_BUCKET_DEFAULT\].
    #[prost(enumeration = "metadata_options::StorageClass", tag = "6")]
    pub storage_class: i32,
    /// Specifies how each object's temporary hold status should be preserved for
    /// transfers between Google Cloud Storage buckets.  If unspecified, the
    /// default behavior is the same as
    /// \[TEMPORARY_HOLD_PRESERVE][google.storagetransfer.v1.MetadataOptions.TemporaryHold.TEMPORARY_HOLD_PRESERVE\].
    #[prost(enumeration = "metadata_options::TemporaryHold", tag = "7")]
    pub temporary_hold: i32,
    /// Specifies how each object's Cloud KMS customer-managed encryption key
    /// (CMEK) is preserved for transfers between Google Cloud Storage buckets.  If
    /// unspecified, the default behavior is the same as
    /// \[KMS_KEY_DESTINATION_BUCKET_DEFAULT][google.storagetransfer.v1.MetadataOptions.KmsKey.KMS_KEY_DESTINATION_BUCKET_DEFAULT\].
    #[prost(enumeration = "metadata_options::KmsKey", tag = "8")]
    pub kms_key: i32,
    /// Specifies how each object's `timeCreated` metadata is preserved for
    /// transfers between Google Cloud Storage buckets.  If unspecified, the
    /// default behavior is the same as
    /// \[TIME_CREATED_SKIP][google.storagetransfer.v1.MetadataOptions.TimeCreated.TIME_CREATED_SKIP\].
    #[prost(enumeration = "metadata_options::TimeCreated", tag = "9")]
    pub time_created: i32,
}
/// Nested message and enum types in `MetadataOptions`.
pub mod metadata_options {
    /// Whether symlinks should be skipped or preserved during a transfer job.
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
    pub enum Symlink {
        /// Symlink behavior is unspecified.
        Unspecified = 0,
        /// Do not preserve symlinks during a transfer job.
        Skip = 1,
        /// Preserve symlinks during a transfer job.
        Preserve = 2,
    }
    impl Symlink {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Symlink::Unspecified => "SYMLINK_UNSPECIFIED",
                Symlink::Skip => "SYMLINK_SKIP",
                Symlink::Preserve => "SYMLINK_PRESERVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SYMLINK_UNSPECIFIED" => Some(Self::Unspecified),
                "SYMLINK_SKIP" => Some(Self::Skip),
                "SYMLINK_PRESERVE" => Some(Self::Preserve),
                _ => None,
            }
        }
    }
    /// Options for handling file mode attribute.
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
    pub enum Mode {
        /// Mode behavior is unspecified.
        Unspecified = 0,
        /// Do not preserve mode during a transfer job.
        Skip = 1,
        /// Preserve mode during a transfer job.
        Preserve = 2,
    }
    impl Mode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Mode::Unspecified => "MODE_UNSPECIFIED",
                Mode::Skip => "MODE_SKIP",
                Mode::Preserve => "MODE_PRESERVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "MODE_SKIP" => Some(Self::Skip),
                "MODE_PRESERVE" => Some(Self::Preserve),
                _ => None,
            }
        }
    }
    /// Options for handling file GID attribute.
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
    pub enum Gid {
        /// GID behavior is unspecified.
        Unspecified = 0,
        /// Do not preserve GID during a transfer job.
        Skip = 1,
        /// Preserve GID during a transfer job.
        Number = 2,
    }
    impl Gid {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Gid::Unspecified => "GID_UNSPECIFIED",
                Gid::Skip => "GID_SKIP",
                Gid::Number => "GID_NUMBER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "GID_UNSPECIFIED" => Some(Self::Unspecified),
                "GID_SKIP" => Some(Self::Skip),
                "GID_NUMBER" => Some(Self::Number),
                _ => None,
            }
        }
    }
    /// Options for handling file UID attribute.
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
    pub enum Uid {
        /// UID behavior is unspecified.
        Unspecified = 0,
        /// Do not preserve UID during a transfer job.
        Skip = 1,
        /// Preserve UID during a transfer job.
        Number = 2,
    }
    impl Uid {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Uid::Unspecified => "UID_UNSPECIFIED",
                Uid::Skip => "UID_SKIP",
                Uid::Number => "UID_NUMBER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UID_UNSPECIFIED" => Some(Self::Unspecified),
                "UID_SKIP" => Some(Self::Skip),
                "UID_NUMBER" => Some(Self::Number),
                _ => None,
            }
        }
    }
    /// Options for handling Cloud Storage object ACLs.
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
    pub enum Acl {
        /// ACL behavior is unspecified.
        Unspecified = 0,
        /// Use the destination bucket's default object ACLS, if applicable.
        DestinationBucketDefault = 1,
        /// Preserve the object's original ACLs. This requires the service account
        /// to have `storage.objects.getIamPolicy` permission for the source object.
        /// [Uniform bucket-level
        /// access](<https://cloud.google.com/storage/docs/uniform-bucket-level-access>)
        /// must not be enabled on either the source or destination buckets.
        Preserve = 2,
    }
    impl Acl {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Acl::Unspecified => "ACL_UNSPECIFIED",
                Acl::DestinationBucketDefault => "ACL_DESTINATION_BUCKET_DEFAULT",
                Acl::Preserve => "ACL_PRESERVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ACL_UNSPECIFIED" => Some(Self::Unspecified),
                "ACL_DESTINATION_BUCKET_DEFAULT" => Some(Self::DestinationBucketDefault),
                "ACL_PRESERVE" => Some(Self::Preserve),
                _ => None,
            }
        }
    }
    /// Options for handling Google Cloud Storage object storage class.
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
    pub enum StorageClass {
        /// Storage class behavior is unspecified.
        Unspecified = 0,
        /// Use the destination bucket's default storage class.
        DestinationBucketDefault = 1,
        /// Preserve the object's original storage class. This is only supported for
        /// transfers from Google Cloud Storage buckets.
        Preserve = 2,
        /// Set the storage class to STANDARD.
        Standard = 3,
        /// Set the storage class to NEARLINE.
        Nearline = 4,
        /// Set the storage class to COLDLINE.
        Coldline = 5,
        /// Set the storage class to ARCHIVE.
        Archive = 6,
    }
    impl StorageClass {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                StorageClass::Unspecified => "STORAGE_CLASS_UNSPECIFIED",
                StorageClass::DestinationBucketDefault => {
                    "STORAGE_CLASS_DESTINATION_BUCKET_DEFAULT"
                }
                StorageClass::Preserve => "STORAGE_CLASS_PRESERVE",
                StorageClass::Standard => "STORAGE_CLASS_STANDARD",
                StorageClass::Nearline => "STORAGE_CLASS_NEARLINE",
                StorageClass::Coldline => "STORAGE_CLASS_COLDLINE",
                StorageClass::Archive => "STORAGE_CLASS_ARCHIVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STORAGE_CLASS_UNSPECIFIED" => Some(Self::Unspecified),
                "STORAGE_CLASS_DESTINATION_BUCKET_DEFAULT" => {
                    Some(Self::DestinationBucketDefault)
                }
                "STORAGE_CLASS_PRESERVE" => Some(Self::Preserve),
                "STORAGE_CLASS_STANDARD" => Some(Self::Standard),
                "STORAGE_CLASS_NEARLINE" => Some(Self::Nearline),
                "STORAGE_CLASS_COLDLINE" => Some(Self::Coldline),
                "STORAGE_CLASS_ARCHIVE" => Some(Self::Archive),
                _ => None,
            }
        }
    }
    /// Options for handling temporary holds for Google Cloud Storage objects.
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
    pub enum TemporaryHold {
        /// Temporary hold behavior is unspecified.
        Unspecified = 0,
        /// Do not set a temporary hold on the destination object.
        Skip = 1,
        /// Preserve the object's original temporary hold status.
        Preserve = 2,
    }
    impl TemporaryHold {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TemporaryHold::Unspecified => "TEMPORARY_HOLD_UNSPECIFIED",
                TemporaryHold::Skip => "TEMPORARY_HOLD_SKIP",
                TemporaryHold::Preserve => "TEMPORARY_HOLD_PRESERVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TEMPORARY_HOLD_UNSPECIFIED" => Some(Self::Unspecified),
                "TEMPORARY_HOLD_SKIP" => Some(Self::Skip),
                "TEMPORARY_HOLD_PRESERVE" => Some(Self::Preserve),
                _ => None,
            }
        }
    }
    /// Options for handling the KmsKey setting for Google Cloud Storage objects.
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
    pub enum KmsKey {
        /// KmsKey behavior is unspecified.
        Unspecified = 0,
        /// Use the destination bucket's default encryption settings.
        DestinationBucketDefault = 1,
        /// Preserve the object's original Cloud KMS customer-managed encryption key
        /// (CMEK) if present. Objects that do not use a Cloud KMS encryption key
        /// will be encrypted using the destination bucket's encryption settings.
        Preserve = 2,
    }
    impl KmsKey {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                KmsKey::Unspecified => "KMS_KEY_UNSPECIFIED",
                KmsKey::DestinationBucketDefault => "KMS_KEY_DESTINATION_BUCKET_DEFAULT",
                KmsKey::Preserve => "KMS_KEY_PRESERVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "KMS_KEY_UNSPECIFIED" => Some(Self::Unspecified),
                "KMS_KEY_DESTINATION_BUCKET_DEFAULT" => {
                    Some(Self::DestinationBucketDefault)
                }
                "KMS_KEY_PRESERVE" => Some(Self::Preserve),
                _ => None,
            }
        }
    }
    /// Options for handling `timeCreated` metadata for Google Cloud Storage
    /// objects.
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
    pub enum TimeCreated {
        /// TimeCreated behavior is unspecified.
        Unspecified = 0,
        /// Do not preserve the `timeCreated` metadata from the source object.
        Skip = 1,
        /// Preserves the source object's `timeCreated` metadata in the `customTime`
        /// field in the destination object.  Note that any value stored in the
        /// source object's `customTime` field will not be propagated to the
        /// destination object.
        PreserveAsCustomTime = 2,
    }
    impl TimeCreated {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TimeCreated::Unspecified => "TIME_CREATED_UNSPECIFIED",
                TimeCreated::Skip => "TIME_CREATED_SKIP",
                TimeCreated::PreserveAsCustomTime => {
                    "TIME_CREATED_PRESERVE_AS_CUSTOM_TIME"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TIME_CREATED_UNSPECIFIED" => Some(Self::Unspecified),
                "TIME_CREATED_SKIP" => Some(Self::Skip),
                "TIME_CREATED_PRESERVE_AS_CUSTOM_TIME" => {
                    Some(Self::PreserveAsCustomTime)
                }
                _ => None,
            }
        }
    }
}
/// Specifies where the manifest is located.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferManifest {
    /// Specifies the path to the manifest in Cloud Storage. The Google-managed
    /// service account for the transfer must have `storage.objects.get`
    /// permission for this object. An example path is
    /// `gs://bucket_name/path/manifest.csv`.
    #[prost(string, tag = "1")]
    pub location: ::prost::alloc::string::String,
}
/// Transfers can be scheduled to recur or to run just once.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schedule {
    /// Required. The start date of a transfer. Date boundaries are determined
    /// relative to UTC time. If `schedule_start_date` and \[start_time_of_day][google.storagetransfer.v1.Schedule.start_time_of_day\]
    /// are in the past relative to the job's creation time, the transfer starts
    /// the day after you schedule the transfer request.
    ///
    /// **Note:** When starting jobs at or near midnight UTC it is possible that
    /// a job starts later than expected. For example, if you send an outbound
    /// request on June 1 one millisecond prior to midnight UTC and the Storage
    /// Transfer Service server receives the request on June 2, then it creates
    /// a TransferJob with `schedule_start_date` set to June 2 and a
    /// `start_time_of_day` set to midnight UTC. The first scheduled
    /// \[TransferOperation][google.storagetransfer.v1.TransferOperation\] takes place on June 3 at midnight UTC.
    #[prost(message, optional, tag = "1")]
    pub schedule_start_date: ::core::option::Option<super::super::r#type::Date>,
    /// The last day a transfer runs. Date boundaries are determined relative to
    /// UTC time. A job runs once per 24 hours within the following guidelines:
    ///
    /// *   If `schedule_end_date` and \[schedule_start_date][google.storagetransfer.v1.Schedule.schedule_start_date\] are the same and in
    ///      the future relative to UTC, the transfer is executed only one time.
    /// *   If `schedule_end_date` is later than `schedule_start_date`  and
    ///      `schedule_end_date` is in the future relative to UTC, the job runs each
    ///      day at \[start_time_of_day][google.storagetransfer.v1.Schedule.start_time_of_day\] through `schedule_end_date`.
    #[prost(message, optional, tag = "2")]
    pub schedule_end_date: ::core::option::Option<super::super::r#type::Date>,
    /// The time in UTC that a transfer job is scheduled to run. Transfers may
    /// start later than this time.
    ///
    /// If `start_time_of_day` is not specified:
    ///
    /// *   One-time transfers run immediately.
    /// *   Recurring transfers run immediately, and each day at midnight UTC,
    ///      through \[schedule_end_date][google.storagetransfer.v1.Schedule.schedule_end_date\].
    ///
    /// If `start_time_of_day` is specified:
    ///
    /// *   One-time transfers run at the specified time.
    /// *   Recurring transfers run at the specified time each day, through
    ///      `schedule_end_date`.
    #[prost(message, optional, tag = "3")]
    pub start_time_of_day: ::core::option::Option<super::super::r#type::TimeOfDay>,
    /// The time in UTC that no further transfer operations are scheduled. Combined
    /// with \[schedule_end_date][google.storagetransfer.v1.Schedule.schedule_end_date\], `end_time_of_day` specifies the end date and
    /// time for starting new transfer operations. This field must be greater than
    /// or equal to the timestamp corresponding to the combintation of
    /// \[schedule_start_date][google.storagetransfer.v1.Schedule.schedule_start_date\] and \[start_time_of_day][google.storagetransfer.v1.Schedule.start_time_of_day\], and is subject to the
    /// following:
    ///
    /// *   If `end_time_of_day` is not set and `schedule_end_date` is set, then
    ///      a default value of `23:59:59` is used for `end_time_of_day`.
    ///
    /// *   If `end_time_of_day` is set and `schedule_end_date` is not set, then
    ///      \[INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT\] is returned.
    #[prost(message, optional, tag = "4")]
    pub end_time_of_day: ::core::option::Option<super::super::r#type::TimeOfDay>,
    /// Interval between the start of each scheduled TransferOperation. If
    /// unspecified, the default value is 24 hours. This value may not be less than
    /// 1 hour.
    #[prost(message, optional, tag = "5")]
    pub repeat_interval: ::core::option::Option<::prost_types::Duration>,
}
/// This resource represents the configuration of a transfer job that runs
/// periodically.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferJob {
    /// A unique name (within the transfer project) assigned when the job is
    /// created.  If this field is empty in a CreateTransferJobRequest, Storage
    /// Transfer Service assigns a unique name. Otherwise, the specified name
    /// is used as the unique name for this job.
    ///
    /// If the specified name is in use by a job, the creation request fails with
    /// an \[ALREADY_EXISTS][google.rpc.Code.ALREADY_EXISTS\] error.
    ///
    /// This name must start with `"transferJobs/"` prefix and end with a letter or
    /// a number, and should be no more than 128 characters. For transfers
    /// involving PosixFilesystem, this name must start with `transferJobs/OPI`
    /// specifically. For all other transfer types, this name must not start with
    /// `transferJobs/OPI`.
    ///
    /// Non-PosixFilesystem example:
    /// `"transferJobs/^(?!OPI)\[A-Za-z0-9-._~]*[A-Za-z0-9\]$"`
    ///
    /// PosixFilesystem example:
    /// `"transferJobs/OPI^\[A-Za-z0-9-._~]*[A-Za-z0-9\]$"`
    ///
    /// Applications must not rely on the enforcement of naming requirements
    /// involving OPI.
    ///
    /// Invalid job names fail with an
    /// \[INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT\] error.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A description provided by the user for the job. Its max length is 1024
    /// bytes when Unicode-encoded.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// The ID of the Google Cloud project that owns the job.
    #[prost(string, tag = "3")]
    pub project_id: ::prost::alloc::string::String,
    /// Transfer specification.
    #[prost(message, optional, tag = "4")]
    pub transfer_spec: ::core::option::Option<TransferSpec>,
    /// Notification configuration. This is not supported for transfers involving
    /// PosixFilesystem.
    #[prost(message, optional, tag = "11")]
    pub notification_config: ::core::option::Option<NotificationConfig>,
    /// Logging configuration.
    #[prost(message, optional, tag = "14")]
    pub logging_config: ::core::option::Option<LoggingConfig>,
    /// Specifies schedule for the transfer job.
    /// This is an optional field. When the field is not set, the job never
    /// executes a transfer, unless you invoke RunTransferJob or update the job to
    /// have a non-empty schedule.
    #[prost(message, optional, tag = "5")]
    pub schedule: ::core::option::Option<Schedule>,
    /// Status of the job. This value MUST be specified for
    /// `CreateTransferJobRequests`.
    ///
    /// **Note:** The effect of the new job status takes place during a subsequent
    /// job run. For example, if you change the job status from
    /// \[ENABLED][google.storagetransfer.v1.TransferJob.Status.ENABLED\] to \[DISABLED][google.storagetransfer.v1.TransferJob.Status.DISABLED\], and an operation
    /// spawned by the transfer is running, the status change would not affect the
    /// current operation.
    #[prost(enumeration = "transfer_job::Status", tag = "6")]
    pub status: i32,
    /// Output only. The time that the transfer job was created.
    #[prost(message, optional, tag = "7")]
    pub creation_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time that the transfer job was last modified.
    #[prost(message, optional, tag = "8")]
    pub last_modification_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time that the transfer job was deleted.
    #[prost(message, optional, tag = "9")]
    pub deletion_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The name of the most recently started TransferOperation of this JobConfig.
    /// Present if a TransferOperation has been created for this JobConfig.
    #[prost(string, tag = "12")]
    pub latest_operation_name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `TransferJob`.
pub mod transfer_job {
    /// The status of the transfer job.
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
    pub enum Status {
        /// Zero is an illegal value.
        Unspecified = 0,
        /// New transfers are performed based on the schedule.
        Enabled = 1,
        /// New transfers are not scheduled.
        Disabled = 2,
        /// This is a soft delete state. After a transfer job is set to this
        /// state, the job and all the transfer executions are subject to
        /// garbage collection. Transfer jobs become eligible for garbage collection
        /// 30 days after their status is set to `DELETED`.
        Deleted = 3,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::Unspecified => "STATUS_UNSPECIFIED",
                Status::Enabled => "ENABLED",
                Status::Disabled => "DISABLED",
                Status::Deleted => "DELETED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATUS_UNSPECIFIED" => Some(Self::Unspecified),
                "ENABLED" => Some(Self::Enabled),
                "DISABLED" => Some(Self::Disabled),
                "DELETED" => Some(Self::Deleted),
                _ => None,
            }
        }
    }
}
/// An entry describing an error that has occurred.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorLogEntry {
    /// Required. A URL that refers to the target (a data source, a data sink,
    /// or an object) with which the error is associated.
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    /// A list of messages that carry the error details.
    #[prost(string, repeated, tag = "3")]
    pub error_details: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A summary of errors by error code, plus a count and sample error log
/// entries.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorSummary {
    /// Required.
    #[prost(enumeration = "super::super::rpc::Code", tag = "1")]
    pub error_code: i32,
    /// Required. Count of this type of error.
    #[prost(int64, tag = "2")]
    pub error_count: i64,
    /// Error samples.
    ///
    /// At most 5 error log entries are recorded for a given
    /// error code for a single transfer operation.
    #[prost(message, repeated, tag = "3")]
    pub error_log_entries: ::prost::alloc::vec::Vec<ErrorLogEntry>,
}
/// A collection of counters that report the progress of a transfer operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferCounters {
    /// Objects found in the data source that are scheduled to be transferred,
    /// excluding any that are filtered based on object conditions or skipped due
    /// to sync.
    #[prost(int64, tag = "1")]
    pub objects_found_from_source: i64,
    /// Bytes found in the data source that are scheduled to be transferred,
    /// excluding any that are filtered based on object conditions or skipped due
    /// to sync.
    #[prost(int64, tag = "2")]
    pub bytes_found_from_source: i64,
    /// Objects found only in the data sink that are scheduled to be deleted.
    #[prost(int64, tag = "3")]
    pub objects_found_only_from_sink: i64,
    /// Bytes found only in the data sink that are scheduled to be deleted.
    #[prost(int64, tag = "4")]
    pub bytes_found_only_from_sink: i64,
    /// Objects in the data source that are not transferred because they already
    /// exist in the data sink.
    #[prost(int64, tag = "5")]
    pub objects_from_source_skipped_by_sync: i64,
    /// Bytes in the data source that are not transferred because they already
    /// exist in the data sink.
    #[prost(int64, tag = "6")]
    pub bytes_from_source_skipped_by_sync: i64,
    /// Objects that are copied to the data sink.
    #[prost(int64, tag = "7")]
    pub objects_copied_to_sink: i64,
    /// Bytes that are copied to the data sink.
    #[prost(int64, tag = "8")]
    pub bytes_copied_to_sink: i64,
    /// Objects that are deleted from the data source.
    #[prost(int64, tag = "9")]
    pub objects_deleted_from_source: i64,
    /// Bytes that are deleted from the data source.
    #[prost(int64, tag = "10")]
    pub bytes_deleted_from_source: i64,
    /// Objects that are deleted from the data sink.
    #[prost(int64, tag = "11")]
    pub objects_deleted_from_sink: i64,
    /// Bytes that are deleted from the data sink.
    #[prost(int64, tag = "12")]
    pub bytes_deleted_from_sink: i64,
    /// Objects in the data source that failed to be transferred or that failed
    /// to be deleted after being transferred.
    #[prost(int64, tag = "13")]
    pub objects_from_source_failed: i64,
    /// Bytes in the data source that failed to be transferred or that failed to
    /// be deleted after being transferred.
    #[prost(int64, tag = "14")]
    pub bytes_from_source_failed: i64,
    /// Objects that failed to be deleted from the data sink.
    #[prost(int64, tag = "15")]
    pub objects_failed_to_delete_from_sink: i64,
    /// Bytes that failed to be deleted from the data sink.
    #[prost(int64, tag = "16")]
    pub bytes_failed_to_delete_from_sink: i64,
    /// For transfers involving PosixFilesystem only.
    ///
    /// Number of directories found while listing. For example, if the root
    /// directory of the transfer is `base/` and there are two other directories,
    /// `a/` and `b/` under this directory, the count after listing `base/`,
    /// `base/a/` and `base/b/` is 3.
    #[prost(int64, tag = "17")]
    pub directories_found_from_source: i64,
    /// For transfers involving PosixFilesystem only.
    ///
    /// Number of listing failures for each directory found at the source.
    /// Potential failures when listing a directory include permission failure or
    /// block failure. If listing a directory fails, no files in the directory are
    /// transferred.
    #[prost(int64, tag = "18")]
    pub directories_failed_to_list_from_source: i64,
    /// For transfers involving PosixFilesystem only.
    ///
    /// Number of successful listings for each directory found at the source.
    #[prost(int64, tag = "19")]
    pub directories_successfully_listed_from_source: i64,
    /// Number of successfully cleaned up intermediate objects.
    #[prost(int64, tag = "22")]
    pub intermediate_objects_cleaned_up: i64,
    /// Number of intermediate objects failed cleaned up.
    #[prost(int64, tag = "23")]
    pub intermediate_objects_failed_cleaned_up: i64,
}
/// Specification to configure notifications published to Pub/Sub.
/// Notifications are published to the customer-provided topic using the
/// following `PubsubMessage.attributes`:
///
/// * `"eventType"`: one of the \[EventType][google.storagetransfer.v1.NotificationConfig.EventType\] values
/// * `"payloadFormat"`: one of the \[PayloadFormat][google.storagetransfer.v1.NotificationConfig.PayloadFormat\] values
/// * `"projectId"`: the \[project_id][google.storagetransfer.v1.TransferOperation.project_id\] of the
/// `TransferOperation`
/// * `"transferJobName"`: the
/// \[transfer_job_name][google.storagetransfer.v1.TransferOperation.transfer_job_name\] of the
/// `TransferOperation`
/// * `"transferOperationName"`: the \[name][google.storagetransfer.v1.TransferOperation.name\] of the
/// `TransferOperation`
///
/// The `PubsubMessage.data` contains a \[TransferOperation][google.storagetransfer.v1.TransferOperation\] resource
/// formatted according to the specified `PayloadFormat`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationConfig {
    /// Required. The `Topic.name` of the Pub/Sub topic to which to publish
    /// notifications. Must be of the format: `projects/{project}/topics/{topic}`.
    /// Not matching this format results in an
    /// \[INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT\] error.
    #[prost(string, tag = "1")]
    pub pubsub_topic: ::prost::alloc::string::String,
    /// Event types for which a notification is desired. If empty, send
    /// notifications for all event types.
    #[prost(enumeration = "notification_config::EventType", repeated, tag = "2")]
    pub event_types: ::prost::alloc::vec::Vec<i32>,
    /// Required. The desired format of the notification message payloads.
    #[prost(enumeration = "notification_config::PayloadFormat", tag = "3")]
    pub payload_format: i32,
}
/// Nested message and enum types in `NotificationConfig`.
pub mod notification_config {
    /// Enum for specifying event types for which notifications are to be
    /// published.
    ///
    /// Additional event types may be added in the future. Clients should either
    /// safely ignore unrecognized event types or explicitly specify which event
    /// types they are prepared to accept.
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
    pub enum EventType {
        /// Illegal value, to avoid allowing a default.
        Unspecified = 0,
        /// `TransferOperation` completed with status
        /// \[SUCCESS][google.storagetransfer.v1.TransferOperation.Status.SUCCESS\].
        TransferOperationSuccess = 1,
        /// `TransferOperation` completed with status
        /// \[FAILED][google.storagetransfer.v1.TransferOperation.Status.FAILED\].
        TransferOperationFailed = 2,
        /// `TransferOperation` completed with status
        /// \[ABORTED][google.storagetransfer.v1.TransferOperation.Status.ABORTED\].
        TransferOperationAborted = 3,
    }
    impl EventType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EventType::Unspecified => "EVENT_TYPE_UNSPECIFIED",
                EventType::TransferOperationSuccess => "TRANSFER_OPERATION_SUCCESS",
                EventType::TransferOperationFailed => "TRANSFER_OPERATION_FAILED",
                EventType::TransferOperationAborted => "TRANSFER_OPERATION_ABORTED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "EVENT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "TRANSFER_OPERATION_SUCCESS" => Some(Self::TransferOperationSuccess),
                "TRANSFER_OPERATION_FAILED" => Some(Self::TransferOperationFailed),
                "TRANSFER_OPERATION_ABORTED" => Some(Self::TransferOperationAborted),
                _ => None,
            }
        }
    }
    /// Enum for specifying the format of a notification message's payload.
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
    pub enum PayloadFormat {
        /// Illegal value, to avoid allowing a default.
        Unspecified = 0,
        /// No payload is included with the notification.
        None = 1,
        /// `TransferOperation` is [formatted as a JSON
        /// response](<https://developers.google.com/protocol-buffers/docs/proto3#json>),
        /// in application/json.
        Json = 2,
    }
    impl PayloadFormat {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PayloadFormat::Unspecified => "PAYLOAD_FORMAT_UNSPECIFIED",
                PayloadFormat::None => "NONE",
                PayloadFormat::Json => "JSON",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PAYLOAD_FORMAT_UNSPECIFIED" => Some(Self::Unspecified),
                "NONE" => Some(Self::None),
                "JSON" => Some(Self::Json),
                _ => None,
            }
        }
    }
}
/// Specifies the logging behavior for transfer operations.
///
/// For cloud-to-cloud transfers, logs are sent to Cloud Logging. See
/// [Read transfer
/// logs](<https://cloud.google.com/storage-transfer/docs/read-transfer-logs>) for
/// details.
///
/// For transfers to or from a POSIX file system, logs are stored in the
/// Cloud Storage bucket that is the source or sink of the transfer.
/// See [Managing Transfer for on-premises jobs]
/// (<https://cloud.google.com/storage-transfer/docs/managing-on-prem-jobs#viewing-logs>)
/// for details.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoggingConfig {
    /// Specifies the actions to be logged. If empty, no logs are generated.
    /// Not supported for transfers with PosixFilesystem data sources; use
    /// \[enable_onprem_gcs_transfer_logs][google.storagetransfer.v1.LoggingConfig.enable_onprem_gcs_transfer_logs\] instead.
    #[prost(enumeration = "logging_config::LoggableAction", repeated, tag = "1")]
    pub log_actions: ::prost::alloc::vec::Vec<i32>,
    /// States in which `log_actions` are logged. If empty, no logs are generated.
    /// Not supported for transfers with PosixFilesystem data sources; use
    /// \[enable_onprem_gcs_transfer_logs][google.storagetransfer.v1.LoggingConfig.enable_onprem_gcs_transfer_logs\] instead.
    #[prost(enumeration = "logging_config::LoggableActionState", repeated, tag = "2")]
    pub log_action_states: ::prost::alloc::vec::Vec<i32>,
    /// For transfers with a PosixFilesystem source, this option enables the Cloud
    /// Storage transfer logs for this transfer.
    #[prost(bool, tag = "3")]
    pub enable_onprem_gcs_transfer_logs: bool,
}
/// Nested message and enum types in `LoggingConfig`.
pub mod logging_config {
    /// Loggable actions.
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
    pub enum LoggableAction {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// Listing objects in a bucket.
        Find = 1,
        /// Deleting objects at the source or the destination.
        Delete = 2,
        /// Copying objects to Google Cloud Storage.
        Copy = 3,
    }
    impl LoggableAction {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LoggableAction::Unspecified => "LOGGABLE_ACTION_UNSPECIFIED",
                LoggableAction::Find => "FIND",
                LoggableAction::Delete => "DELETE",
                LoggableAction::Copy => "COPY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LOGGABLE_ACTION_UNSPECIFIED" => Some(Self::Unspecified),
                "FIND" => Some(Self::Find),
                "DELETE" => Some(Self::Delete),
                "COPY" => Some(Self::Copy),
                _ => None,
            }
        }
    }
    /// Loggable action states.
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
    pub enum LoggableActionState {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// `LoggableAction` completed successfully. `SUCCEEDED` actions are
        /// logged as \[INFO][google.logging.type.LogSeverity.INFO\].
        Succeeded = 1,
        /// `LoggableAction` terminated in an error state. `FAILED` actions are
        /// logged as \[ERROR][google.logging.type.LogSeverity.ERROR\].
        Failed = 2,
    }
    impl LoggableActionState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LoggableActionState::Unspecified => "LOGGABLE_ACTION_STATE_UNSPECIFIED",
                LoggableActionState::Succeeded => "SUCCEEDED",
                LoggableActionState::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LOGGABLE_ACTION_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "SUCCEEDED" => Some(Self::Succeeded),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
/// A description of the execution of a transfer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferOperation {
    /// A globally unique ID assigned by the system.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The ID of the Google Cloud project that owns the operation.
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
    /// Transfer specification.
    #[prost(message, optional, tag = "3")]
    pub transfer_spec: ::core::option::Option<TransferSpec>,
    /// Notification configuration.
    #[prost(message, optional, tag = "10")]
    pub notification_config: ::core::option::Option<NotificationConfig>,
    /// Start time of this transfer execution.
    #[prost(message, optional, tag = "4")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// End time of this transfer execution.
    #[prost(message, optional, tag = "5")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Status of the transfer operation.
    #[prost(enumeration = "transfer_operation::Status", tag = "6")]
    pub status: i32,
    /// Information about the progress of the transfer operation.
    #[prost(message, optional, tag = "7")]
    pub counters: ::core::option::Option<TransferCounters>,
    /// Summarizes errors encountered with sample error log entries.
    #[prost(message, repeated, tag = "8")]
    pub error_breakdowns: ::prost::alloc::vec::Vec<ErrorSummary>,
    /// The name of the transfer job that triggers this transfer operation.
    #[prost(string, tag = "9")]
    pub transfer_job_name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `TransferOperation`.
pub mod transfer_operation {
    /// The status of a TransferOperation.
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
    pub enum Status {
        /// Zero is an illegal value.
        Unspecified = 0,
        /// In progress.
        InProgress = 1,
        /// Paused.
        Paused = 2,
        /// Completed successfully.
        Success = 3,
        /// Terminated due to an unrecoverable failure.
        Failed = 4,
        /// Aborted by the user.
        Aborted = 5,
        /// Temporarily delayed by the system. No user action is required.
        Queued = 6,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::Unspecified => "STATUS_UNSPECIFIED",
                Status::InProgress => "IN_PROGRESS",
                Status::Paused => "PAUSED",
                Status::Success => "SUCCESS",
                Status::Failed => "FAILED",
                Status::Aborted => "ABORTED",
                Status::Queued => "QUEUED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATUS_UNSPECIFIED" => Some(Self::Unspecified),
                "IN_PROGRESS" => Some(Self::InProgress),
                "PAUSED" => Some(Self::Paused),
                "SUCCESS" => Some(Self::Success),
                "FAILED" => Some(Self::Failed),
                "ABORTED" => Some(Self::Aborted),
                "QUEUED" => Some(Self::Queued),
                _ => None,
            }
        }
    }
}
/// Request passed to GetGoogleServiceAccount.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGoogleServiceAccountRequest {
    /// Required. The ID of the Google Cloud project that the Google service
    /// account is associated with.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
}
/// Request passed to CreateTransferJob.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTransferJobRequest {
    /// Required. The job to create.
    #[prost(message, optional, tag = "1")]
    pub transfer_job: ::core::option::Option<TransferJob>,
}
/// Request passed to UpdateTransferJob.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTransferJobRequest {
    /// Required. The name of job to update.
    #[prost(string, tag = "1")]
    pub job_name: ::prost::alloc::string::String,
    /// Required. The ID of the Google Cloud project that owns the
    /// job.
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. The job to update. `transferJob` is expected to specify one or more of
    /// five fields: \[description][google.storagetransfer.v1.TransferJob.description\],
    /// \[transfer_spec][google.storagetransfer.v1.TransferJob.transfer_spec\],
    /// \[notification_config][google.storagetransfer.v1.TransferJob.notification_config\],
    /// \[logging_config][google.storagetransfer.v1.TransferJob.logging_config\], and
    /// \[status][google.storagetransfer.v1.TransferJob.status\].  An `UpdateTransferJobRequest` that specifies
    /// other fields are rejected with the error
    /// \[INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT\]. Updating a job status
    /// to \[DELETED][google.storagetransfer.v1.TransferJob.Status.DELETED\] requires
    /// `storagetransfer.jobs.delete` permission.
    #[prost(message, optional, tag = "3")]
    pub transfer_job: ::core::option::Option<TransferJob>,
    /// The field mask of the fields in `transferJob` that are to be updated in
    /// this request.  Fields in `transferJob` that can be updated are:
    /// \[description][google.storagetransfer.v1.TransferJob.description\],
    /// \[transfer_spec][google.storagetransfer.v1.TransferJob.transfer_spec\],
    /// \[notification_config][google.storagetransfer.v1.TransferJob.notification_config\],
    /// \[logging_config][google.storagetransfer.v1.TransferJob.logging_config\], and
    /// \[status][google.storagetransfer.v1.TransferJob.status\].  To update the `transfer_spec` of the job, a
    /// complete transfer specification must be provided. An incomplete
    /// specification missing any required fields is rejected with the error
    /// \[INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT\].
    #[prost(message, optional, tag = "4")]
    pub update_transfer_job_field_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request passed to GetTransferJob.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransferJobRequest {
    /// Required. The job to get.
    #[prost(string, tag = "1")]
    pub job_name: ::prost::alloc::string::String,
    /// Required. The ID of the Google Cloud project that owns the
    /// job.
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
}
/// Request passed to DeleteTransferJob.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTransferJobRequest {
    /// Required. The job to delete.
    #[prost(string, tag = "1")]
    pub job_name: ::prost::alloc::string::String,
    /// Required. The ID of the Google Cloud project that owns the
    /// job.
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
}
/// `projectId`, `jobNames`, and `jobStatuses` are query parameters that can
/// be specified when listing transfer jobs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransferJobsRequest {
    /// Required. A list of query parameters specified as JSON text in the form of:
    /// `{"projectId":"my_project_id",
    ///   "jobNames":\["jobid1","jobid2",...\],
    ///   "jobStatuses":\["status1","status2",...\]}`
    ///
    /// Since `jobNames` and `jobStatuses` support multiple values, their values
    /// must be specified with array notation. `projectId` is required.
    /// `jobNames` and `jobStatuses` are optional.  The valid values for
    /// `jobStatuses` are case-insensitive:
    /// \[ENABLED][google.storagetransfer.v1.TransferJob.Status.ENABLED\],
    /// \[DISABLED][google.storagetransfer.v1.TransferJob.Status.DISABLED\], and
    /// \[DELETED][google.storagetransfer.v1.TransferJob.Status.DELETED\].
    #[prost(string, tag = "1")]
    pub filter: ::prost::alloc::string::String,
    /// The list page size. The max allowed value is 256.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// The list page token.
    #[prost(string, tag = "5")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response from ListTransferJobs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransferJobsResponse {
    /// A list of transfer jobs.
    #[prost(message, repeated, tag = "1")]
    pub transfer_jobs: ::prost::alloc::vec::Vec<TransferJob>,
    /// The list next page token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request passed to PauseTransferOperation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PauseTransferOperationRequest {
    /// Required. The name of the transfer operation.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request passed to ResumeTransferOperation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResumeTransferOperationRequest {
    /// Required. The name of the transfer operation.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request passed to RunTransferJob.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunTransferJobRequest {
    /// Required. The name of the transfer job.
    #[prost(string, tag = "1")]
    pub job_name: ::prost::alloc::string::String,
    /// Required. The ID of the Google Cloud project that owns the transfer
    /// job.
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
}
/// Specifies the request passed to CreateAgentPool.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAgentPoolRequest {
    /// Required. The ID of the Google Cloud project that owns the
    /// agent pool.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. The agent pool to create.
    #[prost(message, optional, tag = "2")]
    pub agent_pool: ::core::option::Option<AgentPool>,
    /// Required. The ID of the agent pool to create.
    ///
    /// The `agent_pool_id` must meet the following requirements:
    ///
    /// *   Length of 128 characters or less.
    /// *   Not start with the string `goog`.
    /// *   Start with a lowercase ASCII character, followed by:
    ///      *   Zero or more: lowercase Latin alphabet characters, numerals,
    ///          hyphens (`-`), periods (`.`), underscores (`_`), or tildes (`~`).
    ///      *   One or more numerals or lowercase ASCII characters.
    ///
    /// As expressed by the regular expression:
    /// `^(?!goog)\[a-z]([a-z0-9-._~]*[a-z0-9\])?$`.
    #[prost(string, tag = "3")]
    pub agent_pool_id: ::prost::alloc::string::String,
}
/// Specifies the request passed to UpdateAgentPool.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAgentPoolRequest {
    /// Required. The agent pool to update. `agent_pool` is expected to specify following
    /// fields:
    ///
    /// *  \[name][google.storagetransfer.v1.AgentPool.name\]
    ///
    /// *  \[display_name][google.storagetransfer.v1.AgentPool.display_name\]
    ///
    /// *  \[bandwidth_limit][google.storagetransfer.v1.AgentPool.bandwidth_limit\]
    /// An `UpdateAgentPoolRequest` with any other fields is rejected
    /// with the error \[INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT\].
    #[prost(message, optional, tag = "1")]
    pub agent_pool: ::core::option::Option<AgentPool>,
    /// The [field mask]
    /// (<https://developers.google.com/protocol-buffers/docs/reference/google.protobuf>)
    /// of the fields in `agentPool` to update in this request.
    /// The following `agentPool` fields can be updated:
    ///
    /// *  \[display_name][google.storagetransfer.v1.AgentPool.display_name\]
    ///
    /// *  \[bandwidth_limit][google.storagetransfer.v1.AgentPool.bandwidth_limit\]
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Specifies the request passed to GetAgentPool.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAgentPoolRequest {
    /// Required. The name of the agent pool to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Specifies the request passed to DeleteAgentPool.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAgentPoolRequest {
    /// Required. The name of the agent pool to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request passed to ListAgentPools.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAgentPoolsRequest {
    /// Required. The ID of the Google Cloud project that owns the job.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// An optional list of query parameters specified as JSON text in the
    /// form of:
    ///
    /// `{"agentPoolNames":\["agentpool1","agentpool2",...\]}`
    ///
    /// Since `agentPoolNames` support multiple values, its values must be
    /// specified with array notation. When the filter is either empty or not
    /// provided, the list returns all agent pools for the project.
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// The list page size. The max allowed value is `256`.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The list page token.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response from ListAgentPools.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAgentPoolsResponse {
    /// A list of agent pools.
    #[prost(message, repeated, tag = "1")]
    pub agent_pools: ::prost::alloc::vec::Vec<AgentPool>,
    /// The list next page token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod storage_transfer_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Storage Transfer Service and its protos.
    /// Transfers data between between Google Cloud Storage buckets or from a data
    /// source external to Google to a Cloud Storage bucket.
    #[derive(Debug, Clone)]
    pub struct StorageTransferServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> StorageTransferServiceClient<T>
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
        ) -> StorageTransferServiceClient<InterceptedService<T, F>>
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
            StorageTransferServiceClient::new(
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
        /// Returns the Google service account that is used by Storage Transfer
        /// Service to access buckets in the project where transfers
        /// run or in other projects. Each Google service account is associated
        /// with one Google Cloud project. Users
        /// should add this service account to the Google Cloud Storage bucket
        /// ACLs to grant access to Storage Transfer Service. This service
        /// account is created and owned by Storage Transfer Service and can
        /// only be used by Storage Transfer Service.
        pub async fn get_google_service_account(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGoogleServiceAccountRequest>,
        ) -> Result<tonic::Response<super::GoogleServiceAccount>, tonic::Status> {
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
                "/google.storagetransfer.v1.StorageTransferService/GetGoogleServiceAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a transfer job that runs periodically.
        pub async fn create_transfer_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTransferJobRequest>,
        ) -> Result<tonic::Response<super::TransferJob>, tonic::Status> {
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
                "/google.storagetransfer.v1.StorageTransferService/CreateTransferJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a transfer job. Updating a job's transfer spec does not affect
        /// transfer operations that are running already.
        ///
        /// **Note:** The job's [status][google.storagetransfer.v1.TransferJob.status] field can be modified
        /// using this RPC (for example, to set a job's status to
        /// [DELETED][google.storagetransfer.v1.TransferJob.Status.DELETED],
        /// [DISABLED][google.storagetransfer.v1.TransferJob.Status.DISABLED], or
        /// [ENABLED][google.storagetransfer.v1.TransferJob.Status.ENABLED]).
        pub async fn update_transfer_job(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTransferJobRequest>,
        ) -> Result<tonic::Response<super::TransferJob>, tonic::Status> {
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
                "/google.storagetransfer.v1.StorageTransferService/UpdateTransferJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a transfer job.
        pub async fn get_transfer_job(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransferJobRequest>,
        ) -> Result<tonic::Response<super::TransferJob>, tonic::Status> {
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
                "/google.storagetransfer.v1.StorageTransferService/GetTransferJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists transfer jobs.
        pub async fn list_transfer_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTransferJobsRequest>,
        ) -> Result<tonic::Response<super::ListTransferJobsResponse>, tonic::Status> {
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
                "/google.storagetransfer.v1.StorageTransferService/ListTransferJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Pauses a transfer operation.
        pub async fn pause_transfer_operation(
            &mut self,
            request: impl tonic::IntoRequest<super::PauseTransferOperationRequest>,
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
                "/google.storagetransfer.v1.StorageTransferService/PauseTransferOperation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Resumes a transfer operation that is paused.
        pub async fn resume_transfer_operation(
            &mut self,
            request: impl tonic::IntoRequest<super::ResumeTransferOperationRequest>,
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
                "/google.storagetransfer.v1.StorageTransferService/ResumeTransferOperation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Attempts to start a new TransferOperation for the current TransferJob. A
        /// TransferJob has a maximum of one active TransferOperation. If this method
        /// is called while a TransferOperation is active, an error will be returned.
        pub async fn run_transfer_job(
            &mut self,
            request: impl tonic::IntoRequest<super::RunTransferJobRequest>,
        ) -> Result<
            tonic::Response<super::super::super::longrunning::Operation>,
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
                "/google.storagetransfer.v1.StorageTransferService/RunTransferJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a transfer job. Deleting a transfer job sets its status to
        /// [DELETED][google.storagetransfer.v1.TransferJob.Status.DELETED].
        pub async fn delete_transfer_job(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTransferJobRequest>,
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
                "/google.storagetransfer.v1.StorageTransferService/DeleteTransferJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates an agent pool resource.
        pub async fn create_agent_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAgentPoolRequest>,
        ) -> Result<tonic::Response<super::AgentPool>, tonic::Status> {
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
                "/google.storagetransfer.v1.StorageTransferService/CreateAgentPool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an existing agent pool resource.
        pub async fn update_agent_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAgentPoolRequest>,
        ) -> Result<tonic::Response<super::AgentPool>, tonic::Status> {
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
                "/google.storagetransfer.v1.StorageTransferService/UpdateAgentPool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets an agent pool.
        pub async fn get_agent_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAgentPoolRequest>,
        ) -> Result<tonic::Response<super::AgentPool>, tonic::Status> {
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
                "/google.storagetransfer.v1.StorageTransferService/GetAgentPool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists agent pools.
        pub async fn list_agent_pools(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAgentPoolsRequest>,
        ) -> Result<tonic::Response<super::ListAgentPoolsResponse>, tonic::Status> {
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
                "/google.storagetransfer.v1.StorageTransferService/ListAgentPools",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes an agent pool.
        pub async fn delete_agent_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAgentPoolRequest>,
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
                "/google.storagetransfer.v1.StorageTransferService/DeleteAgentPool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
