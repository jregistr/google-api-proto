/// A deny rule in an IAM deny policy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenyRule {
    /// The identities that are prevented from using one or more permissions on
    /// Google Cloud resources. This field can contain the following values:
    ///
    /// * `principalSet://goog/public:all`: A special identifier that represents
    ///   any principal that is on the internet, even if they do not have a Google
    ///   Account or are not logged in.
    ///
    /// * `principal://goog/subject/{email_id}`: A specific Google Account.
    ///   Includes Gmail, Cloud Identity, and Google Workspace user accounts. For
    ///   example, `principal://goog/subject/alice@example.com`.
    ///
    /// * `deleted:principal://goog/subject/{email_id}?uid={uid}`: A specific
    ///   Google Account that was deleted recently. For example,
    ///   `deleted:principal://goog/subject/alice@example.com?uid=1234567890`. If
    ///   the Google Account is recovered, this identifier reverts to the standard
    ///   identifier for a Google Account.
    ///
    /// * `principalSet://goog/group/{group_id}`: A Google group. For example,
    ///   `principalSet://goog/group/admins@example.com`.
    ///
    /// * `deleted:principalSet://goog/group/{group_id}?uid={uid}`: A Google group
    ///   that was deleted recently. For example,
    ///   `deleted:principalSet://goog/group/admins@example.com?uid=1234567890`. If
    ///   the Google group is restored, this identifier reverts to the standard
    ///   identifier for a Google group.
    ///
    /// * `principal://iam.googleapis.com/projects/-/serviceAccounts/{service_account_id}`:
    ///   A Google Cloud service account. For example,
    ///   `principal://iam.googleapis.com/projects/-/serviceAccounts/my-service-account@iam.gserviceaccount.com`.
    ///
    /// * `deleted:principal://iam.googleapis.com/projects/-/serviceAccounts/{service_account_id}?uid={uid}`:
    ///   A Google Cloud service account that was deleted recently. For example,
    ///   `deleted:principal://iam.googleapis.com/projects/-/serviceAccounts/my-service-account@iam.gserviceaccount.com?uid=1234567890`.
    ///   If the service account is undeleted, this identifier reverts to the
    ///   standard identifier for a service account.
    ///
    /// * `principalSet://goog/cloudIdentityCustomerId/{customer_id}`: All of the
    ///   principals associated with the specified Google Workspace or Cloud
    ///   Identity customer ID. For example,
    ///   `principalSet://goog/cloudIdentityCustomerId/C01Abc35`.
    #[prost(string, repeated, tag="1")]
    pub denied_principals: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The identities that are excluded from the deny rule, even if they are
    /// listed in the `denied_principals`. For example, you could add a Google
    /// group to the `denied_principals`, then exclude specific users who belong to
    /// that group.
    ///
    /// This field can contain the same values as the `denied_principals` field,
    /// excluding `principalSet://goog/public:all`, which represents all users on
    /// the internet.
    #[prost(string, repeated, tag="2")]
    pub exception_principals: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The permissions that are explicitly denied by this rule. Each permission
    /// uses the format `{service_fqdn}/{resource}.{verb}`, where `{service_fqdn}`
    /// is the fully qualified domain name for the service. For example,
    /// `iam.googleapis.com/roles.list`.
    #[prost(string, repeated, tag="3")]
    pub denied_permissions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Specifies the permissions that this rule excludes from the set of denied
    /// permissions given by `denied_permissions`. If a permission appears in
    /// `denied_permissions` _and_ in `exception_permissions` then it will _not_ be
    /// denied.
    ///
    /// The excluded permissions can be specified using the same syntax as
    /// `denied_permissions`.
    #[prost(string, repeated, tag="4")]
    pub exception_permissions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The condition that determines whether this deny rule applies to a request.
    /// If the condition expression evaluates to `true`, then the deny rule is
    /// applied; otherwise, the deny rule is not applied.
    ///
    /// Each deny rule is evaluated independently. If this deny rule does not apply
    /// to a request, other deny rules might still apply.
    ///
    /// The condition can use CEL functions that evaluate
    /// [resource
    /// tags](<https://cloud.google.com/iam/help/conditions/resource-tags>). Other
    /// functions and operators are not supported.
    #[prost(message, optional, tag="5")]
    pub denial_condition: ::core::option::Option<super::super::r#type::Expr>,
}
/// Data for an IAM policy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Policy {
    /// Immutable. The resource name of the `Policy`, which must be unique. Format:
    /// `policies/{attachment_point}/denypolicies/{policy_id}`
    ///
    ///
    /// The attachment point is identified by its URL-encoded full resource name,
    /// which means that the forward-slash character, `/`, must be written as
    /// `%2F`. For example,
    /// `policies/cloudresourcemanager.googleapis.com%2Fprojects%2Fmy-project/denypolicies/my-deny-policy`.
    ///
    /// For organizations and folders, use the numeric ID in the full resource
    /// name. For projects, requests can use the alphanumeric or the numeric ID.
    /// Responses always contain the numeric ID.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. The globally unique ID of the `Policy`. Assigned automatically
    /// when the `Policy` is created.
    #[prost(string, tag="2")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. The kind of the `Policy`. Always contains the value
    /// `DenyPolicy`.
    #[prost(string, tag="3")]
    pub kind: ::prost::alloc::string::String,
    /// A user-specified description of the `Policy`. This value can be up to 63
    /// characters.
    #[prost(string, tag="4")]
    pub display_name: ::prost::alloc::string::String,
    /// A key-value map to store arbitrary metadata for the `Policy`. Keys
    /// can be up to 63 characters. Values can be up to 255 characters.
    #[prost(btree_map="string, string", tag="5")]
    pub annotations: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// An opaque tag that identifies the current version of the `Policy`. IAM uses
    /// this value to help manage concurrent updates, so they do not cause one
    /// update to be overwritten by another.
    ///
    /// If this field is present in a \[CreatePolicy][\] request, the value is
    /// ignored.
    #[prost(string, tag="6")]
    pub etag: ::prost::alloc::string::String,
    /// Output only. The time when the `Policy` was created.
    #[prost(message, optional, tag="7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the `Policy` was last updated.
    #[prost(message, optional, tag="8")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the `Policy` was deleted. Empty if the policy is
    /// not deleted.
    #[prost(message, optional, tag="9")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// A list of rules that specify the behavior of the `Policy`. All of the rules
    /// should be of the `kind` specified in the `Policy`.
    #[prost(message, repeated, tag="10")]
    pub rules: ::prost::alloc::vec::Vec<PolicyRule>,
    /// Immutable. Specifies that this policy is managed by an authority and can
    /// only be modified by that authority. Usage is restricted.
    #[prost(string, tag="11")]
    pub managing_authority: ::prost::alloc::string::String,
}
/// A single rule in a `Policy`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyRule {
    /// A user-specified description of the rule. This value can be up to 256
    /// characters.
    #[prost(string, tag="1")]
    pub description: ::prost::alloc::string::String,
    #[prost(oneof="policy_rule::Kind", tags="2")]
    pub kind: ::core::option::Option<policy_rule::Kind>,
}
/// Nested message and enum types in `PolicyRule`.
pub mod policy_rule {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        /// A rule for a deny policy.
        #[prost(message, tag="2")]
        DenyRule(super::DenyRule),
    }
}
/// Request message for `ListPolicies`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPoliciesRequest {
    /// Required. The resource that the policy is attached to, along with the kind
    /// of policy to list. Format: `policies/{attachment_point}/denypolicies`
    ///
    ///
    /// The attachment point is identified by its URL-encoded full resource name,
    /// which means that the forward-slash character, `/`, must be written as
    /// `%2F`. For example,
    /// `policies/cloudresourcemanager.googleapis.com%2Fprojects%2Fmy-project/denypolicies`.
    ///
    /// For organizations and folders, use the numeric ID in the full resource
    /// name. For projects, you can use the alphanumeric or the numeric ID.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of policies to return. IAM ignores this value and uses
    /// the value 1000.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token received in a
    /// \[ListPoliciesResponse][google.iam.v2.ListPoliciesResponse\]. Provide this
    /// token to retrieve the next page.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for `ListPolicies`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPoliciesResponse {
    /// Metadata for the policies that are attached to the resource.
    #[prost(message, repeated, tag="1")]
    pub policies: ::prost::alloc::vec::Vec<Policy>,
    /// A page token that you can use in a
    /// \[ListPoliciesRequest][google.iam.v2.ListPoliciesRequest\] to retrieve the
    /// next page. If this field is omitted, there are no additional pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for `GetPolicy`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPolicyRequest {
    /// Required. The resource name of the policy to retrieve. Format:
    /// `policies/{attachment_point}/denypolicies/{policy_id}`
    ///
    ///
    /// Use the URL-encoded full resource name, which means that the forward-slash
    /// character, `/`, must be written as `%2F`. For example,
    /// `policies/cloudresourcemanager.googleapis.com%2Fprojects%2Fmy-project/denypolicies/my-policy`.
    ///
    /// For organizations and folders, use the numeric ID in the full resource
    /// name. For projects, you can use the alphanumeric or the numeric ID.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for `CreatePolicy`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePolicyRequest {
    /// Required. The resource that the policy is attached to, along with the kind
    /// of policy to create. Format: `policies/{attachment_point}/denypolicies`
    ///
    ///
    /// The attachment point is identified by its URL-encoded full resource name,
    /// which means that the forward-slash character, `/`, must be written as
    /// `%2F`. For example,
    /// `policies/cloudresourcemanager.googleapis.com%2Fprojects%2Fmy-project/denypolicies`.
    ///
    /// For organizations and folders, use the numeric ID in the full resource
    /// name. For projects, you can use the alphanumeric or the numeric ID.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The policy to create.
    #[prost(message, optional, tag="2")]
    pub policy: ::core::option::Option<Policy>,
    /// The ID to use for this policy, which will become the final component of
    /// the policy's resource name. The ID must contain 3 to 63 characters. It can
    /// contain lowercase letters and numbers, as well as dashes (`-`) and periods
    /// (`.`). The first character must be a lowercase letter.
    #[prost(string, tag="3")]
    pub policy_id: ::prost::alloc::string::String,
}
/// Request message for `UpdatePolicy`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePolicyRequest {
    /// Required. The policy to update.
    ///
    /// To prevent conflicting updates, the `etag` value must match the value that
    /// is stored in IAM. If the `etag` values do not match, the request fails with
    /// a `409` error code and `ABORTED` status.
    #[prost(message, optional, tag="1")]
    pub policy: ::core::option::Option<Policy>,
}
/// Request message for `DeletePolicy`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePolicyRequest {
    /// Required. The resource name of the policy to delete. Format:
    /// `policies/{attachment_point}/denypolicies/{policy_id}`
    ///
    ///
    /// Use the URL-encoded full resource name, which means that the forward-slash
    /// character, `/`, must be written as `%2F`. For example,
    /// `policies/cloudresourcemanager.googleapis.com%2Fprojects%2Fmy-project/denypolicies/my-policy`.
    ///
    /// For organizations and folders, use the numeric ID in the full resource
    /// name. For projects, you can use the alphanumeric or the numeric ID.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The expected `etag` of the policy to delete. If the value does
    /// not match the value that is stored in IAM, the request fails with a `409`
    /// error code and `ABORTED` status.
    ///
    /// If you omit this field, the policy is deleted regardless of its current
    /// `etag`.
    #[prost(string, tag="2")]
    pub etag: ::prost::alloc::string::String,
}
/// `ListApplicablePoliciesRequest` represents the Request message for the
/// `ListApplicablePolicies` method. It provides the input for a filterable query
/// of Policies that apply to a certain GCP Resource, specified by the field
/// `attachment_point`, found on this message.
/// Example:
/// ```
/// {
///    attachment_point:
///    'cloudresourcemanager.googleapis.com%2Forganizations%2F212345678901'
///    filter: 'kind:denyPolicies'
/// }
/// ```
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListApplicablePoliciesRequest {
    /// Required. The Cloud resource at which the applicable policies are to be
    /// retrieved. Format: `{attachment-point}` Use the URL-encoded full resource
    /// name, which means that the forward-slash character, `/`, must be written as
    /// `%2F`. For example,
    /// `cloudresourcemanager.googleapis.com%2Fprojects%2Fmy-project`.
    #[prost(string, tag="1")]
    pub attachment_point: ::prost::alloc::string::String,
    /// Filtering currently only supports the kind of policies to return, and
    /// must be in the format “kind:\[policyKind1\] OR kind:\[policyKind2\]”.  New
    /// policy kinds may be added in the future without notice.
    ///
    /// Example value: “kind:denyPolicies”
    #[prost(string, tag="2")]
    pub filter: ::prost::alloc::string::String,
    /// If present, then retrieve the batch of results following the results from
    /// the preceding call to this method.  `page_token` must be the value of
    /// `next_page_token`
    /// \[ListApplicablePoliciesResponse.next_page_token][google.iam.v2.ListApplicablePoliciesResponse.next_page_token\]
    /// from the previous response.  The values of other method parameters should
    /// be identical to those in the previous call.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// Limit on the number of policies to include in the response.
    /// Further policies can subsequently be obtained by including the
    /// \[ListApplicablePoliciesResponse.next_page_token][google.iam.admin.v1.ListApplicablePoliciesResponse.next_page_token\]
    /// in a subsequent request.
    /// The minimum is 25, and the maximum is 100.
    #[prost(int32, tag="4")]
    pub page_size: i32,
}
/// Response message for \[ListApplicablePolicies][\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListApplicablePoliciesResponse {
    /// Ordered list starting from the resource on which this API was called
    /// then proceeding up the hierarchy. Policies for the same attachment point
    /// will be grouped, but no further ordering is guaranteed.
    #[prost(message, repeated, tag="1")]
    pub policies: ::prost::alloc::vec::Vec<Policy>,
    /// A list of resources that the caller does not have permission to retrieve.
    /// List or Get can be used to get detailed error messages.
    /// Get: `policies/{attachment-point}/denypolicies/{policy-id}`
    /// List: `policies/{attachment-point}/denypolicies`
    #[prost(string, repeated, tag="2")]
    pub inaccessible: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A page token that can be used in a
    /// \[ListApplicablePoliciesRequest][google.iam.v2.ListApplicablePoliciesRequest\]
    /// to retrieve the next page. If this field is blank, there are no additional
    /// pages.
    #[prost(string, tag="3")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Metadata for long-running `Policy` operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyOperationMetadata {
    /// Timestamp when the `google.longrunning.Operation` was created.
    #[prost(message, optional, tag="1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Generated client implementations.
