/// Membership contains information about a member cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Membership {
    /// Output only. The full, unique name of this Membership resource in the format
    /// `projects/*/locations/*/memberships/{membership_id}`, set during creation.
    ///
    /// `membership_id` must be a valid RFC 1123 compliant DNS label:
    ///
    ///    1. At most 63 characters in length
    ///    2. It must consist of lower case alphanumeric characters or `-`
    ///    3. It must start and end with an alphanumeric character
    ///
    /// Which can be expressed as the regex: `\[a-z0-9]([-a-z0-9]*[a-z0-9\])?`,
    /// with a maximum length of 63 characters.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. GCP labels for this membership.
    #[prost(btree_map = "string, string", tag = "2")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. Description of this membership, limited to 63 characters.
    /// Must match the regex: `\[a-zA-Z0-9\][a-zA-Z0-9_\-\.\ ]*`
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. State of the Membership resource.
    #[prost(message, optional, tag = "5")]
    pub state: ::core::option::Option<MembershipState>,
    /// Optional. How to identify workloads from this Membership.
    /// See the documentation on Workload Identity for more details:
    /// <https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity>
    #[prost(message, optional, tag = "9")]
    pub authority: ::core::option::Option<Authority>,
    /// Output only. When the Membership was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. When the Membership was last updated.
    #[prost(message, optional, tag = "7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. When the Membership was deleted.
    #[prost(message, optional, tag = "8")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. An externally-generated and managed ID for this Membership. This ID may
    /// be modified after creation, but this is not recommended. For GKE clusters,
    /// external_id is managed by the Hub API and updates will be ignored.
    ///
    /// The ID must match the regex: `\[a-zA-Z0-9][a-zA-Z0-9_\-\.\]*`
    ///
    /// If this Membership represents a Kubernetes cluster, this value should be
    /// set to the UID of the `kube-system` namespace object.
    #[prost(string, tag = "10")]
    pub external_id: ::prost::alloc::string::String,
    /// Output only. For clusters using Connect, the timestamp of the most recent connection
    /// established with Google Cloud. This time is updated every several minutes,
    /// not continuously. For clusters that do not use GKE Connect, or that have
    /// never connected successfully, this field will be unset.
    #[prost(message, optional, tag = "11")]
    pub last_connection_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Google-generated UUID for this resource. This is unique across all
    /// Membership resources. If a Membership resource is deleted and another
    /// resource with the same name is created, it gets a different unique_id.
    #[prost(string, tag = "12")]
    pub unique_id: ::prost::alloc::string::String,
    /// Optional. The infrastructure type this Membership is running on.
    #[prost(enumeration = "membership::InfrastructureType", tag = "13")]
    pub infrastructure_type: i32,
    /// Type of resource represented by this Membership
    #[prost(oneof = "membership::Type", tags = "4")]
    pub r#type: ::core::option::Option<membership::Type>,
}
/// Nested message and enum types in `Membership`.
pub mod membership {
    /// Specifies the infrastructure type of a Membership. Infrastructure type is
    /// used by Hub to control infrastructure-specific behavior, including pricing.
    ///
    /// Each GKE distribution (on-GCP, on-Prem, on-X,...) will set this field
    /// automatically, but Attached Clusters customers should specify a type
    /// during registration.
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
    pub enum InfrastructureType {
        /// No type was specified. Some Hub functionality may require a type be
        /// specified, and will not support Memberships with this value.
        Unspecified = 0,
        /// Private infrastructure that is owned or operated by customer. This
        /// includes GKE distributions such as GKE-OnPrem and GKE-OnBareMetal.
        OnPrem = 1,
        /// Public cloud infrastructure.
        MultiCloud = 2,
    }
    impl InfrastructureType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                InfrastructureType::Unspecified => "INFRASTRUCTURE_TYPE_UNSPECIFIED",
                InfrastructureType::OnPrem => "ON_PREM",
                InfrastructureType::MultiCloud => "MULTI_CLOUD",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INFRASTRUCTURE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "ON_PREM" => Some(Self::OnPrem),
                "MULTI_CLOUD" => Some(Self::MultiCloud),
                _ => None,
            }
        }
    }
    /// Type of resource represented by this Membership
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// Optional. Endpoint information to reach this member.
        #[prost(message, tag = "4")]
        Endpoint(super::MembershipEndpoint),
    }
}
/// MembershipEndpoint contains information needed to contact a Kubernetes API,
/// endpoint and any additional Kubernetes metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MembershipEndpoint {
    /// Output only. Useful Kubernetes-specific metadata.
    #[prost(message, optional, tag = "5")]
    pub kubernetes_metadata: ::core::option::Option<KubernetesMetadata>,
    /// Optional. The in-cluster Kubernetes Resources that should be applied for a correctly
    /// registered cluster, in the steady state. These resources:
    ///
    ///    * Ensure that the cluster is exclusively registered to one and only one
    ///      Hub Membership.
    ///    * Propagate Workload Pool Information available in the Membership
    ///      Authority field.
    ///    * Ensure proper initial configuration of default Hub Features.
    #[prost(message, optional, tag = "6")]
    pub kubernetes_resource: ::core::option::Option<KubernetesResource>,
    /// Cluster information of the registered cluster.
    #[prost(oneof = "membership_endpoint::Type", tags = "4, 7, 8, 9, 10")]
    pub r#type: ::core::option::Option<membership_endpoint::Type>,
}
/// Nested message and enum types in `MembershipEndpoint`.
pub mod membership_endpoint {
    /// Cluster information of the registered cluster.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// Optional. Specific information for a GKE-on-GCP cluster.
        #[prost(message, tag = "4")]
        GkeCluster(super::GkeCluster),
        /// Optional. Specific information for a GKE On-Prem cluster. An onprem user-cluster
        /// who has no resourceLink is not allowed to use this field, it should have
        /// a nil "type" instead.
        #[prost(message, tag = "7")]
        OnPremCluster(super::OnPremCluster),
        /// Optional. Specific information for a GKE Multi-Cloud cluster.
        #[prost(message, tag = "8")]
        MultiCloudCluster(super::MultiCloudCluster),
        /// Optional. Specific information for a Google Edge cluster.
        #[prost(message, tag = "9")]
        EdgeCluster(super::EdgeCluster),
        /// Optional. Specific information for a GDC Edge Appliance cluster.
        #[prost(message, tag = "10")]
        ApplianceCluster(super::ApplianceCluster),
    }
}
/// KubernetesResource contains the YAML manifests and configuration for
/// Membership Kubernetes resources in the cluster. After CreateMembership or
/// UpdateMembership, these resources should be re-applied in the cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KubernetesResource {
    /// Input only. The YAML representation of the Membership CR. This field is ignored for GKE
    /// clusters where Hub can read the CR directly.
    ///
    /// Callers should provide the CR that is currently present in the cluster
    /// during CreateMembership or UpdateMembership, or leave this field empty if
    /// none exists. The CR manifest is used to validate the cluster has not been
    /// registered with another Membership.
    #[prost(string, tag = "1")]
    pub membership_cr_manifest: ::prost::alloc::string::String,
    /// Output only. Additional Kubernetes resources that need to be applied to the cluster
    /// after Membership creation, and after every update.
    ///
    /// This field is only populated in the Membership returned from a successful
    /// long-running operation from CreateMembership or UpdateMembership. It is not
    /// populated during normal GetMembership or ListMemberships requests. To get
    /// the resource manifest after the initial registration, the caller should
    /// make a UpdateMembership call with an empty field mask.
    #[prost(message, repeated, tag = "2")]
    pub membership_resources: ::prost::alloc::vec::Vec<ResourceManifest>,
    /// Output only. The Kubernetes resources for installing the GKE Connect agent
    ///
    /// This field is only populated in the Membership returned from a successful
    /// long-running operation from CreateMembership or UpdateMembership. It is not
    /// populated during normal GetMembership or ListMemberships requests. To get
    /// the resource manifest after the initial registration, the caller should
    /// make a UpdateMembership call with an empty field mask.
    #[prost(message, repeated, tag = "3")]
    pub connect_resources: ::prost::alloc::vec::Vec<ResourceManifest>,
    /// Optional. Options for Kubernetes resource generation.
    #[prost(message, optional, tag = "4")]
    pub resource_options: ::core::option::Option<ResourceOptions>,
}
/// ResourceOptions represent options for Kubernetes resource generation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceOptions {
    /// Optional. The Connect agent version to use for connect_resources. Defaults to the
    /// latest GKE Connect version. The version must be a currently supported
    /// version, obsolete versions will be rejected.
    #[prost(string, tag = "1")]
    pub connect_version: ::prost::alloc::string::String,
    /// Optional. Use `apiextensions/v1beta1` instead of `apiextensions/v1` for
    /// CustomResourceDefinition resources.
    /// This option should be set for clusters with Kubernetes apiserver versions
    /// <1.16.
    #[prost(bool, tag = "2")]
    pub v1beta1_crd: bool,
    /// Optional. Major version of the Kubernetes cluster. This is only used to determine
    /// which version to use for the CustomResourceDefinition resources,
    /// `apiextensions/v1beta1` or`apiextensions/v1`.
    #[prost(string, tag = "3")]
    pub k8s_version: ::prost::alloc::string::String,
}
/// ResourceManifest represents a single Kubernetes resource to be applied to
/// the cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceManifest {
    /// YAML manifest of the resource.
    #[prost(string, tag = "1")]
    pub manifest: ::prost::alloc::string::String,
    /// Whether the resource provided in the manifest is `cluster_scoped`.
    /// If unset, the manifest is assumed to be namespace scoped.
    ///
    /// This field is used for REST mapping when applying the resource in a
    /// cluster.
    #[prost(bool, tag = "2")]
    pub cluster_scoped: bool,
}
/// GkeCluster contains information specific to GKE clusters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GkeCluster {
    /// Immutable. Self-link of the GCP resource for the GKE cluster. For example:
    ///
    ///      //container.googleapis.com/projects/my-project/locations/us-west1-a/clusters/my-cluster
    ///
    /// Zonal clusters are also supported.
    #[prost(string, tag = "1")]
    pub resource_link: ::prost::alloc::string::String,
    /// Output only. If cluster_missing is set then it denotes that the GKE cluster no longer
    /// exists in the GKE Control Plane.
    #[prost(bool, tag = "3")]
    pub cluster_missing: bool,
}
/// OnPremCluster contains information specific to GKE On-Prem clusters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnPremCluster {
    /// Immutable. Self-link of the GCP resource for the GKE On-Prem cluster. For example:
    ///
    ///   //gkeonprem.googleapis.com/projects/my-project/locations/us-west1-a/vmwareClusters/my-cluster
    ///   //gkeonprem.googleapis.com/projects/my-project/locations/us-west1-a/bareMetalClusters/my-cluster
    #[prost(string, tag = "1")]
    pub resource_link: ::prost::alloc::string::String,
    /// Output only. If cluster_missing is set then it denotes that
    /// API(gkeonprem.googleapis.com) resource for this GKE On-Prem cluster no
    /// longer exists.
    #[prost(bool, tag = "2")]
    pub cluster_missing: bool,
    /// Immutable. Whether the cluster is an admin cluster.
    #[prost(bool, tag = "3")]
    pub admin_cluster: bool,
    /// Immutable. The on prem cluster's type.
    #[prost(enumeration = "on_prem_cluster::ClusterType", tag = "4")]
    pub cluster_type: i32,
}
/// Nested message and enum types in `OnPremCluster`.
pub mod on_prem_cluster {
    /// ClusterType describes on prem cluster's type.
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
    pub enum ClusterType {
        /// The ClusterType is not set.
        ClustertypeUnspecified = 0,
        /// The ClusterType is bootstrap cluster.
        Bootstrap = 1,
        /// The ClusterType is baremetal hybrid cluster.
        Hybrid = 2,
        /// The ClusterType is baremetal standalone cluster.
        Standalone = 3,
        /// The ClusterType is user cluster.
        User = 4,
    }
    impl ClusterType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ClusterType::ClustertypeUnspecified => "CLUSTERTYPE_UNSPECIFIED",
                ClusterType::Bootstrap => "BOOTSTRAP",
                ClusterType::Hybrid => "HYBRID",
                ClusterType::Standalone => "STANDALONE",
                ClusterType::User => "USER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CLUSTERTYPE_UNSPECIFIED" => Some(Self::ClustertypeUnspecified),
                "BOOTSTRAP" => Some(Self::Bootstrap),
                "HYBRID" => Some(Self::Hybrid),
                "STANDALONE" => Some(Self::Standalone),
                "USER" => Some(Self::User),
                _ => None,
            }
        }
    }
}
/// MultiCloudCluster contains information specific to GKE Multi-Cloud clusters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiCloudCluster {
    /// Immutable. Self-link of the GCP resource for the GKE Multi-Cloud cluster. For
    /// example:
    ///
    ///   //gkemulticloud.googleapis.com/projects/my-project/locations/us-west1-a/awsClusters/my-cluster
    ///   //gkemulticloud.googleapis.com/projects/my-project/locations/us-west1-a/azureClusters/my-cluster
    #[prost(string, tag = "1")]
    pub resource_link: ::prost::alloc::string::String,
    /// Output only. If cluster_missing is set then it denotes that
    /// API(gkemulticloud.googleapis.com) resource for this GKE Multi-Cloud cluster
    /// no longer exists.
    #[prost(bool, tag = "2")]
    pub cluster_missing: bool,
}
/// EdgeCluster contains information specific to Google Edge Clusters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EdgeCluster {
    /// Immutable. Self-link of the GCP resource for the Edge Cluster. For
    /// example:
    ///
    /// //edgecontainer.googleapis.com/projects/my-project/locations/us-west1-a/clusters/my-cluster
    #[prost(string, tag = "1")]
    pub resource_link: ::prost::alloc::string::String,
}
/// ApplianceCluster contains information specific to GDC Edge Appliance
/// Clusters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplianceCluster {
    /// Immutable. Self-link of the GCP resource for the Appliance Cluster. For
    /// example:
    ///
    /// //transferappliance.googleapis.com/projects/my-project/locations/us-west1-a/appliances/my-appliance
    #[prost(string, tag = "1")]
    pub resource_link: ::prost::alloc::string::String,
}
/// KubernetesMetadata provides informational metadata for Memberships
/// representing Kubernetes clusters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KubernetesMetadata {
    /// Output only. Kubernetes API server version string as reported by '/version'.
    #[prost(string, tag = "1")]
    pub kubernetes_api_server_version: ::prost::alloc::string::String,
    /// Output only. Node providerID as reported by the first node in the list of nodes on
    /// the Kubernetes endpoint. On Kubernetes platforms that support zero-node
    /// clusters (like GKE-on-GCP), the node_count will be zero and the
    /// node_provider_id will be empty.
    #[prost(string, tag = "2")]
    pub node_provider_id: ::prost::alloc::string::String,
    /// Output only. Node count as reported by Kubernetes nodes resources.
    #[prost(int32, tag = "3")]
    pub node_count: i32,
    /// Output only. vCPU count as reported by Kubernetes nodes resources.
    #[prost(int32, tag = "4")]
    pub vcpu_count: i32,
    /// Output only. The total memory capacity as reported by the sum of all Kubernetes nodes
    /// resources, defined in MB.
    #[prost(int32, tag = "5")]
    pub memory_mb: i32,
    /// Output only. The time at which these details were last updated. This update_time is
    /// different from the Membership-level update_time since EndpointDetails are
    /// updated internally for API consumers.
    #[prost(message, optional, tag = "100")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Authority encodes how Google will recognize identities from this Membership.
