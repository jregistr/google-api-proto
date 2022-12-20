/// AWS S3 object metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsS3ObjectMetadata {
    /// Required. Name of the S3 bucket.
    #[prost(string, tag = "1")]
    pub bucket: ::prost::alloc::string::String,
    /// Required. Name/key of the object.
    #[prost(string, tag = "2")]
    pub object_key: ::prost::alloc::string::String,
    /// Last modified time of the object.
    #[prost(message, optional, tag = "3")]
    pub last_modified_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The MD5 checksum of the object's content.
    #[prost(string, tag = "4")]
    pub md5: ::prost::alloc::string::String,
    /// Required. Size of the object in bytes.
    #[prost(int64, tag = "5")]
    pub size: i64,
}
/// AWS S3 bucket metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsS3BucketMetadata {
    /// Required. Name of the S3 bucket.
    #[prost(string, tag = "1")]
    pub bucket: ::prost::alloc::string::String,
    /// The path of transfer objects as an object key prefix ending with "/".
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
}
/// Google Cloud Storage object metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsObjectMetadata {
    /// Required. Name of the Cloud Storage bucket.
    #[prost(string, tag = "1")]
    pub bucket: ::prost::alloc::string::String,
    /// Required. Name/key of the object.
    #[prost(string, tag = "2")]
    pub object_key: ::prost::alloc::string::String,
    /// Last modified time of the object.
    #[prost(message, optional, tag = "3")]
    pub last_modified_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The MD5 checksum of the object's content.
    #[prost(string, tag = "4")]
    pub md5: ::prost::alloc::string::String,
    /// The CRC32C checksum of the object's content.
    #[prost(string, tag = "5")]
    pub crc32c: ::prost::alloc::string::String,
    /// Required. Size of the object in bytes.
    #[prost(int64, tag = "6")]
    pub size: i64,
}
/// Google Cloud Storage bucket metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsBucketMetadata {
    /// Required. Name of the Cloud Storage bucket.
    #[prost(string, tag = "1")]
    pub bucket: ::prost::alloc::string::String,
    /// The path of transfer objects as an object key prefix ending with "/".
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
}
/// Azure Blob Storage blob metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureBlobMetadata {
    /// Required. Name of the Azure Blob storage account.
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    /// Required. Name of the container.
    #[prost(string, tag = "2")]
    pub container: ::prost::alloc::string::String,
    /// Required. Name of the blob.
    #[prost(string, tag = "3")]
    pub blob_name: ::prost::alloc::string::String,
    /// Last modified time of the blob.
    #[prost(message, optional, tag = "4")]
    pub last_modified_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The MD5 checksum of the object's content.
    #[prost(string, tag = "5")]
    pub md5: ::prost::alloc::string::String,
    /// Required. Size of the blob in bytes.
    #[prost(int64, tag = "6")]
    pub size: i64,
}
/// Azure Blob Storage container metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureBlobContainerMetadata {
    /// Required. Name of the Azure Blob storage account.
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    /// Required. Name of the container.
    #[prost(string, tag = "2")]
    pub container: ::prost::alloc::string::String,
    /// The path of transfer blobs as a blob name prefix ending with "/".
    #[prost(string, tag = "3")]
    pub path: ::prost::alloc::string::String,
}
/// POSIX file metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PosixFileMetadata {
    /// Required. Path of a file.
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    /// Last modified time (mtime) of the file.
    #[prost(message, optional, tag = "2")]
    pub last_modified_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The CRC32C checksum of the object's content.
    #[prost(string, tag = "3")]
    pub crc32c: ::prost::alloc::string::String,
    /// Required. Size of the file in bytes.
    #[prost(int64, tag = "4")]
    pub size: i64,
}
/// HTTP file metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpFileMetadata {
    /// Required. Url of the HTTP file.
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    /// The MD5 checksum of the file's content.
    #[prost(string, tag = "2")]
    pub md5: ::prost::alloc::string::String,
    /// Size of the file in bytes.
    #[prost(int64, tag = "3")]
    pub size: i64,
}
/// HTTP manifest file metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpManifestMetadata {
    /// Required. Url of the HTTP manifest which contains the list of HTTP files to
    /// transfer.
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
}
/// Metadata of a blob/file/object.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectMetadata {
    /// Required. Storage system type of the object.
    #[prost(enumeration = "StorageSystemType", tag = "1")]
    pub r#type: i32,
    #[prost(oneof = "object_metadata::Metadata", tags = "3, 4, 5, 6, 7")]
    pub metadata: ::core::option::Option<object_metadata::Metadata>,
}
/// Nested message and enum types in `ObjectMetadata`.
pub mod object_metadata {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Metadata {
        /// Object metadata of AWS S3.
        #[prost(message, tag = "3")]
        AwsS3Object(super::AwsS3ObjectMetadata),
        /// Blob metadata of Azure Blob Storage.
        #[prost(message, tag = "4")]
        AzureBlob(super::AzureBlobMetadata),
        /// Object metadata of Google Cloud Storage.
        #[prost(message, tag = "5")]
        GcsObject(super::GcsObjectMetadata),
        /// File/directory metadata of POSIX file system.
        #[prost(message, tag = "6")]
        PosixFile(super::PosixFileMetadata),
        /// Metadata of a file on a HTTP server.
        #[prost(message, tag = "7")]
        HttpFile(super::HttpFileMetadata),
    }
}
/// Metadata of a bucket/container/directory
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerMetadata {
    /// Required. Storage system type of the object.
    #[prost(enumeration = "StorageSystemType", tag = "1")]
    pub r#type: i32,
    #[prost(oneof = "container_metadata::Metadata", tags = "3, 4, 5, 6, 7")]
    pub metadata: ::core::option::Option<container_metadata::Metadata>,
}
/// Nested message and enum types in `ContainerMetadata`.
pub mod container_metadata {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Metadata {
        /// Bucket metadata of AWS S3.
        #[prost(message, tag = "3")]
        AwsS3Bucket(super::AwsS3BucketMetadata),
        /// Container metadata of Azure Blob Storage.
        #[prost(message, tag = "4")]
        AzureBlobContainer(super::AzureBlobContainerMetadata),
        /// Bucket metadata of Google Cloud Storage.
        #[prost(message, tag = "5")]
        GcsBucket(super::GcsBucketMetadata),
        /// Directory metadata of POSIX file system.
        #[prost(message, tag = "6")]
        PosixDirectory(super::PosixFileMetadata),
        /// Metadata of a manifest file on a HTTP server.
        #[prost(message, tag = "7")]
        HttpManifest(super::HttpManifestMetadata),
    }
}
/// Schema of log payload of transfer activity.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferActivityLog {
    /// Required. Name of the transfer operation.
    #[prost(string, tag = "1")]
    pub operation: ::prost::alloc::string::String,
    /// Required. The action which the transfer operation made.
    #[prost(enumeration = "transfer_activity_log::Action", tag = "2")]
    pub action: i32,
    /// Required. Status of the action.
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<transfer_activity_log::Status>,
    /// Metadata of source bucket/container/directory. Only set if the action is
    /// FIND.
    #[prost(message, optional, tag = "4")]
    pub source_container: ::core::option::Option<ContainerMetadata>,
    /// Metadata of destination bucket/container/directory. Only set if the action
    /// is FIND.
    #[prost(message, optional, tag = "5")]
    pub destination_container: ::core::option::Option<ContainerMetadata>,
    /// Metadata of the source blob/file/object. Only set if the action is COPY or
    /// DELETE when deletion is applied to source.
    #[prost(message, optional, tag = "6")]
    pub source_object: ::core::option::Option<ObjectMetadata>,
    /// Metadata of the destination blob/file/object. Only set if the action is
    /// or DELETE when deletion is applied to destination.
    #[prost(message, optional, tag = "7")]
    pub destination_object: ::core::option::Option<ObjectMetadata>,
    /// Required. Completion time of the action.
    #[prost(message, optional, tag = "8")]
    pub complete_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `TransferActivityLog`.
