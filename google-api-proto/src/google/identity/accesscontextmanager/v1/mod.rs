/// An `AccessLevel` is a label that can be applied to requests to Google Cloud
/// services, along with a list of requirements necessary for the label to be
/// applied.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessLevel {
    /// Required. Resource name for the Access Level. The `short_name` component
    /// must begin with a letter and only include alphanumeric and '_'. Format:
    /// `accessPolicies/{access_policy}/accessLevels/{access_level}`. The maximum
    /// length of the `access_level` component is 50 characters.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Human readable title. Must be unique within the Policy.
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// Description of the `AccessLevel` and its use. Does not affect behavior.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Time the `AccessLevel` was created in UTC.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time the `AccessLevel` was updated in UTC.
    #[prost(message, optional, tag = "7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. Describes the necessary conditions for the level to apply.
    #[prost(oneof = "access_level::Level", tags = "4, 5")]
    pub level: ::core::option::Option<access_level::Level>,
}
/// Nested message and enum types in `AccessLevel`.
pub mod access_level {
    /// Required. Describes the necessary conditions for the level to apply.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Level {
        /// A `BasicLevel` composed of `Conditions`.
        #[prost(message, tag = "4")]
        Basic(super::BasicLevel),
        /// A `CustomLevel` written in the Common Expression Language.
        #[prost(message, tag = "5")]
        Custom(super::CustomLevel),
    }
}
/// `BasicLevel` is an `AccessLevel` using a set of recommended features.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BasicLevel {
    /// Required. A list of requirements for the `AccessLevel` to be granted.
    #[prost(message, repeated, tag = "1")]
    pub conditions: ::prost::alloc::vec::Vec<Condition>,
    /// How the `conditions` list should be combined to determine if a request is
    /// granted this `AccessLevel`. If AND is used, each `Condition` in
    /// `conditions` must be satisfied for the `AccessLevel` to be applied. If OR
    /// is used, at least one `Condition` in `conditions` must be satisfied for the
    /// `AccessLevel` to be applied. Default behavior is AND.
    #[prost(enumeration = "basic_level::ConditionCombiningFunction", tag = "2")]
    pub combining_function: i32,
}
/// Nested message and enum types in `BasicLevel`.
pub mod basic_level {
    /// Options for how the `conditions` list should be combined to determine if
    /// this `AccessLevel` is applied. Default is AND.
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
    pub enum ConditionCombiningFunction {
        /// All `Conditions` must be true for the `BasicLevel` to be true.
        And = 0,
        /// If at least one `Condition` is true, then the `BasicLevel` is true.
        Or = 1,
    }
    impl ConditionCombiningFunction {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConditionCombiningFunction::And => "AND",
                ConditionCombiningFunction::Or => "OR",
            }
        }
    }
}
/// A condition necessary for an `AccessLevel` to be granted. The Condition is an
/// AND over its fields. So a Condition is true if: 1) the request IP is from one
/// of the listed subnetworks AND 2) the originating device complies with the
/// listed device policy AND 3) all listed access levels are granted AND 4) the
/// request was sent at a time allowed by the DateTimeRestriction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Condition {
    /// CIDR block IP subnetwork specification. May be IPv4 or IPv6. Note that for
    /// a CIDR IP address block, the specified IP address portion must be properly
    /// truncated (i.e. all the host bits must be zero) or the input is considered
    /// malformed. For example, "192.0.2.0/24" is accepted but "192.0.2.1/24" is
    /// not. Similarly, for IPv6, "2001:db8::/32" is accepted whereas
    /// "2001:db8::1/32" is not. The originating IP of a request must be in one of
    /// the listed subnets in order for this Condition to be true. If empty, all IP
    /// addresses are allowed.
    #[prost(string, repeated, tag = "1")]
    pub ip_subnetworks: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Device specific restrictions, all restrictions must hold for the
    /// Condition to be true. If not specified, all devices are allowed.
    #[prost(message, optional, tag = "2")]
    pub device_policy: ::core::option::Option<DevicePolicy>,
    /// A list of other access levels defined in the same `Policy`, referenced by
    /// resource name. Referencing an `AccessLevel` which does not exist is an
    /// error. All access levels listed must be granted for the Condition
    /// to be true. Example:
    /// "`accessPolicies/MY_POLICY/accessLevels/LEVEL_NAME"`
    #[prost(string, repeated, tag = "3")]
    pub required_access_levels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Whether to negate the Condition. If true, the Condition becomes a NAND over
    /// its non-empty fields, each field must be false for the Condition overall to
    /// be satisfied. Defaults to false.
    #[prost(bool, tag = "5")]
    pub negate: bool,
    /// The request must be made by one of the provided user or service
    /// accounts. Groups are not supported.
    /// Syntax:
    /// `user:{emailid}`
    /// `serviceAccount:{emailid}`
    /// If not specified, a request may come from any user.
    #[prost(string, repeated, tag = "6")]
    pub members: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The request must originate from one of the provided countries/regions.
    /// Must be valid ISO 3166-1 alpha-2 codes.
    #[prost(string, repeated, tag = "7")]
    pub regions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// `CustomLevel` is an `AccessLevel` using the Cloud Common Expression Language
