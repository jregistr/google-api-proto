/// Request for the `ListCertificateIssuanceConfigs` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCertificateIssuanceConfigsRequest {
    /// Required. The project and location from which the certificate should be
    /// listed, specified in the format `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of certificate configs to return per call.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The value returned by the last `ListCertificateIssuanceConfigsResponse`.
    /// Indicates that this is a continuation of a prior
    /// `ListCertificateIssuanceConfigs` call, and that the system should return
    /// the next page of data.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filter expression to restrict the Certificates Configs returned.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// A list of Certificate Config field names used to specify the order of the
    /// returned results. The default sorting order is ascending. To specify
    /// descending order for a field, add a suffix " desc".
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for the `ListCertificateIssuanceConfigs` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCertificateIssuanceConfigsResponse {
    /// A list of certificate configs for the parent resource.
    #[prost(message, repeated, tag = "1")]
    pub certificate_issuance_configs: ::prost::alloc::vec::Vec<
        CertificateIssuanceConfig,
    >,
    /// If there might be more results than those appearing in this response, then
    /// `next_page_token` is included. To get the next set of results, call this
    /// method again using the value of `next_page_token` as `page_token`.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for the `GetCertificateIssuanceConfig` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCertificateIssuanceConfigRequest {
    /// Required. A name of the certificate issuance config to describe. Must be in
    /// the format `projects/*/locations/*/certificateIssuanceConfigs/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for the `CreateCertificateIssuanceConfig` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCertificateIssuanceConfigRequest {
    /// Required. The parent resource of the certificate issuance config. Must be
    /// in the format `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. A user-provided name of the certificate config.
    #[prost(string, tag = "2")]
    pub certificate_issuance_config_id: ::prost::alloc::string::String,
    /// Required. A definition of the certificate issuance config to create.
    #[prost(message, optional, tag = "3")]
    pub certificate_issuance_config: ::core::option::Option<CertificateIssuanceConfig>,
}
/// Request for the `DeleteCertificateIssuanceConfig` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCertificateIssuanceConfigRequest {
    /// Required. A name of the certificate issuance config to delete. Must be in
    /// the format `projects/*/locations/*/certificateIssuanceConfigs/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// CertificateIssuanceConfig specifies how to issue and manage a certificate.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateIssuanceConfig {
    /// A user-defined name of the certificate issuance config.
    /// CertificateIssuanceConfig names must be unique globally and match pattern
    /// `projects/*/locations/*/certificateIssuanceConfigs/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The creation timestamp of a CertificateIssuanceConfig.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last update timestamp of a CertificateIssuanceConfig.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Set of labels associated with a CertificateIssuanceConfig.
    #[prost(btree_map = "string, string", tag = "4")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// One or more paragraphs of text description of a CertificateIssuanceConfig.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Required. The CA that issues the workload certificate. It includes the CA
    /// address, type, authentication to CA service, etc.
    #[prost(message, optional, tag = "6")]
    pub certificate_authority_config: ::core::option::Option<
        certificate_issuance_config::CertificateAuthorityConfig,
    >,
    /// Required. Workload certificate lifetime requested.
    #[prost(message, optional, tag = "7")]
    pub lifetime: ::core::option::Option<::prost_types::Duration>,
    /// Required. Specifies the percentage of elapsed time of the certificate
    /// lifetime to wait before renewing the certificate. Must be a number between
    /// 1-99, inclusive.
    #[prost(int32, tag = "8")]
    pub rotation_window_percentage: i32,
    /// Required. The key algorithm to use when generating the private key.
    #[prost(enumeration = "certificate_issuance_config::KeyAlgorithm", tag = "9")]
    pub key_algorithm: i32,
}
/// Nested message and enum types in `CertificateIssuanceConfig`.
pub mod certificate_issuance_config {
    /// The CA that issues the workload certificate. It includes CA address, type,
    /// authentication to CA service, etc.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CertificateAuthorityConfig {
        #[prost(oneof = "certificate_authority_config::Kind", tags = "1")]
        pub kind: ::core::option::Option<certificate_authority_config::Kind>,
    }
    /// Nested message and enum types in `CertificateAuthorityConfig`.
    pub mod certificate_authority_config {
        /// Contains information required to contact CA service.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CertificateAuthorityServiceConfig {
            /// Required. A CA pool resource used to issue a certificate.
            /// The CA pool string has a relative resource path following the form
            /// "projects/{project}/locations/{location}/caPools/{ca_pool}".
            #[prost(string, tag = "1")]
            pub ca_pool: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Kind {
            /// Defines a CertificateAuthorityServiceConfig.
            #[prost(message, tag = "1")]
            CertificateAuthorityServiceConfig(CertificateAuthorityServiceConfig),
        }
    }
    /// The type of keypair to generate.
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
    pub enum KeyAlgorithm {
        /// Unspecified key algorithm.
        Unspecified = 0,
        /// Specifies RSA with a 2048-bit modulus.
        Rsa2048 = 1,
        /// Specifies ECDSA with curve P256.
        EcdsaP256 = 4,
    }
    impl KeyAlgorithm {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                KeyAlgorithm::Unspecified => "KEY_ALGORITHM_UNSPECIFIED",
                KeyAlgorithm::Rsa2048 => "RSA_2048",
                KeyAlgorithm::EcdsaP256 => "ECDSA_P256",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "KEY_ALGORITHM_UNSPECIFIED" => Some(Self::Unspecified),
                "RSA_2048" => Some(Self::Rsa2048),
                "ECDSA_P256" => Some(Self::EcdsaP256),
                _ => None,
            }
        }
    }
}
/// Request for the `ListCertificates` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCertificatesRequest {
    /// Required. The project and location from which the certificate should be
    /// listed, specified in the format `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of certificates to return per call.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The value returned by the last `ListCertificatesResponse`. Indicates that
    /// this is a continuation of a prior `ListCertificates` call, and that the
    /// system should return the next page of data.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filter expression to restrict the Certificates returned.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// A list of Certificate field names used to specify the order of the returned
    /// results. The default sorting order is ascending. To specify descending
    /// order for a field, add a suffix " desc".
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for the `ListCertificates` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCertificatesResponse {
    /// A list of certificates for the parent resource.
    #[prost(message, repeated, tag = "1")]
    pub certificates: ::prost::alloc::vec::Vec<Certificate>,
    /// If there might be more results than those appearing in this response, then
    /// `next_page_token` is included. To get the next set of results, call this
    /// method again using the value of `next_page_token` as `page_token`.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// A list of locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for the `GetCertificate` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCertificateRequest {
    /// Required. A name of the certificate to describe. Must be in the format
    /// `projects/*/locations/*/certificates/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for the `CreateCertificate` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCertificateRequest {
    /// Required. The parent resource of the certificate. Must be in the format
    /// `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. A user-provided name of the certificate.
    #[prost(string, tag = "2")]
    pub certificate_id: ::prost::alloc::string::String,
    /// Required. A definition of the certificate to create.
    #[prost(message, optional, tag = "3")]
    pub certificate: ::core::option::Option<Certificate>,
}
/// Request for the `UpdateCertificate` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCertificateRequest {
    /// Required. A definition of the certificate to update.
    #[prost(message, optional, tag = "1")]
    pub certificate: ::core::option::Option<Certificate>,
    /// Required. The update mask applies to the resource. For the `FieldMask`
    /// definition, see
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask.>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request for the `DeleteCertificate` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCertificateRequest {
    /// Required. A name of the certificate to delete. Must be in the format
    /// `projects/*/locations/*/certificates/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for the `ListCertificateMaps` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCertificateMapsRequest {
    /// Required. The project and location from which the certificate maps should
    /// be listed, specified in the format `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of certificate maps to return per call.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The value returned by the last `ListCertificateMapsResponse`. Indicates
    /// that this is a continuation of a prior `ListCertificateMaps` call, and that
    /// the system should return the next page of data.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filter expression to restrict the Certificates Maps returned.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// A list of Certificate Map field names used to specify the order of the
    /// returned results. The default sorting order is ascending. To specify
    /// descending order for a field, add a suffix " desc".
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for the `ListCertificateMaps` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCertificateMapsResponse {
    /// A list of certificate maps for the parent resource.
    #[prost(message, repeated, tag = "1")]
    pub certificate_maps: ::prost::alloc::vec::Vec<CertificateMap>,
    /// If there might be more results than those appearing in this response, then
    /// `next_page_token` is included. To get the next set of results, call this
    /// method again using the value of `next_page_token` as `page_token`.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for the `GetCertificateMap` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCertificateMapRequest {
    /// Required. A name of the certificate map to describe. Must be in the format
    /// `projects/*/locations/*/certificateMaps/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for the `CreateCertificateMap` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCertificateMapRequest {
    /// Required. The parent resource of the certificate map. Must be in the format
    /// `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. A user-provided name of the certificate map.
    #[prost(string, tag = "2")]
    pub certificate_map_id: ::prost::alloc::string::String,
    /// Required. A definition of the certificate map to create.
    #[prost(message, optional, tag = "3")]
    pub certificate_map: ::core::option::Option<CertificateMap>,
}
/// Request for the `UpdateCertificateMap` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCertificateMapRequest {
    /// Required. A definition of the certificate map to update.
    #[prost(message, optional, tag = "1")]
    pub certificate_map: ::core::option::Option<CertificateMap>,
    /// Required. The update mask applies to the resource. For the `FieldMask`
    /// definition, see
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask.>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request for the `DeleteCertificateMap` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCertificateMapRequest {
    /// Required. A name of the certificate map to delete. Must be in the format
    /// `projects/*/locations/*/certificateMaps/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for the `ListCertificateMapEntries` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCertificateMapEntriesRequest {
    /// Required. The project, location and certificate map from which the
    /// certificate map entries should be listed, specified in the format
    /// `projects/*/locations/*/certificateMaps/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of certificate map entries to return. The service may return
    /// fewer than this value.
    /// If unspecified, at most 50 certificate map entries will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The value returned by the last `ListCertificateMapEntriesResponse`.
    /// Indicates that this is a continuation of a prior
    /// `ListCertificateMapEntries` call, and that the system should return the
    /// next page of data.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filter expression to restrict the returned Certificate Map Entries.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// A list of Certificate Map Entry field names used to specify
    /// the order of the returned results. The default sorting order is ascending.
    /// To specify descending order for a field, add a suffix " desc".
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for the `ListCertificateMapEntries` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCertificateMapEntriesResponse {
    /// A list of certificate map entries for the parent resource.
    #[prost(message, repeated, tag = "1")]
    pub certificate_map_entries: ::prost::alloc::vec::Vec<CertificateMapEntry>,
    /// If there might be more results than those appearing in this response, then
    /// `next_page_token` is included. To get the next set of results, call this
    /// method again using the value of `next_page_token` as `page_token`.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for the `GetCertificateMapEntry` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCertificateMapEntryRequest {
    /// Required. A name of the certificate map entry to describe. Must be in the
    /// format `projects/*/locations/*/certificateMaps/*/certificateMapEntries/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for the `CreateCertificateMapEntry` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCertificateMapEntryRequest {
    /// Required. The parent resource of the certificate map entry. Must be in the
    /// format `projects/*/locations/*/certificateMaps/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. A user-provided name of the certificate map entry.
    #[prost(string, tag = "2")]
    pub certificate_map_entry_id: ::prost::alloc::string::String,
    /// Required. A definition of the certificate map entry to create.
    #[prost(message, optional, tag = "3")]
    pub certificate_map_entry: ::core::option::Option<CertificateMapEntry>,
}
/// Request for the `UpdateCertificateMapEntry` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCertificateMapEntryRequest {
    /// Required. A definition of the certificate map entry to create map entry.
    #[prost(message, optional, tag = "1")]
    pub certificate_map_entry: ::core::option::Option<CertificateMapEntry>,
    /// Required. The update mask applies to the resource. For the `FieldMask`
    /// definition, see
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask.>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request for the `DeleteCertificateMapEntry` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCertificateMapEntryRequest {
    /// Required. A name of the certificate map entry to delete. Must be in the
    /// format `projects/*/locations/*/certificateMaps/*/certificateMapEntries/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for the `ListDnsAuthorizations` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDnsAuthorizationsRequest {
    /// Required. The project and location from which the dns authorizations should
    /// be listed, specified in the format `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of dns authorizations to return per call.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The value returned by the last `ListDnsAuthorizationsResponse`. Indicates
    /// that this is a continuation of a prior `ListDnsAuthorizations` call, and
    /// that the system should return the next page of data.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filter expression to restrict the Dns Authorizations returned.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// A list of Dns Authorization field names used to specify the order of the
    /// returned results. The default sorting order is ascending. To specify
    /// descending order for a field, add a suffix " desc".
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for the `ListDnsAuthorizations` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDnsAuthorizationsResponse {
    /// A list of dns authorizations for the parent resource.
    #[prost(message, repeated, tag = "1")]
    pub dns_authorizations: ::prost::alloc::vec::Vec<DnsAuthorization>,
    /// If there might be more results than those appearing in this response, then
    /// `next_page_token` is included. To get the next set of results, call this
    /// method again using the value of `next_page_token` as `page_token`.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for the `GetDnsAuthorization` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDnsAuthorizationRequest {
    /// Required. A name of the dns authorization to describe. Must be in the
    /// format `projects/*/locations/*/dnsAuthorizations/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for the `CreateDnsAuthorization` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDnsAuthorizationRequest {
    /// Required. The parent resource of the dns authorization. Must be in the
    /// format `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. A user-provided name of the dns authorization.
    #[prost(string, tag = "2")]
    pub dns_authorization_id: ::prost::alloc::string::String,
    /// Required. A definition of the dns authorization to create.
    #[prost(message, optional, tag = "3")]
    pub dns_authorization: ::core::option::Option<DnsAuthorization>,
}
/// Request for the `UpdateDnsAuthorization` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDnsAuthorizationRequest {
    /// Required. A definition of the dns authorization to update.
    #[prost(message, optional, tag = "1")]
    pub dns_authorization: ::core::option::Option<DnsAuthorization>,
    /// Required. The update mask applies to the resource. For the `FieldMask`
    /// definition, see
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask.>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request for the `DeleteDnsAuthorization` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDnsAuthorizationRequest {
    /// Required. A name of the dns authorization to delete. Must be in the format
    /// `projects/*/locations/*/dnsAuthorizations/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Represents the metadata of the long-running operation. Output only.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_message: ::prost::alloc::string::String,
    /// Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have \[Operation.error][\] value with a
    /// \[google.rpc.Status.code][google.rpc.Status.code\] of 1, corresponding to
    /// `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Defines TLS certificate.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Certificate {
    /// A user-defined name of the certificate. Certificate names must be unique
    /// globally and match pattern `projects/*/locations/*/certificates/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// One or more paragraphs of text description of a certificate.
    #[prost(string, tag = "8")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The creation timestamp of a Certificate.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last update timestamp of a Certificate.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Set of labels associated with a Certificate.
    #[prost(btree_map = "string, string", tag = "4")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. The list of Subject Alternative Names of dnsName type defined
    /// in the certificate (see RFC 5280 4.2.1.6). Managed certificates that
    /// haven't been provisioned yet have this field populated with a value of the
    /// managed.domains field.
    #[prost(string, repeated, tag = "6")]
    pub san_dnsnames: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The PEM-encoded certificate chain.
    #[prost(string, tag = "9")]
    pub pem_certificate: ::prost::alloc::string::String,
    /// Output only. The expiry timestamp of a Certificate.
    #[prost(message, optional, tag = "7")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Immutable. The scope of the certificate.
    #[prost(enumeration = "certificate::Scope", tag = "12")]
    pub scope: i32,
    #[prost(oneof = "certificate::Type", tags = "5, 11")]
    pub r#type: ::core::option::Option<certificate::Type>,
}
/// Nested message and enum types in `Certificate`.
pub mod certificate {
    /// Certificate data for a SelfManaged Certificate.
    /// SelfManaged Certificates are uploaded by the user. Updating such
    /// certificates before they expire remains the user's responsibility.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SelfManagedCertificate {
        /// Input only. The PEM-encoded certificate chain.
        /// Leaf certificate comes first, followed by intermediate ones if any.
        #[prost(string, tag = "1")]
        pub pem_certificate: ::prost::alloc::string::String,
        /// Input only. The PEM-encoded private key of the leaf certificate.
        #[prost(string, tag = "2")]
        pub pem_private_key: ::prost::alloc::string::String,
    }
    /// Configuration and state of a Managed Certificate.
    /// Certificate Manager provisions and renews Managed Certificates
    /// automatically, for as long as it's authorized to do so.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ManagedCertificate {
        /// Immutable. The domains for which a managed SSL certificate will be
        /// generated. Wildcard domains are only supported with DNS challenge
        /// resolution.
        #[prost(string, repeated, tag = "1")]
        pub domains: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Immutable. Authorizations that will be used for performing domain
        /// authorization.
        #[prost(string, repeated, tag = "2")]
        pub dns_authorizations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Immutable. The resource name for a
        /// \[CertificateIssuanceConfig][google.cloud.certificatemanager.v1.CertificateIssuanceConfig\]
        /// used to configure private PKI certificates in the format
        /// `projects/*/locations/*/certificateIssuanceConfigs/*`.
        /// If this field is not set, the certificates will instead be publicly
        /// signed as documented at
        /// <https://cloud.google.com/load-balancing/docs/ssl-certificates/google-managed-certs#caa.>
        #[prost(string, tag = "6")]
        pub issuance_config: ::prost::alloc::string::String,
        /// Output only. State of the managed certificate resource.
        #[prost(enumeration = "managed_certificate::State", tag = "4")]
        pub state: i32,
        /// Output only. Information about issues with provisioning a Managed
        /// Certificate.
        #[prost(message, optional, tag = "3")]
        pub provisioning_issue: ::core::option::Option<
            managed_certificate::ProvisioningIssue,
        >,
        /// Output only. Detailed state of the latest authorization attempt for each
        /// domain specified for managed certificate resource.
        #[prost(message, repeated, tag = "5")]
        pub authorization_attempt_info: ::prost::alloc::vec::Vec<
            managed_certificate::AuthorizationAttemptInfo,
        >,
    }
    /// Nested message and enum types in `ManagedCertificate`.
    pub mod managed_certificate {
        /// Information about issues with provisioning a Managed Certificate.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ProvisioningIssue {
            /// Output only. Reason for provisioning failures.
            #[prost(enumeration = "provisioning_issue::Reason", tag = "1")]
            pub reason: i32,
            /// Output only. Human readable explanation about the issue. Provided to
            /// help address the configuration issues. Not guaranteed to be stable. For
            /// programmatic access use Reason enum.
            #[prost(string, tag = "2")]
            pub details: ::prost::alloc::string::String,
        }
        /// Nested message and enum types in `ProvisioningIssue`.
        pub mod provisioning_issue {
            /// Reason for provisioning failures.
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
            pub enum Reason {
                /// Reason is unspecified.
                Unspecified = 0,
                /// Certificate provisioning failed due to an issue with one or more of
                /// the domains on the certificate.
                /// For details of which domains failed, consult the
                /// `authorization_attempt_info` field.
                AuthorizationIssue = 1,
                /// Exceeded Certificate Authority quotas or internal rate limits of the
                /// system. Provisioning may take longer to complete.
                RateLimited = 2,
            }
            impl Reason {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        Reason::Unspecified => "REASON_UNSPECIFIED",
                        Reason::AuthorizationIssue => "AUTHORIZATION_ISSUE",
                        Reason::RateLimited => "RATE_LIMITED",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "REASON_UNSPECIFIED" => Some(Self::Unspecified),
                        "AUTHORIZATION_ISSUE" => Some(Self::AuthorizationIssue),
                        "RATE_LIMITED" => Some(Self::RateLimited),
                        _ => None,
                    }
                }
            }
        }
        /// State of the latest attempt to authorize a domain for certificate
        /// issuance.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct AuthorizationAttemptInfo {
            /// Domain name of the authorization attempt.
            #[prost(string, tag = "1")]
            pub domain: ::prost::alloc::string::String,
            /// Output only. State of the domain for managed certificate issuance.
            #[prost(enumeration = "authorization_attempt_info::State", tag = "2")]
            pub state: i32,
            /// Output only. Reason for failure of the authorization attempt for the
            /// domain.
            #[prost(
                enumeration = "authorization_attempt_info::FailureReason",
                tag = "3"
            )]
            pub failure_reason: i32,
            /// Output only. Human readable explanation for reaching the state.
            /// Provided to help address the configuration issues. Not guaranteed to be
            /// stable. For programmatic access use FailureReason enum.
            #[prost(string, tag = "4")]
            pub details: ::prost::alloc::string::String,
        }
        /// Nested message and enum types in `AuthorizationAttemptInfo`.
        pub mod authorization_attempt_info {
            /// State of the domain for managed certificate issuance.
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
                /// State is unspecified.
                Unspecified = 0,
                /// Certificate provisioning for this domain is under way. GCP will
                /// attempt to authorize the domain.
                Authorizing = 1,
                /// A managed certificate can be provisioned, no issues for this domain.
                Authorized = 6,
                /// Attempt to authorize the domain failed. This prevents the Managed
                /// Certificate from being issued.
                /// See `failure_reason` and `details` fields for more information.
                Failed = 7,
            }
            impl State {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        State::Unspecified => "STATE_UNSPECIFIED",
                        State::Authorizing => "AUTHORIZING",
                        State::Authorized => "AUTHORIZED",
                        State::Failed => "FAILED",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                        "AUTHORIZING" => Some(Self::Authorizing),
                        "AUTHORIZED" => Some(Self::Authorized),
                        "FAILED" => Some(Self::Failed),
                        _ => None,
                    }
                }
            }
            /// Reason for failure of the authorization attempt for the domain.
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
            pub enum FailureReason {
                /// FailureReason is unspecified.
                Unspecified = 0,
                /// There was a problem with the user's DNS or load balancer
                /// configuration for this domain.
                Config = 1,
                /// Certificate issuance forbidden by an explicit CAA record for the
                /// domain or a failure to check CAA records for the domain.
                Caa = 2,
                /// Reached a CA or internal rate-limit for the domain,
                /// e.g. for certificates per top-level private domain.
                RateLimited = 3,
            }
            impl FailureReason {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        FailureReason::Unspecified => "FAILURE_REASON_UNSPECIFIED",
                        FailureReason::Config => "CONFIG",
                        FailureReason::Caa => "CAA",
                        FailureReason::RateLimited => "RATE_LIMITED",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "FAILURE_REASON_UNSPECIFIED" => Some(Self::Unspecified),
                        "CONFIG" => Some(Self::Config),
                        "CAA" => Some(Self::Caa),
                        "RATE_LIMITED" => Some(Self::RateLimited),
                        _ => None,
                    }
                }
            }
        }
        /// State of the managed certificate resource.
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
            /// State is unspecified.
            Unspecified = 0,
            /// Certificate Manager attempts to provision or renew the certificate.
            /// If the process takes longer than expected, consult the
            /// `provisioning_issue` field.
            Provisioning = 1,
            /// Multiple certificate provisioning attempts failed and Certificate
            /// Manager gave up. To try again, delete and create a new managed
            /// Certificate resource.
            /// For details see the `provisioning_issue` field.
            Failed = 2,
            /// The certificate management is working, and a certificate has been
            /// provisioned.
            Active = 3,
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
                    State::Failed => "FAILED",
                    State::Active => "ACTIVE",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                    "PROVISIONING" => Some(Self::Provisioning),
                    "FAILED" => Some(Self::Failed),
                    "ACTIVE" => Some(Self::Active),
                    _ => None,
                }
            }
        }
    }
    /// Certificate scope.
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
    pub enum Scope {
        /// Certificates with default scope are served from core Google data centers.
        /// If unsure, choose this option.
        Default = 0,
        /// Certificates with scope EDGE_CACHE are special-purposed certificates,
        /// served from non-core Google data centers.
        EdgeCache = 1,
    }
    impl Scope {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Scope::Default => "DEFAULT",
                Scope::EdgeCache => "EDGE_CACHE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DEFAULT" => Some(Self::Default),
                "EDGE_CACHE" => Some(Self::EdgeCache),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// If set, defines data of a self-managed certificate.
        #[prost(message, tag = "5")]
        SelfManaged(SelfManagedCertificate),
        /// If set, contains configuration and state of a managed certificate.
        #[prost(message, tag = "11")]
        Managed(ManagedCertificate),
    }
}
/// Defines a collection of certificate configurations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateMap {
    /// A user-defined name of the Certificate Map. Certificate Map names must be
    /// unique globally and match pattern
    /// `projects/*/locations/*/certificateMaps/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// One or more paragraphs of text description of a certificate map.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The creation timestamp of a Certificate Map.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The update timestamp of a Certificate Map.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Set of labels associated with a Certificate Map.
    #[prost(btree_map = "string, string", tag = "3")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. A list of GCLB targets that use this Certificate Map.
    /// A Target Proxy is only present on this list if it's attached to a
    /// Forwarding Rule.
    #[prost(message, repeated, tag = "4")]
    pub gclb_targets: ::prost::alloc::vec::Vec<certificate_map::GclbTarget>,
}
/// Nested message and enum types in `CertificateMap`.
pub mod certificate_map {
    /// Describes a Target Proxy that uses this Certificate Map.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GclbTarget {
        /// Output only. IP configurations for this Target Proxy where the
        /// Certificate Map is serving.
        #[prost(message, repeated, tag = "2")]
        pub ip_configs: ::prost::alloc::vec::Vec<gclb_target::IpConfig>,
        /// A Target Proxy to which this map is attached to.
        #[prost(oneof = "gclb_target::TargetProxy", tags = "1, 3")]
        pub target_proxy: ::core::option::Option<gclb_target::TargetProxy>,
    }
    /// Nested message and enum types in `GclbTarget`.
    pub mod gclb_target {
        /// Defines IP configuration where this Certificate Map is serving.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct IpConfig {
            /// Output only. An external IP address.
            #[prost(string, tag = "1")]
            pub ip_address: ::prost::alloc::string::String,
            /// Output only. Ports.
            #[prost(uint32, repeated, packed = "false", tag = "3")]
            pub ports: ::prost::alloc::vec::Vec<u32>,
        }
        /// A Target Proxy to which this map is attached to.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum TargetProxy {
            /// Output only. This field returns the resource name in the following
            /// format:
            /// `//compute.googleapis.com/projects/*/global/targetHttpsProxies/*`.
            #[prost(string, tag = "1")]
            TargetHttpsProxy(::prost::alloc::string::String),
            /// Output only. This field returns the resource name in the following
            /// format:
            /// `//compute.googleapis.com/projects/*/global/targetSslProxies/*`.
            #[prost(string, tag = "3")]
            TargetSslProxy(::prost::alloc::string::String),
        }
    }
}
/// Defines a certificate map entry.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateMapEntry {
    /// A user-defined name of the Certificate Map Entry. Certificate Map Entry
    /// names must be unique globally and match pattern
    /// `projects/*/locations/*/certificateMaps/*/certificateMapEntries/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// One or more paragraphs of text description of a certificate map entry.
    #[prost(string, tag = "9")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The creation timestamp of a Certificate Map Entry.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The update timestamp of a Certificate Map Entry.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Set of labels associated with a Certificate Map Entry.
    #[prost(btree_map = "string, string", tag = "4")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// A set of Certificates defines for the given `hostname`. There can be
    /// defined up to four certificates in each Certificate Map Entry. Each
    /// certificate must match pattern `projects/*/locations/*/certificates/*`.
    #[prost(string, repeated, tag = "7")]
    pub certificates: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. A serving state of this Certificate Map Entry.
    #[prost(enumeration = "ServingState", tag = "8")]
    pub state: i32,
    #[prost(oneof = "certificate_map_entry::Match", tags = "5, 10")]
    pub r#match: ::core::option::Option<certificate_map_entry::Match>,
}
/// Nested message and enum types in `CertificateMapEntry`.
pub mod certificate_map_entry {
    /// Defines predefined cases other than SNI-hostname match when this
    /// configuration should be applied.
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
    pub enum Matcher {
        /// A matcher has't been recognized.
        Unspecified = 0,
        /// A primary certificate that is served when SNI wasn't specified in the
        /// request or SNI couldn't be found in the map.
        Primary = 1,
    }
    impl Matcher {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Matcher::Unspecified => "MATCHER_UNSPECIFIED",
                Matcher::Primary => "PRIMARY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MATCHER_UNSPECIFIED" => Some(Self::Unspecified),
                "PRIMARY" => Some(Self::Primary),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Match {
        /// A Hostname (FQDN, e.g. `example.com`) or a wildcard hostname expression
        /// (`*.example.com`) for a set of hostnames with common suffix. Used as
        /// Server Name Indication (SNI) for selecting a proper certificate.
        #[prost(string, tag = "5")]
        Hostname(::prost::alloc::string::String),
        /// A predefined matcher for particular cases, other than SNI selection.
        #[prost(enumeration = "Matcher", tag = "10")]
        Matcher(i32),
    }
}
/// A DnsAuthorization resource describes a way to perform domain authorization
/// for certificate issuance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DnsAuthorization {
    /// A user-defined name of the dns authorization. DnsAuthorization names must
    /// be unique globally and match pattern
    /// `projects/*/locations/*/dnsAuthorizations/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The creation timestamp of a DnsAuthorization.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last update timestamp of a DnsAuthorization.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Set of labels associated with a DnsAuthorization.
    #[prost(btree_map = "string, string", tag = "4")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// One or more paragraphs of text description of a DnsAuthorization.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Required. Immutable. A domain that is being authorized. A DnsAuthorization
    /// resource covers a single domain and its wildcard, e.g. authorization for
    /// `example.com` can be used to issue certificates for `example.com` and
    /// `*.example.com`.
    #[prost(string, tag = "6")]
    pub domain: ::prost::alloc::string::String,
    /// Output only. DNS Resource Record that needs to be added to DNS
    /// configuration.
    #[prost(message, optional, tag = "10")]
    pub dns_resource_record: ::core::option::Option<
        dns_authorization::DnsResourceRecord,
    >,
}
/// Nested message and enum types in `DnsAuthorization`.
pub mod dns_authorization {
    /// The structure describing the DNS Resource Record that needs to be added
    /// to DNS configuration for the authorization to be usable by
    /// certificate.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DnsResourceRecord {
        /// Output only. Fully qualified name of the DNS Resource Record.
        /// e.g. `_acme-challenge.example.com`
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// Output only. Type of the DNS Resource Record.
        /// Currently always set to "CNAME".
        #[prost(string, tag = "2")]
        pub r#type: ::prost::alloc::string::String,
        /// Output only. Data of the DNS Resource Record.
        #[prost(string, tag = "3")]
        pub data: ::prost::alloc::string::String,
    }
}
/// Defines set of serving states associated with a resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ServingState {
    /// The status is undefined.
    Unspecified = 0,
    /// The configuration is serving.
    Active = 1,
    /// Update is in progress. Some frontends may serve this configuration.
    Pending = 2,
}
impl ServingState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ServingState::Unspecified => "SERVING_STATE_UNSPECIFIED",
            ServingState::Active => "ACTIVE",
            ServingState::Pending => "PENDING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SERVING_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "ACTIVE" => Some(Self::Active),
            "PENDING" => Some(Self::Pending),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod certificate_manager_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// API Overview
    ///
    /// Certificates Manager API allows customers to see and manage all their TLS
    /// certificates.
    ///
    /// Certificates Manager API service provides methods to manage certificates,
    /// group them into collections, and create serving configuration that can be
    /// easily applied to other Cloud resources e.g. Target Proxies.
    ///
    /// Data Model
    ///
    /// The Certificates Manager service exposes the following resources:
    ///
    /// * `Certificate` that describes a single TLS certificate.
    /// * `CertificateMap` that describes a collection of certificates that can be
    /// attached to a target resource.
    /// * `CertificateMapEntry` that describes a single configuration entry that
    /// consists of a SNI and a group of certificates. It's a subresource of
    /// CertificateMap.
    ///
    /// Certificate, CertificateMap and CertificateMapEntry IDs
    /// have to fully match the regexp `[a-z0-9-]{1,63}`. In other words,
    /// - only lower case letters, digits, and hyphen are allowed
    /// - length of the resource ID has to be in [1,63] range.
    ///
    /// Provides methods to manage Cloud Certificate Manager entities.
    #[derive(Debug, Clone)]
    pub struct CertificateManagerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CertificateManagerClient<T>
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
        ) -> CertificateManagerClient<InterceptedService<T, F>>
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
            CertificateManagerClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists Certificates in a given project and location.
        pub async fn list_certificates(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCertificatesRequest>,
        ) -> Result<tonic::Response<super::ListCertificatesResponse>, tonic::Status> {
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
                "/google.cloud.certificatemanager.v1.CertificateManager/ListCertificates",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single Certificate.
        pub async fn get_certificate(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCertificateRequest>,
        ) -> Result<tonic::Response<super::Certificate>, tonic::Status> {
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
                "/google.cloud.certificatemanager.v1.CertificateManager/GetCertificate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new Certificate in a given project and location.
        pub async fn create_certificate(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCertificateRequest>,
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
                "/google.cloud.certificatemanager.v1.CertificateManager/CreateCertificate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a Certificate.
        pub async fn update_certificate(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCertificateRequest>,
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
                "/google.cloud.certificatemanager.v1.CertificateManager/UpdateCertificate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single Certificate.
        pub async fn delete_certificate(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCertificateRequest>,
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
                "/google.cloud.certificatemanager.v1.CertificateManager/DeleteCertificate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists CertificateMaps in a given project and location.
        pub async fn list_certificate_maps(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCertificateMapsRequest>,
        ) -> Result<tonic::Response<super::ListCertificateMapsResponse>, tonic::Status> {
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
                "/google.cloud.certificatemanager.v1.CertificateManager/ListCertificateMaps",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single CertificateMap.
        pub async fn get_certificate_map(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCertificateMapRequest>,
        ) -> Result<tonic::Response<super::CertificateMap>, tonic::Status> {
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
                "/google.cloud.certificatemanager.v1.CertificateManager/GetCertificateMap",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new CertificateMap in a given project and location.
        pub async fn create_certificate_map(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCertificateMapRequest>,
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
                "/google.cloud.certificatemanager.v1.CertificateManager/CreateCertificateMap",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a CertificateMap.
        pub async fn update_certificate_map(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCertificateMapRequest>,
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
                "/google.cloud.certificatemanager.v1.CertificateManager/UpdateCertificateMap",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single CertificateMap. A Certificate Map can't be deleted
        /// if it contains Certificate Map Entries. Remove all the entries from
        /// the map before calling this method.
        pub async fn delete_certificate_map(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCertificateMapRequest>,
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
                "/google.cloud.certificatemanager.v1.CertificateManager/DeleteCertificateMap",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists CertificateMapEntries in a given project and location.
        pub async fn list_certificate_map_entries(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCertificateMapEntriesRequest>,
        ) -> Result<
            tonic::Response<super::ListCertificateMapEntriesResponse>,
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
                "/google.cloud.certificatemanager.v1.CertificateManager/ListCertificateMapEntries",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single CertificateMapEntry.
        pub async fn get_certificate_map_entry(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCertificateMapEntryRequest>,
        ) -> Result<tonic::Response<super::CertificateMapEntry>, tonic::Status> {
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
                "/google.cloud.certificatemanager.v1.CertificateManager/GetCertificateMapEntry",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new CertificateMapEntry in a given project and location.
        pub async fn create_certificate_map_entry(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCertificateMapEntryRequest>,
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
                "/google.cloud.certificatemanager.v1.CertificateManager/CreateCertificateMapEntry",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a CertificateMapEntry.
        pub async fn update_certificate_map_entry(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCertificateMapEntryRequest>,
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
                "/google.cloud.certificatemanager.v1.CertificateManager/UpdateCertificateMapEntry",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single CertificateMapEntry.
        pub async fn delete_certificate_map_entry(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCertificateMapEntryRequest>,
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
                "/google.cloud.certificatemanager.v1.CertificateManager/DeleteCertificateMapEntry",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists DnsAuthorizations in a given project and location.
        pub async fn list_dns_authorizations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDnsAuthorizationsRequest>,
        ) -> Result<
            tonic::Response<super::ListDnsAuthorizationsResponse>,
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
                "/google.cloud.certificatemanager.v1.CertificateManager/ListDnsAuthorizations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single DnsAuthorization.
        pub async fn get_dns_authorization(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDnsAuthorizationRequest>,
        ) -> Result<tonic::Response<super::DnsAuthorization>, tonic::Status> {
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
                "/google.cloud.certificatemanager.v1.CertificateManager/GetDnsAuthorization",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new DnsAuthorization in a given project and location.
        pub async fn create_dns_authorization(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDnsAuthorizationRequest>,
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
                "/google.cloud.certificatemanager.v1.CertificateManager/CreateDnsAuthorization",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a DnsAuthorization.
        pub async fn update_dns_authorization(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDnsAuthorizationRequest>,
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
                "/google.cloud.certificatemanager.v1.CertificateManager/UpdateDnsAuthorization",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single DnsAuthorization.
        pub async fn delete_dns_authorization(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDnsAuthorizationRequest>,
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
                "/google.cloud.certificatemanager.v1.CertificateManager/DeleteDnsAuthorization",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists CertificateIssuanceConfigs in a given project and location.
        pub async fn list_certificate_issuance_configs(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListCertificateIssuanceConfigsRequest,
            >,
        ) -> Result<
            tonic::Response<super::ListCertificateIssuanceConfigsResponse>,
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
                "/google.cloud.certificatemanager.v1.CertificateManager/ListCertificateIssuanceConfigs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single CertificateIssuanceConfig.
        pub async fn get_certificate_issuance_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCertificateIssuanceConfigRequest>,
        ) -> Result<tonic::Response<super::CertificateIssuanceConfig>, tonic::Status> {
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
                "/google.cloud.certificatemanager.v1.CertificateManager/GetCertificateIssuanceConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new CertificateIssuanceConfig in a given project and location.
        pub async fn create_certificate_issuance_config(
            &mut self,
            request: impl tonic::IntoRequest<
                super::CreateCertificateIssuanceConfigRequest,
            >,
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
                "/google.cloud.certificatemanager.v1.CertificateManager/CreateCertificateIssuanceConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single CertificateIssuanceConfig.
        pub async fn delete_certificate_issuance_config(
            &mut self,
            request: impl tonic::IntoRequest<
                super::DeleteCertificateIssuanceConfigRequest,
            >,
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
                "/google.cloud.certificatemanager.v1.CertificateManager/DeleteCertificateIssuanceConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