pub mod transfer_activity_log {
    /// Status of an action.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Status {
        /// Required. A string value of the Google RPC code as the status of the
        /// action. The action succeeded if it's `OK`, and failed otherwise.
        #[prost(string, tag = "1")]
        pub status_code: ::prost::alloc::string::String,
        /// A string that represents the type of error encountered. Populated only if
        /// status_code is not `OK`.
        #[prost(string, tag = "2")]
        pub error_type: ::prost::alloc::string::String,
        /// A human-readable error message for the failure. Populated only if
        /// status_code is not `OK`.
        #[prost(string, tag = "3")]
        pub error_message: ::prost::alloc::string::String,
    }
    /// Possible actions which a transfer operation can make.
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
    pub enum Action {
        /// Unspeficied action.
        Unspecified = 0,
        /// Finding work to do, such as listing files in a directory or listing
        /// objects in a bucket.
        Find = 1,
        /// Copying files or objects.
        Copy = 2,
        /// Deleting files or objects at destination.
        Delete = 3,
    }
    impl Action {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Action::Unspecified => "ACTION_UNSPECIFIED",
                Action::Find => "FIND",
                Action::Copy => "COPY",
                Action::Delete => "DELETE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ACTION_UNSPECIFIED" => Some(Self::Unspecified),
                "FIND" => Some(Self::Find),
                "COPY" => Some(Self::Copy),
                "DELETE" => Some(Self::Delete),
                _ => None,
            }
        }
    }
}
/// Type of the storage system.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StorageSystemType {
    /// Unspecified.
    Unspecified = 0,
    /// AWS S3.
    AwsS3 = 1,
    /// Azure Blob Storage.
    AzureBlob = 2,
    /// Google Cloud Storage.
    Gcs = 3,
    /// POSIX file system.
    PosixFs = 4,
    /// HTTP/HTTPS servers.
    Http = 5,
}
impl StorageSystemType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StorageSystemType::Unspecified => "STORAGE_SYSTEM_TYPE_UNSPECIFIED",
            StorageSystemType::AwsS3 => "AWS_S3",
            StorageSystemType::AzureBlob => "AZURE_BLOB",
            StorageSystemType::Gcs => "GCS",
            StorageSystemType::PosixFs => "POSIX_FS",
            StorageSystemType::Http => "HTTP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STORAGE_SYSTEM_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "AWS_S3" => Some(Self::AwsS3),
            "AZURE_BLOB" => Some(Self::AzureBlob),
            "GCS" => Some(Self::Gcs),
            "POSIX_FS" => Some(Self::PosixFs),
            "HTTP" => Some(Self::Http),
            _ => None,
        }
    }
}