/// to represent the necessary conditions for the level to apply to a request.
/// See CEL spec at: <https://github.com/google/cel-spec>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomLevel {
    /// Required. A Cloud CEL expression evaluating to a boolean.
    #[prost(message, optional, tag = "1")]
    pub expr: ::core::option::Option<super::super::super::r#type::Expr>,
}
/// `DevicePolicy` specifies device specific restrictions necessary to acquire a
/// given access level. A `DevicePolicy` specifies requirements for requests from
/// devices to be granted access levels, it does not do any enforcement on the
/// device. `DevicePolicy` acts as an AND over all specified fields, and each
/// repeated field is an OR over its elements. Any unset fields are ignored. For
/// example, if the proto is { os_type : DESKTOP_WINDOWS, os_type :
/// DESKTOP_LINUX, encryption_status: ENCRYPTED}, then the DevicePolicy will be
/// true for requests originating from encrypted Linux desktops and encrypted
/// Windows desktops.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DevicePolicy {
    /// Whether or not screenlock is required for the DevicePolicy to be true.
    /// Defaults to `false`.
    #[prost(bool, tag = "1")]
    pub require_screenlock: bool,
    /// Allowed encryptions statuses, an empty list allows all statuses.
    #[prost(enumeration = "super::r#type::DeviceEncryptionStatus", repeated, tag = "2")]
    pub allowed_encryption_statuses: ::prost::alloc::vec::Vec<i32>,
    /// Allowed OS versions, an empty list allows all types and all versions.
    #[prost(message, repeated, tag = "3")]
    pub os_constraints: ::prost::alloc::vec::Vec<OsConstraint>,
    /// Allowed device management levels, an empty list allows all management
    /// levels.
    #[prost(enumeration = "super::r#type::DeviceManagementLevel", repeated, tag = "6")]
    pub allowed_device_management_levels: ::prost::alloc::vec::Vec<i32>,
    /// Whether the device needs to be approved by the customer admin.
    #[prost(bool, tag = "7")]
    pub require_admin_approval: bool,
    /// Whether the device needs to be corp owned.
    #[prost(bool, tag = "8")]
    pub require_corp_owned: bool,
}
/// A restriction on the OS type and version of devices making requests.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OsConstraint {
    /// Required. The allowed OS type.
    #[prost(enumeration = "super::r#type::OsType", tag = "1")]
    pub os_type: i32,
    /// The minimum allowed OS version. If not set, any version of this OS
    /// satisfies the constraint. Format: `"major.minor.patch"`.
    /// Examples: `"10.5.301"`, `"9.2.1"`.
    #[prost(string, tag = "2")]
    pub minimum_version: ::prost::alloc::string::String,
    /// Only allows requests from devices with a verified Chrome OS.
    /// Verifications includes requirements that the device is enterprise-managed,
    /// conformant to domain policies, and the caller has permission to call
    /// the API targeted by the request.
    #[prost(bool, tag = "3")]
    pub require_verified_chrome_os: bool,
}
/// `AccessPolicy` is a container for `AccessLevels` (which define the necessary
/// attributes to use Google Cloud services) and `ServicePerimeters` (which
/// define regions of services able to freely pass data within a perimeter). An
/// access policy is globally visible within an organization, and the
/// restrictions it specifies apply to all projects within an organization.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessPolicy {
    /// Output only. Resource name of the `AccessPolicy`. Format:
    /// `accessPolicies/{access_policy}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The parent of this `AccessPolicy` in the Cloud Resource
    /// Hierarchy. Currently immutable once created. Format:
    /// `organizations/{organization_id}`
    #[prost(string, tag = "2")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Human readable title. Does not affect behavior.
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    /// The scopes of a policy define which resources an ACM policy can restrict,
    /// and where ACM resources can be referenced.
    /// For example, a policy with scopes=\["folders/123"\] has the following
    /// behavior:
    /// - vpcsc perimeters can only restrict projects within folders/123
    /// - access levels can only be referenced by resources within folders/123.
    /// If empty, there are no limitations on which resources can be restricted by
    /// an ACM policy, and there are no limitations on where ACM resources can be
    /// referenced.
    /// Only one policy can include a given scope (attempting to create a second
    /// policy which includes "folders/123" will result in an error).
    /// Currently, scopes cannot be modified after a policy is created.
    /// Currently, policies can only have a single scope.
    /// Format: list of `folders/{folder_number}` or `projects/{project_number}`
    #[prost(string, repeated, tag = "7")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. Time the `AccessPolicy` was created in UTC.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time the `AccessPolicy` was updated in UTC.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. An opaque identifier for the current version of the
    /// `AccessPolicy`. This will always be a strongly validated etag, meaning that
    /// two Access Polices will be identical if and only if their etags are
    /// identical. Clients should not expect this to be in any specific format.
    #[prost(string, tag = "6")]
    pub etag: ::prost::alloc::string::String,
}
/// `ServicePerimeter` describes a set of Google Cloud resources which can freely
/// import and export data amongst themselves, but not export outside of the
/// `ServicePerimeter`. If a request with a source within this `ServicePerimeter`
/// has a target outside of the `ServicePerimeter`, the request will be blocked.
/// Otherwise the request is allowed. There are two types of Service Perimeter -
/// Regular and Bridge. Regular Service Perimeters cannot overlap, a single
/// Google Cloud project can only belong to a single regular Service Perimeter.
/// Service Perimeter Bridges can contain only Google Cloud projects as members,
/// a single Google Cloud project may belong to multiple Service Perimeter
/// Bridges.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServicePerimeter {
    /// Required. Resource name for the ServicePerimeter.  The `short_name`
    /// component must begin with a letter and only include alphanumeric and '_'.
    /// Format:
    /// `accessPolicies/{access_policy}/servicePerimeters/{service_perimeter}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Human readable title. Must be unique within the Policy.
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// Description of the `ServicePerimeter` and its use. Does not affect
    /// behavior.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Time the `ServicePerimeter` was created in UTC.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time the `ServicePerimeter` was updated in UTC.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Perimeter type indicator. A single project is
    /// allowed to be a member of single regular perimeter, but multiple service
    /// perimeter bridges. A project cannot be a included in a perimeter bridge
    /// without being included in regular perimeter. For perimeter bridges,
    /// the restricted service list as well as access level lists must be
    /// empty.
    #[prost(enumeration = "service_perimeter::PerimeterType", tag = "6")]
    pub perimeter_type: i32,
    /// Current ServicePerimeter configuration. Specifies sets of resources,
    /// restricted services and access levels that determine perimeter
    /// content and boundaries.
    #[prost(message, optional, tag = "7")]
    pub status: ::core::option::Option<ServicePerimeterConfig>,
    /// Proposed (or dry run) ServicePerimeter configuration. This configuration
    /// allows to specify and test ServicePerimeter configuration without enforcing
    /// actual access restrictions. Only allowed to be set when the
    /// "use_explicit_dry_run_spec" flag is set.
    #[prost(message, optional, tag = "8")]
    pub spec: ::core::option::Option<ServicePerimeterConfig>,
    /// Use explicit dry run spec flag. Ordinarily, a dry-run spec implicitly
    /// exists  for all Service Perimeters, and that spec is identical to the
    /// status for those Service Perimeters. When this flag is set, it inhibits the
    /// generation of the implicit spec, thereby allowing the user to explicitly
    /// provide a configuration ("spec") to use in a dry-run version of the Service
    /// Perimeter. This allows the user to test changes to the enforced config
    /// ("status") without actually enforcing them. This testing is done through
    /// analyzing the differences between currently enforced and suggested
    /// restrictions. use_explicit_dry_run_spec must bet set to True if any of the
    /// fields in the spec are set to non-default values.
    #[prost(bool, tag = "9")]
    pub use_explicit_dry_run_spec: bool,
}
/// Nested message and enum types in `ServicePerimeter`.
pub mod service_perimeter {
    /// Specifies the type of the Perimeter. There are two types: regular and
    /// bridge. Regular Service Perimeter contains resources, access levels, and
    /// restricted services. Every resource can be in at most ONE
    /// regular Service Perimeter.
    ///
    /// In addition to being in a regular service perimeter, a resource can also
    /// be in zero or more perimeter bridges.  A perimeter bridge only contains
    /// resources.  Cross project operations are permitted if all effected
    /// resources share some perimeter (whether bridge or regular). Perimeter
    /// Bridge does not contain access levels or services: those are governed
    /// entirely by the regular perimeter that resource is in.
    ///
    /// Perimeter Bridges are typically useful when building more complex toplogies
    /// with many independent perimeters that need to share some data with a common
    /// perimeter, but should not be able to share data among themselves.
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
    pub enum PerimeterType {
        /// Regular Perimeter.
        Regular = 0,
        /// Perimeter Bridge.
        Bridge = 1,
    }
    impl PerimeterType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PerimeterType::Regular => "PERIMETER_TYPE_REGULAR",
                PerimeterType::Bridge => "PERIMETER_TYPE_BRIDGE",
            }
        }
    }
}
/// `ServicePerimeterConfig` specifies a set of Google Cloud resources that
/// describe specific Service Perimeter configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServicePerimeterConfig {
    /// A list of Google Cloud resources that are inside of the service perimeter.
    /// Currently only projects are allowed. Format: `projects/{project_number}`
    #[prost(string, repeated, tag = "1")]
    pub resources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of `AccessLevel` resource names that allow resources within the
    /// `ServicePerimeter` to be accessed from the internet. `AccessLevels` listed
    /// must be in the same policy as this `ServicePerimeter`. Referencing a
    /// nonexistent `AccessLevel` is a syntax error. If no `AccessLevel` names are
    /// listed, resources within the perimeter can only be accessed via Google
    /// Cloud calls with request origins within the perimeter. Example:
    /// `"accessPolicies/MY_POLICY/accessLevels/MY_LEVEL"`.
    /// For Service Perimeter Bridge, must be empty.
    #[prost(string, repeated, tag = "2")]
    pub access_levels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Google Cloud services that are subject to the Service Perimeter
    /// restrictions. For example, if `storage.googleapis.com` is specified, access
    /// to the storage buckets inside the perimeter must meet the perimeter's
    /// access restrictions.
    #[prost(string, repeated, tag = "4")]
    pub restricted_services: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Configuration for APIs allowed within Perimeter.
    #[prost(message, optional, tag = "10")]
    pub vpc_accessible_services: ::core::option::Option<
        service_perimeter_config::VpcAccessibleServices,
    >,
    /// List of \[IngressPolicies\]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.IngressPolicy\]
    /// to apply to the perimeter. A perimeter may have multiple \[IngressPolicies\]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.IngressPolicy\],
    /// each of which is evaluated separately. Access is granted if any [Ingress
    /// Policy]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.IngressPolicy\]
    /// grants it. Must be empty for a perimeter bridge.
    #[prost(message, repeated, tag = "8")]
    pub ingress_policies: ::prost::alloc::vec::Vec<
        service_perimeter_config::IngressPolicy,
    >,
    /// List of \[EgressPolicies\]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.EgressPolicy\]
    /// to apply to the perimeter. A perimeter may have multiple \[EgressPolicies\]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.EgressPolicy\],
    /// each of which is evaluated separately. Access is granted if any
    /// \[EgressPolicy\]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.EgressPolicy\]
    /// grants it. Must be empty for a perimeter bridge.
    #[prost(message, repeated, tag = "9")]
    pub egress_policies: ::prost::alloc::vec::Vec<
        service_perimeter_config::EgressPolicy,
    >,
}
/// Nested message and enum types in `ServicePerimeterConfig`.
pub mod service_perimeter_config {
    /// Specifies how APIs are allowed to communicate within the Service
    /// Perimeter.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VpcAccessibleServices {
        /// Whether to restrict API calls within the Service Perimeter to the list of
        /// APIs specified in 'allowed_services'.
        #[prost(bool, tag = "1")]
        pub enable_restriction: bool,
        /// The list of APIs usable within the Service Perimeter. Must be empty
        /// unless 'enable_restriction' is True. You can specify a list of individual
        /// services, as well as include the 'RESTRICTED-SERVICES' value, which
        /// automatically includes all of the services protected by the perimeter.
        #[prost(string, repeated, tag = "2")]
        pub allowed_services: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// An allowed method or permission of a service specified in \[ApiOperation\]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.ApiOperation\].
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MethodSelector {
        /// The API method name or Cloud IAM permission name to allow.
        #[prost(oneof = "method_selector::Kind", tags = "1, 2")]
        pub kind: ::core::option::Option<method_selector::Kind>,
    }
    /// Nested message and enum types in `MethodSelector`.
    pub mod method_selector {
        /// The API method name or Cloud IAM permission name to allow.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Kind {
            /// Value for `method` should be a valid method name for the corresponding
            /// `service_name` in \[ApiOperation\]
            /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.ApiOperation\].
            /// If `*` used as value for `method`, then ALL methods and permissions are
            /// allowed.
            #[prost(string, tag = "1")]
            Method(::prost::alloc::string::String),
            /// Value for `permission` should be a valid Cloud IAM permission for the
            /// corresponding `service_name` in \[ApiOperation\]
            /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.ApiOperation\].
            #[prost(string, tag = "2")]
            Permission(::prost::alloc::string::String),
        }
    }
    /// Identification for an API Operation.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ApiOperation {
        /// The name of the API whose methods or permissions the \[IngressPolicy\]
        /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.IngressPolicy\]
        /// or \[EgressPolicy\]
        /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.EgressPolicy\]
        /// want to allow. A single \[ApiOperation\]
        /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.ApiOperation\]
        /// with `service_name` field set to `*` will allow all methods AND
        /// permissions for all services.
        #[prost(string, tag = "1")]
        pub service_name: ::prost::alloc::string::String,
        /// API methods or permissions to allow. Method or permission must belong to
        /// the service specified by `service_name` field. A single \[MethodSelector\]
        /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.MethodSelector\]
        /// entry with `*` specified for the `method` field will allow all methods
        /// AND permissions for the service specified in `service_name`.
        #[prost(message, repeated, tag = "2")]
        pub method_selectors: ::prost::alloc::vec::Vec<MethodSelector>,
    }
    /// The source that \[IngressPolicy\]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.IngressPolicy\]
    /// authorizes access from.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IngressSource {
        /// Allowed ingress source. It can be one of \[AccessLevel\]
        /// \[google.identity.accesscontextmanager.v1.AccessLevel\] or Google
        /// Cloud resource.
        #[prost(oneof = "ingress_source::Source", tags = "1, 2")]
        pub source: ::core::option::Option<ingress_source::Source>,
    }
    /// Nested message and enum types in `IngressSource`.
    pub mod ingress_source {
        /// Allowed ingress source. It can be one of \[AccessLevel\]
        /// \[google.identity.accesscontextmanager.v1.AccessLevel\] or Google
        /// Cloud resource.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Source {
            /// An \[AccessLevel\]
            /// \[google.identity.accesscontextmanager.v1.AccessLevel\] resource
            /// name that allow resources within the \[ServicePerimeters\]
            /// \[google.identity.accesscontextmanager.v1.ServicePerimeter\] to be
            /// accessed from the internet. \[AccessLevels\]
            /// \[google.identity.accesscontextmanager.v1.AccessLevel\] listed must
            /// be in the same policy as this \[ServicePerimeter\]
            /// \[google.identity.accesscontextmanager.v1.ServicePerimeter\].
            /// Referencing a nonexistent \[AccessLevel\]
            /// \[google.identity.accesscontextmanager.v1.AccessLevel\] will cause
            /// an error. If no \[AccessLevel\]
            /// \[google.identity.accesscontextmanager.v1.AccessLevel\] names are
            /// listed, resources within the perimeter can only be accessed via Google
            /// Cloud calls with request origins within the perimeter. Example:
            /// `accessPolicies/MY_POLICY/accessLevels/MY_LEVEL`. If a single `*` is
            /// specified for `access_level`, then all \[IngressSources\]
            /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.IngressSource\]
            /// will be allowed.
            #[prost(string, tag = "1")]
            AccessLevel(::prost::alloc::string::String),
            /// A Google Cloud resource that is allowed to ingress the perimeter.
            /// Requests from these resources will be allowed to access perimeter data.
            /// Currently only projects are allowed.
            /// Format: `projects/{project_number}`
            /// The project may be in any Google Cloud organization, not just the
            /// organization that the perimeter is defined in. `*` is not allowed, the
            /// case of allowing all Google Cloud resources only is not supported.
            #[prost(string, tag = "2")]
            Resource(::prost::alloc::string::String),
        }
    }
    /// Defines the conditions under which an \[IngressPolicy\]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.IngressPolicy\]
    /// matches a request. Conditions are based on information about the source of
    /// the request. The request must satisfy what is defined in `sources` AND
    /// identity related fields in order to match.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IngressFrom {
        /// Sources that this \[IngressPolicy\]
        /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.IngressPolicy\]
        /// authorizes access from.
        #[prost(message, repeated, tag = "1")]
        pub sources: ::prost::alloc::vec::Vec<IngressSource>,
        /// A list of identities that are allowed access through this ingress
        /// policy. Should be in the format of email address. The email address
        /// should represent individual user or service account only.
        #[prost(string, repeated, tag = "2")]
        pub identities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Specifies the type of identities that are allowed access from outside the
        /// perimeter. If left unspecified, then members of `identities` field will
        /// be allowed access.
        #[prost(enumeration = "IdentityType", tag = "3")]
        pub identity_type: i32,
    }
    /// Defines the conditions under which an \[IngressPolicy\]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.IngressPolicy\]
    /// matches a request. Conditions are based on information about the
    /// \[ApiOperation\]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.ApiOperation\]
    /// intended to be performed on the target resource of the request. The request
    /// must satisfy what is defined in `operations` AND `resources` in order to
    /// match.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IngressTo {
        /// A list of \[ApiOperations\]
        /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.ApiOperation\]
        /// allowed to be performed by the sources specified in corresponding
        /// \[IngressFrom\]
        /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.IngressFrom\]
        /// in this \[ServicePerimeter\]
        /// \[google.identity.accesscontextmanager.v1.ServicePerimeter\].
        #[prost(message, repeated, tag = "1")]
        pub operations: ::prost::alloc::vec::Vec<ApiOperation>,
        /// A list of resources, currently only projects in the form
        /// `projects/<projectnumber>`, protected by this \[ServicePerimeter\]
        /// \[google.identity.accesscontextmanager.v1.ServicePerimeter\] that are
        /// allowed to be accessed by sources defined in the corresponding
        /// \[IngressFrom\]
        /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.IngressFrom\].
        /// If a single `*` is specified, then access to all resources inside the
        /// perimeter are allowed.
        #[prost(string, repeated, tag = "2")]
        pub resources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// Policy for ingress into \[ServicePerimeter\]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeter\].
    ///
    /// \[IngressPolicies\]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.IngressPolicy\]
    /// match requests based on `ingress_from` and `ingress_to` stanzas.  For an
    /// ingress policy to match, both the `ingress_from` and `ingress_to` stanzas
    /// must be matched. If an \[IngressPolicy\]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.IngressPolicy\]
    /// matches a request, the request is allowed through the perimeter boundary
    /// from outside the perimeter.
    ///
    /// For example, access from the internet can be allowed either
    /// based on an \[AccessLevel\]
    /// \[google.identity.accesscontextmanager.v1.AccessLevel\] or, for traffic
    /// hosted on Google Cloud, the project of the source network. For access from
    /// private networks, using the project of the hosting network is required.
    ///
    /// Individual ingress policies can be limited by restricting which
    /// services and/or actions they match using the `ingress_to` field.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IngressPolicy {
        /// Defines the conditions on the source of a request causing this
        /// \[IngressPolicy\]
        /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.IngressPolicy\]
        /// to apply.
        #[prost(message, optional, tag = "1")]
        pub ingress_from: ::core::option::Option<IngressFrom>,
        /// Defines the conditions on the \[ApiOperation\]
        /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.ApiOperation\]
        /// and request destination that cause this \[IngressPolicy\]
        /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.IngressPolicy\]
        /// to apply.
        #[prost(message, optional, tag = "2")]
        pub ingress_to: ::core::option::Option<IngressTo>,
    }
    /// Defines the conditions under which an \[EgressPolicy\]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.EgressPolicy\]
    /// matches a request. Conditions based on information about the source of the
    /// request. Note that if the destination of the request is also protected by a
    /// \[ServicePerimeter\]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeter\], then that
    /// \[ServicePerimeter\]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeter\] must have
    /// an \[IngressPolicy\]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.IngressPolicy\]
    /// which allows access in order for this request to succeed.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EgressFrom {
        /// A list of identities that are allowed access through this \[EgressPolicy\].
        /// Should be in the format of email address. The email address should
        /// represent individual user or service account only.
        #[prost(string, repeated, tag = "1")]
        pub identities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Specifies the type of identities that are allowed access to outside the
        /// perimeter. If left unspecified, then members of `identities` field will
        /// be allowed access.
        #[prost(enumeration = "IdentityType", tag = "2")]
        pub identity_type: i32,
    }
    /// Defines the conditions under which an \[EgressPolicy\]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.EgressPolicy\]
    /// matches a request. Conditions are based on information about the
    /// \[ApiOperation\]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.ApiOperation\]
    /// intended to be performed on the `resources` specified. Note that if the
    /// destination of the request is also protected by a \[ServicePerimeter\]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeter\], then that
    /// \[ServicePerimeter\]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeter\] must have
    /// an \[IngressPolicy\]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.IngressPolicy\]
    /// which allows access in order for this request to succeed. The request must
    /// match `operations` AND `resources` fields in order to be allowed egress out
    /// of the perimeter.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EgressTo {
        /// A list of resources, currently only projects in the form
        /// `projects/<projectnumber>`, that are allowed to be accessed by sources
        /// defined in the corresponding \[EgressFrom\]
        /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.EgressFrom\].
        /// A request matches if it contains a resource in this list.  If `*` is
        /// specified for `resources`, then this \[EgressTo\]
        /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.EgressTo\]
        /// rule will authorize access to all resources outside the perimeter.
        #[prost(string, repeated, tag = "1")]
        pub resources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// A list of \[ApiOperations\]
        /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.ApiOperation\]
        /// allowed to be performed by the sources specified in the corresponding
        /// \[EgressFrom\]
        /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.EgressFrom\].
        /// A request matches if it uses an operation/service in this list.
        #[prost(message, repeated, tag = "2")]
        pub operations: ::prost::alloc::vec::Vec<ApiOperation>,
        /// A list of external resources that are allowed to be accessed. Only AWS
        /// and Azure resources are supported. For Amazon S3, the supported format is
        /// s3://BUCKET_NAME. For Azure Storage, the supported format is
        /// azure://myaccount.blob.core.windows.net/CONTAINER_NAME. A request matches
        /// if it contains an external resource in this list (Example:
        /// s3://bucket/path). Currently '*' is not allowed.
        #[prost(string, repeated, tag = "3")]
        pub external_resources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// Policy for egress from perimeter.
    ///
    /// \[EgressPolicies\]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.EgressPolicy\]
    /// match requests based on `egress_from` and `egress_to` stanzas.  For an
    /// \[EgressPolicy\]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.EgressPolicy\]
    /// to match, both `egress_from` and `egress_to` stanzas must be matched. If an
    /// \[EgressPolicy\]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.EgressPolicy\]
    /// matches a request, the request is allowed to span the \[ServicePerimeter\]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeter\] boundary.
    /// For example, an \[EgressPolicy\]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.EgressPolicy\]
    /// can be used to allow VMs on networks within the \[ServicePerimeter\]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeter\] to access a
    /// defined set of projects outside the perimeter in certain contexts (e.g. to
    /// read data from a Cloud Storage bucket or query against a BigQuery dataset).
    ///
    /// \[EgressPolicies\]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.EgressPolicy\]
    /// are concerned with the *resources* that a request relates as well as the
    /// API services and API actions being used.  They do not related to the
    /// direction of data movement.  More detailed documentation for this concept
    /// can be found in the descriptions of \[EgressFrom\]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.EgressFrom\]
    /// and \[EgressTo\]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.EgressTo\].
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EgressPolicy {
        /// Defines conditions on the source of a request causing this \[EgressPolicy\]
        /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.EgressPolicy\]
        /// to apply.
        #[prost(message, optional, tag = "1")]
        pub egress_from: ::core::option::Option<EgressFrom>,
        /// Defines the conditions on the \[ApiOperation\]
        /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.ApiOperation\]
        /// and destination resources that cause this \[EgressPolicy\]
        /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.EgressPolicy\]
        /// to apply.
        #[prost(message, optional, tag = "2")]
        pub egress_to: ::core::option::Option<EgressTo>,
    }
    /// Specifies the types of identities that are allowed access in either
    /// \[IngressFrom\]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.IngressFrom\]
    /// or \[EgressFrom\]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeterConfig.EgressFrom\]
    /// rules.
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
    pub enum IdentityType {
        /// No blanket identity group specified.
        Unspecified = 0,
        /// Authorize access from all identities outside the perimeter.
        AnyIdentity = 1,
        /// Authorize access from all human users outside the perimeter.
        AnyUserAccount = 2,
        /// Authorize access from all service accounts outside the perimeter.
        AnyServiceAccount = 3,
    }
    impl IdentityType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                IdentityType::Unspecified => "IDENTITY_TYPE_UNSPECIFIED",
                IdentityType::AnyIdentity => "ANY_IDENTITY",
                IdentityType::AnyUserAccount => "ANY_USER_ACCOUNT",
                IdentityType::AnyServiceAccount => "ANY_SERVICE_ACCOUNT",
            }
        }
    }
}
/// Restricts access to Cloud Console and Google Cloud APIs for a set of users
/// using Context-Aware Access.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcpUserAccessBinding {
    /// Immutable. Assigned by the server during creation. The last segment has an arbitrary
    /// length and has only URI unreserved characters (as defined by
    /// [RFC 3986 Section 2.3](<https://tools.ietf.org/html/rfc3986#section-2.3>)).
    /// Should not be specified by the client during creation.
    /// Example: "organizations/256/gcpUserAccessBindings/b3-BhcX_Ud5N"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Immutable. Google Group id whose members are subject to this binding's restrictions.
    /// See "id" in the [G Suite Directory API's Groups resource]
    /// (<https://developers.google.com/admin-sdk/directory/v1/reference/groups#resource>).
    /// If a group's email address/alias is changed, this resource will continue
    /// to point at the changed group. This field does not accept group email
    /// addresses or aliases.
    /// Example: "01d520gv4vjcrht"
    #[prost(string, tag = "2")]
    pub group_key: ::prost::alloc::string::String,
    /// Required. Access level that a user must have to be granted access. Only one access
    /// level is supported, not multiple. This repeated field must have exactly
    /// one element.
    /// Example: "accessPolicies/9522/accessLevels/device_trusted"
    #[prost(string, repeated, tag = "3")]
    pub access_levels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A request to list all `AccessPolicies` for a container.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccessPoliciesRequest {
    /// Required. Resource name for the container to list AccessPolicy instances
    /// from.
    ///
    /// Format:
    /// `organizations/{org_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Number of AccessPolicy instances to include in the list. Default 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Next page token for the next batch of AccessPolicy instances. Defaults to
    /// the first page of results.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// A response to `ListAccessPoliciesRequest`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccessPoliciesResponse {
    /// List of the AccessPolicy instances.
    #[prost(message, repeated, tag = "1")]
    pub access_policies: ::prost::alloc::vec::Vec<AccessPolicy>,
    /// The pagination token to retrieve the next page of results. If the value is
    /// empty, no further results remain.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A request to get a particular `AccessPolicy`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccessPolicyRequest {
    /// Required. Resource name for the access policy to get.
    ///
    /// Format `accessPolicies/{policy_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request to update an `AccessPolicy`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAccessPolicyRequest {
    /// Required. The updated AccessPolicy.
    #[prost(message, optional, tag = "1")]
    pub policy: ::core::option::Option<AccessPolicy>,
    /// Required. Mask to control which fields get updated. Must be non-empty.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// A request to delete an `AccessPolicy`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAccessPolicyRequest {
    /// Required. Resource name for the access policy to delete.
    ///
    /// Format `accessPolicies/{policy_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request to list all `AccessLevels` in an `AccessPolicy`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccessLevelsRequest {
    /// Required. Resource name for the access policy to list [Access Levels]
    /// \[google.identity.accesscontextmanager.v1.AccessLevel\] from.
    ///
    /// Format:
    /// `accessPolicies/{policy_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Number of [Access Levels]
    /// \[google.identity.accesscontextmanager.v1.AccessLevel\] to include in
    /// the list. Default 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Next page token for the next batch of [Access Level]
    /// \[google.identity.accesscontextmanager.v1.AccessLevel\] instances.
    /// Defaults to the first page of results.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Whether to return `BasicLevels` in the Cloud Common Expression language, as
    /// `CustomLevels`, rather than as `BasicLevels`. Defaults to returning
    /// `AccessLevels` in the format they were defined.
    #[prost(enumeration = "LevelFormat", tag = "4")]
    pub access_level_format: i32,
}
/// A response to `ListAccessLevelsRequest`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccessLevelsResponse {
    /// List of the [Access Level]
    /// \[google.identity.accesscontextmanager.v1.AccessLevel\] instances.
    #[prost(message, repeated, tag = "1")]
    pub access_levels: ::prost::alloc::vec::Vec<AccessLevel>,
    /// The pagination token to retrieve the next page of results. If the value is
    /// empty, no further results remain.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A request to get a particular `AccessLevel`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccessLevelRequest {
    /// Required. Resource name for the [Access Level]
    /// \[google.identity.accesscontextmanager.v1.AccessLevel\].
    ///
    /// Format:
    /// `accessPolicies/{policy_id}/accessLevels/{access_level_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Whether to return `BasicLevels` in the Cloud Common Expression
    /// Language rather than as `BasicLevels`. Defaults to AS_DEFINED, where
    /// [Access Levels] \[google.identity.accesscontextmanager.v1.AccessLevel\]
    /// are returned as `BasicLevels` or `CustomLevels` based on how they were
    /// created. If set to CEL, all [Access Levels]
    /// \[google.identity.accesscontextmanager.v1.AccessLevel\] are returned as
    /// `CustomLevels`. In the CEL case, `BasicLevels` are translated to equivalent
    /// `CustomLevels`.
    #[prost(enumeration = "LevelFormat", tag = "2")]
    pub access_level_format: i32,
}
/// A request to create an `AccessLevel`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAccessLevelRequest {
    /// Required. Resource name for the access policy which owns this [Access
    /// Level] \[google.identity.accesscontextmanager.v1.AccessLevel\].
    ///
    /// Format: `accessPolicies/{policy_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The [Access Level]
    /// \[google.identity.accesscontextmanager.v1.AccessLevel\] to create.
    /// Syntactic correctness of the [Access Level]
    /// \[google.identity.accesscontextmanager.v1.AccessLevel\] is a
    /// precondition for creation.
    #[prost(message, optional, tag = "2")]
    pub access_level: ::core::option::Option<AccessLevel>,
}
/// A request to update an `AccessLevel`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAccessLevelRequest {
    /// Required. The updated [Access Level]
    /// \[google.identity.accesscontextmanager.v1.AccessLevel\]. Syntactic
    /// correctness of the [Access Level]
    /// \[google.identity.accesscontextmanager.v1.AccessLevel\] is a
    /// precondition for creation.
    #[prost(message, optional, tag = "1")]
    pub access_level: ::core::option::Option<AccessLevel>,
    /// Required. Mask to control which fields get updated. Must be non-empty.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// A request to delete an `AccessLevel`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAccessLevelRequest {
    /// Required. Resource name for the [Access Level]
    /// \[google.identity.accesscontextmanager.v1.AccessLevel\].
    ///
    /// Format:
    /// `accessPolicies/{policy_id}/accessLevels/{access_level_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request to replace all existing Access Levels in an Access Policy with
