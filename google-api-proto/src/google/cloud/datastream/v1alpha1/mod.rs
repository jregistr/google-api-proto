/// Oracle database profile.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OracleProfile {
    /// Required. Hostname for the Oracle connection.
    #[prost(string, tag="1")]
    pub hostname: ::prost::alloc::string::String,
    /// Port for the Oracle connection, default value is 1521.
    #[prost(int32, tag="2")]
    pub port: i32,
    /// Required. Username for the Oracle connection.
    #[prost(string, tag="3")]
    pub username: ::prost::alloc::string::String,
    /// Required. Password for the Oracle connection.
    #[prost(string, tag="4")]
    pub password: ::prost::alloc::string::String,
    /// Required. Database for the Oracle connection.
    #[prost(string, tag="5")]
    pub database_service: ::prost::alloc::string::String,
    /// Connection string attributes
    #[prost(btree_map="string, string", tag="6")]
    pub connection_attributes: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// MySQL database profile.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MysqlProfile {
    /// Required. Hostname for the MySQL connection.
    #[prost(string, tag="1")]
    pub hostname: ::prost::alloc::string::String,
    /// Port for the MySQL connection, default value is 3306.
    #[prost(int32, tag="2")]
    pub port: i32,
    /// Required. Username for the MySQL connection.
    #[prost(string, tag="3")]
    pub username: ::prost::alloc::string::String,
    /// Required. Input only. Password for the MySQL connection.
    #[prost(string, tag="4")]
    pub password: ::prost::alloc::string::String,
    /// SSL configuration for the MySQL connection.
    #[prost(message, optional, tag="5")]
    pub ssl_config: ::core::option::Option<MysqlSslConfig>,
}
/// Cloud Storage bucket profile.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsProfile {
    /// Required. The full project and resource path for Cloud Storage bucket including the
    /// name.
    #[prost(string, tag="1")]
    pub bucket_name: ::prost::alloc::string::String,
    /// The root path inside the Cloud Storage bucket.
    #[prost(string, tag="2")]
    pub root_path: ::prost::alloc::string::String,
}
/// No connectivity settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoConnectivitySettings {
}
/// Static IP address connectivity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticServiceIpConnectivity {
}
/// Forward SSH Tunnel connectivity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForwardSshTunnelConnectivity {
    /// Required. Hostname for the SSH tunnel.
    #[prost(string, tag="1")]
    pub hostname: ::prost::alloc::string::String,
    /// Required. Username for the SSH tunnel.
    #[prost(string, tag="2")]
    pub username: ::prost::alloc::string::String,
    /// Port for the SSH tunnel, default value is 22.
    #[prost(int32, tag="3")]
    pub port: i32,
    #[prost(oneof="forward_ssh_tunnel_connectivity::AuthenticationMethod", tags="100, 101")]
    pub authentication_method: ::core::option::Option<forward_ssh_tunnel_connectivity::AuthenticationMethod>,
}
/// Nested message and enum types in `ForwardSshTunnelConnectivity`.
pub mod forward_ssh_tunnel_connectivity {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AuthenticationMethod {
        /// Input only. SSH password.
        #[prost(string, tag="100")]
        Password(::prost::alloc::string::String),
        /// Input only. SSH private key.
        #[prost(string, tag="101")]
        PrivateKey(::prost::alloc::string::String),
    }
}
/// The VPC Peering configuration is used to create VPC peering between
/// Datastream and the consumer's VPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VpcPeeringConfig {
    /// Required. fully qualified name of the VPC Datastream will peer to.
    #[prost(string, tag="1")]
    pub vpc_name: ::prost::alloc::string::String,
    /// Required. A free subnet for peering. (CIDR of /29)
    #[prost(string, tag="2")]
    pub subnet: ::prost::alloc::string::String,
}
/// The PrivateConnection resource is used to establish private connectivity
/// between Datastream and a customer's network.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateConnection {
    /// Output only. The resource's name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The create time of the resource.
    #[prost(message, optional, tag="2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The update time of the resource.
    #[prost(message, optional, tag="3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels.
    #[prost(btree_map="string, string", tag="4")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Required. Display name.
    #[prost(string, tag="5")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. The state of the Private Connection.
    #[prost(enumeration="private_connection::State", tag="6")]
    pub state: i32,
    /// Output only. In case of error, the details of the error in a user-friendly format.
    #[prost(message, optional, tag="7")]
    pub error: ::core::option::Option<Error>,
    /// VPC Peering Config
    #[prost(message, optional, tag="100")]
    pub vpc_peering_config: ::core::option::Option<VpcPeeringConfig>,
}
/// Nested message and enum types in `PrivateConnection`.
pub mod private_connection {
    /// Private Connection state.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        Unspecified = 0,
        /// The private connection is in creation state - creating resources.
        Creating = 1,
        /// The private connection has been created with all of it's resources.
        Created = 2,
        /// The private connection creation has failed.
        Failed = 3,
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
                State::Failed => "FAILED",
            }
        }
    }
}
/// Private Connectivity
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateConnectivity {
    #[prost(string, tag="1")]
    pub private_connection_name: ::prost::alloc::string::String,
}
/// The Route resource is the child of the PrivateConnection resource.
/// It used to define a route for a PrivateConnection setup.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Route {
    /// Output only. The resource's name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The create time of the resource.
    #[prost(message, optional, tag="2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The update time of the resource.
    #[prost(message, optional, tag="3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels.
    #[prost(btree_map="string, string", tag="4")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Required. Display name.
    #[prost(string, tag="5")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. Destination address for connection
    #[prost(string, tag="6")]
    pub destination_address: ::prost::alloc::string::String,
    /// Destination port for connection
    #[prost(int32, tag="7")]
    pub destination_port: i32,
}
/// MySQL SSL configuration information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MysqlSslConfig {
    /// Input only. PEM-encoded private key associated with the Client Certificate.
    /// If this field is used then the 'client_certificate' and the
    /// 'ca_certificate' fields are mandatory.
    #[prost(string, tag="11")]
    pub client_key: ::prost::alloc::string::String,
    /// Output only. Indicates whether the client_key field is set.
    #[prost(bool, tag="12")]
    pub client_key_set: bool,
    /// Input only. PEM-encoded certificate that will be used by the replica to
    /// authenticate against the source database server. If this field is used
    /// then the 'client_key' and the 'ca_certificate' fields are mandatory.
    #[prost(string, tag="13")]
    pub client_certificate: ::prost::alloc::string::String,
    /// Output only. Indicates whether the client_certificate field is set.
    #[prost(bool, tag="14")]
    pub client_certificate_set: bool,
    /// Input only. PEM-encoded certificate of the CA that signed the source database
    /// server's certificate.
    #[prost(string, tag="15")]
    pub ca_certificate: ::prost::alloc::string::String,
    /// Output only. Indicates whether the ca_certificate field is set.
    #[prost(bool, tag="16")]
    pub ca_certificate_set: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionProfile {
    /// Output only. The resource's name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The create time of the resource.
    #[prost(message, optional, tag="2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The update time of the resource.
    #[prost(message, optional, tag="3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels.
    #[prost(btree_map="string, string", tag="4")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Required. Display name.
    #[prost(string, tag="5")]
    pub display_name: ::prost::alloc::string::String,
    /// Connection configuration for the ConnectionProfile.
    #[prost(oneof="connection_profile::Profile", tags="100, 101, 102")]
    pub profile: ::core::option::Option<connection_profile::Profile>,
    /// Connectivity options used to establish a connection to the profile.
    #[prost(oneof="connection_profile::Connectivity", tags="200, 201, 202, 203")]
    pub connectivity: ::core::option::Option<connection_profile::Connectivity>,
}
/// Nested message and enum types in `ConnectionProfile`.
pub mod connection_profile {
    /// Connection configuration for the ConnectionProfile.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Profile {
        /// Oracle ConnectionProfile configuration.
        #[prost(message, tag="100")]
        OracleProfile(super::OracleProfile),
        /// Cloud Storage ConnectionProfile configuration.
        #[prost(message, tag="101")]
        GcsProfile(super::GcsProfile),
        /// MySQL ConnectionProfile configuration.
        #[prost(message, tag="102")]
        MysqlProfile(super::MysqlProfile),
    }
    /// Connectivity options used to establish a connection to the profile.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Connectivity {
        /// No connectivity option chosen.
        #[prost(message, tag="200")]
        NoConnectivity(super::NoConnectivitySettings),
        /// Static Service IP connectivity.
        #[prost(message, tag="201")]
        StaticServiceIpConnectivity(super::StaticServiceIpConnectivity),
        /// Forward SSH tunnel connectivity.
        #[prost(message, tag="202")]
        ForwardSshConnectivity(super::ForwardSshTunnelConnectivity),
        /// Private connectivity.
        #[prost(message, tag="203")]
        PrivateConnectivity(super::PrivateConnectivity),
    }
}
/// Oracle Column.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OracleColumn {
    /// Column name.
    #[prost(string, tag="1")]
    pub column_name: ::prost::alloc::string::String,
    /// The Oracle data type.
    #[prost(string, tag="2")]
    pub data_type: ::prost::alloc::string::String,
    /// Column length.
    #[prost(int32, tag="3")]
    pub length: i32,
    /// Column precision.
    #[prost(int32, tag="4")]
    pub precision: i32,
    /// Column scale.
    #[prost(int32, tag="5")]
    pub scale: i32,
    /// Column encoding.
    #[prost(string, tag="6")]
    pub encoding: ::prost::alloc::string::String,
    /// Whether or not the column represents a primary key.
    #[prost(bool, tag="7")]
    pub primary_key: bool,
    /// Whether or not the column can accept a null value.
    #[prost(bool, tag="8")]
    pub nullable: bool,
    /// The ordinal position of the column in the table.
    #[prost(int32, tag="9")]
    pub ordinal_position: i32,
}
/// Oracle table.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OracleTable {
    /// Table name.
    #[prost(string, tag="1")]
    pub table_name: ::prost::alloc::string::String,
    /// Oracle columns in the schema.
    /// When unspecified as part of inclue/exclude lists, includes/excludes
    /// everything.
    #[prost(message, repeated, tag="2")]
    pub oracle_columns: ::prost::alloc::vec::Vec<OracleColumn>,
}
/// Oracle schema.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OracleSchema {
    /// Schema name.
    #[prost(string, tag="1")]
    pub schema_name: ::prost::alloc::string::String,
    /// Tables in the schema.
    #[prost(message, repeated, tag="2")]
    pub oracle_tables: ::prost::alloc::vec::Vec<OracleTable>,
}
/// Oracle database structure.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OracleRdbms {
    /// Oracle schemas/databases in the database server.
    #[prost(message, repeated, tag="1")]
    pub oracle_schemas: ::prost::alloc::vec::Vec<OracleSchema>,
}
/// Oracle data source configuration
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OracleSourceConfig {
    /// Oracle objects to include in the stream.
    #[prost(message, optional, tag="1")]
    pub allowlist: ::core::option::Option<OracleRdbms>,
    /// Oracle objects to exclude from the stream.
    #[prost(message, optional, tag="2")]
    pub rejectlist: ::core::option::Option<OracleRdbms>,
}
/// MySQL Column.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MysqlColumn {
    /// Column name.
    #[prost(string, tag="1")]
    pub column_name: ::prost::alloc::string::String,
    /// The MySQL data type. Full data types list can be found here:
    /// <https://dev.mysql.com/doc/refman/8.0/en/data-types.html>
    #[prost(string, tag="2")]
    pub data_type: ::prost::alloc::string::String,
    /// Column length.
    #[prost(int32, tag="3")]
    pub length: i32,
    /// Column collation.
    #[prost(string, tag="4")]
    pub collation: ::prost::alloc::string::String,
    /// Whether or not the column represents a primary key.
    #[prost(bool, tag="5")]
    pub primary_key: bool,
    /// Whether or not the column can accept a null value.
    #[prost(bool, tag="6")]
    pub nullable: bool,
    /// The ordinal position of the column in the table.
    #[prost(int32, tag="7")]
    pub ordinal_position: i32,
}
/// MySQL table.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MysqlTable {
    /// Table name.
    #[prost(string, tag="1")]
    pub table_name: ::prost::alloc::string::String,
    /// MySQL columns in the database.
    /// When unspecified as part of include/exclude lists, includes/excludes
    /// everything.
    #[prost(message, repeated, tag="2")]
    pub mysql_columns: ::prost::alloc::vec::Vec<MysqlColumn>,
}
/// MySQL database.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MysqlDatabase {
    /// Database name.
    #[prost(string, tag="1")]
    pub database_name: ::prost::alloc::string::String,
    /// Tables in the database.
    #[prost(message, repeated, tag="2")]
    pub mysql_tables: ::prost::alloc::vec::Vec<MysqlTable>,
}
/// MySQL database structure
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MysqlRdbms {
    /// Mysql databases on the server
    #[prost(message, repeated, tag="1")]
    pub mysql_databases: ::prost::alloc::vec::Vec<MysqlDatabase>,
}
/// MySQL source configuration
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MysqlSourceConfig {
    /// MySQL objects to retrieve from the source.
    #[prost(message, optional, tag="1")]
    pub allowlist: ::core::option::Option<MysqlRdbms>,
    /// MySQL objects to exclude from the stream.
    #[prost(message, optional, tag="2")]
    pub rejectlist: ::core::option::Option<MysqlRdbms>,
}
/// The configuration of the stream source.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceConfig {
    /// Required. Source connection profile identifier.
    #[prost(string, tag="1")]
    pub source_connection_profile_name: ::prost::alloc::string::String,
    /// Stream configuration that is specific to the data source type.
    #[prost(oneof="source_config::SourceStreamConfig", tags="100, 101")]
    pub source_stream_config: ::core::option::Option<source_config::SourceStreamConfig>,
}
/// Nested message and enum types in `SourceConfig`.
pub mod source_config {
    /// Stream configuration that is specific to the data source type.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SourceStreamConfig {
        /// Oracle data source configuration
        #[prost(message, tag="100")]
        OracleSourceConfig(super::OracleSourceConfig),
        /// MySQL data source configuration
        #[prost(message, tag="101")]
        MysqlSourceConfig(super::MysqlSourceConfig),
    }
}
/// AVRO file format configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AvroFileFormat {
}
/// JSON file format configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JsonFileFormat {
    /// The schema file format along JSON data files.
    #[prost(enumeration="SchemaFileFormat", tag="1")]
    pub schema_file_format: i32,
    /// Compression of the loaded JSON file.
    #[prost(enumeration="json_file_format::JsonCompression", tag="2")]
    pub compression: i32,
}
/// Nested message and enum types in `JsonFileFormat`.
pub mod json_file_format {
    /// Json file compression.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum JsonCompression {
        /// Unspecified json file compression.
        Unspecified = 0,
        /// Do not compress JSON file.
        NoCompression = 1,
        /// Gzip compression.
        Gzip = 2,
    }
    impl JsonCompression {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                JsonCompression::Unspecified => "JSON_COMPRESSION_UNSPECIFIED",
                JsonCompression::NoCompression => "NO_COMPRESSION",
                JsonCompression::Gzip => "GZIP",
            }
        }
    }
}
/// Google Cloud Storage destination configuration
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsDestinationConfig {
    /// Path inside the Cloud Storage bucket to write data to.
    #[prost(string, tag="1")]
    pub path: ::prost::alloc::string::String,
    /// File format that data should be written in.
    /// Deprecated field - use file_format instead.
    #[deprecated]
    #[prost(enumeration="GcsFileFormat", tag="2")]
    pub gcs_file_format: i32,
    /// The maximum file size to be saved in the bucket.
    #[prost(int32, tag="3")]
    pub file_rotation_mb: i32,
    /// The maximum duration for which new events are added before a file is
    /// closed and a new file is created.
    #[prost(message, optional, tag="4")]
    pub file_rotation_interval: ::core::option::Option<::prost_types::Duration>,
    /// File Format that the data should be written in.
    #[prost(oneof="gcs_destination_config::FileFormat", tags="100, 101")]
    pub file_format: ::core::option::Option<gcs_destination_config::FileFormat>,
}
/// Nested message and enum types in `GcsDestinationConfig`.
pub mod gcs_destination_config {
    /// File Format that the data should be written in.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum FileFormat {
        /// AVRO file format configuration.
        #[prost(message, tag="100")]
        AvroFileFormat(super::AvroFileFormat),
        /// JSON file format configuration.
        #[prost(message, tag="101")]
        JsonFileFormat(super::JsonFileFormat),
    }
}
/// The configuration of the stream destination.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestinationConfig {
    /// Required. Destination connection profile identifier.
    #[prost(string, tag="1")]
    pub destination_connection_profile_name: ::prost::alloc::string::String,
    /// Stream configuration that is specific to the data destination type.
    #[prost(oneof="destination_config::DestinationStreamConfig", tags="100")]
    pub destination_stream_config: ::core::option::Option<destination_config::DestinationStreamConfig>,
}
/// Nested message and enum types in `DestinationConfig`.
pub mod destination_config {
    /// Stream configuration that is specific to the data destination type.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DestinationStreamConfig {
        #[prost(message, tag="100")]
        GcsDestinationConfig(super::GcsDestinationConfig),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stream {
    /// Output only. The stream's name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The creation time of the stream.
    #[prost(message, optional, tag="2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last update time of the stream.
    #[prost(message, optional, tag="3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels.
    #[prost(btree_map="string, string", tag="4")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Required. Display name.
    #[prost(string, tag="5")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. Source connection profile configuration.
    #[prost(message, optional, tag="6")]
    pub source_config: ::core::option::Option<SourceConfig>,
    /// Required. Destination connection profile configuration.
    #[prost(message, optional, tag="7")]
    pub destination_config: ::core::option::Option<DestinationConfig>,
    /// The state of the stream.
    #[prost(enumeration="stream::State", tag="8")]
    pub state: i32,
    /// Output only. Errors on the Stream.
    #[prost(message, repeated, tag="9")]
    pub errors: ::prost::alloc::vec::Vec<Error>,
    /// Stream backfill strategy.
    #[prost(oneof="stream::BackfillStrategy", tags="101, 102")]
    pub backfill_strategy: ::core::option::Option<stream::BackfillStrategy>,
}
/// Nested message and enum types in `Stream`.
pub mod stream {
    /// Backfill strategy to automatically backfill the Stream's objects.
    /// Specific objects can be excluded.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BackfillAllStrategy {
        /// List of objects to exclude.
        #[prost(oneof="backfill_all_strategy::ExcludedObjects", tags="1, 2")]
        pub excluded_objects: ::core::option::Option<backfill_all_strategy::ExcludedObjects>,
    }
    /// Nested message and enum types in `BackfillAllStrategy`.
    pub mod backfill_all_strategy {
        /// List of objects to exclude.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ExcludedObjects {
            /// Oracle data source objects to avoid backfilling.
            #[prost(message, tag="1")]
            OracleExcludedObjects(super::super::OracleRdbms),
            /// MySQL data source objects to avoid backfilling.
            #[prost(message, tag="2")]
            MysqlExcludedObjects(super::super::MysqlRdbms),
        }
    }
    /// Backfill strategy to disable automatic backfill for the Stream's objects.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BackfillNoneStrategy {
    }
    /// Stream state.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified stream state.
        Unspecified = 0,
        /// The stream has been created.
        Created = 1,
        /// The stream is running.
        Running = 2,
        /// The stream is paused.
        Paused = 3,
        /// The stream is in maintenance mode.
        ///
        /// Updates are rejected on the resource in this state.
        Maintenance = 4,
        /// The stream is experiencing an error that is preventing data from being
        /// streamed.
        Failed = 5,
        /// The stream has experienced a terminal failure.
        FailedPermanently = 6,
        /// The stream is starting, but not yet running.
        Starting = 7,
        /// The Stream is no longer reading new events, but still writing events in
        /// the buffer.
        Draining = 8,
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
                State::Running => "RUNNING",
                State::Paused => "PAUSED",
                State::Maintenance => "MAINTENANCE",
                State::Failed => "FAILED",
                State::FailedPermanently => "FAILED_PERMANENTLY",
                State::Starting => "STARTING",
                State::Draining => "DRAINING",
            }
        }
    }
    /// Stream backfill strategy.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum BackfillStrategy {
        /// Automatically backfill objects included in the stream source
        /// configuration. Specific objects can be excluded.
        #[prost(message, tag="101")]
        BackfillAll(BackfillAllStrategy),
        /// Do not automatically backfill any objects.
        #[prost(message, tag="102")]
        BackfillNone(BackfillNoneStrategy),
    }
}
/// Represent a user-facing Error.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    /// A title that explains the reason for the error.
    #[prost(string, tag="1")]
    pub reason: ::prost::alloc::string::String,
    /// A unique identifier for this specific error,
    /// allowing it to be traced throughout the system in logs and API responses.
    #[prost(string, tag="2")]
    pub error_uuid: ::prost::alloc::string::String,
    /// A message containing more information about the error that occurred.
    #[prost(string, tag="3")]
    pub message: ::prost::alloc::string::String,
    /// The time when the error occurred.
    #[prost(message, optional, tag="4")]
    pub error_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Additional information about the error.
    #[prost(btree_map="string, string", tag="5")]
    pub details: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Contains the current validation results.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidationResult {
    /// A list of validations (includes both executed as well as not executed
    /// validations).
    #[prost(message, repeated, tag="1")]
    pub validations: ::prost::alloc::vec::Vec<Validation>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Validation {
    /// A short description of the validation.
    #[prost(string, tag="1")]
    pub description: ::prost::alloc::string::String,
    /// Validation execution status.
    #[prost(enumeration="validation::Status", tag="2")]
    pub status: i32,
    /// Messages reflecting the validation results.
    #[prost(message, repeated, tag="3")]
    pub message: ::prost::alloc::vec::Vec<ValidationMessage>,
    /// A custom code identifying this validation.
    #[prost(string, tag="4")]
    pub code: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Validation`.
pub mod validation {
    /// Validation execution status.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        /// Unspecified status.
        Unspecified = 0,
        /// Validation did not execute.
        NotExecuted = 1,
        /// Validation failed.
        Failed = 2,
        /// Validation passed.
        Passed = 3,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::Unspecified => "STATUS_UNSPECIFIED",
                Status::NotExecuted => "NOT_EXECUTED",
                Status::Failed => "FAILED",
                Status::Passed => "PASSED",
            }
        }
    }
}
/// Represent user-facing validation result message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidationMessage {
    /// The result of the validation.
    #[prost(string, tag="1")]
    pub message: ::prost::alloc::string::String,
    /// Message severity level (warning or error).
    #[prost(enumeration="validation_message::Level", tag="2")]
    pub level: i32,
    /// Additional metadata related to the result.
    #[prost(btree_map="string, string", tag="3")]
    pub metadata: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// A custom code identifying this specific message.
    #[prost(string, tag="4")]
    pub code: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ValidationMessage`.
pub mod validation_message {
    /// Validation message level.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Level {
        /// Unspecified level.
        Unspecified = 0,
        /// Potentially cause issues with the Stream.
        Warning = 1,
        /// Definitely cause issues with the Stream.
        Error = 2,
    }
    impl Level {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Level::Unspecified => "LEVEL_UNSPECIFIED",
                Level::Warning => "WARNING",
                Level::Error => "ERROR",
            }
        }
    }
}
/// File format in Cloud Storage.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GcsFileFormat {
    /// Unspecified Cloud Storage file format.
    Unspecified = 0,
    /// Avro file format
    Avro = 1,
}
impl GcsFileFormat {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            GcsFileFormat::Unspecified => "GCS_FILE_FORMAT_UNSPECIFIED",
            GcsFileFormat::Avro => "AVRO",
        }
    }
}
/// Schema file format.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SchemaFileFormat {
    /// Unspecified schema file format.
    Unspecified = 0,
    /// Do not attach schema file.
    NoSchemaFile = 1,
    /// Avro schema format.
    AvroSchemaFile = 2,
}
impl SchemaFileFormat {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SchemaFileFormat::Unspecified => "SCHEMA_FILE_FORMAT_UNSPECIFIED",
            SchemaFileFormat::NoSchemaFile => "NO_SCHEMA_FILE",
            SchemaFileFormat::AvroSchemaFile => "AVRO_SCHEMA_FILE",
        }
    }
}
/// Request message for 'discover' ConnectionProfile request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscoverConnectionProfileRequest {
    /// Required. The parent resource of the ConnectionProfile type. Must be in the
    /// format `projects/*/locations/*`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The connection profile on which to run discover.
    #[prost(oneof="discover_connection_profile_request::Target", tags="200, 201")]
    pub target: ::core::option::Option<discover_connection_profile_request::Target>,
    #[prost(oneof="discover_connection_profile_request::Depth", tags="3, 4")]
    pub depth: ::core::option::Option<discover_connection_profile_request::Depth>,
    /// The data object to enrich with child data objects and metadata.
    #[prost(oneof="discover_connection_profile_request::DataObject", tags="100, 101")]
    pub data_object: ::core::option::Option<discover_connection_profile_request::DataObject>,
}
/// Nested message and enum types in `DiscoverConnectionProfileRequest`.
pub mod discover_connection_profile_request {
    /// The connection profile on which to run discover.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Target {
        /// An ad-hoc ConnectionProfile configuration.
        #[prost(message, tag="200")]
        ConnectionProfile(super::ConnectionProfile),
        /// A reference to an existing ConnectionProfile.
        #[prost(string, tag="201")]
        ConnectionProfileName(::prost::alloc::string::String),
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Depth {
        /// Whether to retrieve the full hierarchy of data objects (TRUE) or only the
        /// current level (FALSE).
        #[prost(bool, tag="3")]
        Recursive(bool),
        /// The number of hierarchy levels below the current level to be retrieved.
        #[prost(int32, tag="4")]
        RecursionDepth(i32),
    }
    /// The data object to enrich with child data objects and metadata.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DataObject {
        /// Oracle RDBMS to enrich with child data objects and metadata.
        #[prost(message, tag="100")]
        OracleRdbms(super::OracleRdbms),
        /// MySQL RDBMS to enrich with child data objects and metadata.
        #[prost(message, tag="101")]
        MysqlRdbms(super::MysqlRdbms),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscoverConnectionProfileResponse {
    /// The data object that has been enriched by the discover API call.
    #[prost(oneof="discover_connection_profile_response::DataObject", tags="100, 101")]
    pub data_object: ::core::option::Option<discover_connection_profile_response::DataObject>,
}
/// Nested message and enum types in `DiscoverConnectionProfileResponse`.
pub mod discover_connection_profile_response {
    /// The data object that has been enriched by the discover API call.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DataObject {
        /// Enriched Oracle RDBMS object.
        #[prost(message, tag="100")]
        OracleRdbms(super::OracleRdbms),
        /// Enriched MySQL RDBMS object.
        #[prost(message, tag="101")]
        MysqlRdbms(super::MysqlRdbms),
    }
}
/// Request message for 'FetchStaticIps' request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchStaticIpsRequest {
    /// Required. The name resource of the Response type. Must be in the
    /// format `projects/*/locations/*`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Maximum number of Ips to return, will likely not be specified.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListStaticIps` call.
    /// will likely not be specified.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for a 'FetchStaticIps' response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchStaticIpsResponse {
    /// list of static ips by account
    #[prost(string, repeated, tag="1")]
    pub static_ips: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A token that can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for 'FetchErrors' request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchErrorsRequest {
    /// Name of the Stream resource for which to fetch any errors.
    #[prost(string, tag="1")]
    pub stream: ::prost::alloc::string::String,
}
/// Response message for a 'FetchErrors' response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchErrorsResponse {
    /// The list of errors on the Stream.
    #[prost(message, repeated, tag="1")]
    pub errors: ::prost::alloc::vec::Vec<Error>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectionProfilesRequest {
    /// Required. The parent that owns the collection of connection profiles.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of connection profiles to return.
    /// If unspecified, at most 50 connection profiles will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Page token received from a previous `ListConnectionProfiles` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListConnectionProfiles`
    /// must match the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filter request.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
    /// Order by fields for the result.
    #[prost(string, tag="5")]
    pub order_by: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectionProfilesResponse {
    /// List of connection profiles.
    #[prost(message, repeated, tag="1")]
    pub connection_profiles: ::prost::alloc::vec::Vec<ConnectionProfile>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag="3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConnectionProfileRequest {
    /// Required. The name of the connection profile resource to get.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateConnectionProfileRequest {
    /// Required. The parent that owns the collection of ConnectionProfiles.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The connection profile identifier.
    #[prost(string, tag="2")]
    pub connection_profile_id: ::prost::alloc::string::String,
    /// Required. The connection profile resource to create.
    #[prost(message, optional, tag="3")]
    pub connection_profile: ::core::option::Option<ConnectionProfile>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
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
    #[prost(string, tag="4")]
    pub request_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateConnectionProfileRequest {
    /// Optional. Field mask is used to specify the fields to be overwritten in the
    /// ConnectionProfile resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag="1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The ConnectionProfile to update.
    #[prost(message, optional, tag="2")]
    pub connection_profile: ::core::option::Option<ConnectionProfile>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
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
    #[prost(string, tag="3")]
    pub request_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteConnectionProfileRequest {
    /// Required. The name of the connection profile resource to delete.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID
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
    #[prost(string, tag="2")]
    pub request_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStreamsRequest {
    /// Required. The parent that owns the collection of streams.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of streams to return.
    /// If unspecified, at most 50 streams will  be returned. The maximum
    /// value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Page token received from a previous `ListStreams` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListStreams`
    /// must match the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filter request.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
    /// Order by fields for the result.
    #[prost(string, tag="5")]
    pub order_by: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStreamsResponse {
    /// List of streams
    #[prost(message, repeated, tag="1")]
    pub streams: ::prost::alloc::vec::Vec<Stream>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag="3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStreamRequest {
    /// Required. The name of the stream resource to get.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateStreamRequest {
    /// Required. The parent that owns the collection of streams.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The stream identifier.
    #[prost(string, tag="2")]
    pub stream_id: ::prost::alloc::string::String,
    /// Required. The stream resource to create.
    #[prost(message, optional, tag="3")]
    pub stream: ::core::option::Option<Stream>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
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
    #[prost(string, tag="4")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. Only validate the stream, but do not create any resources.
    /// The default is false.
    #[prost(bool, tag="5")]
    pub validate_only: bool,
    /// Optional. Create the stream without validating it.
    #[prost(bool, tag="6")]
    pub force: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateStreamRequest {
    /// Optional. Field mask is used to specify the fields to be overwritten in the
    /// stream resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag="1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The stream resource to update.
    #[prost(message, optional, tag="2")]
    pub stream: ::core::option::Option<Stream>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
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
    #[prost(string, tag="3")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. Only validate the stream with the changes, without actually updating it.
    /// The default is false.
    #[prost(bool, tag="4")]
    pub validate_only: bool,
    /// Optional. Execute the update without validating it.
    #[prost(bool, tag="5")]
    pub force: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteStreamRequest {
    /// Required. The name of the stream resource to delete.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID
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
    #[prost(string, tag="2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Represents the metadata of the long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Output only. The time the operation was created.
    #[prost(message, optional, tag="1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the operation finished running.
    #[prost(message, optional, tag="2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Server-defined resource path for the target of the operation.
    #[prost(string, tag="3")]
    pub target: ::prost::alloc::string::String,
    /// Output only. Name of the verb executed by the operation.
    #[prost(string, tag="4")]
    pub verb: ::prost::alloc::string::String,
    /// Output only. Human-readable status of the operation, if any.
    #[prost(string, tag="5")]
    pub status_message: ::prost::alloc::string::String,
    /// Output only. Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have \[Operation.error][\] value with a \[google.rpc.Status.code][google.rpc.Status.code\] of 1,
    /// corresponding to `Code.CANCELLED`.
    #[prost(bool, tag="6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag="7")]
    pub api_version: ::prost::alloc::string::String,
    /// Output only. Results of executed validations if there are any.
    #[prost(message, optional, tag="8")]
    pub validation_result: ::core::option::Option<ValidationResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePrivateConnectionRequest {
    /// Required. The parent that owns the collection of PrivateConnections.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The private connectivity identifier.
    #[prost(string, tag="2")]
    pub private_connection_id: ::prost::alloc::string::String,
    /// Required. The Private Connectivity resource to create.
    #[prost(message, optional, tag="3")]
    pub private_connection: ::core::option::Option<PrivateConnection>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
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
    #[prost(string, tag="4")]
    pub request_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPrivateConnectionsRequest {
    /// Required. The parent that owns the collection of private connectivity configurations.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of private connectivity configurations to return.
    /// If unspecified, at most 50 private connectivity configurations that will be
    /// returned. The maximum value is 1000; values above 1000 will be coerced to
    /// 1000.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Page token received from a previous `ListPrivateConnections` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `ListPrivateConnections` must match the call that provided the page
    /// token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filter request.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
    /// Order by fields for the result.
    #[prost(string, tag="5")]
    pub order_by: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPrivateConnectionsResponse {
    /// List of private connectivity configurations.
    #[prost(message, repeated, tag="1")]
    pub private_connections: ::prost::alloc::vec::Vec<PrivateConnection>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag="3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePrivateConnectionRequest {
    /// Required. The name of the private connectivity configuration to delete.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID
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
    #[prost(string, tag="2")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set to true, any child routes that belong to this PrivateConnection will
    /// also be deleted.
    #[prost(bool, tag="3")]
    pub force: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPrivateConnectionRequest {
    /// Required. The name of the  private connectivity configuration to get.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// route creation request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRouteRequest {
    /// Required. The parent that owns the collection of Routes.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The Route identifier.
    #[prost(string, tag="2")]
    pub route_id: ::prost::alloc::string::String,
    /// Required. The Route resource to create.
    #[prost(message, optional, tag="3")]
    pub route: ::core::option::Option<Route>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
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
    #[prost(string, tag="4")]
    pub request_id: ::prost::alloc::string::String,
}
/// route list request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRoutesRequest {
    /// Required. The parent that owns the collection of Routess.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of Routes to return. The service may return
    /// fewer than this value. If unspecified, at most 50 Routes
    /// will be returned. The maximum value is 1000; values above 1000 will be
    /// coerced to 1000.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Page token received from a previous `ListRoutes` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `ListRoutes` must match the call that provided the page
    /// token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filter request.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
    /// Order by fields for the result.
    #[prost(string, tag="5")]
    pub order_by: ::prost::alloc::string::String,
}
/// route list response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRoutesResponse {
    /// List of Routes.
    #[prost(message, repeated, tag="1")]
    pub routes: ::prost::alloc::vec::Vec<Route>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag="3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// route deletion request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRouteRequest {
    /// Required. The name of the Route resource to delete.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID
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
    #[prost(string, tag="2")]
    pub request_id: ::prost::alloc::string::String,
}
/// route get request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRouteRequest {
    /// Required. The name of the Route resource to get.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod datastream_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Datastream service
    #[derive(Debug, Clone)]
    pub struct DatastreamClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DatastreamClient<T>
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
        ) -> DatastreamClient<InterceptedService<T, F>>
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
            DatastreamClient::new(InterceptedService::new(inner, interceptor))
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
        /// Use this method to list connection profiles created in a project and
        /// location.
        pub async fn list_connection_profiles(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConnectionProfilesRequest>,
        ) -> Result<
            tonic::Response<super::ListConnectionProfilesResponse>,
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
                "/google.cloud.datastream.v1alpha1.Datastream/ListConnectionProfiles",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Use this method to get details about a connection profile.
        pub async fn get_connection_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConnectionProfileRequest>,
        ) -> Result<tonic::Response<super::ConnectionProfile>, tonic::Status> {
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
                "/google.cloud.datastream.v1alpha1.Datastream/GetConnectionProfile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Use this method to create a connection profile in a project and location.
        pub async fn create_connection_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateConnectionProfileRequest>,
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
                "/google.cloud.datastream.v1alpha1.Datastream/CreateConnectionProfile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Use this method to update the parameters of a connection profile.
        pub async fn update_connection_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateConnectionProfileRequest>,
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
                "/google.cloud.datastream.v1alpha1.Datastream/UpdateConnectionProfile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Use this method to delete a connection profile..
        pub async fn delete_connection_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteConnectionProfileRequest>,
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
                "/google.cloud.datastream.v1alpha1.Datastream/DeleteConnectionProfile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Use this method to discover a connection profile.
        /// The discover API call exposes the data objects and metadata belonging to
        /// the profile. Typically, a request returns children data objects under a
        /// parent data object that's optionally supplied in the request.
        pub async fn discover_connection_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::DiscoverConnectionProfileRequest>,
        ) -> Result<
            tonic::Response<super::DiscoverConnectionProfileResponse>,
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
                "/google.cloud.datastream.v1alpha1.Datastream/DiscoverConnectionProfile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Use this method to list streams in a project and location.
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
                "/google.cloud.datastream.v1alpha1.Datastream/ListStreams",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Use this method to get details about a stream.
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
                "/google.cloud.datastream.v1alpha1.Datastream/GetStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Use this method to create a stream.
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
                "/google.cloud.datastream.v1alpha1.Datastream/CreateStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Use this method to update the configuration of a stream.
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
                "/google.cloud.datastream.v1alpha1.Datastream/UpdateStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Use this method to delete a stream.
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
                "/google.cloud.datastream.v1alpha1.Datastream/DeleteStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Use this method to fetch any errors associated with a stream.
        pub async fn fetch_errors(
            &mut self,
            request: impl tonic::IntoRequest<super::FetchErrorsRequest>,
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
                "/google.cloud.datastream.v1alpha1.Datastream/FetchErrors",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// The FetchStaticIps API call exposes the static ips used by Datastream.
        /// Typically, a request returns children data objects under
        /// a parent data object that's optionally supplied in the request.
        pub async fn fetch_static_ips(
            &mut self,
            request: impl tonic::IntoRequest<super::FetchStaticIpsRequest>,
        ) -> Result<tonic::Response<super::FetchStaticIpsResponse>, tonic::Status> {
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
                "/google.cloud.datastream.v1alpha1.Datastream/FetchStaticIps",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Use this method to create a private connectivity configuration.
        pub async fn create_private_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePrivateConnectionRequest>,
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
                "/google.cloud.datastream.v1alpha1.Datastream/CreatePrivateConnection",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Use this method to get details about a private connectivity configuration.
        pub async fn get_private_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPrivateConnectionRequest>,
        ) -> Result<tonic::Response<super::PrivateConnection>, tonic::Status> {
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
                "/google.cloud.datastream.v1alpha1.Datastream/GetPrivateConnection",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Use this method to list private connectivity configurations in a project
        /// and location.
        pub async fn list_private_connections(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPrivateConnectionsRequest>,
        ) -> Result<
            tonic::Response<super::ListPrivateConnectionsResponse>,
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
                "/google.cloud.datastream.v1alpha1.Datastream/ListPrivateConnections",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Use this method to delete a private connectivity configuration.
        pub async fn delete_private_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePrivateConnectionRequest>,
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
                "/google.cloud.datastream.v1alpha1.Datastream/DeletePrivateConnection",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Use this method to create a route for a private connectivity in a project
        /// and location.
        pub async fn create_route(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRouteRequest>,
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
                "/google.cloud.datastream.v1alpha1.Datastream/CreateRoute",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Use this method to get details about a route.
        pub async fn get_route(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRouteRequest>,
        ) -> Result<tonic::Response<super::Route>, tonic::Status> {
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
                "/google.cloud.datastream.v1alpha1.Datastream/GetRoute",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Use this method to list routes created for a private connectivity in a
        /// project and location.
        pub async fn list_routes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRoutesRequest>,
        ) -> Result<tonic::Response<super::ListRoutesResponse>, tonic::Status> {
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
                "/google.cloud.datastream.v1alpha1.Datastream/ListRoutes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Use this method to delete a route.
        pub async fn delete_route(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRouteRequest>,
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
                "/google.cloud.datastream.v1alpha1.Datastream/DeleteRoute",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
