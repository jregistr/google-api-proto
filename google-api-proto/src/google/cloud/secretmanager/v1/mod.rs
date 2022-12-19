/// A \[Secret][google.cloud.secretmanager.v1.Secret\] is a logical secret whose
/// value and versions can be accessed.
///
/// A \[Secret][google.cloud.secretmanager.v1.Secret\] is made up of zero or more
/// \[SecretVersions][google.cloud.secretmanager.v1.SecretVersion\] that represent
/// the secret data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Secret {
    /// Output only. The resource name of the
    /// \[Secret][google.cloud.secretmanager.v1.Secret\] in the format
    /// `projects/*/secrets/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Immutable. The replication policy of the secret data attached to
    /// the \[Secret][google.cloud.secretmanager.v1.Secret\].
    ///
    /// The replication policy cannot be changed after the Secret has been created.
    #[prost(message, optional, tag = "2")]
    pub replication: ::core::option::Option<Replication>,
    /// Output only. The time at which the
    /// \[Secret][google.cloud.secretmanager.v1.Secret\] was created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The labels assigned to this Secret.
    ///
    /// Label keys must be between 1 and 63 characters long, have a UTF-8 encoding
    /// of maximum 128 bytes, and must conform to the following PCRE regular
    /// expression: `\[\p{Ll}\p{Lo}][\p{Ll}\p{Lo}\p{N}_-\]{0,62}`
    ///
    /// Label values must be between 0 and 63 characters long, have a UTF-8
    /// encoding of maximum 128 bytes, and must conform to the following PCRE
    /// regular expression: `\[\p{Ll}\p{Lo}\p{N}_-\]{0,63}`
    ///
    /// No more than 64 labels can be assigned to a given resource.
    #[prost(btree_map = "string, string", tag = "4")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. A list of up to 10 Pub/Sub topics to which messages are published
    /// when control plane operations are called on the secret or its versions.
    #[prost(message, repeated, tag = "5")]
    pub topics: ::prost::alloc::vec::Vec<Topic>,
    /// Optional. Etag of the currently stored
    /// \[Secret][google.cloud.secretmanager.v1.Secret\].
    #[prost(string, tag = "8")]
    pub etag: ::prost::alloc::string::String,
    /// Optional. Rotation policy attached to the
    /// \[Secret][google.cloud.secretmanager.v1.Secret\]. May be excluded if there is
    /// no rotation policy.
    #[prost(message, optional, tag = "9")]
    pub rotation: ::core::option::Option<Rotation>,
    /// Optional. Mapping from version alias to version name.
    ///
    /// A version alias is a string with a maximum length of 63 characters and can
    /// contain uppercase and lowercase letters, numerals, and the hyphen (`-`)
    /// and underscore ('_') characters. An alias string must start with a
    /// letter and cannot be the string 'latest' or 'NEW'.
    /// No more than 50 aliases can be assigned to a given secret.
    ///
    /// Version-Alias pairs will be viewable via GetSecret and modifiable via
    /// UpdateSecret. At launch access by alias will only be supported on
    /// GetSecretVersion and AccessSecretVersion.
    #[prost(btree_map = "string, int64", tag = "11")]
    pub version_aliases: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        i64,
    >,
    /// Optional. Custom metadata about the secret.
    ///
    /// Annotations are distinct from various forms of labels.
    /// Annotations exist to allow client tools to store their own state
    /// information without requiring a database.
    ///
    /// Annotation keys must be between 1 and 63 characters long, have a UTF-8
    /// encoding of maximum 128 bytes, begin and end with an alphanumeric character
    /// (\[a-z0-9A-Z\]), and may have dashes (-), underscores (_), dots (.), and
    /// alphanumerics in between these symbols.
    ///
    /// The total size of annotation keys and values must be less than 16KiB.
    #[prost(btree_map = "string, string", tag = "13")]
    pub annotations: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Expiration policy attached to the
    /// \[Secret][google.cloud.secretmanager.v1.Secret\]. If specified the
    /// \[Secret][google.cloud.secretmanager.v1.Secret\] and all
    /// \[SecretVersions][google.cloud.secretmanager.v1.SecretVersion\] will be
    /// automatically deleted at expiration. Expired secrets are irreversibly
    /// deleted.
    ///
    /// Expiration is *not* the recommended way to set time-based permissions. [IAM
    /// Conditions](<https://cloud.google.com/secret-manager/docs/access-control#conditions>)
    /// is recommended for granting time-based permissions because the operation
    /// can be reversed.
    #[prost(oneof = "secret::Expiration", tags = "6, 7")]
    pub expiration: ::core::option::Option<secret::Expiration>,
}
/// Nested message and enum types in `Secret`.
pub mod secret {
    /// Expiration policy attached to the
    /// \[Secret][google.cloud.secretmanager.v1.Secret\]. If specified the
    /// \[Secret][google.cloud.secretmanager.v1.Secret\] and all
    /// \[SecretVersions][google.cloud.secretmanager.v1.SecretVersion\] will be
    /// automatically deleted at expiration. Expired secrets are irreversibly
    /// deleted.
    ///
    /// Expiration is *not* the recommended way to set time-based permissions. [IAM
    /// Conditions](<https://cloud.google.com/secret-manager/docs/access-control#conditions>)
    /// is recommended for granting time-based permissions because the operation
    /// can be reversed.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Expiration {
        /// Optional. Timestamp in UTC when the
        /// \[Secret][google.cloud.secretmanager.v1.Secret\] is scheduled to expire.
        /// This is always provided on output, regardless of what was sent on input.
        #[prost(message, tag = "6")]
        ExpireTime(::prost_types::Timestamp),
        /// Input only. The TTL for the
        /// \[Secret][google.cloud.secretmanager.v1.Secret\].
        #[prost(message, tag = "7")]
        Ttl(::prost_types::Duration),
    }
}
/// A secret version resource in the Secret Manager API.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretVersion {
    /// Output only. The resource name of the
    /// \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\] in the format
    /// `projects/*/secrets/*/versions/*`.
    ///
    /// \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\] IDs in a
    /// \[Secret][google.cloud.secretmanager.v1.Secret\] start at 1 and are
    /// incremented for each subsequent version of the secret.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time at which the
    /// \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\] was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time this
    /// \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\] was destroyed.
    /// Only present if \[state][google.cloud.secretmanager.v1.SecretVersion.state\]
    /// is
    /// \[DESTROYED][google.cloud.secretmanager.v1.SecretVersion.State.DESTROYED\].
    #[prost(message, optional, tag = "3")]
    pub destroy_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The current state of the
    /// \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\].
    #[prost(enumeration = "secret_version::State", tag = "4")]
    pub state: i32,
    /// The replication status of the
    /// \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\].
    #[prost(message, optional, tag = "5")]
    pub replication_status: ::core::option::Option<ReplicationStatus>,
    /// Output only. Etag of the currently stored
    /// \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\].
    #[prost(string, tag = "6")]
    pub etag: ::prost::alloc::string::String,
    /// Output only. True if payload checksum specified in
    /// \[SecretPayload][google.cloud.secretmanager.v1.SecretPayload\] object has
    /// been received by
    /// \[SecretManagerService][google.cloud.secretmanager.v1.SecretManagerService\]
    /// on
    /// \[SecretManagerService.AddSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.AddSecretVersion\].
    #[prost(bool, tag = "7")]
    pub client_specified_payload_checksum: bool,
}
/// Nested message and enum types in `SecretVersion`.
pub mod secret_version {
    /// The state of a
    /// \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\], indicating if
    /// it can be accessed.
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
        /// Not specified. This value is unused and invalid.
        Unspecified = 0,
        /// The \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\] may be
        /// accessed.
        Enabled = 1,
        /// The \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\] may not
        /// be accessed, but the secret data is still available and can be placed
        /// back into the
        /// \[ENABLED][google.cloud.secretmanager.v1.SecretVersion.State.ENABLED\]
        /// state.
        Disabled = 2,
        /// The \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\] is
        /// destroyed and the secret data is no longer stored. A version may not
        /// leave this state once entered.
        Destroyed = 3,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Enabled => "ENABLED",
                State::Disabled => "DISABLED",
                State::Destroyed => "DESTROYED",
            }
        }
    }
}
/// A policy that defines the replication and encryption configuration of data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Replication {
    /// The replication policy for this secret.
    #[prost(oneof = "replication::Replication", tags = "1, 2")]
    pub replication: ::core::option::Option<replication::Replication>,
}
/// Nested message and enum types in `Replication`.
pub mod replication {
    /// A replication policy that replicates the
    /// \[Secret][google.cloud.secretmanager.v1.Secret\] payload without any
    /// restrictions.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Automatic {
        /// Optional. The customer-managed encryption configuration of the
        /// \[Secret][google.cloud.secretmanager.v1.Secret\]. If no configuration is
        /// provided, Google-managed default encryption is used.
        ///
        /// Updates to the \[Secret][google.cloud.secretmanager.v1.Secret\] encryption
        /// configuration only apply to
        /// \[SecretVersions][google.cloud.secretmanager.v1.SecretVersion\] added
        /// afterwards. They do not apply retroactively to existing
        /// \[SecretVersions][google.cloud.secretmanager.v1.SecretVersion\].
        #[prost(message, optional, tag = "1")]
        pub customer_managed_encryption: ::core::option::Option<
            super::CustomerManagedEncryption,
        >,
    }
    /// A replication policy that replicates the
    /// \[Secret][google.cloud.secretmanager.v1.Secret\] payload into the locations
    /// specified in \[Secret.replication.user_managed.replicas][\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UserManaged {
        /// Required. The list of Replicas for this
        /// \[Secret][google.cloud.secretmanager.v1.Secret\].
        ///
        /// Cannot be empty.
        #[prost(message, repeated, tag = "1")]
        pub replicas: ::prost::alloc::vec::Vec<user_managed::Replica>,
    }
    /// Nested message and enum types in `UserManaged`.
    pub mod user_managed {
        /// Represents a Replica for this
        /// \[Secret][google.cloud.secretmanager.v1.Secret\].
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Replica {
            /// The canonical IDs of the location to replicate data.
            /// For example: `"us-east1"`.
            #[prost(string, tag = "1")]
            pub location: ::prost::alloc::string::String,
            /// Optional. The customer-managed encryption configuration of the
            /// [User-Managed Replica]\[Replication.UserManaged.Replica\]. If no
            /// configuration is provided, Google-managed default encryption is used.
            ///
            /// Updates to the \[Secret][google.cloud.secretmanager.v1.Secret\]
            /// encryption configuration only apply to
            /// \[SecretVersions][google.cloud.secretmanager.v1.SecretVersion\] added
            /// afterwards. They do not apply retroactively to existing
            /// \[SecretVersions][google.cloud.secretmanager.v1.SecretVersion\].
            #[prost(message, optional, tag = "2")]
            pub customer_managed_encryption: ::core::option::Option<
                super::super::CustomerManagedEncryption,
            >,
        }
    }
    /// The replication policy for this secret.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Replication {
        /// The \[Secret][google.cloud.secretmanager.v1.Secret\] will automatically be
        /// replicated without any restrictions.
        #[prost(message, tag = "1")]
        Automatic(Automatic),
        /// The \[Secret][google.cloud.secretmanager.v1.Secret\] will only be
        /// replicated into the locations specified.
        #[prost(message, tag = "2")]
        UserManaged(UserManaged),
    }
}
/// Configuration for encrypting secret payloads using customer-managed
/// encryption keys (CMEK).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerManagedEncryption {
    /// Required. The resource name of the Cloud KMS CryptoKey used to encrypt
    /// secret payloads.
    ///
    /// For secrets using the
    /// \[UserManaged][google.cloud.secretmanager.v1.Replication.UserManaged\]
    /// replication policy type, Cloud KMS CryptoKeys must reside in the same
    /// location as the [replica location]\[Secret.UserManaged.Replica.location\].
    ///
    /// For secrets using the
    /// \[Automatic][google.cloud.secretmanager.v1.Replication.Automatic\]
    /// replication policy type, Cloud KMS CryptoKeys must reside in `global`.
    ///
    /// The expected format is `projects/*/locations/*/keyRings/*/cryptoKeys/*`.
    #[prost(string, tag = "1")]
    pub kms_key_name: ::prost::alloc::string::String,
}
/// The replication status of a
/// \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicationStatus {
    /// The replication status of the
    /// \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\].
    #[prost(oneof = "replication_status::ReplicationStatus", tags = "1, 2")]
    pub replication_status: ::core::option::Option<
        replication_status::ReplicationStatus,
    >,
}
/// Nested message and enum types in `ReplicationStatus`.
pub mod replication_status {
    /// The replication status of a
    /// \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\] using
    /// automatic replication.
    ///
    /// Only populated if the parent \[Secret][google.cloud.secretmanager.v1.Secret\]
    /// has an automatic replication policy.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AutomaticStatus {
        /// Output only. The customer-managed encryption status of the
        /// \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\]. Only
        /// populated if customer-managed encryption is used.
        #[prost(message, optional, tag = "1")]
        pub customer_managed_encryption: ::core::option::Option<
            super::CustomerManagedEncryptionStatus,
        >,
    }
    /// The replication status of a
    /// \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\] using
    /// user-managed replication.
    ///
    /// Only populated if the parent \[Secret][google.cloud.secretmanager.v1.Secret\]
    /// has a user-managed replication policy.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UserManagedStatus {
        /// Output only. The list of replica statuses for the
        /// \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\].
        #[prost(message, repeated, tag = "1")]
        pub replicas: ::prost::alloc::vec::Vec<user_managed_status::ReplicaStatus>,
    }
    /// Nested message and enum types in `UserManagedStatus`.
    pub mod user_managed_status {
        /// Describes the status of a user-managed replica for the
        /// \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\].
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ReplicaStatus {
            /// Output only. The canonical ID of the replica location.
            /// For example: `"us-east1"`.
            #[prost(string, tag = "1")]
            pub location: ::prost::alloc::string::String,
            /// Output only. The customer-managed encryption status of the
            /// \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\]. Only
            /// populated if customer-managed encryption is used.
            #[prost(message, optional, tag = "2")]
            pub customer_managed_encryption: ::core::option::Option<
                super::super::CustomerManagedEncryptionStatus,
            >,
        }
    }
    /// The replication status of the
    /// \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\].
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ReplicationStatus {
        /// Describes the replication status of a
        /// \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\] with
        /// automatic replication.
        ///
        /// Only populated if the parent
        /// \[Secret][google.cloud.secretmanager.v1.Secret\] has an automatic
        /// replication policy.
        #[prost(message, tag = "1")]
        Automatic(AutomaticStatus),
        /// Describes the replication status of a
        /// \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\] with
        /// user-managed replication.
        ///
        /// Only populated if the parent
        /// \[Secret][google.cloud.secretmanager.v1.Secret\] has a user-managed
        /// replication policy.
        #[prost(message, tag = "2")]
        UserManaged(UserManagedStatus),
    }
}
/// Describes the status of customer-managed encryption.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerManagedEncryptionStatus {
    /// Required. The resource name of the Cloud KMS CryptoKeyVersion used to
    /// encrypt the secret payload, in the following format:
    /// `projects/*/locations/*/keyRings/*/cryptoKeys/*/versions/*`.
    #[prost(string, tag = "1")]
    pub kms_key_version_name: ::prost::alloc::string::String,
}
/// A Pub/Sub topic which Secret Manager will publish to when control plane
/// events occur on this secret.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Topic {
    /// Required. The resource name of the Pub/Sub topic that will be published to,
    /// in the following format: `projects/*/topics/*`. For publication to succeed,
    /// the Secret Manager P4SA must have `pubsub.publisher` permissions on the
    /// topic.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The rotation time and period for a