/// the Access Levels provided. This is done atomically.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplaceAccessLevelsRequest {
    /// Required. Resource name for the access policy which owns these
    /// [Access Levels]
    /// \[google.identity.accesscontextmanager.v1.AccessLevel\].
    ///
    /// Format: `accessPolicies/{policy_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The desired [Access Levels]
    /// \[google.identity.accesscontextmanager.v1.AccessLevel\] that should
    /// replace all existing [Access Levels]
    /// \[google.identity.accesscontextmanager.v1.AccessLevel\] in the
    /// [Access Policy]
    /// \[google.identity.accesscontextmanager.v1.AccessPolicy\].
    #[prost(message, repeated, tag = "2")]
    pub access_levels: ::prost::alloc::vec::Vec<AccessLevel>,
    /// Optional. The etag for the version of the [Access Policy]
    /// \[google.identity.accesscontextmanager.v1.AccessPolicy\] that this
    /// replace operation is to be performed on. If, at the time of replace, the
    /// etag for the Access Policy stored in Access Context Manager is different
    /// from the specified etag, then the replace operation will not be performed
    /// and the call will fail. This field is not required. If etag is not
    /// provided, the operation will be performed as if a valid etag is provided.
    #[prost(string, tag = "4")]
    pub etag: ::prost::alloc::string::String,
}
/// A response to ReplaceAccessLevelsRequest. This will be put inside of
/// Operation.response field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplaceAccessLevelsResponse {
    /// List of the [Access Level]
    /// \[google.identity.accesscontextmanager.v1.AccessLevel\] instances.
    #[prost(message, repeated, tag = "1")]
    pub access_levels: ::prost::alloc::vec::Vec<AccessLevel>,
}
/// A request to list all `ServicePerimeters` in an `AccessPolicy`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicePerimetersRequest {
    /// Required. Resource name for the access policy to list [Service Perimeters]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeter\] from.
    ///
    /// Format:
    /// `accessPolicies/{policy_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Number of [Service Perimeters]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeter\] to include
    /// in the list. Default 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Next page token for the next batch of [Service Perimeter]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeter\] instances.
    /// Defaults to the first page of results.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// A response to `ListServicePerimetersRequest`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicePerimetersResponse {
    /// List of the [Service Perimeter]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeter\] instances.
    #[prost(message, repeated, tag = "1")]
    pub service_perimeters: ::prost::alloc::vec::Vec<ServicePerimeter>,
    /// The pagination token to retrieve the next page of results. If the value is
    /// empty, no further results remain.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A request to get a particular `ServicePerimeter`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServicePerimeterRequest {
    /// Required. Resource name for the [Service Perimeter]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeter\].
    ///
    /// Format:
    /// `accessPolicies/{policy_id}/servicePerimeters/{service_perimeters_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request to create a `ServicePerimeter`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateServicePerimeterRequest {
    /// Required. Resource name for the access policy which owns this [Service
    /// Perimeter] \[google.identity.accesscontextmanager.v1.ServicePerimeter\].
    ///
    /// Format: `accessPolicies/{policy_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The [Service Perimeter]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeter\] to create.
    /// Syntactic correctness of the [Service Perimeter]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeter\] is a
    /// precondition for creation.
    #[prost(message, optional, tag = "2")]
    pub service_perimeter: ::core::option::Option<ServicePerimeter>,
}
/// A request to update a `ServicePerimeter`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateServicePerimeterRequest {
    /// Required. The updated `ServicePerimeter`. Syntactic correctness of the
    /// `ServicePerimeter` is a precondition for creation.
    #[prost(message, optional, tag = "1")]
    pub service_perimeter: ::core::option::Option<ServicePerimeter>,
    /// Required. Mask to control which fields get updated. Must be non-empty.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// A request to delete a `ServicePerimeter`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteServicePerimeterRequest {
    /// Required. Resource name for the [Service Perimeter]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeter\].
    ///
    /// Format:
    /// `accessPolicies/{policy_id}/servicePerimeters/{service_perimeter_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request to replace all existing Service Perimeters in an Access Policy
