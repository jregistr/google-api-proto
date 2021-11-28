/// Provenance of a build. Contains all information needed to verify the full
/// details about the build from source to completion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildProvenance {
    /// Required. Unique identifier of the build.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// ID of the project.
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
    /// Commands requested by the build.
    #[prost(message, repeated, tag = "3")]
    pub commands: ::prost::alloc::vec::Vec<Command>,
    /// Output of the build.
    #[prost(message, repeated, tag = "4")]
    pub built_artifacts: ::prost::alloc::vec::Vec<Artifact>,
    /// Time at which the build was created.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Time at which execution of the build was started.
    #[prost(message, optional, tag = "6")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Time at which execution of the build was finished.
    #[prost(message, optional, tag = "7")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// E-mail address of the user who initiated this build. Note that this was the
    /// user's e-mail address at the time the build was initiated; this address may
    /// not represent the same end-user for all time.
    #[prost(string, tag = "8")]
    pub creator: ::prost::alloc::string::String,
    /// URI where any logs for this provenance were written.
    #[prost(string, tag = "9")]
    pub logs_uri: ::prost::alloc::string::String,
    /// Details of the Source input to the build.
    #[prost(message, optional, tag = "10")]
    pub source_provenance: ::core::option::Option<Source>,
    /// Trigger identifier if the build was triggered automatically; empty if not.
    #[prost(string, tag = "11")]
    pub trigger_id: ::prost::alloc::string::String,
    /// Special options applied to this build. This is a catch-all field where
    /// build providers can enter any desired additional details.
    #[prost(btree_map = "string, string", tag = "12")]
    pub build_options: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Version string of the builder at the time this build was executed.
    #[prost(string, tag = "13")]
    pub builder_version: ::prost::alloc::string::String,
}
/// Source describes the location of the source used for the build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Source {
    /// If provided, the input binary artifacts for the build came from this
    /// location.
    #[prost(string, tag = "1")]
    pub artifact_storage_source_uri: ::prost::alloc::string::String,
    /// Hash(es) of the build source, which can be used to verify that the original
    /// source integrity was maintained in the build.
    ///
    /// The keys to this map are file paths used as build source and the values
    /// contain the hash values for those files.
    ///
    /// If the build source came in a single package such as a gzipped tarfile
    /// (.tar.gz), the FileHash will be for the single path to that file.
    #[prost(btree_map = "string, message", tag = "2")]
    pub file_hashes:
        ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, FileHashes>,
    /// If provided, the source code used for the build came from this location.
    #[prost(message, optional, tag = "3")]
    pub context: ::core::option::Option<super::source::SourceContext>,
    /// If provided, some of the source code used for the build may be found in
    /// these locations, in the case where the source repository had multiple
    /// remotes or submodules. This list will not include the context specified in
    /// the context field.
    #[prost(message, repeated, tag = "4")]
    pub additional_contexts: ::prost::alloc::vec::Vec<super::source::SourceContext>,
}
/// Container message for hashes of byte content of files, used in source
/// messages to verify integrity of source input to the build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileHashes {
    /// Required. Collection of file hashes.
    #[prost(message, repeated, tag = "1")]
    pub file_hash: ::prost::alloc::vec::Vec<Hash>,
}
/// Container message for hash values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hash {
    /// Required. The type of hash that was performed.
    #[prost(enumeration = "hash::HashType", tag = "1")]
    pub r#type: i32,
    /// Required. The hash value.
    #[prost(bytes = "bytes", tag = "2")]
    pub value: ::prost::bytes::Bytes,
}
/// Nested message and enum types in `Hash`.
pub mod hash {
    /// Specifies the hash algorithm.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum HashType {
        /// Unknown.
        Unspecified = 0,
        /// A SHA-256 hash.
        Sha256 = 1,
    }
}
/// Command describes a step performed as part of the build pipeline.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Command {
    /// Required. Name of the command, as presented on the command line, or if the
    /// command is packaged as a Docker container, as presented to `docker pull`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Environment variables set before running this command.
    #[prost(string, repeated, tag = "2")]
    pub env: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Command-line arguments used when executing this command.
    #[prost(string, repeated, tag = "3")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Working directory (relative to project source root) used when running this
    /// command.
    #[prost(string, tag = "4")]
    pub dir: ::prost::alloc::string::String,
    /// Optional unique identifier for this command, used in wait_for to reference
    /// this command as a dependency.
    #[prost(string, tag = "5")]
    pub id: ::prost::alloc::string::String,
    /// The ID(s) of the command(s) that this command depends on.
    #[prost(string, repeated, tag = "6")]
    pub wait_for: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Artifact describes a build product.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Artifact {
    /// Hash or checksum value of a binary, or Docker Registry 2.0 digest of a
    /// container.
    #[prost(string, tag = "1")]
    pub checksum: ::prost::alloc::string::String,
    /// Artifact ID, if any; for container images, this will be a URL by digest
    /// like `gcr.io/projectID/imagename@sha256:123456`.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// Related artifact names. This may be the path to a binary or jar file, or in
    /// the case of a container build, the name used to push the container image to
    /// Google Container Registry, as presented to `docker push`. Note that a
    /// single Artifact ID can have multiple names, for example if two tags are
    /// applied to one image.
    #[prost(string, repeated, tag = "3")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