/// \[Secret][google.cloud.secretmanager.v1.Secret\]. At next_rotation_time, Secret
/// Manager will send a Pub/Sub notification to the topics configured on the
/// Secret. \[Secret.topics][google.cloud.secretmanager.v1.Secret.topics\] must be
/// set to configure rotation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rotation {
    /// Optional. Timestamp in UTC at which the
    /// \[Secret][google.cloud.secretmanager.v1.Secret\] is scheduled to rotate.
    /// Cannot be set to less than 300s (5 min) in the future and at most
    /// 3153600000s (100 years).
    ///
    /// \[next_rotation_time][google.cloud.secretmanager.v1.Rotation.next_rotation_time\]
    /// MUST  be set if
    /// \[rotation_period][google.cloud.secretmanager.v1.Rotation.rotation_period\]
    /// is set.
    #[prost(message, optional, tag = "1")]
    pub next_rotation_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Input only. The Duration between rotation notifications. Must be in seconds
    /// and at least 3600s (1h) and at most 3153600000s (100 years).
    ///
    /// If
    /// \[rotation_period][google.cloud.secretmanager.v1.Rotation.rotation_period\]
    /// is set,
    /// \[next_rotation_time][google.cloud.secretmanager.v1.Rotation.next_rotation_time\]
    /// must be set.
    /// \[next_rotation_time][google.cloud.secretmanager.v1.Rotation.next_rotation_time\]
    /// will be advanced by this period when the service automatically sends
    /// rotation notifications.
    #[prost(message, optional, tag = "2")]
    pub rotation_period: ::core::option::Option<::prost_types::Duration>,
}
/// A secret payload resource in the Secret Manager API. This contains the
/// sensitive secret payload that is associated with a
/// \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretPayload {
    /// The secret data. Must be no larger than 64KiB.
    #[prost(bytes = "bytes", tag = "1")]
    pub data: ::prost::bytes::Bytes,
    /// Optional. If specified,
    /// \[SecretManagerService][google.cloud.secretmanager.v1.SecretManagerService\]
    /// will verify the integrity of the received
    /// \[data][google.cloud.secretmanager.v1.SecretPayload.data\] on
    /// \[SecretManagerService.AddSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.AddSecretVersion\]
    /// calls using the crc32c checksum and store it to include in future
    /// \[SecretManagerService.AccessSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.AccessSecretVersion\]
    /// responses. If a checksum is not provided in the
    /// \[SecretManagerService.AddSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.AddSecretVersion\]
    /// request, the
    /// \[SecretManagerService][google.cloud.secretmanager.v1.SecretManagerService\]
    /// will generate and store one for you.
    ///
    /// The CRC32C value is encoded as a Int64 for compatibility, and can be
    /// safely downconverted to uint32 in languages that support this type.
    /// <https://cloud.google.com/apis/design/design_patterns#integer_types>
    #[prost(int64, optional, tag = "2")]
    pub data_crc32c: ::core::option::Option<i64>,
}
/// Request message for \[SecretManagerService.ListSecrets][google.cloud.secretmanager.v1.SecretManagerService.ListSecrets\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSecretsRequest {
    /// Required. The resource name of the project associated with the
    /// \[Secrets][google.cloud.secretmanager.v1.Secret\], in the format `projects/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of results to be returned in a single page. If
    /// set to 0, the server decides the number of results to return. If the
    /// number is greater than 25000, it is capped at 25000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Pagination token, returned earlier via
    /// \[ListSecretsResponse.next_page_token][google.cloud.secretmanager.v1.ListSecretsResponse.next_page_token\].
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filter string, adhering to the rules in
    /// [List-operation
    /// filtering](<https://cloud.google.com/secret-manager/docs/filtering>). List
    /// only secrets matching the filter. If filter is empty, all secrets are
    /// listed.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message for \[SecretManagerService.ListSecrets][google.cloud.secretmanager.v1.SecretManagerService.ListSecrets\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSecretsResponse {
    /// The list of \[Secrets][google.cloud.secretmanager.v1.Secret\] sorted in reverse by create_time (newest
    /// first).
    #[prost(message, repeated, tag = "1")]
    pub secrets: ::prost::alloc::vec::Vec<Secret>,
    /// A token to retrieve the next page of results. Pass this value in
    /// \[ListSecretsRequest.page_token][google.cloud.secretmanager.v1.ListSecretsRequest.page_token\] to retrieve the next page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The total number of \[Secrets][google.cloud.secretmanager.v1.Secret\].
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Request message for \[SecretManagerService.CreateSecret][google.cloud.secretmanager.v1.SecretManagerService.CreateSecret\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSecretRequest {
    /// Required. The resource name of the project to associate with the
    /// \[Secret][google.cloud.secretmanager.v1.Secret\], in the format `projects/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. This must be unique within the project.
    ///
    /// A secret ID is a string with a maximum length of 255 characters and can
    /// contain uppercase and lowercase letters, numerals, and the hyphen (`-`) and
    /// underscore (`_`) characters.
    #[prost(string, tag = "2")]
    pub secret_id: ::prost::alloc::string::String,
    /// Required. A \[Secret][google.cloud.secretmanager.v1.Secret\] with initial field values.
    #[prost(message, optional, tag = "3")]
    pub secret: ::core::option::Option<Secret>,
}
/// Request message for \[SecretManagerService.AddSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.AddSecretVersion\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddSecretVersionRequest {
    /// Required. The resource name of the \[Secret][google.cloud.secretmanager.v1.Secret\] to associate with the
    /// \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\] in the format `projects/*/secrets/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The secret payload of the \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\].
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<SecretPayload>,
}
/// Request message for \[SecretManagerService.GetSecret][google.cloud.secretmanager.v1.SecretManagerService.GetSecret\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSecretRequest {
    /// Required. The resource name of the \[Secret][google.cloud.secretmanager.v1.Secret\], in the format `projects/*/secrets/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for \[SecretManagerService.ListSecretVersions][google.cloud.secretmanager.v1.SecretManagerService.ListSecretVersions\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSecretVersionsRequest {
    /// Required. The resource name of the \[Secret][google.cloud.secretmanager.v1.Secret\] associated with the
    /// \[SecretVersions][google.cloud.secretmanager.v1.SecretVersion\] to list, in the format
    /// `projects/*/secrets/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of results to be returned in a single page. If
    /// set to 0, the server decides the number of results to return. If the
    /// number is greater than 25000, it is capped at 25000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Pagination token, returned earlier via
    /// ListSecretVersionsResponse.next_page_token][].
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filter string, adhering to the rules in
    /// [List-operation
    /// filtering](<https://cloud.google.com/secret-manager/docs/filtering>). List
    /// only secret versions matching the filter. If filter is empty, all secret
    /// versions are listed.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message for \[SecretManagerService.ListSecretVersions][google.cloud.secretmanager.v1.SecretManagerService.ListSecretVersions\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSecretVersionsResponse {
    /// The list of \[SecretVersions][google.cloud.secretmanager.v1.SecretVersion\] sorted in reverse by
    /// create_time (newest first).
    #[prost(message, repeated, tag = "1")]
    pub versions: ::prost::alloc::vec::Vec<SecretVersion>,
    /// A token to retrieve the next page of results. Pass this value in
    /// \[ListSecretVersionsRequest.page_token][google.cloud.secretmanager.v1.ListSecretVersionsRequest.page_token\] to retrieve the next page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The total number of \[SecretVersions][google.cloud.secretmanager.v1.SecretVersion\].
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Request message for \[SecretManagerService.GetSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.GetSecretVersion\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSecretVersionRequest {
    /// Required. The resource name of the \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\] in the format
    /// `projects/*/secrets/*/versions/*`.
    ///
    /// `projects/*/secrets/*/versions/latest` is an alias to the most recently
    /// created \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\].
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for \[SecretManagerService.UpdateSecret][google.cloud.secretmanager.v1.SecretManagerService.UpdateSecret\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSecretRequest {
    /// Required. \[Secret][google.cloud.secretmanager.v1.Secret\] with updated field values.
    #[prost(message, optional, tag = "1")]
    pub secret: ::core::option::Option<Secret>,
    /// Required. Specifies the fields to be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for \[SecretManagerService.AccessSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.AccessSecretVersion\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessSecretVersionRequest {
    /// Required. The resource name of the \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\] in the format
    /// `projects/*/secrets/*/versions/*`.
    ///
    /// `projects/*/secrets/*/versions/latest` is an alias to the most recently
    /// created \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\].
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Response message for \[SecretManagerService.AccessSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.AccessSecretVersion\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessSecretVersionResponse {
    /// The resource name of the \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\] in the format
    /// `projects/*/secrets/*/versions/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Secret payload
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<SecretPayload>,
}
/// Request message for \[SecretManagerService.DeleteSecret][google.cloud.secretmanager.v1.SecretManagerService.DeleteSecret\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSecretRequest {
    /// Required. The resource name of the \[Secret][google.cloud.secretmanager.v1.Secret\] to delete in the format
    /// `projects/*/secrets/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Etag of the \[Secret][google.cloud.secretmanager.v1.Secret\]. The request succeeds if it matches
    /// the etag of the currently stored secret object. If the etag is omitted,
    /// the request succeeds.
    #[prost(string, tag = "2")]
    pub etag: ::prost::alloc::string::String,
}
/// Request message for \[SecretManagerService.DisableSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.DisableSecretVersion\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableSecretVersionRequest {
    /// Required. The resource name of the \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\] to disable in the format
    /// `projects/*/secrets/*/versions/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Etag of the \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\]. The request succeeds if it matches
    /// the etag of the currently stored secret version object. If the etag is
    /// omitted, the request succeeds.
    #[prost(string, tag = "2")]
    pub etag: ::prost::alloc::string::String,
}
/// Request message for \[SecretManagerService.EnableSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.EnableSecretVersion\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableSecretVersionRequest {
    /// Required. The resource name of the \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\] to enable in the format
    /// `projects/*/secrets/*/versions/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Etag of the \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\]. The request succeeds if it matches
    /// the etag of the currently stored secret version object. If the etag is
    /// omitted, the request succeeds.
    #[prost(string, tag = "2")]
    pub etag: ::prost::alloc::string::String,
}
/// Request message for \[SecretManagerService.DestroySecretVersion][google.cloud.secretmanager.v1.SecretManagerService.DestroySecretVersion\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestroySecretVersionRequest {
    /// Required. The resource name of the \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\] to destroy in the format
    /// `projects/*/secrets/*/versions/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Etag of the \[SecretVersion][google.cloud.secretmanager.v1.SecretVersion\]. The request succeeds if it matches
    /// the etag of the currently stored secret version object. If the etag is
    /// omitted, the request succeeds.
    #[prost(string, tag = "2")]
    pub etag: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod secret_manager_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Secret Manager Service
    ///
    /// Manages secrets and operations using those secrets. Implements a REST
    /// model with the following objects:
    ///
    /// * [Secret][google.cloud.secretmanager.v1.Secret]
    /// * [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]
    #[derive(Debug, Clone)]
    pub struct SecretManagerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SecretManagerServiceClient<T>
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
        ) -> SecretManagerServiceClient<InterceptedService<T, F>>
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
            SecretManagerServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists [Secrets][google.cloud.secretmanager.v1.Secret].
        pub async fn list_secrets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSecretsRequest>,
        ) -> Result<tonic::Response<super::ListSecretsResponse>, tonic::Status> {
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
                "/google.cloud.secretmanager.v1.SecretManagerService/ListSecrets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new [Secret][google.cloud.secretmanager.v1.Secret] containing no [SecretVersions][google.cloud.secretmanager.v1.SecretVersion].
        pub async fn create_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSecretRequest>,
        ) -> Result<tonic::Response<super::Secret>, tonic::Status> {
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
                "/google.cloud.secretmanager.v1.SecretManagerService/CreateSecret",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] containing secret data and attaches
        /// it to an existing [Secret][google.cloud.secretmanager.v1.Secret].
        pub async fn add_secret_version(
            &mut self,
            request: impl tonic::IntoRequest<super::AddSecretVersionRequest>,
        ) -> Result<tonic::Response<super::SecretVersion>, tonic::Status> {
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
                "/google.cloud.secretmanager.v1.SecretManagerService/AddSecretVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets metadata for a given [Secret][google.cloud.secretmanager.v1.Secret].
        pub async fn get_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSecretRequest>,
        ) -> Result<tonic::Response<super::Secret>, tonic::Status> {
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
                "/google.cloud.secretmanager.v1.SecretManagerService/GetSecret",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates metadata of an existing [Secret][google.cloud.secretmanager.v1.Secret].
        pub async fn update_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSecretRequest>,
        ) -> Result<tonic::Response<super::Secret>, tonic::Status> {
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
                "/google.cloud.secretmanager.v1.SecretManagerService/UpdateSecret",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a [Secret][google.cloud.secretmanager.v1.Secret].
        pub async fn delete_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSecretRequest>,
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
                "/google.cloud.secretmanager.v1.SecretManagerService/DeleteSecret",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists [SecretVersions][google.cloud.secretmanager.v1.SecretVersion]. This call does not return secret
        /// data.
        pub async fn list_secret_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSecretVersionsRequest>,
        ) -> Result<tonic::Response<super::ListSecretVersionsResponse>, tonic::Status> {
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
                "/google.cloud.secretmanager.v1.SecretManagerService/ListSecretVersions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets metadata for a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
        ///
        /// `projects/*/secrets/*/versions/latest` is an alias to the most recently
        /// created [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
        pub async fn get_secret_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSecretVersionRequest>,
        ) -> Result<tonic::Response<super::SecretVersion>, tonic::Status> {
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
                "/google.cloud.secretmanager.v1.SecretManagerService/GetSecretVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Accesses a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]. This call returns the secret data.
        ///
        /// `projects/*/secrets/*/versions/latest` is an alias to the most recently
        /// created [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
        pub async fn access_secret_version(
            &mut self,
            request: impl tonic::IntoRequest<super::AccessSecretVersionRequest>,
        ) -> Result<tonic::Response<super::AccessSecretVersionResponse>, tonic::Status> {
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
                "/google.cloud.secretmanager.v1.SecretManagerService/AccessSecretVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Disables a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
        ///
        /// Sets the [state][google.cloud.secretmanager.v1.SecretVersion.state] of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to
        /// [DISABLED][google.cloud.secretmanager.v1.SecretVersion.State.DISABLED].
        pub async fn disable_secret_version(
            &mut self,
            request: impl tonic::IntoRequest<super::DisableSecretVersionRequest>,
        ) -> Result<tonic::Response<super::SecretVersion>, tonic::Status> {
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
                "/google.cloud.secretmanager.v1.SecretManagerService/DisableSecretVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Enables a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
        ///
        /// Sets the [state][google.cloud.secretmanager.v1.SecretVersion.state] of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to
        /// [ENABLED][google.cloud.secretmanager.v1.SecretVersion.State.ENABLED].
        pub async fn enable_secret_version(
            &mut self,
            request: impl tonic::IntoRequest<super::EnableSecretVersionRequest>,
        ) -> Result<tonic::Response<super::SecretVersion>, tonic::Status> {
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
                "/google.cloud.secretmanager.v1.SecretManagerService/EnableSecretVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Destroys a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
        ///
        /// Sets the [state][google.cloud.secretmanager.v1.SecretVersion.state] of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to
        /// [DESTROYED][google.cloud.secretmanager.v1.SecretVersion.State.DESTROYED] and irrevocably destroys the
        /// secret data.
        pub async fn destroy_secret_version(
            &mut self,
            request: impl tonic::IntoRequest<super::DestroySecretVersionRequest>,
        ) -> Result<tonic::Response<super::SecretVersion>, tonic::Status> {
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
                "/google.cloud.secretmanager.v1.SecretManagerService/DestroySecretVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the access control policy on the specified secret. Replaces any
        /// existing policy.
        ///
        /// Permissions on [SecretVersions][google.cloud.secretmanager.v1.SecretVersion] are enforced according
        /// to the policy set on the associated [Secret][google.cloud.secretmanager.v1.Secret].
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
                "/google.cloud.secretmanager.v1.SecretManagerService/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the access control policy for a secret.
        /// Returns empty policy if the secret exists and does not have a policy set.
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
                "/google.cloud.secretmanager.v1.SecretManagerService/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns permissions that a caller has for the specified secret.
        /// If the secret does not exist, this call returns an empty set of
        /// permissions, not a NOT_FOUND error.
        ///
        /// Note: This operation is designed to be used for building permission-aware
        /// UIs and command-line tools, not for authorization checking. This operation
        /// may "fail open" without warning.
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
                "/google.cloud.secretmanager.v1.SecretManagerService/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