/// with the Service Perimeters provided. This is done atomically.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplaceServicePerimetersRequest {
    /// Required. Resource name for the access policy which owns these
    /// [Service Perimeters]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeter\].
    ///
    /// Format: `accessPolicies/{policy_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The desired [Service Perimeters]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeter\] that should
    /// replace all existing [Service Perimeters]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeter\] in the
    /// [Access Policy]
    /// \[google.identity.accesscontextmanager.v1.AccessPolicy\].
    #[prost(message, repeated, tag = "2")]
    pub service_perimeters: ::prost::alloc::vec::Vec<ServicePerimeter>,
    /// Optional. The etag for the version of the [Access Policy]
    /// \[google.identity.accesscontextmanager.v1.AccessPolicy\] that this
    /// replace operation is to be performed on. If, at the time of replace, the
    /// etag for the Access Policy stored in Access Context Manager is different
    /// from the specified etag, then the replace operation will not be performed
    /// and the call will fail. This field is not required. If etag is not
    /// provided, the operation will be performed as if a valid etag is provided.
    #[prost(string, tag = "3")]
    pub etag: ::prost::alloc::string::String,
}
/// A response to ReplaceServicePerimetersRequest. This will be put inside of
/// Operation.response field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplaceServicePerimetersResponse {
    /// List of the [Service Perimeter]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeter\] instances.
    #[prost(message, repeated, tag = "1")]
    pub service_perimeters: ::prost::alloc::vec::Vec<ServicePerimeter>,
}
/// A request to commit dry-run specs in all [Service Perimeters]
/// \[google.identity.accesscontextmanager.v1.ServicePerimeter\] belonging to
/// an [Access Policy]\[google.identity.accesscontextmanager.v1.AccessPolicy\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitServicePerimetersRequest {
    /// Required. Resource name for the parent [Access Policy]
    /// \[google.identity.accesscontextmanager.v1.AccessPolicy\] which owns all
    /// [Service Perimeters]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeter\] in scope for
    /// the commit operation.
    ///
    /// Format: `accessPolicies/{policy_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The etag for the version of the [Access Policy]
    /// \[google.identity.accesscontextmanager.v1.AccessPolicy\] that this
    /// commit operation is to be performed on. If, at the time of commit, the
    /// etag for the Access Policy stored in Access Context Manager is different
    /// from the specified etag, then the commit operation will not be performed
    /// and the call will fail. This field is not required. If etag is not
    /// provided, the operation will be performed as if a valid etag is provided.
    #[prost(string, tag = "2")]
    pub etag: ::prost::alloc::string::String,
}
/// A response to CommitServicePerimetersRequest. This will be put inside of
/// Operation.response field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitServicePerimetersResponse {
    /// List of all the [Service Perimeter]
    /// \[google.identity.accesscontextmanager.v1.ServicePerimeter\] instances in
    /// the [Access Policy]
    /// \[google.identity.accesscontextmanager.v1.AccessPolicy\].
    #[prost(message, repeated, tag = "1")]
    pub service_perimeters: ::prost::alloc::vec::Vec<ServicePerimeter>,
}
/// Request of \[ListGcpUserAccessBindings\]
/// \[google.identity.accesscontextmanager.v1.AccessContextManager.ListGcpUserAccessBindings\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGcpUserAccessBindingsRequest {
    /// Required. Example: "organizations/256"
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of items to return. The server may return fewer items.
    /// If left blank, the server may return any number of items.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. If left blank, returns the first page. To enumerate all items, use the
    /// \[next_page_token\]
    /// \[google.identity.accesscontextmanager.v1.ListGcpUserAccessBindingsResponse.next_page_token\]
    /// from your previous list operation.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response of \[ListGcpUserAccessBindings\]