pub mod policies_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// An interface for managing Identity and Access Management (IAM) policies.
    #[derive(Debug, Clone)]
    pub struct PoliciesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> PoliciesClient<T>
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
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> PoliciesClient<InterceptedService<T, F>>
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
            PoliciesClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// Retrieves the policies of the specified kind that are attached to a
        /// resource.
        ///
        /// The response lists only policy metadata. In particular, policy rules are
        /// omitted.
        pub async fn list_policies(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPoliciesRequest>,
        ) -> Result<tonic::Response<super::ListPoliciesResponse>, tonic::Status> {
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
                "/google.iam.v2.Policies/ListPolicies",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a policy.
        pub async fn get_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPolicyRequest>,
        ) -> Result<tonic::Response<super::Policy>, tonic::Status> {
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
                "/google.iam.v2.Policies/GetPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a policy.
        pub async fn create_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePolicyRequest>,
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
                "/google.iam.v2.Policies/CreatePolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the specified policy.
        ///
        /// You can update only the rules and the display name for the policy.
        ///
        /// To update a policy, you should use a read-modify-write loop:
        ///
        /// 1. Use [GetPolicy][google.iam.v2.Policies.GetPolicy] to read the current
        /// version of the policy.
        /// 2. Modify the policy as needed.
        /// 3. Use `UpdatePolicy` to write the updated policy.
        ///
        /// This pattern helps prevent conflicts between concurrent updates.
        pub async fn update_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePolicyRequest>,
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
                "/google.iam.v2.Policies/UpdatePolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a policy. This action is permanent.
        pub async fn delete_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePolicyRequest>,
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
                "/google.iam.v2.Policies/DeletePolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves all the policies that are attached to the specified resource,
        /// or anywhere in the ancestry of the resource. For example, for a project
        /// this endpoint would return all the `denyPolicy` kind policies attached to
        /// the project, its parent folder (if any), and its parent organization (if
        /// any).
        /// The endpoint requires the same permissions that it would take to call
        /// `ListPolicies` or `GetPolicy`.
        ///
        /// The main reason to use this endpoint is as a policy admin to debug access
        /// issues for a resource.
        pub async fn list_applicable_policies(
            &mut self,
            request: impl tonic::IntoRequest<super::ListApplicablePoliciesRequest>,
        ) -> Result<
            tonic::Response<super::ListApplicablePoliciesResponse>,
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
                "/google.iam.v2.Policies/ListApplicablePolicies",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
