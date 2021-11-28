/// Represents a Pubsub snapshot.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubsubSnapshotMetadata {
    /// The name of the Pubsub topic.
    #[prost(string, tag = "1")]
    pub topic_name: ::prost::alloc::string::String,
    /// The name of the Pubsub snapshot.
    #[prost(string, tag = "2")]
    pub snapshot_name: ::prost::alloc::string::String,
    /// The expire time of the Pubsub snapshot.
    #[prost(message, optional, tag = "3")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Represents a snapshot of a job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Snapshot {
    /// The unique ID of this snapshot.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The project this snapshot belongs to.
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
    /// The job this snapshot was created from.
    #[prost(string, tag = "3")]
    pub source_job_id: ::prost::alloc::string::String,
    /// The time this snapshot was created.
    #[prost(message, optional, tag = "4")]
    pub creation_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time after which this snapshot will be automatically deleted.
    #[prost(message, optional, tag = "5")]
    pub ttl: ::core::option::Option<::prost_types::Duration>,
    /// State of the snapshot.
    #[prost(enumeration = "SnapshotState", tag = "6")]
    pub state: i32,
    /// PubSub snapshot metadata.
    #[prost(message, repeated, tag = "7")]
    pub pubsub_metadata: ::prost::alloc::vec::Vec<PubsubSnapshotMetadata>,
    /// User specified description of the snapshot. Maybe empty.
    #[prost(string, tag = "8")]
    pub description: ::prost::alloc::string::String,
    /// The disk byte size of the snapshot. Only available for snapshots in READY
    /// state.
    #[prost(int64, tag = "9")]
    pub disk_size_bytes: i64,
    /// Cloud region where this snapshot lives in, e.g., "us-central1".
    #[prost(string, tag = "10")]
    pub region: ::prost::alloc::string::String,
}
/// Request to get information about a snapshot
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSnapshotRequest {
    /// The ID of the Cloud Platform project that the snapshot belongs to.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// The ID of the snapshot.
    #[prost(string, tag = "2")]
    pub snapshot_id: ::prost::alloc::string::String,
    /// The location that contains this snapshot.
    #[prost(string, tag = "3")]
    pub location: ::prost::alloc::string::String,
}
/// Request to delete a snapshot.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSnapshotRequest {
    /// The ID of the Cloud Platform project that the snapshot belongs to.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// The ID of the snapshot.
    #[prost(string, tag = "2")]
    pub snapshot_id: ::prost::alloc::string::String,
    /// The location that contains this snapshot.
    #[prost(string, tag = "3")]
    pub location: ::prost::alloc::string::String,
}
/// Response from deleting a snapshot.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSnapshotResponse {}
/// Request to list snapshots.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSnapshotsRequest {
    /// The project ID to list snapshots for.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// If specified, list snapshots created from this job.
    #[prost(string, tag = "3")]
    pub job_id: ::prost::alloc::string::String,
    /// The location to list snapshots in.
    #[prost(string, tag = "2")]
    pub location: ::prost::alloc::string::String,
}
/// List of snapshots.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSnapshotsResponse {
    /// Returned snapshots.
    #[prost(message, repeated, tag = "1")]
    pub snapshots: ::prost::alloc::vec::Vec<Snapshot>,
}
/// Snapshot state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SnapshotState {
    /// Unknown state.
    UnknownSnapshotState = 0,
    /// Snapshot intent to create has been persisted, snapshotting of state has not
    /// yet started.
    Pending = 1,
    /// Snapshotting is being performed.
    Running = 2,
    /// Snapshot has been created and is ready to be used.
    Ready = 3,
    /// Snapshot failed to be created.
    Failed = 4,
    /// Snapshot has been deleted.
    Deleted = 5,
}
#[doc = r" Generated client implementations."]
pub mod snapshots_v1_beta3_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Provides methods to manage snapshots of Google Cloud Dataflow jobs."]
    #[derive(Debug, Clone)]
    pub struct SnapshotsV1Beta3Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SnapshotsV1Beta3Client<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SnapshotsV1Beta3Client<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            SnapshotsV1Beta3Client::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Gets information about a snapshot."]
        pub async fn get_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSnapshotRequest>,
        ) -> Result<tonic::Response<super::Snapshot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.dataflow.v1beta3.SnapshotsV1Beta3/GetSnapshot",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a snapshot."]
        pub async fn delete_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSnapshotRequest>,
        ) -> Result<tonic::Response<super::DeleteSnapshotResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.dataflow.v1beta3.SnapshotsV1Beta3/DeleteSnapshot",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists snapshots."]
        pub async fn list_snapshots(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSnapshotsRequest>,
        ) -> Result<tonic::Response<super::ListSnapshotsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.dataflow.v1beta3.SnapshotsV1Beta3/ListSnapshots",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Describes the environment in which a Dataflow Job runs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Environment {
    /// The prefix of the resources the system should use for temporary
    /// storage.  The system will append the suffix "/temp-{JOBNAME} to
    /// this resource prefix, where {JOBNAME} is the value of the
    /// job_name field.  The resulting bucket and object prefix is used
    /// as the prefix of the resources used to store temporary data
    /// needed during the job execution.  NOTE: This will override the
    /// value in taskrunner_settings.
    /// The supported resource type is:
    ///
    /// Google Cloud Storage:
    ///
    ///   storage.googleapis.com/{bucket}/{object}
    ///   bucket.storage.googleapis.com/{object}
    #[prost(string, tag = "1")]
    pub temp_storage_prefix: ::prost::alloc::string::String,
    /// The type of cluster manager API to use.  If unknown or
    /// unspecified, the service will attempt to choose a reasonable
    /// default.  This should be in the form of the API service name,
    /// e.g. "compute.googleapis.com".
    #[prost(string, tag = "2")]
    pub cluster_manager_api_service: ::prost::alloc::string::String,
    /// The list of experiments to enable. This field should be used for SDK
    /// related experiments and not for service related experiments. The proper
    /// field for service related experiments is service_options.
    #[prost(string, repeated, tag = "3")]
    pub experiments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The list of service options to enable. This field should be used for
    /// service related experiments only. These experiments, when graduating to GA,
    /// should be replaced by dedicated fields or become default (i.e. always on).
    #[prost(string, repeated, tag = "16")]
    pub service_options: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// If set, contains the Cloud KMS key identifier used to encrypt data
    /// at rest, AKA a Customer Managed Encryption Key (CMEK).
    ///
    /// Format:
    ///   projects/PROJECT_ID/locations/LOCATION/keyRings/KEY_RING/cryptoKeys/KEY
    #[prost(string, tag = "12")]
    pub service_kms_key_name: ::prost::alloc::string::String,
    /// The worker pools. At least one "harness" worker pool must be
    /// specified in order for the job to have workers.
    #[prost(message, repeated, tag = "4")]
    pub worker_pools: ::prost::alloc::vec::Vec<WorkerPool>,
    /// A description of the process that generated the request.
    #[prost(message, optional, tag = "5")]
    pub user_agent: ::core::option::Option<::prost_types::Struct>,
    /// A structure describing which components and their versions of the service
    /// are required in order to run the job.
    #[prost(message, optional, tag = "6")]
    pub version: ::core::option::Option<::prost_types::Struct>,
    /// The dataset for the current project where various workflow
    /// related tables are stored.
    ///
    /// The supported resource type is:
    ///
    /// Google BigQuery:
    ///   bigquery.googleapis.com/{dataset}
    #[prost(string, tag = "7")]
    pub dataset: ::prost::alloc::string::String,
    /// The Cloud Dataflow SDK pipeline options specified by the user. These
    /// options are passed through the service and are used to recreate the
    /// SDK pipeline options on the worker in a language agnostic and platform
    /// independent way.
    #[prost(message, optional, tag = "8")]
    pub sdk_pipeline_options: ::core::option::Option<::prost_types::Struct>,
    /// Experimental settings.
    #[prost(message, optional, tag = "9")]
    pub internal_experiments: ::core::option::Option<::prost_types::Any>,
    /// Identity to run virtual machines as. Defaults to the default account.
    #[prost(string, tag = "10")]
    pub service_account_email: ::prost::alloc::string::String,
    /// Which Flexible Resource Scheduling mode to run in.
    #[prost(enumeration = "FlexResourceSchedulingGoal", tag = "11")]
    pub flex_resource_scheduling_goal: i32,
    /// The Compute Engine region
    /// (<https://cloud.google.com/compute/docs/regions-zones/regions-zones>) in
    /// which worker processing should occur, e.g. "us-west1". Mutually exclusive
    /// with worker_zone. If neither worker_region nor worker_zone is specified,
    /// default to the control plane's region.
    #[prost(string, tag = "13")]
    pub worker_region: ::prost::alloc::string::String,
    /// The Compute Engine zone
    /// (<https://cloud.google.com/compute/docs/regions-zones/regions-zones>) in
    /// which worker processing should occur, e.g. "us-west1-a". Mutually exclusive
    /// with worker_region. If neither worker_region nor worker_zone is specified,
    /// a zone in the control plane's region is chosen based on available capacity.
    #[prost(string, tag = "14")]
    pub worker_zone: ::prost::alloc::string::String,
    /// Output only. The shuffle mode used for the job.
    #[prost(enumeration = "ShuffleMode", tag = "15")]
    pub shuffle_mode: i32,
    /// Any debugging options to be supplied to the job.
    #[prost(message, optional, tag = "17")]
    pub debug_options: ::core::option::Option<DebugOptions>,
}
/// The packages that must be installed in order for a worker to run the
/// steps of the Cloud Dataflow job that will be assigned to its worker
/// pool.
///
/// This is the mechanism by which the Cloud Dataflow SDK causes code to
/// be loaded onto the workers. For example, the Cloud Dataflow Java SDK
/// might use this to install jars containing the user's code and all of the
/// various dependencies (libraries, data files, etc.) required in order
/// for that code to run.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Package {
    /// The name of the package.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The resource to read the package from. The supported resource type is:
    ///
    /// Google Cloud Storage:
    ///
    ///   storage.googleapis.com/{bucket}
    ///   bucket.storage.googleapis.com/
    #[prost(string, tag = "2")]
    pub location: ::prost::alloc::string::String,
}
/// Describes the data disk used by a workflow job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Disk {
    /// Size of disk in GB.  If zero or unspecified, the service will
    /// attempt to choose a reasonable default.
    #[prost(int32, tag = "1")]
    pub size_gb: i32,
    /// Disk storage type, as defined by Google Compute Engine.  This
    /// must be a disk type appropriate to the project and zone in which
    /// the workers will run.  If unknown or unspecified, the service
    /// will attempt to choose a reasonable default.
    ///
    /// For example, the standard persistent disk type is a resource name
    /// typically ending in "pd-standard".  If SSD persistent disks are
    /// available, the resource name typically ends with "pd-ssd".  The
    /// actual valid values are defined the Google Compute Engine API,
    /// not by the Cloud Dataflow API; consult the Google Compute Engine
    /// documentation for more information about determining the set of
    /// available disk types for a particular project and zone.
    ///
    /// Google Compute Engine Disk types are local to a particular
    /// project in a particular zone, and so the resource name will
    /// typically look something like this:
    ///
    /// compute.googleapis.com/projects/project-id/zones/zone/diskTypes/pd-standard
    #[prost(string, tag = "2")]
    pub disk_type: ::prost::alloc::string::String,
    /// Directory in a VM where disk is mounted.
    #[prost(string, tag = "3")]
    pub mount_point: ::prost::alloc::string::String,
}
/// Provides data to pass through to the worker harness.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerSettings {
    /// The base URL for accessing Google Cloud APIs.
    ///
    /// When workers access Google Cloud APIs, they logically do so via
    /// relative URLs.  If this field is specified, it supplies the base
    /// URL to use for resolving these relative URLs.  The normative
    /// algorithm used is defined by RFC 1808, "Relative Uniform Resource
    /// Locators".
    ///
    /// If not specified, the default value is "<http://www.googleapis.com/">
    #[prost(string, tag = "1")]
    pub base_url: ::prost::alloc::string::String,
    /// Whether to send work progress updates to the service.
    #[prost(bool, tag = "2")]
    pub reporting_enabled: bool,
    /// The Cloud Dataflow service path relative to the root URL, for example,
    /// "dataflow/v1b3/projects".
    #[prost(string, tag = "3")]
    pub service_path: ::prost::alloc::string::String,
    /// The Shuffle service path relative to the root URL, for example,
    /// "shuffle/v1beta1".
    #[prost(string, tag = "4")]
    pub shuffle_service_path: ::prost::alloc::string::String,
    /// The ID of the worker running this pipeline.
    #[prost(string, tag = "5")]
    pub worker_id: ::prost::alloc::string::String,
    /// The prefix of the resources the system should use for temporary
    /// storage.
    ///
    /// The supported resource type is:
    ///
    /// Google Cloud Storage:
    ///
    ///   storage.googleapis.com/{bucket}/{object}
    ///   bucket.storage.googleapis.com/{object}
    #[prost(string, tag = "6")]
    pub temp_storage_prefix: ::prost::alloc::string::String,
}
/// Taskrunner configuration settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskRunnerSettings {
    /// The UNIX user ID on the worker VM to use for tasks launched by
    /// taskrunner; e.g. "root".
    #[prost(string, tag = "1")]
    pub task_user: ::prost::alloc::string::String,
    /// The UNIX group ID on the worker VM to use for tasks launched by
    /// taskrunner; e.g. "wheel".
    #[prost(string, tag = "2")]
    pub task_group: ::prost::alloc::string::String,
    /// The OAuth2 scopes to be requested by the taskrunner in order to
    /// access the Cloud Dataflow API.
    #[prost(string, repeated, tag = "3")]
    pub oauth_scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The base URL for the taskrunner to use when accessing Google Cloud APIs.
    ///
    /// When workers access Google Cloud APIs, they logically do so via
    /// relative URLs.  If this field is specified, it supplies the base
    /// URL to use for resolving these relative URLs.  The normative
    /// algorithm used is defined by RFC 1808, "Relative Uniform Resource
    /// Locators".
    ///
    /// If not specified, the default value is "<http://www.googleapis.com/">
    #[prost(string, tag = "4")]
    pub base_url: ::prost::alloc::string::String,
    /// The API version of endpoint, e.g. "v1b3"
    #[prost(string, tag = "5")]
    pub dataflow_api_version: ::prost::alloc::string::String,
    /// The settings to pass to the parallel worker harness.
    #[prost(message, optional, tag = "6")]
    pub parallel_worker_settings: ::core::option::Option<WorkerSettings>,
    /// The location on the worker for task-specific subdirectories.
    #[prost(string, tag = "7")]
    pub base_task_dir: ::prost::alloc::string::String,
    /// Whether to continue taskrunner if an exception is hit.
    #[prost(bool, tag = "8")]
    pub continue_on_exception: bool,
    /// Whether to send taskrunner log info to Google Compute Engine VM serial
    /// console.
    #[prost(bool, tag = "9")]
    pub log_to_serialconsole: bool,
    /// Whether to also send taskrunner log info to stderr.
    #[prost(bool, tag = "10")]
    pub alsologtostderr: bool,
    /// Indicates where to put logs.  If this is not specified, the logs
    /// will not be uploaded.
    ///
    /// The supported resource type is:
    ///
    /// Google Cloud Storage:
    ///   storage.googleapis.com/{bucket}/{object}
    ///   bucket.storage.googleapis.com/{object}
    #[prost(string, tag = "11")]
    pub log_upload_location: ::prost::alloc::string::String,
    /// The directory on the VM to store logs.
    #[prost(string, tag = "12")]
    pub log_dir: ::prost::alloc::string::String,
    /// The prefix of the resources the taskrunner should use for
    /// temporary storage.
    ///
    /// The supported resource type is:
    ///
    /// Google Cloud Storage:
    ///   storage.googleapis.com/{bucket}/{object}
    ///   bucket.storage.googleapis.com/{object}
    #[prost(string, tag = "13")]
    pub temp_storage_prefix: ::prost::alloc::string::String,
    /// The command to launch the worker harness.
    #[prost(string, tag = "14")]
    pub harness_command: ::prost::alloc::string::String,
    /// The file to store the workflow in.
    #[prost(string, tag = "15")]
    pub workflow_file_name: ::prost::alloc::string::String,
    /// The file to store preprocessing commands in.
    #[prost(string, tag = "16")]
    pub commandlines_file_name: ::prost::alloc::string::String,
    /// The ID string of the VM.
    #[prost(string, tag = "17")]
    pub vm_id: ::prost::alloc::string::String,
    /// The suggested backend language.
    #[prost(string, tag = "18")]
    pub language_hint: ::prost::alloc::string::String,
    /// The streaming worker main class name.
    #[prost(string, tag = "19")]
    pub streaming_worker_main_class: ::prost::alloc::string::String,
}
/// Settings for WorkerPool autoscaling.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoscalingSettings {
    /// The algorithm to use for autoscaling.
    #[prost(enumeration = "AutoscalingAlgorithm", tag = "1")]
    pub algorithm: i32,
    /// The maximum number of workers to cap scaling at.
    #[prost(int32, tag = "2")]
    pub max_num_workers: i32,
}
/// Defines a SDK harness container for executing Dataflow pipelines.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SdkHarnessContainerImage {
    /// A docker container image that resides in Google Container Registry.
    #[prost(string, tag = "1")]
    pub container_image: ::prost::alloc::string::String,
    /// If true, recommends the Dataflow service to use only one core per SDK
    /// container instance with this image. If false (or unset) recommends using
    /// more than one core per SDK container instance with this image for
    /// efficiency. Note that Dataflow service may choose to override this property
    /// if needed.
    #[prost(bool, tag = "2")]
    pub use_single_core_per_container: bool,
    /// Environment ID for the Beam runner API proto Environment that corresponds
    /// to the current SDK Harness.
    #[prost(string, tag = "3")]
    pub environment_id: ::prost::alloc::string::String,
}
/// Describes one particular pool of Cloud Dataflow workers to be
/// instantiated by the Cloud Dataflow service in order to perform the
/// computations required by a job.  Note that a workflow job may use
/// multiple pools, in order to match the various computational
/// requirements of the various stages of the job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerPool {
    /// The kind of the worker pool; currently only `harness` and `shuffle`
    /// are supported.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// Number of Google Compute Engine workers in this pool needed to
    /// execute the job.  If zero or unspecified, the service will
    /// attempt to choose a reasonable default.
    #[prost(int32, tag = "2")]
    pub num_workers: i32,
    /// Packages to be installed on workers.
    #[prost(message, repeated, tag = "3")]
    pub packages: ::prost::alloc::vec::Vec<Package>,
    /// The default package set to install.  This allows the service to
    /// select a default set of packages which are useful to worker
    /// harnesses written in a particular language.
    #[prost(enumeration = "DefaultPackageSet", tag = "4")]
    pub default_package_set: i32,
    /// Machine type (e.g. "n1-standard-1").  If empty or unspecified, the
    /// service will attempt to choose a reasonable default.
    #[prost(string, tag = "5")]
    pub machine_type: ::prost::alloc::string::String,
    /// Sets the policy for determining when to turndown worker pool.
    /// Allowed values are: `TEARDOWN_ALWAYS`, `TEARDOWN_ON_SUCCESS`, and
    /// `TEARDOWN_NEVER`.
    /// `TEARDOWN_ALWAYS` means workers are always torn down regardless of whether
    /// the job succeeds. `TEARDOWN_ON_SUCCESS` means workers are torn down
    /// if the job succeeds. `TEARDOWN_NEVER` means the workers are never torn
    /// down.
    ///
    /// If the workers are not torn down by the service, they will
    /// continue to run and use Google Compute Engine VM resources in the
    /// user's project until they are explicitly terminated by the user.
    /// Because of this, Google recommends using the `TEARDOWN_ALWAYS`
    /// policy except for small, manually supervised test jobs.
    ///
    /// If unknown or unspecified, the service will attempt to choose a reasonable
    /// default.
    #[prost(enumeration = "TeardownPolicy", tag = "6")]
    pub teardown_policy: i32,
    /// Size of root disk for VMs, in GB.  If zero or unspecified, the service will
    /// attempt to choose a reasonable default.
    #[prost(int32, tag = "7")]
    pub disk_size_gb: i32,
    /// Type of root disk for VMs.  If empty or unspecified, the service will
    /// attempt to choose a reasonable default.
    #[prost(string, tag = "16")]
    pub disk_type: ::prost::alloc::string::String,
    /// Fully qualified source image for disks.
    #[prost(string, tag = "8")]
    pub disk_source_image: ::prost::alloc::string::String,
    /// Zone to run the worker pools in.  If empty or unspecified, the service
    /// will attempt to choose a reasonable default.
    #[prost(string, tag = "9")]
    pub zone: ::prost::alloc::string::String,
    /// Settings passed through to Google Compute Engine workers when
    /// using the standard Dataflow task runner.  Users should ignore
    /// this field.
    #[prost(message, optional, tag = "10")]
    pub taskrunner_settings: ::core::option::Option<TaskRunnerSettings>,
    /// The action to take on host maintenance, as defined by the Google
    /// Compute Engine API.
    #[prost(string, tag = "11")]
    pub on_host_maintenance: ::prost::alloc::string::String,
    /// Data disks that are used by a VM in this workflow.
    #[prost(message, repeated, tag = "12")]
    pub data_disks: ::prost::alloc::vec::Vec<Disk>,
    /// Metadata to set on the Google Compute Engine VMs.
    #[prost(btree_map = "string, string", tag = "13")]
    pub metadata: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Settings for autoscaling of this WorkerPool.
    #[prost(message, optional, tag = "14")]
    pub autoscaling_settings: ::core::option::Option<AutoscalingSettings>,
    /// Extra arguments for this worker pool.
    #[prost(message, optional, tag = "15")]
    pub pool_args: ::core::option::Option<::prost_types::Any>,
    /// Network to which VMs will be assigned.  If empty or unspecified,
    /// the service will use the network "default".
    #[prost(string, tag = "17")]
    pub network: ::prost::alloc::string::String,
    /// Subnetwork to which VMs will be assigned, if desired.  Expected to be of
    /// the form "regions/REGION/subnetworks/SUBNETWORK".
    #[prost(string, tag = "19")]
    pub subnetwork: ::prost::alloc::string::String,
    /// Required. Docker container image that executes the Cloud Dataflow worker
    /// harness, residing in Google Container Registry.
    ///
    /// Deprecated for the Fn API path. Use sdk_harness_container_images instead.
    #[prost(string, tag = "18")]
    pub worker_harness_container_image: ::prost::alloc::string::String,
    /// The number of threads per worker harness. If empty or unspecified, the
    /// service will choose a number of threads (according to the number of cores
    /// on the selected machine type for batch, or 1 by convention for streaming).
    #[prost(int32, tag = "20")]
    pub num_threads_per_worker: i32,
    /// Configuration for VM IPs.
    #[prost(enumeration = "WorkerIpAddressConfiguration", tag = "21")]
    pub ip_configuration: i32,
    /// Set of SDK harness containers needed to execute this pipeline. This will
    /// only be set in the Fn API path. For non-cross-language pipelines this
    /// should have only one entry. Cross-language pipelines will have two or more
    /// entries.
    #[prost(message, repeated, tag = "22")]
    pub sdk_harness_container_images: ::prost::alloc::vec::Vec<SdkHarnessContainerImage>,
}
/// Describes any options that have an effect on the debugging of pipelines.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugOptions {
    /// When true, enables the logging of the literal hot key to the user's Cloud
    /// Logging.
    #[prost(bool, tag = "1")]
    pub enable_hot_key_logging: bool,
}
/// Specifies the processing model used by a
/// \[google.dataflow.v1beta3.Job\], which determines the way the Job is
/// managed by the Cloud Dataflow service (how workers are scheduled, how
/// inputs are sharded, etc).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum JobType {
    /// The type of the job is unspecified, or unknown.
    Unknown = 0,
    /// A batch job with a well-defined end point: data is read, data is
    /// processed, data is written, and the job is done.
    Batch = 1,
    /// A continuously streaming job with no end: data is read,
    /// processed, and written continuously.
    Streaming = 2,
}
/// Specifies the resource to optimize for in Flexible Resource Scheduling.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FlexResourceSchedulingGoal {
    /// Run in the default mode.
    FlexrsUnspecified = 0,
    /// Optimize for lower execution time.
    FlexrsSpeedOptimized = 1,
    /// Optimize for lower cost.
    FlexrsCostOptimized = 2,
}
/// Specifies what happens to a resource when a Cloud Dataflow
/// \[google.dataflow.v1beta3.Job][google.dataflow.v1beta3.Job\] has completed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TeardownPolicy {
    /// The teardown policy isn't specified, or is unknown.
    Unknown = 0,
    /// Always teardown the resource.
    TeardownAlways = 1,
    /// Teardown the resource on success. This is useful for debugging
    /// failures.
    TeardownOnSuccess = 2,
    /// Never teardown the resource. This is useful for debugging and
    /// development.
    TeardownNever = 3,
}
/// The default set of packages to be staged on a pool of workers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DefaultPackageSet {
    /// The default set of packages to stage is unknown, or unspecified.
    Unknown = 0,
    /// Indicates that no packages should be staged at the worker unless
    /// explicitly specified by the job.
    None = 1,
    /// Stage packages typically useful to workers written in Java.
    Java = 2,
    /// Stage packages typically useful to workers written in Python.
    Python = 3,
}
/// Specifies the algorithm used to determine the number of worker
/// processes to run at any given point in time, based on the amount of
/// data left to process, the number of workers, and how quickly
/// existing workers are processing data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AutoscalingAlgorithm {
    /// The algorithm is unknown, or unspecified.
    Unknown = 0,
    /// Disable autoscaling.
    None = 1,
    /// Increase worker count over time to reduce job execution time.
    Basic = 2,
}
/// Specifies how IP addresses should be allocated to the worker machines.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WorkerIpAddressConfiguration {
    /// The configuration is unknown, or unspecified.
    WorkerIpUnspecified = 0,
    /// Workers should have public IP addresses.
    WorkerIpPublic = 1,
    /// Workers should have private IP addresses.
    WorkerIpPrivate = 2,
}
/// Specifies the shuffle mode used by a
/// \[google.dataflow.v1beta3.Job\], which determines the approach data is shuffled
/// during processing. More details in:
/// <https://cloud.google.com/dataflow/docs/guides/deploying-a-pipeline#dataflow-shuffle>
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ShuffleMode {
    /// Shuffle mode information is not available.
    Unspecified = 0,
    /// Shuffle is done on the worker VMs.
    VmBased = 1,
    /// Shuffle is done on the service side.
    ServiceBased = 2,
}
/// Defines a job to be run by the Cloud Dataflow service.
/// nextID: 26
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Job {
    /// The unique ID of this job.
    ///
    /// This field is set by the Cloud Dataflow service when the Job is
    /// created, and is immutable for the life of the job.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The ID of the Cloud Platform project that the job belongs to.
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
    /// The user-specified Cloud Dataflow job name.
    ///
    /// Only one Job with a given name may exist in a project at any
    /// given time. If a caller attempts to create a Job with the same
    /// name as an already-existing Job, the attempt returns the
    /// existing Job.
    ///
    /// The name must match the regular expression
    /// `\[a-z]([-a-z0-9]{0,38}[a-z0-9\])?`
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// The type of Cloud Dataflow job.
    #[prost(enumeration = "JobType", tag = "4")]
    pub r#type: i32,
    /// The environment for the job.
    #[prost(message, optional, tag = "5")]
    pub environment: ::core::option::Option<Environment>,
    /// Exactly one of step or steps_location should be specified.
    ///
    /// The top-level steps that constitute the entire job. Only retrieved with
    /// JOB_VIEW_ALL.
    #[prost(message, repeated, tag = "6")]
    pub steps: ::prost::alloc::vec::Vec<Step>,
    /// The Cloud Storage location where the steps are stored.
    #[prost(string, tag = "24")]
    pub steps_location: ::prost::alloc::string::String,
    /// The current state of the job.
    ///
    /// Jobs are created in the `JOB_STATE_STOPPED` state unless otherwise
    /// specified.
    ///
    /// A job in the `JOB_STATE_RUNNING` state may asynchronously enter a
    /// terminal state. After a job has reached a terminal state, no
    /// further state updates may be made.
    ///
    /// This field may be mutated by the Cloud Dataflow service;
    /// callers cannot mutate it.
    #[prost(enumeration = "JobState", tag = "7")]
    pub current_state: i32,
    /// The timestamp associated with the current state.
    #[prost(message, optional, tag = "8")]
    pub current_state_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The job's requested state.
    ///
    /// `UpdateJob` may be used to switch between the `JOB_STATE_STOPPED` and
    /// `JOB_STATE_RUNNING` states, by setting requested_state.  `UpdateJob` may
    /// also be used to directly set a job's requested state to
    /// `JOB_STATE_CANCELLED` or `JOB_STATE_DONE`, irrevocably terminating the
    /// job if it has not already reached a terminal state.
    #[prost(enumeration = "JobState", tag = "9")]
    pub requested_state: i32,
    /// Deprecated.
    #[prost(message, optional, tag = "10")]
    pub execution_info: ::core::option::Option<JobExecutionInfo>,
    /// The timestamp when the job was initially created. Immutable and set by the
    /// Cloud Dataflow service.
    #[prost(message, optional, tag = "11")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// If this job is an update of an existing job, this field is the job ID
    /// of the job it replaced.
    ///
    /// When sending a `CreateJobRequest`, you can update a job by specifying it
    /// here. The job named here is stopped, and its intermediate state is
    /// transferred to this job.
    #[prost(string, tag = "12")]
    pub replace_job_id: ::prost::alloc::string::String,
    /// The map of transform name prefixes of the job to be replaced to the
    /// corresponding name prefixes of the new job.
    #[prost(btree_map = "string, string", tag = "13")]
    pub transform_name_mapping: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The client's unique identifier of the job, re-used across retried attempts.
    /// If this field is set, the service will ensure its uniqueness.
    /// The request to create a job will fail if the service has knowledge of a
    /// previously submitted job with the same client's ID and job name.
    /// The caller may use this field to ensure idempotence of job
    /// creation across retried attempts to create a job.
    /// By default, the field is empty and, in that case, the service ignores it.
    #[prost(string, tag = "14")]
    pub client_request_id: ::prost::alloc::string::String,
    /// If another job is an update of this job (and thus, this job is in
    /// `JOB_STATE_UPDATED`), this field contains the ID of that job.
    #[prost(string, tag = "15")]
    pub replaced_by_job_id: ::prost::alloc::string::String,
    /// A set of files the system should be aware of that are used
    /// for temporary storage. These temporary files will be
    /// removed on job completion.
    /// No duplicates are allowed.
    /// No file patterns are supported.
    ///
    /// The supported files are:
    ///
    /// Google Cloud Storage:
    ///
    ///    storage.googleapis.com/{bucket}/{object}
    ///    bucket.storage.googleapis.com/{object}
    #[prost(string, repeated, tag = "16")]
    pub temp_files: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// User-defined labels for this job.
    ///
    /// The labels map can contain no more than 64 entries.  Entries of the labels
    /// map are UTF8 strings that comply with the following restrictions:
    ///
    /// * Keys must conform to regexp:  \[\p{Ll}\p{Lo}][\p{Ll}\p{Lo}\p{N}_-\]{0,62}
    /// * Values must conform to regexp:  \[\p{Ll}\p{Lo}\p{N}_-\]{0,63}
    /// * Both keys and values are additionally constrained to be <= 128 bytes in
    /// size.
    #[prost(btree_map = "string, string", tag = "17")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The [regional endpoint]
    /// (<https://cloud.google.com/dataflow/docs/concepts/regional-endpoints>) that
    /// contains this job.
    #[prost(string, tag = "18")]
    pub location: ::prost::alloc::string::String,
    /// Preliminary field: The format of this data may change at any time.
    /// A description of the user pipeline and stages through which it is executed.
    /// Created by Cloud Dataflow service.  Only retrieved with
    /// JOB_VIEW_DESCRIPTION or JOB_VIEW_ALL.
    #[prost(message, optional, tag = "19")]
    pub pipeline_description: ::core::option::Option<PipelineDescription>,
    /// This field may be mutated by the Cloud Dataflow service;
    /// callers cannot mutate it.
    #[prost(message, repeated, tag = "20")]
    pub stage_states: ::prost::alloc::vec::Vec<ExecutionStageState>,
    /// This field is populated by the Dataflow service to support filtering jobs
    /// by the metadata values provided here. Populated for ListJobs and all GetJob
    /// views SUMMARY and higher.
    #[prost(message, optional, tag = "21")]
    pub job_metadata: ::core::option::Option<JobMetadata>,
    /// The timestamp when the job was started (transitioned to JOB_STATE_PENDING).
    /// Flexible resource scheduling jobs are started with some delay after job
    /// creation, so start_time is unset before start and is updated when the
    /// job is started by the Cloud Dataflow service. For other jobs, start_time
    /// always equals to create_time and is immutable and set by the Cloud Dataflow
    /// service.
    #[prost(message, optional, tag = "22")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// If this is specified, the job's initial state is populated from the given
    /// snapshot.
    #[prost(string, tag = "23")]
    pub created_from_snapshot_id: ::prost::alloc::string::String,
    /// Reserved for future use. This field is set only in responses from the
    /// server; it is ignored if it is set in any requests.
    #[prost(bool, tag = "25")]
    pub satisfies_pzs: bool,
}
/// Metadata for a Datastore connector used by the job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatastoreIoDetails {
    /// Namespace used in the connection.
    #[prost(string, tag = "1")]
    pub namespace: ::prost::alloc::string::String,
    /// ProjectId accessed in the connection.
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
}
/// Metadata for a Pub/Sub connector used by the job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubSubIoDetails {
    /// Topic accessed in the connection.
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
    /// Subscription used in the connection.
    #[prost(string, tag = "2")]
    pub subscription: ::prost::alloc::string::String,
}
/// Metadata for a File connector used by the job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileIoDetails {
    /// File Pattern used to access files by the connector.
    #[prost(string, tag = "1")]
    pub file_pattern: ::prost::alloc::string::String,
}
/// Metadata for a Cloud BigTable connector used by the job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigTableIoDetails {
    /// ProjectId accessed in the connection.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// InstanceId accessed in the connection.
    #[prost(string, tag = "2")]
    pub instance_id: ::prost::alloc::string::String,
    /// TableId accessed in the connection.
    #[prost(string, tag = "3")]
    pub table_id: ::prost::alloc::string::String,
}
/// Metadata for a BigQuery connector used by the job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQueryIoDetails {
    /// Table accessed in the connection.
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    /// Dataset accessed in the connection.
    #[prost(string, tag = "2")]
    pub dataset: ::prost::alloc::string::String,
    /// Project accessed in the connection.
    #[prost(string, tag = "3")]
    pub project_id: ::prost::alloc::string::String,
    /// Query used to access data in the connection.
    #[prost(string, tag = "4")]
    pub query: ::prost::alloc::string::String,
}
/// Metadata for a Spanner connector used by the job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpannerIoDetails {
    /// ProjectId accessed in the connection.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// InstanceId accessed in the connection.
    #[prost(string, tag = "2")]
    pub instance_id: ::prost::alloc::string::String,
    /// DatabaseId accessed in the connection.
    #[prost(string, tag = "3")]
    pub database_id: ::prost::alloc::string::String,
}
/// The version of the SDK used to run the job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SdkVersion {
    /// The version of the SDK used to run the job.
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    /// A readable string describing the version of the SDK.
    #[prost(string, tag = "2")]
    pub version_display_name: ::prost::alloc::string::String,
    /// The support status for this SDK version.
    #[prost(enumeration = "sdk_version::SdkSupportStatus", tag = "3")]
    pub sdk_support_status: i32,
}
/// Nested message and enum types in `SdkVersion`.
pub mod sdk_version {
    /// The support status of the SDK used to run the job.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SdkSupportStatus {
        /// Cloud Dataflow is unaware of this version.
        Unknown = 0,
        /// This is a known version of an SDK, and is supported.
        Supported = 1,
        /// A newer version of the SDK family exists, and an update is recommended.
        Stale = 2,
        /// This version of the SDK is deprecated and will eventually be
        /// unsupported.
        Deprecated = 3,
        /// Support for this SDK version has ended and it should no longer be used.
        Unsupported = 4,
    }
}
/// Metadata available primarily for filtering jobs. Will be included in the
/// ListJob response and Job SUMMARY view.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobMetadata {
    /// The SDK version used to run the job.
    #[prost(message, optional, tag = "1")]
    pub sdk_version: ::core::option::Option<SdkVersion>,
    /// Identification of a Spanner source used in the Dataflow job.
    #[prost(message, repeated, tag = "2")]
    pub spanner_details: ::prost::alloc::vec::Vec<SpannerIoDetails>,
    /// Identification of a BigQuery source used in the Dataflow job.
    #[prost(message, repeated, tag = "3")]
    pub bigquery_details: ::prost::alloc::vec::Vec<BigQueryIoDetails>,
    /// Identification of a Cloud BigTable source used in the Dataflow job.
    #[prost(message, repeated, tag = "4")]
    pub big_table_details: ::prost::alloc::vec::Vec<BigTableIoDetails>,
    /// Identification of a PubSub source used in the Dataflow job.
    #[prost(message, repeated, tag = "5")]
    pub pubsub_details: ::prost::alloc::vec::Vec<PubSubIoDetails>,
    /// Identification of a File source used in the Dataflow job.
    #[prost(message, repeated, tag = "6")]
    pub file_details: ::prost::alloc::vec::Vec<FileIoDetails>,
    /// Identification of a Datastore source used in the Dataflow job.
    #[prost(message, repeated, tag = "7")]
    pub datastore_details: ::prost::alloc::vec::Vec<DatastoreIoDetails>,
}
/// A message describing the state of a particular execution stage.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionStageState {
    /// The name of the execution stage.
    #[prost(string, tag = "1")]
    pub execution_stage_name: ::prost::alloc::string::String,
    /// Executions stage states allow the same set of values as JobState.
    #[prost(enumeration = "JobState", tag = "2")]
    pub execution_stage_state: i32,
    /// The time at which the stage transitioned to this state.
    #[prost(message, optional, tag = "3")]
    pub current_state_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// A descriptive representation of submitted pipeline as well as the executed