/// \[google.identity.accesscontextmanager.v1.AccessContextManager.ListGcpUserAccessBindings\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGcpUserAccessBindingsResponse {
    /// \[GcpUserAccessBinding\]
    /// \[google.identity.accesscontextmanager.v1.GcpUserAccessBinding\]
    #[prost(message, repeated, tag = "1")]
    pub gcp_user_access_bindings: ::prost::alloc::vec::Vec<GcpUserAccessBinding>,
    /// Token to get the next page of items. If blank, there are no more items.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request of \[GetGcpUserAccessBinding\]
/// \[google.identity.accesscontextmanager.v1.AccessContextManager.GetGcpUserAccessBinding\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGcpUserAccessBindingRequest {
    /// Required. Example: "organizations/256/gcpUserAccessBindings/b3-BhcX_Ud5N"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request of \[CreateGcpUserAccessBinding\]
/// \[google.identity.accesscontextmanager.v1.AccessContextManager.CreateGcpUserAccessBinding\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGcpUserAccessBindingRequest {
    /// Required. Example: "organizations/256"
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. \[GcpUserAccessBinding\]
    /// \[google.identity.accesscontextmanager.v1.GcpUserAccessBinding\]
    #[prost(message, optional, tag = "2")]
    pub gcp_user_access_binding: ::core::option::Option<GcpUserAccessBinding>,
}
/// Request of \[UpdateGcpUserAccessBinding\]
/// \[google.identity.accesscontextmanager.v1.AccessContextManager.UpdateGcpUserAccessBinding\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGcpUserAccessBindingRequest {
    /// Required. \[GcpUserAccessBinding\]
    /// \[google.identity.accesscontextmanager.v1.GcpUserAccessBinding\]
    #[prost(message, optional, tag = "1")]
    pub gcp_user_access_binding: ::core::option::Option<GcpUserAccessBinding>,
    /// Required. Only the fields specified in this mask are updated. Because name and
    /// group_key cannot be changed, update_mask is required and must always be:
    ///
    /// update_mask {
    /// paths: "access_levels"
    /// }
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request of \[DeleteGcpUserAccessBinding\]
/// \[google.identity.accesscontextmanager.v1.AccessContextManager.DeleteGcpUserAccessBinding\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteGcpUserAccessBindingRequest {
    /// Required. Example: "organizations/256/gcpUserAccessBindings/b3-BhcX_Ud5N"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Currently, a completed operation means nothing. In the future, this metadata