/// See the workload identity documentation for more details:
/// <https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Authority {
    /// Optional. A JSON Web Token (JWT) issuer URI. `issuer` must start with `<https://`> and
    /// be a valid URL with length <2000 characters.
    ///
    /// If set, then Google will allow valid OIDC tokens from this issuer to
    /// authenticate within the workload_identity_pool. OIDC discovery will be
    /// performed on this URI to validate tokens from the issuer.
    ///
    /// Clearing `issuer` disables Workload Identity. `issuer` cannot be directly
    /// modified; it must be cleared (and Workload Identity disabled) before using
    /// a new issuer (and re-enabling Workload Identity).
    #[prost(string, tag = "1")]
    pub issuer: ::prost::alloc::string::String,
    /// Output only. The name of the workload identity pool in which `issuer` will be
    /// recognized.
    ///
    /// There is a single Workload Identity Pool per Hub that is shared
    /// between all Memberships that belong to that Hub. For a Hub hosted in
    /// {PROJECT_ID}, the workload pool format is `{PROJECT_ID}.hub.id.goog`,
    /// although this is subject to change in newer versions of this API.
    #[prost(string, tag = "2")]
    pub workload_identity_pool: ::prost::alloc::string::String,
    /// Output only. An identity provider that reflects the `issuer` in the workload identity
    /// pool.
    #[prost(string, tag = "3")]
    pub identity_provider: ::prost::alloc::string::String,
    /// Optional. OIDC verification keys for this Membership in JWKS format (RFC 7517).
    ///
    /// When this field is set, OIDC discovery will NOT be performed on `issuer`,
    /// and instead OIDC tokens will be validated using this field.
    #[prost(bytes = "bytes", tag = "4")]
    pub oidc_jwks: ::prost::bytes::Bytes,
}
/// State of the Membership resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MembershipState {
    /// Output only. The current state of the Membership resource.
    #[prost(enumeration = "membership_state::Code", tag = "1")]
    pub code: i32,
    /// This field is never set by the Hub Service.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// This field is never set by the Hub Service.
    #[deprecated]
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `MembershipState`.
pub mod membership_state {
    /// Code describes the state of a Membership resource.
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
    pub enum Code {
        /// The code is not set.
        Unspecified = 0,
        /// The cluster is being registered.
        Creating = 1,
        /// The cluster is registered.
        Ready = 2,
        /// The cluster is being unregistered.
        Deleting = 3,
        /// The Membership is being updated.
        Updating = 4,
        /// The Membership is being updated by the Hub Service.
        ServiceUpdating = 5,
    }
    impl Code {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Code::Unspecified => "CODE_UNSPECIFIED",
                Code::Creating => "CREATING",
                Code::Ready => "READY",
                Code::Deleting => "DELETING",
                Code::Updating => "UPDATING",
                Code::ServiceUpdating => "SERVICE_UPDATING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CODE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "READY" => Some(Self::Ready),
                "DELETING" => Some(Self::Deleting),
                "UPDATING" => Some(Self::Updating),
                "SERVICE_UPDATING" => Some(Self::ServiceUpdating),
                _ => None,
            }
        }
    }
}
/// Request message for `GkeHubMembershipService.ListMemberships` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMembershipsRequest {
    /// Required. The parent (project and location) where the Memberships will be listed.
    /// Specified in the format `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. When requesting a 'page' of resources, `page_size` specifies number of
    /// resources to return. If unspecified or set to 0, all resources will
    /// be returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Token returned by previous call to `ListMemberships` which
    /// specifies the position in the list from where to continue listing the
    /// resources.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Lists Memberships that match the filter expression, following the syntax
    /// outlined in <https://google.aip.dev/160.>
    ///
    /// Examples:
    ///
    ///    - Name is `bar` in project `foo-proj` and location `global`:
    ///
    ///        name = "projects/foo-proj/locations/global/membership/bar"
    ///
    ///    - Memberships that have a label called `foo`:
    ///
    ///        labels.foo:*
    ///
    ///    - Memberships that have a label called `foo` whose value is `bar`:
    ///
    ///        labels.foo = bar
    ///
    ///    - Memberships in the CREATING state:
    ///
    ///        state = CREATING
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. One or more fields to compare and use to sort the output.
    /// See <https://google.aip.dev/132#ordering.>
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for the `GkeHubMembershipService.ListMemberships` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMembershipsResponse {
    /// The list of matching Memberships.
    #[prost(message, repeated, tag = "1")]
    pub resources: ::prost::alloc::vec::Vec<Membership>,
    /// A token to request the next page of resources from the
    /// `ListMemberships` method. The value of an empty string means that
    /// there are no more resources to return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// List of locations that could not be reached while fetching this list.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for `GkeHubMembershipService.GetMembership` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMembershipRequest {
    /// Required. The Membership resource name in the format
    /// `projects/*/locations/*/memberships/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for the `GkeHubMembershipService.CreateMembership` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMembershipRequest {
    /// Required. The parent (project and location) where the Memberships will be created.
    /// Specified in the format `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Client chosen ID for the membership. `membership_id` must be a valid RFC
    /// 1123 compliant DNS label:
    ///
    ///    1. At most 63 characters in length
    ///    2. It must consist of lower case alphanumeric characters or `-`
    ///    3. It must start and end with an alphanumeric character
    ///
    /// Which can be expressed as the regex: `\[a-z0-9]([-a-z0-9]*[a-z0-9\])?`,
    /// with a maximum length of 63 characters.
    #[prost(string, tag = "2")]
    pub membership_id: ::prost::alloc::string::String,
    /// Required. The membership to create.
    #[prost(message, optional, tag = "3")]
    pub resource: ::core::option::Option<Membership>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for `GkeHubMembershipService.DeleteMembership` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMembershipRequest {
    /// Required. The Membership resource name in the format
    /// `projects/*/locations/*/memberships/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for `GkeHubMembershipService.UpdateMembership` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMembershipRequest {
    /// Required. The membership resource name in the format:
    /// `projects/\[project_id]/locations/global/memberships/[membership_id\]`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Mask of fields to update. At least one field path must be specified in this
    /// mask.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. Only fields specified in update_mask are updated.
    /// If you specify a field in the update_mask but don't specify its value here
    /// that field will be deleted.
    /// If you are updating a map field, set the value of a key to null or empty
    /// string to delete the key from the map. It's not possible to update a key's
    /// value to the empty string.
    /// If you specify the update_mask to be a special path "*", fully replaces all
    /// user-modifiable fields to match `resource`.
    #[prost(message, optional, tag = "3")]
    pub resource: ::core::option::Option<Membership>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for `GkeHubMembershipService.GenerateConnectManifest`
/// method.
/// .
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateConnectManifestRequest {
    /// Required. The Membership resource name the Agent will associate with, in the format
    /// `projects/*/locations/*/memberships/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The connect agent to generate manifest for.
    #[prost(message, optional, tag = "2")]
    pub connect_agent: ::core::option::Option<ConnectAgent>,
    /// Optional. The Connect agent version to use. Defaults to the most current version.
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
    /// Optional. If true, generate the resources for upgrade only. Some resources
    /// generated only for installation (e.g. secrets) will be excluded.
    #[prost(bool, tag = "4")]
    pub is_upgrade: bool,
    /// Optional. The registry to fetch the connect agent image from. Defaults to
    /// gcr.io/gkeconnect.
    #[prost(string, tag = "5")]
    pub registry: ::prost::alloc::string::String,
    /// Optional. The image pull secret content for the registry, if not public.
    #[prost(bytes = "bytes", tag = "6")]
    pub image_pull_secret_content: ::prost::bytes::Bytes,
}
/// GenerateConnectManifestResponse contains manifest information for
/// installing/upgrading a Connect agent.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateConnectManifestResponse {
    /// The ordered list of Kubernetes resources that need to be applied to the
    /// cluster for GKE Connect agent installation/upgrade.
    #[prost(message, repeated, tag = "1")]
    pub manifest: ::prost::alloc::vec::Vec<ConnectAgentResource>,
}
/// ConnectAgentResource represents a Kubernetes resource manifest for Connect
/// Agent deployment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectAgentResource {
    /// Kubernetes type of the resource.
    #[prost(message, optional, tag = "1")]
    pub r#type: ::core::option::Option<TypeMeta>,
    /// YAML manifest of the resource.
    #[prost(string, tag = "2")]
    pub manifest: ::prost::alloc::string::String,
}
/// TypeMeta is the type information needed for content unmarshalling of
/// Kubernetes resources in the manifest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeMeta {
    /// Kind of the resource (e.g. Deployment).
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// APIVersion of the resource (e.g. v1).
    #[prost(string, tag = "2")]
    pub api_version: ::prost::alloc::string::String,
}
/// The information required from end users to use GKE Connect.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectAgent {
    /// Do not set.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. URI of a proxy if connectivity from the agent to gkeconnect.googleapis.com
    /// requires the use of a proxy. Format must be in the form
    /// `http(s)://{proxy_address}`, depending on the HTTP/HTTPS protocol
    /// supported by the proxy. This will direct the connect agent's outbound
    /// traffic through a HTTP(S) proxy.
    #[prost(bytes = "bytes", tag = "2")]
    pub proxy: ::prost::bytes::Bytes,
    /// Optional. Namespace for GKE Connect agent resources. Defaults to `gke-connect`.
    ///
    /// The Connect Agent is authorized automatically when run in the default
    /// namespace. Otherwise, explicit authorization must be granted with an
    /// additional IAM binding.
    #[prost(string, tag = "3")]
    pub namespace: ::prost::alloc::string::String,
}
/// The request to validate the existing state of the membership CR in the
/// cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateExclusivityRequest {
    /// Required. The parent (project and location) where the Memberships will be created.
    /// Specified in the format `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The YAML of the membership CR in the cluster. Empty if the membership
    /// CR does not exist.
    #[prost(string, tag = "2")]
    pub cr_manifest: ::prost::alloc::string::String,
    /// Required. The intended membership name under the `parent`. This method only does
    /// validation in anticipation of a CreateMembership call with the same name.
    #[prost(string, tag = "3")]
    pub intended_membership: ::prost::alloc::string::String,
}
/// The response of exclusivity artifacts validation result status.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateExclusivityResponse {
    /// The validation result.
    ///
    /// * `OK` means that exclusivity is validated, assuming the manifest produced
    ///     by GenerateExclusivityManifest is successfully applied.
    /// * `ALREADY_EXISTS` means that the Membership CRD is already owned by
    ///     another Hub. See `status.message` for more information.
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<super::super::super::rpc::Status>,
}
/// The request to generate the manifests for exclusivity artifacts.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateExclusivityManifestRequest {
    /// Required. The Membership resource name in the format
    /// `projects/*/locations/*/memberships/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The YAML manifest of the membership CRD retrieved by
    /// `kubectl get customresourcedefinitions membership`.
    /// Leave empty if the resource does not exist.
    #[prost(string, tag = "2")]
    pub crd_manifest: ::prost::alloc::string::String,
    /// Optional. The YAML manifest of the membership CR retrieved by
    /// `kubectl get memberships membership`.
    /// Leave empty if the resource does not exist.
    #[prost(string, tag = "3")]
    pub cr_manifest: ::prost::alloc::string::String,
}
/// The response of the exclusivity artifacts manifests for the client to apply.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateExclusivityManifestResponse {
    /// The YAML manifest of the membership CRD to apply if a newer version of the
    /// CRD is available. Empty if no update needs to be applied.
    #[prost(string, tag = "1")]
    pub crd_manifest: ::prost::alloc::string::String,
    /// The YAML manifest of the membership CR to apply if a new version of the
    /// CR is available. Empty if no update needs to be applied.
    #[prost(string, tag = "2")]
    pub cr_manifest: ::prost::alloc::string::String,
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
    pub status_detail: ::prost::alloc::string::String,
    /// Output only. Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have \[Operation.error][\] value with a \[google.rpc.Status.code][google.rpc.Status.code\] of 1,
    /// corresponding to `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub cancel_requested: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod gke_hub_membership_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The GKE Hub MembershipService handles the registration of many Kubernetes
    /// clusters to Google Cloud, represented with the [Membership][google.cloud.gkehub.v1beta1.Membership] resource.
    ///
    /// GKE Hub is currently only available in the global region.
    ///
    /// **Membership management may be non-trivial:** it is recommended to use one
    /// of the Google-provided client libraries or tools where possible when working
    /// with Membership resources.
    #[derive(Debug, Clone)]
    pub struct GkeHubMembershipServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> GkeHubMembershipServiceClient<T>
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
        ) -> GkeHubMembershipServiceClient<InterceptedService<T, F>>
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
            GkeHubMembershipServiceClient::new(
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
        /// Lists Memberships in a given project and location.
        pub async fn list_memberships(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMembershipsRequest>,
        ) -> Result<tonic::Response<super::ListMembershipsResponse>, tonic::Status> {
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
                "/google.cloud.gkehub.v1beta1.GkeHubMembershipService/ListMemberships",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the details of a Membership.
        pub async fn get_membership(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMembershipRequest>,
        ) -> Result<tonic::Response<super::Membership>, tonic::Status> {
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
                "/google.cloud.gkehub.v1beta1.GkeHubMembershipService/GetMembership",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new Membership.
        ///
        /// **This is currently only supported for GKE clusters on Google Cloud**.
        /// To register other clusters, follow the instructions at
        /// https://cloud.google.com/anthos/multicluster-management/connect/registering-a-cluster.
        pub async fn create_membership(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateMembershipRequest>,
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
                "/google.cloud.gkehub.v1beta1.GkeHubMembershipService/CreateMembership",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Removes a Membership.
        ///
        /// **This is currently only supported for GKE clusters on Google Cloud**.
        /// To unregister other clusters, follow the instructions at
        /// https://cloud.google.com/anthos/multicluster-management/connect/unregistering-a-cluster.
        pub async fn delete_membership(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteMembershipRequest>,
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
                "/google.cloud.gkehub.v1beta1.GkeHubMembershipService/DeleteMembership",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an existing Membership.
        pub async fn update_membership(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateMembershipRequest>,
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
                "/google.cloud.gkehub.v1beta1.GkeHubMembershipService/UpdateMembership",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Generates the manifest for deployment of the GKE connect agent.
        ///
        /// **This method is used internally by Google-provided libraries.**
        /// Most clients should not need to call this method directly.
        pub async fn generate_connect_manifest(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateConnectManifestRequest>,
        ) -> Result<
            tonic::Response<super::GenerateConnectManifestResponse>,
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
                "/google.cloud.gkehub.v1beta1.GkeHubMembershipService/GenerateConnectManifest",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ValidateExclusivity validates the state of exclusivity in the cluster.
        /// The validation does not depend on an existing Hub membership resource.
        pub async fn validate_exclusivity(
            &mut self,
            request: impl tonic::IntoRequest<super::ValidateExclusivityRequest>,
        ) -> Result<tonic::Response<super::ValidateExclusivityResponse>, tonic::Status> {
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
                "/google.cloud.gkehub.v1beta1.GkeHubMembershipService/ValidateExclusivity",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GenerateExclusivityManifest generates the manifests to update the
        /// exclusivity artifacts in the cluster if needed.
        ///
        /// Exclusivity artifacts include the Membership custom resource definition
        /// (CRD) and the singleton Membership custom resource (CR). Combined with
        /// ValidateExclusivity, exclusivity artifacts guarantee that a Kubernetes
        /// cluster is only registered to a single GKE Hub.
        ///
        /// The Membership CRD is versioned, and may require conversion when the GKE
        /// Hub API server begins serving a newer version of the CRD and
        /// corresponding CR. The response will be the converted CRD and CR if there
        /// are any differences between the versions.
        pub async fn generate_exclusivity_manifest(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateExclusivityManifestRequest>,
        ) -> Result<
            tonic::Response<super::GenerateExclusivityManifestResponse>,
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
                "/google.cloud.gkehub.v1beta1.GkeHubMembershipService/GenerateExclusivityManifest",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