/// form.  This data is provided by the Dataflow service for ease of visualizing
/// the pipeline and interpreting Dataflow provided metrics.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PipelineDescription {
    /// Description of each transform in the pipeline and collections between them.
    #[prost(message, repeated, tag = "1")]
    pub original_pipeline_transform: ::prost::alloc::vec::Vec<TransformSummary>,
    /// Description of each stage of execution of the pipeline.
    #[prost(message, repeated, tag = "2")]
    pub execution_pipeline_stage: ::prost::alloc::vec::Vec<ExecutionStageSummary>,
    /// Pipeline level display data.
    #[prost(message, repeated, tag = "3")]
    pub display_data: ::prost::alloc::vec::Vec<DisplayData>,
}
/// Description of the type, names/ids, and input/outputs for a transform.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransformSummary {
    /// Type of transform.
    #[prost(enumeration = "KindType", tag = "1")]
    pub kind: i32,
    /// SDK generated id of this transform instance.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// User provided name for this transform instance.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Transform-specific display data.
    #[prost(message, repeated, tag = "4")]
    pub display_data: ::prost::alloc::vec::Vec<DisplayData>,
    /// User  names for all collection outputs to this transform.
    #[prost(string, repeated, tag = "5")]
    pub output_collection_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// User names for all collection inputs to this transform.
    #[prost(string, repeated, tag = "6")]
    pub input_collection_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Description of the composing transforms, names/ids, and input/outputs of a