/// and a completed operation may indicate that the binding has taken effect and
/// is affecting access decisions for all users.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcpUserAccessBindingOperationMetadata {}
/// Metadata of Access Context Manager's Long Running Operations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessContextManagerOperationMetadata {}
/// The format used in an `AccessLevel`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LevelFormat {
    /// The format was not specified.
    Unspecified = 0,
    /// Uses the format the resource was defined in. BasicLevels are returned as
    /// BasicLevels, CustomLevels are returned as CustomLevels.
    AsDefined = 1,
    /// Use Cloud Common Expression Language when returning the resource.  Both
    /// BasicLevels and CustomLevels are returned as CustomLevels.
    Cel = 2,
}
impl LevelFormat {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LevelFormat::Unspecified => "LEVEL_FORMAT_UNSPECIFIED",
            LevelFormat::AsDefined => "AS_DEFINED",
            LevelFormat::Cel => "CEL",
        }
    }
}
/// Generated client implementations.
pub mod access_context_manager_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// API for setting [access levels]
    /// [google.identity.accesscontextmanager.v1.AccessLevel] and [service
    /// perimeters] [google.identity.accesscontextmanager.v1.ServicePerimeter]
    /// for Google Cloud projects. Each organization has one [access policy]
    /// [google.identity.accesscontextmanager.v1.AccessPolicy] that contains the
    /// [access levels] [google.identity.accesscontextmanager.v1.AccessLevel]
    /// and [service perimeters]
    /// [google.identity.accesscontextmanager.v1.ServicePerimeter]. This
    /// [access policy] [google.identity.accesscontextmanager.v1.AccessPolicy] is
    /// applicable to all resources in the organization.
    /// AccessPolicies
    #[derive(Debug, Clone)]
    pub struct AccessContextManagerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AccessContextManagerClient<T>
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
        ) -> AccessContextManagerClient<InterceptedService<T, F>>
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
            AccessContextManagerClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists all [access policies]
        /// [google.identity.accesscontextmanager.v1.AccessPolicy] in an
        /// organization.
        pub async fn list_access_policies(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAccessPoliciesRequest>,
        ) -> Result<tonic::Response<super::ListAccessPoliciesResponse>, tonic::Status> {
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
                "/google.identity.accesscontextmanager.v1.AccessContextManager/ListAccessPolicies",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns an [access policy]
        /// [google.identity.accesscontextmanager.v1.AccessPolicy] based on the name.
        pub async fn get_access_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccessPolicyRequest>,
        ) -> Result<tonic::Response<super::AccessPolicy>, tonic::Status> {
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
                "/google.identity.accesscontextmanager.v1.AccessContextManager/GetAccessPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates an access policy. This method fails if the organization already has
        /// an access policy. The long-running operation has a successful status
        /// after the access policy propagates to long-lasting storage.
        /// Syntactic and basic semantic errors are returned in `metadata` as a
        /// BadRequest proto.
        pub async fn create_access_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::AccessPolicy>,
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
                "/google.identity.accesscontextmanager.v1.AccessContextManager/CreateAccessPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an [access policy]
        /// [google.identity.accesscontextmanager.v1.AccessPolicy]. The
        /// long-running operation from this RPC has a successful status after the
        /// changes to the [access policy]
        /// [google.identity.accesscontextmanager.v1.AccessPolicy] propagate
        /// to long-lasting storage.
        pub async fn update_access_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAccessPolicyRequest>,
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
                "/google.identity.accesscontextmanager.v1.AccessContextManager/UpdateAccessPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes an [access policy]
        /// [google.identity.accesscontextmanager.v1.AccessPolicy] based on the
        /// resource name. The long-running operation has a successful status after the
        /// [access policy] [google.identity.accesscontextmanager.v1.AccessPolicy]
        /// is removed from long-lasting storage.
        pub async fn delete_access_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAccessPolicyRequest>,
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
                "/google.identity.accesscontextmanager.v1.AccessContextManager/DeleteAccessPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all [access levels]
        /// [google.identity.accesscontextmanager.v1.AccessLevel] for an access
        /// policy.
        pub async fn list_access_levels(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAccessLevelsRequest>,
        ) -> Result<tonic::Response<super::ListAccessLevelsResponse>, tonic::Status> {
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
                "/google.identity.accesscontextmanager.v1.AccessContextManager/ListAccessLevels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets an [access level]
        /// [google.identity.accesscontextmanager.v1.AccessLevel] based on the resource
        /// name.
        pub async fn get_access_level(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccessLevelRequest>,
        ) -> Result<tonic::Response<super::AccessLevel>, tonic::Status> {
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
                "/google.identity.accesscontextmanager.v1.AccessContextManager/GetAccessLevel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates an [access level]
        /// [google.identity.accesscontextmanager.v1.AccessLevel]. The long-running
        /// operation from this RPC has a successful status after the [access
        /// level] [google.identity.accesscontextmanager.v1.AccessLevel]
        /// propagates to long-lasting storage. If [access levels]
        /// [google.identity.accesscontextmanager.v1.AccessLevel] contain
        /// errors, an error response is returned for the first error encountered.
        pub async fn create_access_level(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAccessLevelRequest>,
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
                "/google.identity.accesscontextmanager.v1.AccessContextManager/CreateAccessLevel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an [access level]
        /// [google.identity.accesscontextmanager.v1.AccessLevel]. The long-running
        /// operation from this RPC has a successful status after the changes to
        /// the [access level]
        /// [google.identity.accesscontextmanager.v1.AccessLevel] propagate
        /// to long-lasting storage. If [access levels]
        /// [google.identity.accesscontextmanager.v1.AccessLevel] contain
        /// errors, an error response is returned for the first error encountered.
        pub async fn update_access_level(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAccessLevelRequest>,
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
                "/google.identity.accesscontextmanager.v1.AccessContextManager/UpdateAccessLevel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes an [access level]
        /// [google.identity.accesscontextmanager.v1.AccessLevel] based on the resource
        /// name. The long-running operation from this RPC has a successful status
        /// after the [access level]
        /// [google.identity.accesscontextmanager.v1.AccessLevel] has been removed
        /// from long-lasting storage.
        pub async fn delete_access_level(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAccessLevelRequest>,
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
                "/google.identity.accesscontextmanager.v1.AccessContextManager/DeleteAccessLevel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Replaces all existing [access levels]
        /// [google.identity.accesscontextmanager.v1.AccessLevel] in an [access
        /// policy] [google.identity.accesscontextmanager.v1.AccessPolicy] with
        /// the [access levels]
        /// [google.identity.accesscontextmanager.v1.AccessLevel] provided. This
        /// is done atomically. The long-running operation from this RPC has a
        /// successful status after all replacements propagate to long-lasting
        /// storage. If the replacement contains errors, an error response is returned
        /// for the first error encountered.  Upon error, the replacement is cancelled,
        /// and existing [access levels]
        /// [google.identity.accesscontextmanager.v1.AccessLevel] are not
        /// affected. The Operation.response field contains
        /// ReplaceAccessLevelsResponse. Removing [access levels]
        /// [google.identity.accesscontextmanager.v1.AccessLevel] contained in existing
        /// [service perimeters]
        /// [google.identity.accesscontextmanager.v1.ServicePerimeter] result in an
        /// error.
        pub async fn replace_access_levels(
            &mut self,
            request: impl tonic::IntoRequest<super::ReplaceAccessLevelsRequest>,
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
                "/google.identity.accesscontextmanager.v1.AccessContextManager/ReplaceAccessLevels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all [service perimeters]
        /// [google.identity.accesscontextmanager.v1.ServicePerimeter] for an
        /// access policy.
        pub async fn list_service_perimeters(
            &mut self,
            request: impl tonic::IntoRequest<super::ListServicePerimetersRequest>,
        ) -> Result<
            tonic::Response<super::ListServicePerimetersResponse>,
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
                "/google.identity.accesscontextmanager.v1.AccessContextManager/ListServicePerimeters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a [service perimeter]
        /// [google.identity.accesscontextmanager.v1.ServicePerimeter] based on the
        /// resource name.
        pub async fn get_service_perimeter(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServicePerimeterRequest>,
        ) -> Result<tonic::Response<super::ServicePerimeter>, tonic::Status> {
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
                "/google.identity.accesscontextmanager.v1.AccessContextManager/GetServicePerimeter",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a [service perimeter]
        /// [google.identity.accesscontextmanager.v1.ServicePerimeter]. The
        /// long-running operation from this RPC has a successful status after the
        /// [service perimeter]
        /// [google.identity.accesscontextmanager.v1.ServicePerimeter]
        /// propagates to long-lasting storage. If a [service perimeter]
        /// [google.identity.accesscontextmanager.v1.ServicePerimeter] contains
        /// errors, an error response is returned for the first error encountered.
        pub async fn create_service_perimeter(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateServicePerimeterRequest>,
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
                "/google.identity.accesscontextmanager.v1.AccessContextManager/CreateServicePerimeter",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a [service perimeter]
        /// [google.identity.accesscontextmanager.v1.ServicePerimeter]. The
        /// long-running operation from this RPC has a successful status after the
        /// [service perimeter]
        /// [google.identity.accesscontextmanager.v1.ServicePerimeter]
        /// propagates to long-lasting storage. If a [service perimeter]
        /// [google.identity.accesscontextmanager.v1.ServicePerimeter] contains
        /// errors, an error response is returned for the first error encountered.
        pub async fn update_service_perimeter(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateServicePerimeterRequest>,
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
                "/google.identity.accesscontextmanager.v1.AccessContextManager/UpdateServicePerimeter",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a [service perimeter]
        /// [google.identity.accesscontextmanager.v1.ServicePerimeter] based on the
        /// resource name. The long-running operation from this RPC has a successful
        /// status after the [service perimeter]
        /// [google.identity.accesscontextmanager.v1.ServicePerimeter] is removed from
        /// long-lasting storage.
        pub async fn delete_service_perimeter(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteServicePerimeterRequest>,
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
                "/google.identity.accesscontextmanager.v1.AccessContextManager/DeleteServicePerimeter",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Replace all existing [service perimeters]
        /// [google.identity.accesscontextmanager.v1.ServicePerimeter] in an [access
        /// policy] [google.identity.accesscontextmanager.v1.AccessPolicy] with the
        /// [service perimeters]
        /// [google.identity.accesscontextmanager.v1.ServicePerimeter] provided. This
        /// is done atomically. The long-running operation from this RPC has a
        /// successful status after all replacements propagate to long-lasting storage.
        /// Replacements containing errors result in an error response for the first
        /// error encountered. Upon an error, replacement are cancelled and existing
        /// [service perimeters]
        /// [google.identity.accesscontextmanager.v1.ServicePerimeter] are not
        /// affected. The Operation.response field contains
        /// ReplaceServicePerimetersResponse.
        pub async fn replace_service_perimeters(
            &mut self,
            request: impl tonic::IntoRequest<super::ReplaceServicePerimetersRequest>,
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
                "/google.identity.accesscontextmanager.v1.AccessContextManager/ReplaceServicePerimeters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Commits the dry-run specification for all the [service perimeters]
        /// [google.identity.accesscontextmanager.v1.ServicePerimeter] in an
        /// [access policy][google.identity.accesscontextmanager.v1.AccessPolicy].
        /// A commit operation on a service perimeter involves copying its `spec` field
        /// to the `status` field of the service perimeter. Only [service perimeters]
        /// [google.identity.accesscontextmanager.v1.ServicePerimeter] with
        /// `use_explicit_dry_run_spec` field set to true are affected by a commit
        /// operation. The long-running operation from this RPC has a successful
        /// status after the dry-run specifications for all the [service perimeters]
        /// [google.identity.accesscontextmanager.v1.ServicePerimeter] have been
        /// committed. If a commit fails, it causes the long-running operation to
        /// return an error response and the entire commit operation is cancelled.
        /// When successful, the Operation.response field contains
        /// CommitServicePerimetersResponse. The `dry_run` and the `spec` fields are
        /// cleared after a successful commit operation.
        pub async fn commit_service_perimeters(
            &mut self,
            request: impl tonic::IntoRequest<super::CommitServicePerimetersRequest>,
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
                "/google.identity.accesscontextmanager.v1.AccessContextManager/CommitServicePerimeters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all [GcpUserAccessBindings]
        /// [google.identity.accesscontextmanager.v1.GcpUserAccessBinding] for a
        /// Google Cloud organization.
        pub async fn list_gcp_user_access_bindings(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGcpUserAccessBindingsRequest>,
        ) -> Result<
            tonic::Response<super::ListGcpUserAccessBindingsResponse>,
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
                "/google.identity.accesscontextmanager.v1.AccessContextManager/ListGcpUserAccessBindings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the [GcpUserAccessBinding]
        /// [google.identity.accesscontextmanager.v1.GcpUserAccessBinding] with
        /// the given name.
        pub async fn get_gcp_user_access_binding(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGcpUserAccessBindingRequest>,
        ) -> Result<tonic::Response<super::GcpUserAccessBinding>, tonic::Status> {
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
                "/google.identity.accesscontextmanager.v1.AccessContextManager/GetGcpUserAccessBinding",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a [GcpUserAccessBinding]
        /// [google.identity.accesscontextmanager.v1.GcpUserAccessBinding]. If the
        /// client specifies a [name]
        /// [google.identity.accesscontextmanager.v1.GcpUserAccessBinding.name],
        /// the server ignores it. Fails if a resource already exists with the same
        /// [group_key]
        /// [google.identity.accesscontextmanager.v1.GcpUserAccessBinding.group_key].
        /// Completion of this long-running operation does not necessarily signify that
        /// the new binding is deployed onto all affected users, which may take more
        /// time.
        pub async fn create_gcp_user_access_binding(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateGcpUserAccessBindingRequest>,
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
                "/google.identity.accesscontextmanager.v1.AccessContextManager/CreateGcpUserAccessBinding",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a [GcpUserAccessBinding]
        /// [google.identity.accesscontextmanager.v1.GcpUserAccessBinding].
        /// Completion of this long-running operation does not necessarily signify that
        /// the changed binding is deployed onto all affected users, which may take
        /// more time.
        pub async fn update_gcp_user_access_binding(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateGcpUserAccessBindingRequest>,
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
                "/google.identity.accesscontextmanager.v1.AccessContextManager/UpdateGcpUserAccessBinding",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a [GcpUserAccessBinding]
        /// [google.identity.accesscontextmanager.v1.GcpUserAccessBinding].
        /// Completion of this long-running operation does not necessarily signify that
        /// the binding deletion is deployed onto all affected users, which may take
        /// more time.
        pub async fn delete_gcp_user_access_binding(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteGcpUserAccessBindingRequest>,
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
                "/google.identity.accesscontextmanager.v1.AccessContextManager/DeleteGcpUserAccessBinding",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the IAM policy for the specified Access Context Manager
        /// [access policy][google.identity.accesscontextmanager.v1.AccessPolicy].
        /// This method replaces the existing IAM policy on the access policy. The IAM
        /// policy controls the set of users who can perform specific operations on the
        /// Access Context Manager [access
        /// policy][google.identity.accesscontextmanager.v1.AccessPolicy].
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
                "/google.identity.accesscontextmanager.v1.AccessContextManager/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the IAM policy for the specified Access Context Manager
        /// [access policy][google.identity.accesscontextmanager.v1.AccessPolicy].
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
                "/google.identity.accesscontextmanager.v1.AccessContextManager/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the IAM permissions that the caller has on the specified Access
        /// Context Manager resource. The resource can be an
        /// [AccessPolicy][google.identity.accesscontextmanager.v1.AccessPolicy],
        /// [AccessLevel][google.identity.accesscontextmanager.v1.AccessLevel], or
        /// [ServicePerimeter][google.identity.accesscontextmanager.v1.ServicePerimeter
        /// ]. This method does not support other resources.
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
                "/google.identity.accesscontextmanager.v1.AccessContextManager/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