/// stage of execution.  Some composing transforms and sources may have been
/// generated by the Dataflow service during execution planning.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionStageSummary {
    /// Dataflow service generated name for this stage.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Dataflow service generated id for this stage.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// Type of transform this stage is executing.
    #[prost(enumeration = "KindType", tag = "3")]
    pub kind: i32,
    /// Input sources for this stage.
    #[prost(message, repeated, tag = "4")]
    pub input_source: ::prost::alloc::vec::Vec<execution_stage_summary::StageSource>,
    /// Output sources for this stage.
    #[prost(message, repeated, tag = "5")]
    pub output_source: ::prost::alloc::vec::Vec<execution_stage_summary::StageSource>,
    /// Other stages that must complete before this stage can run.
    #[prost(string, repeated, tag = "8")]
    pub prerequisite_stage: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Transforms that comprise this execution stage.
    #[prost(message, repeated, tag = "6")]
    pub component_transform: ::prost::alloc::vec::Vec<execution_stage_summary::ComponentTransform>,
    /// Collections produced and consumed by component transforms of this stage.
    #[prost(message, repeated, tag = "7")]
    pub component_source: ::prost::alloc::vec::Vec<execution_stage_summary::ComponentSource>,
}
/// Nested message and enum types in `ExecutionStageSummary`.
pub mod execution_stage_summary {
    /// Description of an input or output of an execution stage.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StageSource {
        /// Human-readable name for this source; may be user or system generated.
        #[prost(string, tag = "1")]
        pub user_name: ::prost::alloc::string::String,
        /// Dataflow service generated name for this source.
        #[prost(string, tag = "2")]
        pub name: ::prost::alloc::string::String,
        /// User name for the original user transform or collection with which this
        /// source is most closely associated.
        #[prost(string, tag = "3")]
        pub original_transform_or_collection: ::prost::alloc::string::String,
        /// Size of the source, if measurable.
        #[prost(int64, tag = "4")]
        pub size_bytes: i64,
    }
    /// Description of a transform executed as part of an execution stage.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ComponentTransform {
        /// Human-readable name for this transform; may be user or system generated.
        #[prost(string, tag = "1")]
        pub user_name: ::prost::alloc::string::String,
        /// Dataflow service generated name for this source.
        #[prost(string, tag = "2")]
        pub name: ::prost::alloc::string::String,
        /// User name for the original user transform with which this transform is
        /// most closely associated.
        #[prost(string, tag = "3")]
        pub original_transform: ::prost::alloc::string::String,
    }
    /// Description of an interstitial value between transforms in an execution
    /// stage.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ComponentSource {
        /// Human-readable name for this transform; may be user or system generated.
        #[prost(string, tag = "1")]
        pub user_name: ::prost::alloc::string::String,
        /// Dataflow service generated name for this source.
        #[prost(string, tag = "2")]
        pub name: ::prost::alloc::string::String,
        /// User name for the original user transform or collection with which this
        /// source is most closely associated.
        #[prost(string, tag = "3")]
        pub original_transform_or_collection: ::prost::alloc::string::String,
    }
}
/// Data provided with a pipeline or transform to provide descriptive info.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisplayData {
    /// The key identifying the display data.
    /// This is intended to be used as a label for the display data
    /// when viewed in a dax monitoring system.
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// The namespace for the key. This is usually a class name or programming
    /// language namespace (i.e. python module) which defines the display data.
    /// This allows a dax monitoring system to specially handle the data
    /// and perform custom rendering.
    #[prost(string, tag = "2")]
    pub namespace: ::prost::alloc::string::String,
    /// A possible additional shorter value to display.
    /// For example a java_class_name_value of com.mypackage.MyDoFn
    /// will be stored with MyDoFn as the short_str_value and
    /// com.mypackage.MyDoFn as the java_class_name value.
    /// short_str_value can be displayed and java_class_name_value
    /// will be displayed as a tooltip.
    #[prost(string, tag = "11")]
    pub short_str_value: ::prost::alloc::string::String,
    /// An optional full URL.
    #[prost(string, tag = "12")]
    pub url: ::prost::alloc::string::String,
    /// An optional label to display in a dax UI for the element.
    #[prost(string, tag = "13")]
    pub label: ::prost::alloc::string::String,
    /// Various value types which can be used for display data.  Only one will be
    /// set.
    #[prost(oneof = "display_data::Value", tags = "4, 5, 6, 7, 8, 9, 10")]
    pub value: ::core::option::Option<display_data::Value>,
}
/// Nested message and enum types in `DisplayData`.
pub mod display_data {
    /// Various value types which can be used for display data.  Only one will be
    /// set.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// Contains value if the data is of string type.
        #[prost(string, tag = "4")]
        StrValue(::prost::alloc::string::String),
        /// Contains value if the data is of int64 type.
        #[prost(int64, tag = "5")]
        Int64Value(i64),
        /// Contains value if the data is of float type.
        #[prost(float, tag = "6")]
        FloatValue(f32),
        /// Contains value if the data is of java class type.
        #[prost(string, tag = "7")]
        JavaClassValue(::prost::alloc::string::String),
        /// Contains value if the data is of timestamp type.
        #[prost(message, tag = "8")]
        TimestampValue(::prost_types::Timestamp),
        /// Contains value if the data is of duration type.
        #[prost(message, tag = "9")]
        DurationValue(::prost_types::Duration),
        /// Contains value if the data is of a boolean type.
        #[prost(bool, tag = "10")]
        BoolValue(bool),
    }
}
/// Defines a particular step within a Cloud Dataflow job.
///
/// A job consists of multiple steps, each of which performs some
/// specific operation as part of the overall job.  Data is typically
/// passed from one step to another as part of the job.
///
/// Here's an example of a sequence of steps which together implement a
/// Map-Reduce job:
///
///   * Read a collection of data from some source, parsing the
///     collection's elements.
///
///   * Validate the elements.
///
///   * Apply a user-defined function to map each element to some value
///     and extract an element-specific key value.
///
///   * Group elements with the same key into a single element with
///     that key, transforming a multiply-keyed collection into a
///     uniquely-keyed collection.
///
///   * Write the elements out to some data sink.
///
/// Note that the Cloud Dataflow service may be used to run many different
/// types of jobs, not just Map-Reduce.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Step {
    /// The kind of step in the Cloud Dataflow job.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// The name that identifies the step. This must be unique for each
    /// step with respect to all other steps in the Cloud Dataflow job.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// Named properties associated with the step. Each kind of
    /// predefined step has its own required set of properties.
    /// Must be provided on Create.  Only retrieved with JOB_VIEW_ALL.
    #[prost(message, optional, tag = "3")]
    pub properties: ::core::option::Option<::prost_types::Struct>,
}
/// Additional information about how a Cloud Dataflow job will be executed that
/// isn't contained in the submitted job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobExecutionInfo {
    /// A mapping from each stage to the information about that stage.
    #[prost(btree_map = "string, message", tag = "1")]
    pub stages: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        JobExecutionStageInfo,
    >,
}
/// Contains information about how a particular
/// \[google.dataflow.v1beta3.Step][google.dataflow.v1beta3.Step\] will be executed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobExecutionStageInfo {
    /// The steps associated with the execution stage.
    /// Note that stages may have several steps, and that a given step
    /// might be run by more than one stage.
    #[prost(string, repeated, tag = "1")]
    pub step_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request to create a Cloud Dataflow job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateJobRequest {
    /// The ID of the Cloud Platform project that the job belongs to.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// The job to create.
    #[prost(message, optional, tag = "2")]
    pub job: ::core::option::Option<Job>,
    /// The level of information requested in response.
    #[prost(enumeration = "JobView", tag = "3")]
    pub view: i32,
    /// Deprecated. This field is now in the Job message.
    #[prost(string, tag = "4")]
    pub replace_job_id: ::prost::alloc::string::String,
    /// The [regional endpoint]
    /// (<https://cloud.google.com/dataflow/docs/concepts/regional-endpoints>) that
    /// contains this job.
    #[prost(string, tag = "5")]
    pub location: ::prost::alloc::string::String,
}
/// Request to get the state of a Cloud Dataflow job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetJobRequest {
    /// The ID of the Cloud Platform project that the job belongs to.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// The job ID.
    #[prost(string, tag = "2")]
    pub job_id: ::prost::alloc::string::String,
    /// The level of information requested in response.
    #[prost(enumeration = "JobView", tag = "3")]
    pub view: i32,
    /// The [regional endpoint]
    /// (<https://cloud.google.com/dataflow/docs/concepts/regional-endpoints>) that
    /// contains this job.
    #[prost(string, tag = "4")]
    pub location: ::prost::alloc::string::String,
}
/// Request to update a Cloud Dataflow job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateJobRequest {
    /// The ID of the Cloud Platform project that the job belongs to.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// The job ID.
    #[prost(string, tag = "2")]
    pub job_id: ::prost::alloc::string::String,
    /// The updated job.
    /// Only the job state is updatable; other fields will be ignored.
    #[prost(message, optional, tag = "3")]
    pub job: ::core::option::Option<Job>,
    /// The [regional endpoint]
    /// (<https://cloud.google.com/dataflow/docs/concepts/regional-endpoints>) that
    /// contains this job.
    #[prost(string, tag = "4")]
    pub location: ::prost::alloc::string::String,
}
/// Request to list Cloud Dataflow jobs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListJobsRequest {
    /// The kind of filter to use.
    #[prost(enumeration = "list_jobs_request::Filter", tag = "5")]
    pub filter: i32,
    /// The project which owns the jobs.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Deprecated. ListJobs always returns summaries now.
    /// Use GetJob for other JobViews.
    #[deprecated]
    #[prost(enumeration = "JobView", tag = "2")]
    pub view: i32,
    /// If there are many jobs, limit response to at most this many.
    /// The actual number of jobs returned will be the lesser of max_responses
    /// and an unspecified server-defined limit.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Set this to the 'next_page_token' field of a previous response
    /// to request additional results in a long list.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
    /// The [regional endpoint]
    /// (<https://cloud.google.com/dataflow/docs/concepts/regional-endpoints>) that
    /// contains this job.
    #[prost(string, tag = "17")]
    pub location: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ListJobsRequest`.
pub mod list_jobs_request {
    /// This field filters out and returns jobs in the specified job state. The
    /// order of data returned is determined by the filter used, and is subject to
    /// change.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Filter {
        /// The filter isn't specified, or is unknown. This returns all jobs ordered
        /// on descending `JobUuid`.
        Unknown = 0,
        /// Returns all running jobs first ordered on creation timestamp, then
        /// returns all terminated jobs ordered on the termination timestamp.
        All = 1,
        /// Filters the jobs that have a terminated state, ordered on the
        /// termination timestamp. Example terminated states: `JOB_STATE_STOPPED`,
        /// `JOB_STATE_UPDATED`, `JOB_STATE_DRAINED`, etc.
        Terminated = 2,
        /// Filters the jobs that are running ordered on the creation timestamp.
        Active = 3,
    }
}
/// Indicates which [regional endpoint]
/// (<https://cloud.google.com/dataflow/docs/concepts/regional-endpoints>) failed
/// to respond to a request for data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FailedLocation {
    /// The name of the [regional endpoint]
    /// (<https://cloud.google.com/dataflow/docs/concepts/regional-endpoints>) that
    /// failed to respond.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Response to a request to list Cloud Dataflow jobs in a project. This might
/// be a partial response, depending on the page size in the ListJobsRequest.
/// However, if the project does not have any jobs, an instance of
/// ListJobsResponse is not returned and the requests's response
/// body is empty {}.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListJobsResponse {
    /// A subset of the requested job information.
    #[prost(message, repeated, tag = "1")]
    pub jobs: ::prost::alloc::vec::Vec<Job>,
    /// Set if there may be more results than fit in this response.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Zero or more messages describing the [regional endpoints]
    /// (<https://cloud.google.com/dataflow/docs/concepts/regional-endpoints>) that
    /// failed to respond.
    #[prost(message, repeated, tag = "3")]
    pub failed_location: ::prost::alloc::vec::Vec<FailedLocation>,
}
/// Request to create a snapshot of a job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotJobRequest {
    /// The project which owns the job to be snapshotted.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// The job to be snapshotted.
    #[prost(string, tag = "2")]
    pub job_id: ::prost::alloc::string::String,
    /// TTL for the snapshot.
    #[prost(message, optional, tag = "3")]
    pub ttl: ::core::option::Option<::prost_types::Duration>,
    /// The location that contains this job.
    #[prost(string, tag = "4")]
    pub location: ::prost::alloc::string::String,
    /// If true, perform snapshots for sources which support this.
    #[prost(bool, tag = "5")]
    pub snapshot_sources: bool,
    /// User specified description of the snapshot. Maybe empty.
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
}
/// Request to check is active jobs exists for a project
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckActiveJobsRequest {
    /// The project which owns the jobs.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
}
/// Response for CheckActiveJobsRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckActiveJobsResponse {
    /// If True, active jobs exists for project. False otherwise.
    #[prost(bool, tag = "1")]
    pub active_jobs_exist: bool,
}
/// Type of transform or stage operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum KindType {
    /// Unrecognized transform type.
    UnknownKind = 0,
    /// ParDo transform.
    ParDoKind = 1,
    /// Group By Key transform.
    GroupByKeyKind = 2,
    /// Flatten transform.
    FlattenKind = 3,
    /// Read transform.
    ReadKind = 4,
    /// Write transform.
    WriteKind = 5,
    /// Constructs from a constant value, such as with Create.of.
    ConstantKind = 6,
    /// Creates a Singleton view of a collection.
    SingletonKind = 7,
    /// Opening or closing a shuffle session, often as part of a GroupByKey.
    ShuffleKind = 8,
}
/// Describes the overall state of a \[google.dataflow.v1beta3.Job][google.dataflow.v1beta3.Job\].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum JobState {
    /// The job's run state isn't specified.
    Unknown = 0,
    /// `JOB_STATE_STOPPED` indicates that the job has not
    /// yet started to run.
    Stopped = 1,
    /// `JOB_STATE_RUNNING` indicates that the job is currently running.
    Running = 2,
    /// `JOB_STATE_DONE` indicates that the job has successfully completed.
    /// This is a terminal job state.  This state may be set by the Cloud Dataflow
    /// service, as a transition from `JOB_STATE_RUNNING`. It may also be set via a
    /// Cloud Dataflow `UpdateJob` call, if the job has not yet reached a terminal
    /// state.
    Done = 3,
    /// `JOB_STATE_FAILED` indicates that the job has failed.  This is a
    /// terminal job state.  This state may only be set by the Cloud Dataflow
    /// service, and only as a transition from `JOB_STATE_RUNNING`.
    Failed = 4,
    /// `JOB_STATE_CANCELLED` indicates that the job has been explicitly
    /// cancelled. This is a terminal job state. This state may only be
    /// set via a Cloud Dataflow `UpdateJob` call, and only if the job has not
    /// yet reached another terminal state.
    Cancelled = 5,
    /// `JOB_STATE_UPDATED` indicates that the job was successfully updated,
    /// meaning that this job was stopped and another job was started, inheriting
    /// state from this one. This is a terminal job state. This state may only be
    /// set by the Cloud Dataflow service, and only as a transition from
    /// `JOB_STATE_RUNNING`.
    Updated = 6,
    /// `JOB_STATE_DRAINING` indicates that the job is in the process of draining.
    /// A draining job has stopped pulling from its input sources and is processing
    /// any data that remains in-flight. This state may be set via a Cloud Dataflow
    /// `UpdateJob` call, but only as a transition from `JOB_STATE_RUNNING`. Jobs
    /// that are draining may only transition to `JOB_STATE_DRAINED`,
    /// `JOB_STATE_CANCELLED`, or `JOB_STATE_FAILED`.
    Draining = 7,
    /// `JOB_STATE_DRAINED` indicates that the job has been drained.
    /// A drained job terminated by stopping pulling from its input sources and
    /// processing any data that remained in-flight when draining was requested.
    /// This state is a terminal state, may only be set by the Cloud Dataflow
    /// service, and only as a transition from `JOB_STATE_DRAINING`.
    Drained = 8,
    /// `JOB_STATE_PENDING` indicates that the job has been created but is not yet
    /// running.  Jobs that are pending may only transition to `JOB_STATE_RUNNING`,
    /// or `JOB_STATE_FAILED`.
    Pending = 9,
    /// `JOB_STATE_CANCELLING` indicates that the job has been explicitly cancelled
    /// and is in the process of stopping.  Jobs that are cancelling may only
    /// transition to `JOB_STATE_CANCELLED` or `JOB_STATE_FAILED`.
    Cancelling = 10,
    /// `JOB_STATE_QUEUED` indicates that the job has been created but is being
    /// delayed until launch. Jobs that are queued may only transition to
    /// `JOB_STATE_PENDING` or `JOB_STATE_CANCELLED`.
    Queued = 11,
    /// `JOB_STATE_RESOURCE_CLEANING_UP` indicates that the batch job's associated
    /// resources are currently being cleaned up after a successful run.
    /// Currently, this is an opt-in feature, please reach out to Cloud support
    /// team if you are interested.
    ResourceCleaningUp = 12,
}
/// Selector for how much information is returned in Job responses.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum JobView {
    /// The job view to return isn't specified, or is unknown.
    /// Responses will contain at least the `JOB_VIEW_SUMMARY` information,
    /// and may contain additional information.
    Unknown = 0,
    /// Request summary information only:
    /// Project ID, Job ID, job name, job type, job status, start/end time,
    /// and Cloud SDK version details.
    Summary = 1,
    /// Request all information available for this job.
    All = 2,
    /// Request summary info and limited job description data for steps, labels and
    /// environment.
    Description = 3,
}
#[doc = r" Generated client implementations."]
pub mod jobs_v1_beta3_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Provides a method to create and modify Google Cloud Dataflow jobs."]
    #[doc = " A Job is a multi-stage computation graph run by the Cloud Dataflow service."]
    #[derive(Debug, Clone)]
    pub struct JobsV1Beta3Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> JobsV1Beta3Client<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> JobsV1Beta3Client<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            JobsV1Beta3Client::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Creates a Cloud Dataflow job."]
        #[doc = ""]
        #[doc = " To create a job, we recommend using `projects.locations.jobs.create` with a"]
        #[doc = " [regional endpoint]"]
        #[doc = " (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using"]
        #[doc = " `projects.jobs.create` is not recommended, as your job will always start"]
        #[doc = " in `us-central1`."]
        pub async fn create_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateJobRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.dataflow.v1beta3.JobsV1Beta3/CreateJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the state of the specified Cloud Dataflow job."]
        #[doc = ""]
        #[doc = " To get the state of a job, we recommend using `projects.locations.jobs.get`"]
        #[doc = " with a [regional endpoint]"]
        #[doc = " (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using"]
        #[doc = " `projects.jobs.get` is not recommended, as you can only get the state of"]
        #[doc = " jobs that are running in `us-central1`."]
        pub async fn get_job(
            &mut self,
            request: impl tonic::IntoRequest<super::GetJobRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.dataflow.v1beta3.JobsV1Beta3/GetJob");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the state of an existing Cloud Dataflow job."]
        #[doc = ""]
        #[doc = " To update the state of an existing job, we recommend using"]
        #[doc = " `projects.locations.jobs.update` with a [regional endpoint]"]
        #[doc = " (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using"]
        #[doc = " `projects.jobs.update` is not recommended, as you can only update the state"]
        #[doc = " of jobs that are running in `us-central1`."]
        pub async fn update_job(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateJobRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.dataflow.v1beta3.JobsV1Beta3/UpdateJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List the jobs of a project."]
        #[doc = ""]
        #[doc = " To list the jobs of a project in a region, we recommend using"]
        #[doc = " `projects.locations.jobs.list` with a [regional endpoint]"]
        #[doc = " (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). To"]
        #[doc = " list the all jobs across all regions, use `projects.jobs.aggregated`. Using"]
        #[doc = " `projects.jobs.list` is not recommended, as you can only get the list of"]
        #[doc = " jobs that are running in `us-central1`."]
        pub async fn list_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListJobsRequest>,
        ) -> Result<tonic::Response<super::ListJobsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.dataflow.v1beta3.JobsV1Beta3/ListJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List the jobs of a project across all regions."]
        pub async fn aggregated_list_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListJobsRequest>,
        ) -> Result<tonic::Response<super::ListJobsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.dataflow.v1beta3.JobsV1Beta3/AggregatedListJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Check for existence of active jobs in the given project across all regions."]
        pub async fn check_active_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::CheckActiveJobsRequest>,
        ) -> Result<tonic::Response<super::CheckActiveJobsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.dataflow.v1beta3.JobsV1Beta3/CheckActiveJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Snapshot the state of a streaming job."]
        pub async fn snapshot_job(
            &mut self,
            request: impl tonic::IntoRequest<super::SnapshotJobRequest>,
        ) -> Result<tonic::Response<super::Snapshot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.dataflow.v1beta3.JobsV1Beta3/SnapshotJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Identifies a metric, by describing the source which generated the
/// metric.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricStructuredName {
    /// Origin (namespace) of metric name. May be blank for user-define metrics;
    /// will be "dataflow" for metrics defined by the Dataflow service or SDK.
    #[prost(string, tag = "1")]
    pub origin: ::prost::alloc::string::String,
    /// Worker-defined metric name.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// Zero or more labeled fields which identify the part of the job this
    /// metric is associated with, such as the name of a step or collection.
    ///
    /// For example, built-in counters associated with steps will have
    /// context\['step'\] = <step-name>. Counters associated with PCollections
    /// in the SDK will have context\['pcollection'\] = <pcollection-name>.
    #[prost(btree_map = "string, string", tag = "3")]
    pub context: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Describes the state of a metric.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricUpdate {
    /// Name of the metric.
    #[prost(message, optional, tag = "1")]
    pub name: ::core::option::Option<MetricStructuredName>,
    /// Metric aggregation kind.  The possible metric aggregation kinds are
    /// "Sum", "Max", "Min", "Mean", "Set", "And", "Or", and "Distribution".
    /// The specified aggregation kind is case-insensitive.
    ///
    /// If omitted, this is not an aggregated value but instead
    /// a single metric sample value.
    #[prost(string, tag = "2")]
    pub kind: ::prost::alloc::string::String,
    /// True if this metric is reported as the total cumulative aggregate
    /// value accumulated since the worker started working on this WorkItem.
    /// By default this is false, indicating that this metric is reported
    /// as a delta that is not associated with any WorkItem.
    #[prost(bool, tag = "3")]
    pub cumulative: bool,
    /// Worker-computed aggregate value for aggregation kinds "Sum", "Max", "Min",
    /// "And", and "Or".  The possible value types are Long, Double, and Boolean.
    #[prost(message, optional, tag = "4")]
    pub scalar: ::core::option::Option<::prost_types::Value>,
    /// Worker-computed aggregate value for the "Mean" aggregation kind.
    /// This holds the sum of the aggregated values and is used in combination
    /// with mean_count below to obtain the actual mean aggregate value.
    /// The only possible value types are Long and Double.
    #[prost(message, optional, tag = "5")]
    pub mean_sum: ::core::option::Option<::prost_types::Value>,
    /// Worker-computed aggregate value for the "Mean" aggregation kind.
    /// This holds the count of the aggregated values and is used in combination
    /// with mean_sum above to obtain the actual mean aggregate value.
    /// The only possible value type is Long.
    #[prost(message, optional, tag = "6")]
    pub mean_count: ::core::option::Option<::prost_types::Value>,
    /// Worker-computed aggregate value for the "Set" aggregation kind.  The only
    /// possible value type is a list of Values whose type can be Long, Double,
    /// or String, according to the metric's type.  All Values in the list must
    /// be of the same type.
    #[prost(message, optional, tag = "7")]
    pub set: ::core::option::Option<::prost_types::Value>,
    /// A struct value describing properties of a distribution of numeric values.
    #[prost(message, optional, tag = "11")]
    pub distribution: ::core::option::Option<::prost_types::Value>,
    /// A struct value describing properties of a Gauge.
    /// Metrics of gauge type show the value of a metric across time, and is
    /// aggregated based on the newest value.
    #[prost(message, optional, tag = "12")]
    pub gauge: ::core::option::Option<::prost_types::Value>,
    /// Worker-computed aggregate value for internal use by the Dataflow
    /// service.
    #[prost(message, optional, tag = "8")]
    pub internal: ::core::option::Option<::prost_types::Value>,
    /// Timestamp associated with the metric value. Optional when workers are
    /// reporting work progress; it will be filled in responses from the
    /// metrics API.
    #[prost(message, optional, tag = "9")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request to get job metrics.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetJobMetricsRequest {
    /// A project id.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// The job to get metrics for.
    #[prost(string, tag = "2")]
    pub job_id: ::prost::alloc::string::String,
    /// Return only metric data that has changed since this time.
    /// Default is to return all information about all metrics for the job.
    #[prost(message, optional, tag = "3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The [regional endpoint]
    /// (<https://cloud.google.com/dataflow/docs/concepts/regional-endpoints>) that
    /// contains the job specified by job_id.
    #[prost(string, tag = "4")]
    pub location: ::prost::alloc::string::String,
}
/// JobMetrics contains a collection of metrics describing the detailed progress
/// of a Dataflow job. Metrics correspond to user-defined and system-defined
/// metrics in the job.
///
/// This resource captures only the most recent values of each metric;
/// time-series data can be queried for them (under the same metric names)
/// from Cloud Monitoring.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobMetrics {
    /// Timestamp as of which metric values are current.
    #[prost(message, optional, tag = "1")]
    pub metric_time: ::core::option::Option<::prost_types::Timestamp>,
    /// All metrics for this job.
    #[prost(message, repeated, tag = "2")]
    pub metrics: ::prost::alloc::vec::Vec<MetricUpdate>,
}
/// Request to get job execution details.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetJobExecutionDetailsRequest {
    /// A project id.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// The job to get execution details for.
    #[prost(string, tag = "2")]
    pub job_id: ::prost::alloc::string::String,
    /// The [regional endpoint]
    /// (<https://cloud.google.com/dataflow/docs/concepts/regional-endpoints>) that
    /// contains the job specified by job_id.
    #[prost(string, tag = "3")]
    pub location: ::prost::alloc::string::String,
    /// If specified, determines the maximum number of stages to
    /// return.  If unspecified, the service may choose an appropriate
    /// default, or may return an arbitrarily large number of results.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// If supplied, this should be the value of next_page_token returned
    /// by an earlier call. This will cause the next page of results to
    /// be returned.
    #[prost(string, tag = "5")]
    pub page_token: ::prost::alloc::string::String,
}
/// Information about the progress of some component of job execution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProgressTimeseries {
    /// The current progress of the component, in the range \[0,1\].
    #[prost(double, tag = "1")]
    pub current_progress: f64,
    /// History of progress for the component.
    ///
    /// Points are sorted by time.
    #[prost(message, repeated, tag = "2")]
    pub data_points: ::prost::alloc::vec::Vec<progress_timeseries::Point>,
}
/// Nested message and enum types in `ProgressTimeseries`.
pub mod progress_timeseries {
    /// A point in the timeseries.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Point {
        /// The timestamp of the point.
        #[prost(message, optional, tag = "1")]
        pub time: ::core::option::Option<::prost_types::Timestamp>,
        /// The value of the point.
        #[prost(double, tag = "2")]
        pub value: f64,
    }
}
/// Information about a particular execution stage of a job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StageSummary {
    /// ID of this stage
    #[prost(string, tag = "1")]
    pub stage_id: ::prost::alloc::string::String,
    /// State of this stage.
    #[prost(enumeration = "ExecutionState", tag = "2")]
    pub state: i32,
    /// Start time of this stage.
    #[prost(message, optional, tag = "3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// End time of this stage.
    ///
    /// If the work item is completed, this is the actual end time of the stage.
    /// Otherwise, it is the predicted end time.
    #[prost(message, optional, tag = "4")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Progress for this stage.
    /// Only applicable to Batch jobs.
    #[prost(message, optional, tag = "5")]
    pub progress: ::core::option::Option<ProgressTimeseries>,
    /// Metrics for this stage.
    #[prost(message, repeated, tag = "6")]
    pub metrics: ::prost::alloc::vec::Vec<MetricUpdate>,
}
/// Information about the execution of a job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobExecutionDetails {
    /// The stages of the job execution.
    #[prost(message, repeated, tag = "1")]
    pub stages: ::prost::alloc::vec::Vec<StageSummary>,
    /// If present, this response does not contain all requested tasks.  To obtain
    /// the next page of results, repeat the request with page_token set to this
    /// value.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request to get information about a particular execution stage of a job.
/// Currently only tracked for Batch jobs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStageExecutionDetailsRequest {
    /// A project id.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// The job to get execution details for.
    #[prost(string, tag = "2")]
    pub job_id: ::prost::alloc::string::String,
    /// The [regional endpoint]
    /// (<https://cloud.google.com/dataflow/docs/concepts/regional-endpoints>) that
    /// contains the job specified by job_id.
    #[prost(string, tag = "3")]
    pub location: ::prost::alloc::string::String,
    /// The stage for which to fetch information.
    #[prost(string, tag = "4")]
    pub stage_id: ::prost::alloc::string::String,
    /// If specified, determines the maximum number of work items to
    /// return.  If unspecified, the service may choose an appropriate
    /// default, or may return an arbitrarily large number of results.
    #[prost(int32, tag = "5")]
    pub page_size: i32,
    /// If supplied, this should be the value of next_page_token returned
    /// by an earlier call. This will cause the next page of results to
    /// be returned.
    #[prost(string, tag = "6")]
    pub page_token: ::prost::alloc::string::String,
    /// Lower time bound of work items to include, by start time.
    #[prost(message, optional, tag = "7")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Upper time bound of work items to include, by start time.
    #[prost(message, optional, tag = "8")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Information about an individual work item execution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkItemDetails {
    /// Name of this work item.
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    /// Attempt ID of this work item
    #[prost(string, tag = "2")]
    pub attempt_id: ::prost::alloc::string::String,
    /// Start time of this work item attempt.
    #[prost(message, optional, tag = "3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// End time of this work item attempt.
    ///
    /// If the work item is completed, this is the actual end time of the work
    /// item.  Otherwise, it is the predicted end time.
    #[prost(message, optional, tag = "4")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// State of this work item.
    #[prost(enumeration = "ExecutionState", tag = "5")]
    pub state: i32,
    /// Progress of this work item.
    #[prost(message, optional, tag = "6")]
    pub progress: ::core::option::Option<ProgressTimeseries>,
    /// Metrics for this work item.
    #[prost(message, repeated, tag = "7")]
    pub metrics: ::prost::alloc::vec::Vec<MetricUpdate>,
}
/// Information about a worker
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerDetails {
    /// Name of this worker
    #[prost(string, tag = "1")]
    pub worker_name: ::prost::alloc::string::String,
    /// Work items processed by this worker, sorted by time.
    #[prost(message, repeated, tag = "2")]
    pub work_items: ::prost::alloc::vec::Vec<WorkItemDetails>,
}
/// Information about the workers and work items within a stage.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StageExecutionDetails {
    /// Workers that have done work on the stage.
    #[prost(message, repeated, tag = "1")]
    pub workers: ::prost::alloc::vec::Vec<WorkerDetails>,
    /// If present, this response does not contain all requested tasks.  To obtain
    /// the next page of results, repeat the request with page_token set to this
    /// value.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The state of some component of job execution.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExecutionState {
    /// The component state is unknown or unspecified.
    Unknown = 0,
    /// The component is not yet running.
    NotStarted = 1,
    /// The component is currently running.
    Running = 2,
    /// The component succeeded.
    Succeeded = 3,
    /// The component failed.
    Failed = 4,
    /// Execution of the component was cancelled.
    Cancelled = 5,
}
#[doc = r" Generated client implementations."]
pub mod metrics_v1_beta3_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " The Dataflow Metrics API lets you monitor the progress of Dataflow"]
    #[doc = " jobs."]
    #[derive(Debug, Clone)]
    pub struct MetricsV1Beta3Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> MetricsV1Beta3Client<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MetricsV1Beta3Client<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            MetricsV1Beta3Client::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Request the job status."]
        #[doc = ""]
        #[doc = " To request the status of a job, we recommend using"]
        #[doc = " `projects.locations.jobs.getMetrics` with a [regional endpoint]"]
        #[doc = " (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using"]
        #[doc = " `projects.jobs.getMetrics` is not recommended, as you can only request the"]
        #[doc = " status of jobs that are running in `us-central1`."]
        pub async fn get_job_metrics(
            &mut self,
            request: impl tonic::IntoRequest<super::GetJobMetricsRequest>,
        ) -> Result<tonic::Response<super::JobMetrics>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.dataflow.v1beta3.MetricsV1Beta3/GetJobMetrics",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Request detailed information about the execution status of the job."]
        #[doc = ""]
        #[doc = " EXPERIMENTAL.  This API is subject to change or removal without notice."]
        pub async fn get_job_execution_details(
            &mut self,
            request: impl tonic::IntoRequest<super::GetJobExecutionDetailsRequest>,
        ) -> Result<tonic::Response<super::JobExecutionDetails>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.dataflow.v1beta3.MetricsV1Beta3/GetJobExecutionDetails",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Request detailed information about the execution status of a stage of the"]
        #[doc = " job."]
        #[doc = ""]
        #[doc = " EXPERIMENTAL.  This API is subject to change or removal without notice."]
        pub async fn get_stage_execution_details(
            &mut self,
            request: impl tonic::IntoRequest<super::GetStageExecutionDetailsRequest>,
        ) -> Result<tonic::Response<super::StageExecutionDetails>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.dataflow.v1beta3.MetricsV1Beta3/GetStageExecutionDetails",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Global topology of the streaming Dataflow job, including all
/// computations and their sharded locations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopologyConfig {
    /// The computations associated with a streaming Dataflow job.
    #[prost(message, repeated, tag = "1")]
    pub computations: ::prost::alloc::vec::Vec<ComputationTopology>,
    /// The disks assigned to a streaming Dataflow job.
    #[prost(message, repeated, tag = "2")]
    pub data_disk_assignments: ::prost::alloc::vec::Vec<DataDiskAssignment>,
    /// Maps user stage names to stable computation names.
    #[prost(btree_map = "string, string", tag = "3")]
    pub user_stage_to_computation_name_map: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The size (in bits) of keys that will be assigned to source messages.
    #[prost(int32, tag = "4")]
    pub forwarding_key_bits: i32,
    /// Version number for persistent state.
    #[prost(int32, tag = "5")]
    pub persistent_state_version: i32,
}
/// Identifies a pubsub location to use for transferring data into or
/// out of a streaming Dataflow job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubsubLocation {
    /// A pubsub topic, in the form of
    /// "pubsub.googleapis.com/topics/<project-id>/<topic-name>"
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
    /// A pubsub subscription, in the form of
    /// "pubsub.googleapis.com/subscriptions/<project-id>/<subscription-name>"
    #[prost(string, tag = "2")]
    pub subscription: ::prost::alloc::string::String,
    /// If set, contains a pubsub label from which to extract record timestamps.
    /// If left empty, record timestamps will be generated upon arrival.
    #[prost(string, tag = "3")]
    pub timestamp_label: ::prost::alloc::string::String,
    /// If set, contains a pubsub label from which to extract record ids.
    /// If left empty, record deduplication will be strictly best effort.
    #[prost(string, tag = "4")]
    pub id_label: ::prost::alloc::string::String,
    /// Indicates whether the pipeline allows late-arriving data.
    #[prost(bool, tag = "5")]
    pub drop_late_data: bool,
    /// If set, specifies the pubsub subscription that will be used for tracking
    /// custom time timestamps for watermark estimation.
    #[prost(string, tag = "6")]
    pub tracking_subscription: ::prost::alloc::string::String,
    /// If true, then the client has requested to get pubsub attributes.
    #[prost(bool, tag = "7")]
    pub with_attributes: bool,
}
/// Identifies the location of a streaming computation stage, for
/// stage-to-stage communication.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingStageLocation {
    /// Identifies the particular stream within the streaming Dataflow
    /// job.
    #[prost(string, tag = "1")]
    pub stream_id: ::prost::alloc::string::String,
}
/// Identifies the location of a streaming side input.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingSideInputLocation {
    /// Identifies the particular side input within the streaming Dataflow job.
    #[prost(string, tag = "1")]
    pub tag: ::prost::alloc::string::String,
    /// Identifies the state family where this side input is stored.
    #[prost(string, tag = "2")]
    pub state_family: ::prost::alloc::string::String,
}
/// Identifies the location of a custom souce.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomSourceLocation {
    /// Whether this source is stateful.
    #[prost(bool, tag = "1")]
    pub stateful: bool,
}
/// Describes a stream of data, either as input to be processed or as
/// output of a streaming Dataflow job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamLocation {
    /// A specification of a stream's location.
    #[prost(oneof = "stream_location::Location", tags = "1, 2, 3, 4")]
    pub location: ::core::option::Option<stream_location::Location>,
}
/// Nested message and enum types in `StreamLocation`.
pub mod stream_location {
    /// A specification of a stream's location.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Location {
        /// The stream is part of another computation within the current
        /// streaming Dataflow job.
        #[prost(message, tag = "1")]
        StreamingStageLocation(super::StreamingStageLocation),
        /// The stream is a pubsub stream.
        #[prost(message, tag = "2")]
        PubsubLocation(super::PubsubLocation),
        /// The stream is a streaming side input.
        #[prost(message, tag = "3")]
        SideInputLocation(super::StreamingSideInputLocation),
        /// The stream is a custom source.
        #[prost(message, tag = "4")]
        CustomSourceLocation(super::CustomSourceLocation),
    }
}
/// State family configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StateFamilyConfig {
    /// The state family value.
    #[prost(string, tag = "1")]
    pub state_family: ::prost::alloc::string::String,
    /// If true, this family corresponds to a read operation.
    #[prost(bool, tag = "2")]
    pub is_read: bool,
}
/// All configuration data for a particular Computation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputationTopology {
    /// The system stage name.
    #[prost(string, tag = "1")]
    pub system_stage_name: ::prost::alloc::string::String,
    /// The ID of the computation.
    #[prost(string, tag = "5")]
    pub computation_id: ::prost::alloc::string::String,
    /// The key ranges processed by the computation.
    #[prost(message, repeated, tag = "2")]
    pub key_ranges: ::prost::alloc::vec::Vec<KeyRangeLocation>,
    /// The inputs to the computation.
    #[prost(message, repeated, tag = "3")]
    pub inputs: ::prost::alloc::vec::Vec<StreamLocation>,
    /// The outputs from the computation.
    #[prost(message, repeated, tag = "4")]
    pub outputs: ::prost::alloc::vec::Vec<StreamLocation>,
    /// The state family values.
    #[prost(message, repeated, tag = "7")]
    pub state_families: ::prost::alloc::vec::Vec<StateFamilyConfig>,
}
/// Location information for a specific key-range of a sharded computation.
/// Currently we only support UTF-8 character splits to simplify encoding into
/// JSON.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyRangeLocation {
    /// The start (inclusive) of the key range.
    #[prost(string, tag = "1")]
    pub start: ::prost::alloc::string::String,
    /// The end (exclusive) of the key range.
    #[prost(string, tag = "2")]
    pub end: ::prost::alloc::string::String,
    /// The physical location of this range assignment to be used for
    /// streaming computation cross-worker message delivery.
    #[prost(string, tag = "3")]
    pub delivery_endpoint: ::prost::alloc::string::String,
    /// The name of the data disk where data for this range is stored.
    /// This name is local to the Google Cloud Platform project and uniquely
    /// identifies the disk within that project, for example
    /// "myproject-1014-104817-4c2-harness-0-disk-1".
    #[prost(string, tag = "5")]
    pub data_disk: ::prost::alloc::string::String,
    /// DEPRECATED. The location of the persistent state for this range, as a
    /// persistent directory in the worker local filesystem.
    #[deprecated]
    #[prost(string, tag = "4")]
    pub deprecated_persistent_directory: ::prost::alloc::string::String,
}
/// Describes mounted data disk.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MountedDataDisk {
    /// The name of the data disk.
    /// This name is local to the Google Cloud Platform project and uniquely
    /// identifies the disk within that project, for example
    /// "myproject-1014-104817-4c2-harness-0-disk-1".
    #[prost(string, tag = "1")]
    pub data_disk: ::prost::alloc::string::String,
}
/// Data disk assignment for a given VM instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataDiskAssignment {
    /// VM instance name the data disks mounted to, for example
    /// "myproject-1014-104817-4c2-harness-0".
    #[prost(string, tag = "1")]
    pub vm_instance: ::prost::alloc::string::String,
    /// Mounted data disks. The order is important a data disk's 0-based index in
    /// this list defines which persistent directory the disk is mounted to, for
    /// example the list of { "myproject-1014-104817-4c2-harness-0-disk-0" },
    /// { "myproject-1014-104817-4c2-harness-0-disk-1" }.
    #[prost(string, repeated, tag = "2")]
    pub data_disks: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Data disk assignment information for a specific key-range of a sharded
/// computation.
/// Currently we only support UTF-8 character splits to simplify encoding into
/// JSON.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyRangeDataDiskAssignment {
    /// The start (inclusive) of the key range.
    #[prost(string, tag = "1")]
    pub start: ::prost::alloc::string::String,
    /// The end (exclusive) of the key range.
    #[prost(string, tag = "2")]
    pub end: ::prost::alloc::string::String,
    /// The name of the data disk where data for this range is stored.
    /// This name is local to the Google Cloud Platform project and uniquely
    /// identifies the disk within that project, for example
    /// "myproject-1014-104817-4c2-harness-0-disk-1".
    #[prost(string, tag = "3")]
    pub data_disk: ::prost::alloc::string::String,
}
/// Describes full or partial data disk assignment information of the computation
/// ranges.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingComputationRanges {
    /// The ID of the computation.
    #[prost(string, tag = "1")]
    pub computation_id: ::prost::alloc::string::String,
    /// Data disk assignments for ranges from this computation.
    #[prost(message, repeated, tag = "2")]
    pub range_assignments: ::prost::alloc::vec::Vec<KeyRangeDataDiskAssignment>,
}
/// Streaming appliance snapshot configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingApplianceSnapshotConfig {
    /// If set, indicates the snapshot id for the snapshot being performed.
    #[prost(string, tag = "1")]
    pub snapshot_id: ::prost::alloc::string::String,
    /// Indicates which endpoint is used to import appliance state.
    #[prost(string, tag = "2")]
    pub import_state_endpoint: ::prost::alloc::string::String,
}
/// A particular message pertaining to a Dataflow job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobMessage {
    /// Deprecated.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The timestamp of the message.
    #[prost(message, optional, tag = "2")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// The text of the message.
    #[prost(string, tag = "3")]
    pub message_text: ::prost::alloc::string::String,
    /// Importance level of the message.
    #[prost(enumeration = "JobMessageImportance", tag = "4")]
    pub message_importance: i32,
}
/// A rich message format, including a human readable string, a key for
/// identifying the message, and structured data associated with the message for
/// programmatic consumption.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StructuredMessage {
    /// Human-readable version of message.
    #[prost(string, tag = "1")]
    pub message_text: ::prost::alloc::string::String,
    /// Identifier for this message type.  Used by external systems to
    /// internationalize or personalize message.
    #[prost(string, tag = "2")]
    pub message_key: ::prost::alloc::string::String,
    /// The structured data associated with this message.
    #[prost(message, repeated, tag = "3")]
    pub parameters: ::prost::alloc::vec::Vec<structured_message::Parameter>,
}
/// Nested message and enum types in `StructuredMessage`.
pub mod structured_message {
    /// Structured data associated with this message.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Parameter {
        /// Key or name for this parameter.
        #[prost(string, tag = "1")]
        pub key: ::prost::alloc::string::String,
        /// Value for this parameter.
        #[prost(message, optional, tag = "2")]
        pub value: ::core::option::Option<::prost_types::Value>,
    }
}
/// A structured message reporting an autoscaling decision made by the Dataflow
/// service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoscalingEvent {
    /// The current number of workers the job has.
    #[prost(int64, tag = "1")]
    pub current_num_workers: i64,
    /// The target number of workers the worker pool wants to resize to use.
    #[prost(int64, tag = "2")]
    pub target_num_workers: i64,
    /// The type of autoscaling event to report.
    #[prost(enumeration = "autoscaling_event::AutoscalingEventType", tag = "3")]
    pub event_type: i32,
    /// A message describing why the system decided to adjust the current
    /// number of workers, why it failed, or why the system decided to
    /// not make any changes to the number of workers.
    #[prost(message, optional, tag = "4")]
    pub description: ::core::option::Option<StructuredMessage>,
    /// The time this event was emitted to indicate a new target or current
    /// num_workers value.
    #[prost(message, optional, tag = "5")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// A short and friendly name for the worker pool this event refers to,
    /// populated from the value of PoolStageRelation::user_pool_name.
    #[prost(string, tag = "7")]
    pub worker_pool: ::prost::alloc::string::String,
}
/// Nested message and enum types in `AutoscalingEvent`.
pub mod autoscaling_event {
    /// Indicates the type of autoscaling event.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AutoscalingEventType {
        /// Default type for the enum.  Value should never be returned.
        TypeUnknown = 0,
        /// The TARGET_NUM_WORKERS_CHANGED type should be used when the target
        /// worker pool size has changed at the start of an actuation. An event
        /// should always be specified as TARGET_NUM_WORKERS_CHANGED if it reflects
        /// a change in the target_num_workers.
        TargetNumWorkersChanged = 1,
        /// The CURRENT_NUM_WORKERS_CHANGED type should be used when actual worker
        /// pool size has been changed, but the target_num_workers has not changed.
        CurrentNumWorkersChanged = 2,
        /// The ACTUATION_FAILURE type should be used when we want to report
        /// an error to the user indicating why the current number of workers
        /// in the pool could not be changed.
        /// Displayed in the current status and history widgets.
        ActuationFailure = 3,
        /// Used when we want to report to the user a reason why we are
        /// not currently adjusting the number of workers.
        /// Should specify both target_num_workers, current_num_workers and a
        /// decision_message.
        NoChange = 4,
    }
}
/// Request to list job messages.
/// Up to max_results messages will be returned in the time range specified
/// starting with the oldest messages first. If no time range is specified
/// the results with start with the oldest message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListJobMessagesRequest {
    /// A project id.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// The job to get messages about.
    #[prost(string, tag = "2")]
    pub job_id: ::prost::alloc::string::String,
    /// Filter to only get messages with importance >= level
    #[prost(enumeration = "JobMessageImportance", tag = "3")]
    pub minimum_importance: i32,
    /// If specified, determines the maximum number of messages to
    /// return.  If unspecified, the service may choose an appropriate
    /// default, or may return an arbitrarily large number of results.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// If supplied, this should be the value of next_page_token returned
    /// by an earlier call. This will cause the next page of results to
    /// be returned.
    #[prost(string, tag = "5")]
    pub page_token: ::prost::alloc::string::String,
    /// If specified, return only messages with timestamps >= start_time.
    /// The default is the job creation time (i.e. beginning of messages).
    #[prost(message, optional, tag = "6")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Return only messages with timestamps < end_time. The default is now
    /// (i.e. return up to the latest messages available).
    #[prost(message, optional, tag = "7")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The [regional endpoint]
    /// (<https://cloud.google.com/dataflow/docs/concepts/regional-endpoints>) that
    /// contains the job specified by job_id.
    #[prost(string, tag = "8")]
    pub location: ::prost::alloc::string::String,
}
/// Response to a request to list job messages.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListJobMessagesResponse {
    /// Messages in ascending timestamp order.
    #[prost(message, repeated, tag = "1")]
    pub job_messages: ::prost::alloc::vec::Vec<JobMessage>,
    /// The token to obtain the next page of results if there are more.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Autoscaling events in ascending timestamp order.
    #[prost(message, repeated, tag = "3")]
    pub autoscaling_events: ::prost::alloc::vec::Vec<AutoscalingEvent>,
}
/// Indicates the importance of the message.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum JobMessageImportance {
    /// The message importance isn't specified, or is unknown.
    Unknown = 0,
    /// The message is at the 'debug' level: typically only useful for
    /// software engineers working on the code the job is running.
    /// Typically, Dataflow pipeline runners do not display log messages
    /// at this level by default.
    JobMessageDebug = 1,
    /// The message is at the 'detailed' level: somewhat verbose, but
    /// potentially useful to users.  Typically, Dataflow pipeline
    /// runners do not display log messages at this level by default.
    /// These messages are displayed by default in the Dataflow
    /// monitoring UI.
    JobMessageDetailed = 2,
    /// The message is at the 'basic' level: useful for keeping
    /// track of the execution of a Dataflow pipeline.  Typically,
    /// Dataflow pipeline runners display log messages at this level by
    /// default, and these messages are displayed by default in the
    /// Dataflow monitoring UI.
    JobMessageBasic = 5,
    /// The message is at the 'warning' level: indicating a condition
    /// pertaining to a job which may require human intervention.
    /// Typically, Dataflow pipeline runners display log messages at this
    /// level by default, and these messages are displayed by default in
    /// the Dataflow monitoring UI.
    JobMessageWarning = 3,
    /// The message is at the 'error' level: indicating a condition
    /// preventing a job from succeeding.  Typically, Dataflow pipeline
    /// runners display log messages at this level by default, and these
    /// messages are displayed by default in the Dataflow monitoring UI.
    JobMessageError = 4,
}
#[doc = r" Generated client implementations."]
pub mod messages_v1_beta3_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " The Dataflow Messages API is used for monitoring the progress of"]
    #[doc = " Dataflow jobs."]
    #[derive(Debug, Clone)]
    pub struct MessagesV1Beta3Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> MessagesV1Beta3Client<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MessagesV1Beta3Client<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            MessagesV1Beta3Client::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Request the job status."]
        #[doc = ""]
        #[doc = " To request the status of a job, we recommend using"]
        #[doc = " `projects.locations.jobs.messages.list` with a [regional endpoint]"]
        #[doc = " (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using"]
        #[doc = " `projects.jobs.messages.list` is not recommended, as you can only request"]
        #[doc = " the status of jobs that are running in `us-central1`."]
        pub async fn list_job_messages(
            &mut self,
            request: impl tonic::IntoRequest<super::ListJobMessagesRequest>,
        ) -> Result<tonic::Response<super::ListJobMessagesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.dataflow.v1beta3.MessagesV1Beta3/ListJobMessages",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Response to the request to launch a job from Flex Template.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LaunchFlexTemplateResponse {
    /// The job that was launched, if the request was not a dry run and
    /// the job was successfully launched.
    #[prost(message, optional, tag = "1")]
    pub job: ::core::option::Option<Job>,
}
/// Container Spec.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerSpec {
    /// Name of the docker container image. E.g., gcr.io/project/some-image
    #[prost(string, tag = "1")]
    pub image: ::prost::alloc::string::String,
    /// Metadata describing a template including description and validation rules.
    #[prost(message, optional, tag = "2")]
    pub metadata: ::core::option::Option<TemplateMetadata>,
    /// Required. SDK info of the Flex Template.
    #[prost(message, optional, tag = "3")]
    pub sdk_info: ::core::option::Option<SdkInfo>,
    /// Default runtime environment for the job.
    #[prost(message, optional, tag = "4")]
    pub default_environment: ::core::option::Option<FlexTemplateRuntimeEnvironment>,
}
/// Launch FlexTemplate Parameter.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LaunchFlexTemplateParameter {
    /// Required. The job name to use for the created job. For update job request,
    /// job name should be same as the existing running job.
    #[prost(string, tag = "1")]
    pub job_name: ::prost::alloc::string::String,
    /// The parameters for FlexTemplate.
    /// Ex. {"num_workers":"5"}
    #[prost(btree_map = "string, string", tag = "2")]
    pub parameters: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Launch options for this flex template job. This is a common set of options
    /// across languages and templates. This should not be used to pass job
    /// parameters.
    #[prost(btree_map = "string, string", tag = "6")]
    pub launch_options: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The runtime environment for the FlexTemplate job
    #[prost(message, optional, tag = "7")]
    pub environment: ::core::option::Option<FlexTemplateRuntimeEnvironment>,
    /// Set this to true if you are sending a request to update a running
    /// streaming job. When set, the job name should be the same as the
    /// running job.
    #[prost(bool, tag = "8")]
    pub update: bool,
    /// Use this to pass transform_name_mappings for streaming update jobs.
    /// Ex:{"oldTransformName":"newTransformName",...}'
    #[prost(btree_map = "string, string", tag = "9")]
    pub transform_name_mappings: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Launch Mechanism.
    #[prost(oneof = "launch_flex_template_parameter::Template", tags = "4, 5")]
    pub template: ::core::option::Option<launch_flex_template_parameter::Template>,
}
/// Nested message and enum types in `LaunchFlexTemplateParameter`.
pub mod launch_flex_template_parameter {
    /// Launch Mechanism.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Template {
        /// Spec about the container image to launch.
        #[prost(message, tag = "4")]
        ContainerSpec(super::ContainerSpec),
        /// Cloud Storage path to a file with json serialized ContainerSpec as
        /// content.
        #[prost(string, tag = "5")]
        ContainerSpecGcsPath(::prost::alloc::string::String),
    }
}
/// The environment values to be set at runtime for flex template.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlexTemplateRuntimeEnvironment {
    /// The initial number of Google Compute Engine instances for the job.
    #[prost(int32, tag = "1")]
    pub num_workers: i32,
    /// The maximum number of Google Compute Engine instances to be made
    /// available to your pipeline during execution, from 1 to 1000.
    #[prost(int32, tag = "2")]
    pub max_workers: i32,
    /// The Compute Engine [availability
    /// zone](<https://cloud.google.com/compute/docs/regions-zones/regions-zones>)
    /// for launching worker instances to run your pipeline.
    /// In the future, worker_zone will take precedence.
    #[prost(string, tag = "3")]
    pub zone: ::prost::alloc::string::String,
    /// The email address of the service account to run the job as.
    #[prost(string, tag = "4")]
    pub service_account_email: ::prost::alloc::string::String,
    /// The Cloud Storage path to use for temporary files.
    /// Must be a valid Cloud Storage URL, beginning with `gs://`.
    #[prost(string, tag = "5")]
    pub temp_location: ::prost::alloc::string::String,
    /// The machine type to use for the job. Defaults to the value from the
    /// template if not specified.
    #[prost(string, tag = "6")]
    pub machine_type: ::prost::alloc::string::String,
    /// Additional experiment flags for the job.
    #[prost(string, repeated, tag = "7")]
    pub additional_experiments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Network to which VMs will be assigned.  If empty or unspecified,
    /// the service will use the network "default".
    #[prost(string, tag = "8")]
    pub network: ::prost::alloc::string::String,
    /// Subnetwork to which VMs will be assigned, if desired. You can specify a
    /// subnetwork using either a complete URL or an abbreviated path. Expected to
    /// be of the form
    /// "<https://www.googleapis.com/compute/v1/projects/HOST_PROJECT_ID/regions/REGION/subnetworks/SUBNETWORK">
    /// or "regions/REGION/subnetworks/SUBNETWORK". If the subnetwork is located in
    /// a Shared VPC network, you must use the complete URL.
    #[prost(string, tag = "9")]
    pub subnetwork: ::prost::alloc::string::String,
    /// Additional user labels to be specified for the job.
    /// Keys and values must follow the restrictions specified in the [labeling
    /// restrictions](<https://cloud.google.com/compute/docs/labeling-resources#restrictions>)
    /// page.
    /// An object containing a list of "key": value pairs.
    /// Example: { "name": "wrench", "mass": "1kg", "count": "3" }.
    #[prost(btree_map = "string, string", tag = "10")]
    pub additional_user_labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Name for the Cloud KMS key for the job.
    /// Key format is:
    /// projects/<project>/locations/<location>/keyRings/<keyring>/cryptoKeys/<key>
    #[prost(string, tag = "11")]
    pub kms_key_name: ::prost::alloc::string::String,
    /// Configuration for VM IPs.
    #[prost(enumeration = "WorkerIpAddressConfiguration", tag = "12")]
    pub ip_configuration: i32,
    /// The Compute Engine region
    /// (<https://cloud.google.com/compute/docs/regions-zones/regions-zones>) in
    /// which worker processing should occur, e.g. "us-west1". Mutually exclusive
    /// with worker_zone. If neither worker_region nor worker_zone is specified,
    /// default to the control plane's region.
    #[prost(string, tag = "13")]
    pub worker_region: ::prost::alloc::string::String,
    /// The Compute Engine zone
    /// (<https://cloud.google.com/compute/docs/regions-zones/regions-zones>) in
    /// which worker processing should occur, e.g. "us-west1-a". Mutually exclusive
    /// with worker_region. If neither worker_region nor worker_zone is specified,
    /// a zone in the control plane's region is chosen based on available capacity.
    /// If both `worker_zone` and `zone` are set, `worker_zone` takes precedence.
    #[prost(string, tag = "14")]
    pub worker_zone: ::prost::alloc::string::String,
    /// Whether to enable Streaming Engine for the job.
    #[prost(bool, tag = "15")]
    pub enable_streaming_engine: bool,
    /// Set FlexRS goal for the job.
    /// <https://cloud.google.com/dataflow/docs/guides/flexrs>
    #[prost(enumeration = "FlexResourceSchedulingGoal", tag = "16")]
    pub flexrs_goal: i32,
    /// The Cloud Storage path for staging local files.
    /// Must be a valid Cloud Storage URL, beginning with `gs://`.
    #[prost(string, tag = "17")]
    pub staging_location: ::prost::alloc::string::String,
    /// Docker registry location of container image to use for the 'worker harness.
    /// Default is the container for the version of the SDK. Note this field is
    /// only valid for portable pipelines.
    #[prost(string, tag = "18")]
    pub sdk_container_image: ::prost::alloc::string::String,
}
/// A request to launch a Cloud Dataflow job from a FlexTemplate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LaunchFlexTemplateRequest {
    /// Required. The ID of the Cloud Platform project that the job belongs to.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Parameter to launch a job form Flex Template.
    #[prost(message, optional, tag = "2")]
    pub launch_parameter: ::core::option::Option<LaunchFlexTemplateParameter>,
    /// Required. The [regional endpoint]
    /// (<https://cloud.google.com/dataflow/docs/concepts/regional-endpoints>) to
    /// which to direct the request. E.g., us-central1, us-west1.
    #[prost(string, tag = "3")]
    pub location: ::prost::alloc::string::String,
    /// If true, the request is validated but not actually executed.
    /// Defaults to false.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// The environment values to set at runtime.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeEnvironment {
    /// The initial number of Google Compute Engine instnaces for the job.
    #[prost(int32, tag = "11")]
    pub num_workers: i32,
    /// The maximum number of Google Compute Engine instances to be made
    /// available to your pipeline during execution, from 1 to 1000.
    #[prost(int32, tag = "1")]
    pub max_workers: i32,
    /// The Compute Engine [availability
    /// zone](<https://cloud.google.com/compute/docs/regions-zones/regions-zones>)
    /// for launching worker instances to run your pipeline.
    /// In the future, worker_zone will take precedence.
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// The email address of the service account to run the job as.
    #[prost(string, tag = "3")]
    pub service_account_email: ::prost::alloc::string::String,
    /// The Cloud Storage path to use for temporary files.
    /// Must be a valid Cloud Storage URL, beginning with `gs://`.
    #[prost(string, tag = "4")]
    pub temp_location: ::prost::alloc::string::String,
    /// Whether to bypass the safety checks for the job's temporary directory.
    /// Use with caution.
    #[prost(bool, tag = "5")]
    pub bypass_temp_dir_validation: bool,
    /// The machine type to use for the job. Defaults to the value from the
    /// template if not specified.
    #[prost(string, tag = "6")]
    pub machine_type: ::prost::alloc::string::String,
    /// Additional experiment flags for the job.
    #[prost(string, repeated, tag = "7")]
    pub additional_experiments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Network to which VMs will be assigned.  If empty or unspecified,
    /// the service will use the network "default".
    #[prost(string, tag = "8")]
    pub network: ::prost::alloc::string::String,
    /// Subnetwork to which VMs will be assigned, if desired. You can specify a
    /// subnetwork using either a complete URL or an abbreviated path. Expected to
    /// be of the form
    /// "<https://www.googleapis.com/compute/v1/projects/HOST_PROJECT_ID/regions/REGION/subnetworks/SUBNETWORK">
    /// or "regions/REGION/subnetworks/SUBNETWORK". If the subnetwork is located in
    /// a Shared VPC network, you must use the complete URL.
    #[prost(string, tag = "9")]
    pub subnetwork: ::prost::alloc::string::String,
    /// Additional user labels to be specified for the job.
    /// Keys and values should follow the restrictions specified in the [labeling
    /// restrictions](<https://cloud.google.com/compute/docs/labeling-resources#restrictions>)
    /// page.
    /// An object containing a list of "key": value pairs.
    /// Example: { "name": "wrench", "mass": "1kg", "count": "3" }.
    #[prost(btree_map = "string, string", tag = "10")]
    pub additional_user_labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Name for the Cloud KMS key for the job.
    /// Key format is:
    /// projects/<project>/locations/<location>/keyRings/<keyring>/cryptoKeys/<key>
    #[prost(string, tag = "12")]
    pub kms_key_name: ::prost::alloc::string::String,
    /// Configuration for VM IPs.
    #[prost(enumeration = "WorkerIpAddressConfiguration", tag = "14")]
    pub ip_configuration: i32,
    /// The Compute Engine region
    /// (<https://cloud.google.com/compute/docs/regions-zones/regions-zones>) in
    /// which worker processing should occur, e.g. "us-west1". Mutually exclusive
    /// with worker_zone. If neither worker_region nor worker_zone is specified,
    /// default to the control plane's region.
    #[prost(string, tag = "15")]
    pub worker_region: ::prost::alloc::string::String,
    /// The Compute Engine zone
    /// (<https://cloud.google.com/compute/docs/regions-zones/regions-zones>) in
    /// which worker processing should occur, e.g. "us-west1-a". Mutually exclusive
    /// with worker_region. If neither worker_region nor worker_zone is specified,
    /// a zone in the control plane's region is chosen based on available capacity.
    /// If both `worker_zone` and `zone` are set, `worker_zone` takes precedence.
    #[prost(string, tag = "16")]
    pub worker_zone: ::prost::alloc::string::String,
    /// Whether to enable Streaming Engine for the job.
    #[prost(bool, tag = "17")]
    pub enable_streaming_engine: bool,
}
/// Metadata for a specific parameter.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParameterMetadata {
    /// Required. The name of the parameter.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The label to display for the parameter.
    #[prost(string, tag = "2")]
    pub label: ::prost::alloc::string::String,
    /// Required. The help text to display for the parameter.
    #[prost(string, tag = "3")]
    pub help_text: ::prost::alloc::string::String,
    /// Optional. Whether the parameter is optional. Defaults to false.
    #[prost(bool, tag = "4")]
    pub is_optional: bool,
    /// Optional. Regexes that the parameter must match.
    #[prost(string, repeated, tag = "5")]
    pub regexes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. The type of the parameter.
    /// Used for selecting input picker.
    #[prost(enumeration = "ParameterType", tag = "6")]
    pub param_type: i32,
    /// Optional. Additional metadata for describing this parameter.
    #[prost(btree_map = "string, string", tag = "7")]
    pub custom_metadata: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Metadata describing a template.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TemplateMetadata {
    /// Required. The name of the template.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A description of the template.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// The parameters for the template.
    #[prost(message, repeated, tag = "3")]
    pub parameters: ::prost::alloc::vec::Vec<ParameterMetadata>,
}
/// SDK Information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SdkInfo {
    /// Required. The SDK Language.
    #[prost(enumeration = "sdk_info::Language", tag = "1")]
    pub language: i32,
    /// Optional. The SDK version.
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
/// Nested message and enum types in `SDKInfo`.
pub mod sdk_info {
    /// SDK Language.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Language {
        /// UNKNOWN Language.
        Unknown = 0,
        /// Java.
        Java = 1,
        /// Python.
        Python = 2,
    }
}
/// RuntimeMetadata describing a runtime environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeMetadata {
    /// SDK Info for the template.
    #[prost(message, optional, tag = "1")]
    pub sdk_info: ::core::option::Option<SdkInfo>,
    /// The parameters for the template.
    #[prost(message, repeated, tag = "2")]
    pub parameters: ::prost::alloc::vec::Vec<ParameterMetadata>,
}
/// A request to create a Cloud Dataflow job from a template.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateJobFromTemplateRequest {
    /// Required. The ID of the Cloud Platform project that the job belongs to.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. The job name to use for the created job.
    #[prost(string, tag = "4")]
    pub job_name: ::prost::alloc::string::String,
    /// The runtime parameters to pass to the job.
    #[prost(btree_map = "string, string", tag = "3")]
    pub parameters: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The runtime environment for the job.
    #[prost(message, optional, tag = "5")]
    pub environment: ::core::option::Option<RuntimeEnvironment>,
    /// The [regional endpoint]
    /// (<https://cloud.google.com/dataflow/docs/concepts/regional-endpoints>) to
    /// which to direct the request.
    #[prost(string, tag = "6")]
    pub location: ::prost::alloc::string::String,
    /// The template from which to create the job.
    #[prost(oneof = "create_job_from_template_request::Template", tags = "2")]
    pub template: ::core::option::Option<create_job_from_template_request::Template>,
}
/// Nested message and enum types in `CreateJobFromTemplateRequest`.
pub mod create_job_from_template_request {
    /// The template from which to create the job.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Template {
        /// Required. A Cloud Storage path to the template from which to
        /// create the job.
        /// Must be a valid Cloud Storage URL, beginning with `gs://`.
        #[prost(string, tag = "2")]
        GcsPath(::prost::alloc::string::String),
    }
}
/// A request to retrieve a Cloud Dataflow job template.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTemplateRequest {
    /// Required. The ID of the Cloud Platform project that the job belongs to.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// The view to retrieve. Defaults to METADATA_ONLY.
    #[prost(enumeration = "get_template_request::TemplateView", tag = "3")]
    pub view: i32,
    /// The [regional endpoint]
    /// (<https://cloud.google.com/dataflow/docs/concepts/regional-endpoints>) to
    /// which to direct the request.
    #[prost(string, tag = "4")]
    pub location: ::prost::alloc::string::String,
    /// The template from which to create the job.
    #[prost(oneof = "get_template_request::Template", tags = "2")]
    pub template: ::core::option::Option<get_template_request::Template>,
}
/// Nested message and enum types in `GetTemplateRequest`.
pub mod get_template_request {
    /// The various views of a template that may be retrieved.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TemplateView {
        /// Template view that retrieves only the metadata associated with the
        /// template.
        MetadataOnly = 0,
    }
    /// The template from which to create the job.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Template {
        /// Required. A Cloud Storage path to the template from which to
        /// create the job.
        /// Must be valid Cloud Storage URL, beginning with 'gs://'.
        #[prost(string, tag = "2")]
        GcsPath(::prost::alloc::string::String),
    }
}
/// The response to a GetTemplate request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTemplateResponse {
    /// The status of the get template request. Any problems with the
    /// request will be indicated in the error_details.
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<super::super::rpc::Status>,
    /// The template metadata describing the template name, available
    /// parameters, etc.
    #[prost(message, optional, tag = "2")]
    pub metadata: ::core::option::Option<TemplateMetadata>,
    /// Template Type.
    #[prost(enumeration = "get_template_response::TemplateType", tag = "3")]
    pub template_type: i32,
    /// Describes the runtime metadata with SDKInfo and available parameters.
    #[prost(message, optional, tag = "4")]
    pub runtime_metadata: ::core::option::Option<RuntimeMetadata>,
}
/// Nested message and enum types in `GetTemplateResponse`.
pub mod get_template_response {
    /// Template Type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TemplateType {
        /// Unknown Template Type.
        Unknown = 0,
        /// Legacy Template.
        Legacy = 1,
        /// Flex Template.
        Flex = 2,
    }
}
/// Parameters to provide to the template being launched.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LaunchTemplateParameters {
    /// Required. The job name to use for the created job.
    #[prost(string, tag = "1")]
    pub job_name: ::prost::alloc::string::String,
    /// The runtime parameters to pass to the job.
    #[prost(btree_map = "string, string", tag = "2")]
    pub parameters: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The runtime environment for the job.
    #[prost(message, optional, tag = "3")]
    pub environment: ::core::option::Option<RuntimeEnvironment>,
    /// If set, replace the existing pipeline with the name specified by jobName
    /// with this pipeline, preserving state.
    #[prost(bool, tag = "4")]
    pub update: bool,
    /// Only applicable when updating a pipeline. Map of transform name prefixes of
    /// the job to be replaced to the corresponding name prefixes of the new job.
    #[prost(btree_map = "string, string", tag = "5")]
    pub transform_name_mapping: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// A request to launch a template.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LaunchTemplateRequest {
    /// Required. The ID of the Cloud Platform project that the job belongs to.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// If true, the request is validated but not actually executed.
    /// Defaults to false.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
    /// The parameters of the template to launch. This should be part of the
    /// body of the POST request.
    #[prost(message, optional, tag = "4")]
    pub launch_parameters: ::core::option::Option<LaunchTemplateParameters>,
    /// The [regional endpoint]
    /// (<https://cloud.google.com/dataflow/docs/concepts/regional-endpoints>) to
    /// which to direct the request.
    #[prost(string, tag = "5")]
    pub location: ::prost::alloc::string::String,
    /// The template from which to create the job.
    #[prost(oneof = "launch_template_request::Template", tags = "3, 6")]
    pub template: ::core::option::Option<launch_template_request::Template>,
}
/// Nested message and enum types in `LaunchTemplateRequest`.
pub mod launch_template_request {
    /// The template from which to create the job.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Template {
        /// A Cloud Storage path to the template from which to create
        /// the job.
        /// Must be valid Cloud Storage URL, beginning with 'gs://'.
        #[prost(string, tag = "3")]
        GcsPath(::prost::alloc::string::String),
        /// Params for launching a dynamic template.
        #[prost(message, tag = "6")]
        DynamicTemplate(super::DynamicTemplateLaunchParams),
    }
}
/// Response to the request to launch a template.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LaunchTemplateResponse {
    /// The job that was launched, if the request was not a dry run and
    /// the job was successfully launched.
    #[prost(message, optional, tag = "1")]
    pub job: ::core::option::Option<Job>,
}
/// Used in the error_details field of a google.rpc.Status message, this
/// indicates problems with the template parameter.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvalidTemplateParameters {
    /// Describes all parameter violations in a template request.
    #[prost(message, repeated, tag = "1")]
    pub parameter_violations:
        ::prost::alloc::vec::Vec<invalid_template_parameters::ParameterViolation>,
}
/// Nested message and enum types in `InvalidTemplateParameters`.
pub mod invalid_template_parameters {
    /// A specific template-parameter violation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ParameterViolation {
        /// The parameter that failed to validate.
        #[prost(string, tag = "1")]
        pub parameter: ::prost::alloc::string::String,
        /// A description of why the parameter failed to validate.
        #[prost(string, tag = "2")]
        pub description: ::prost::alloc::string::String,
    }
}
/// Params which should be passed when launching a dynamic template.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicTemplateLaunchParams {
    /// Path to dynamic template spec file on Cloud Storage.
    /// The file must be a Json serialized DynamicTemplateFieSpec object.
    #[prost(string, tag = "1")]
    pub gcs_path: ::prost::alloc::string::String,
    /// Cloud Storage path for staging dependencies.
    /// Must be a valid Cloud Storage URL, beginning with `gs://`.
    #[prost(string, tag = "2")]
    pub staging_location: ::prost::alloc::string::String,
}
/// ParameterType specifies what kind of input we need for this parameter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ParameterType {
    /// Default input type.
    Default = 0,
    /// The parameter specifies generic text input.
    Text = 1,
    /// The parameter specifies a Cloud Storage Bucket to read from.
    GcsReadBucket = 2,
    /// The parameter specifies a Cloud Storage Bucket to write to.
    GcsWriteBucket = 3,
    /// The parameter specifies a Cloud Storage file path to read from.
    GcsReadFile = 4,
    /// The parameter specifies a Cloud Storage file path to write to.
    GcsWriteFile = 5,
    /// The parameter specifies a Cloud Storage folder path to read from.
    GcsReadFolder = 6,
    /// The parameter specifies a Cloud Storage folder to write to.
    GcsWriteFolder = 7,
    /// The parameter specifies a Pub/Sub Topic.
    PubsubTopic = 8,
    /// The parameter specifies a Pub/Sub Subscription.
    PubsubSubscription = 9,
}
#[doc = r" Generated client implementations."]
pub mod templates_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Provides a method to create Cloud Dataflow jobs from templates."]
    #[derive(Debug, Clone)]
    pub struct TemplatesServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> TemplatesServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> TemplatesServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            TemplatesServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Creates a Cloud Dataflow job from a template."]
        pub async fn create_job_from_template(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateJobFromTemplateRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.dataflow.v1beta3.TemplatesService/CreateJobFromTemplate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Launch a template."]
        pub async fn launch_template(
            &mut self,
            request: impl tonic::IntoRequest<super::LaunchTemplateRequest>,
        ) -> Result<tonic::Response<super::LaunchTemplateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.dataflow.v1beta3.TemplatesService/LaunchTemplate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get the template associated with a template."]
        pub async fn get_template(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTemplateRequest>,
        ) -> Result<tonic::Response<super::GetTemplateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.dataflow.v1beta3.TemplatesService/GetTemplate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod flex_templates_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Provides a service for Flex templates. This feature is not ready yet."]
    #[derive(Debug, Clone)]
    pub struct FlexTemplatesServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> FlexTemplatesServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> FlexTemplatesServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            FlexTemplatesServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Launch a job with a FlexTemplate."]
        pub async fn launch_flex_template(
            &mut self,
            request: impl tonic::IntoRequest<super::LaunchFlexTemplateRequest>,
        ) -> Result<tonic::Response<super::LaunchFlexTemplateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.dataflow.v1beta3.FlexTemplatesService/LaunchFlexTemplate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
