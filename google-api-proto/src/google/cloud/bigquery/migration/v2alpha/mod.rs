/// Mapping between an input and output file to be translated in a subtask.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TranslationFileMapping {
    /// The Cloud Storage path for a file to translation in a subtask.
    #[prost(string, tag="1")]
    pub input_path: ::prost::alloc::string::String,
    /// The Cloud Storage path to write back the corresponding input file to.
    #[prost(string, tag="2")]
    pub output_path: ::prost::alloc::string::String,
}
/// The translation task config to capture necessary settings for a translation
/// task and subtask.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TranslationTaskDetails {
    /// The Cloud Storage path for translation input files.
    #[prost(string, tag="1")]
    pub input_path: ::prost::alloc::string::String,
    /// The Cloud Storage path for translation output files.
    #[prost(string, tag="2")]
    pub output_path: ::prost::alloc::string::String,
    /// Cloud Storage files to be processed for translation.
    #[prost(message, repeated, tag="12")]
    pub file_paths: ::prost::alloc::vec::Vec<TranslationFileMapping>,
    /// The Cloud Storage path to DDL files as table schema to assist semantic
    /// translation.
    #[prost(string, tag="3")]
    pub schema_path: ::prost::alloc::string::String,
    /// The file encoding type.
    #[prost(enumeration="translation_task_details::FileEncoding", tag="4")]
    pub file_encoding: i32,
    /// The settings for SQL identifiers.
    #[prost(message, optional, tag="5")]
    pub identifier_settings: ::core::option::Option<IdentifierSettings>,
    /// The map capturing special tokens to be replaced during translation. The key
    /// is special token in string. The value is the token data type. This is used
    /// to translate SQL query template which contains special token as place
    /// holder. The special token makes a query invalid to parse. This map will be
    /// applied to annotate those special token with types to let parser understand
    /// how to parse them into proper structure with type information.
    #[prost(btree_map="string, enumeration(translation_task_details::TokenType)", tag="6")]
    pub special_token_map: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, i32>,
    /// The filter applied to translation details.
    #[prost(message, optional, tag="7")]
    pub filter: ::core::option::Option<Filter>,
    /// Specifies the exact name of the bigquery table ("dataset.table") to be used
    /// for surfacing raw translation errors. If the table does not exist, we will
    /// create it. If it already exists and the schema is the same, we will re-use.
    /// If the table exists and the schema is different, we will throw an error.
    #[prost(string, tag="13")]
    pub translation_exception_table: ::prost::alloc::string::String,
    /// The language specific settings for the translation task.
    #[prost(oneof="translation_task_details::LanguageOptions", tags="10, 11")]
    pub language_options: ::core::option::Option<translation_task_details::LanguageOptions>,
}
/// Nested message and enum types in `TranslationTaskDetails`.
pub mod translation_task_details {
    /// The file encoding types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FileEncoding {
        /// File encoding setting is not specified.
        Unspecified = 0,
        /// File encoding is UTF_8.
        Utf8 = 1,
        /// File encoding is ISO_8859_1.
        Iso88591 = 2,
        /// File encoding is US_ASCII.
        UsAscii = 3,
        /// File encoding is UTF_16.
        Utf16 = 4,
        /// File encoding is UTF_16LE.
        Utf16le = 5,
        /// File encoding is UTF_16BE.
        Utf16be = 6,
    }
    /// The special token data type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TokenType {
        /// Token type is not specified.
        Unspecified = 0,
        /// Token type as string.
        String = 1,
        /// Token type as integer.
        Int64 = 2,
        /// Token type as numeric.
        Numeric = 3,
        /// Token type as boolean.
        Bool = 4,
        /// Token type as float.
        Float64 = 5,
        /// Token type as date.
        Date = 6,
        /// Token type as timestamp.
        Timestamp = 7,
    }
    /// The language specific settings for the translation task.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LanguageOptions {
        /// The Teradata SQL specific settings for the translation task.
        #[prost(message, tag="10")]
        TeradataOptions(super::TeradataOptions),
        /// The BTEQ specific settings for the translation task.
        #[prost(message, tag="11")]
        BteqOptions(super::BteqOptions),
    }
}
/// The filter applied to fields of translation details.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Filter {
    /// The list of prefixes used to exclude processing for input files.
    #[prost(string, repeated, tag="1")]
    pub input_file_exclusion_prefixes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Settings related to SQL identifiers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentifierSettings {
    /// The setting to control output queries' identifier case.
    #[prost(enumeration="identifier_settings::IdentifierCase", tag="1")]
    pub output_identifier_case: i32,
    /// Specifies the rewrite mode for SQL identifiers.
    #[prost(enumeration="identifier_settings::IdentifierRewriteMode", tag="2")]
    pub identifier_rewrite_mode: i32,
}
/// Nested message and enum types in `IdentifierSettings`.
pub mod identifier_settings {
    /// The identifier case type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum IdentifierCase {
        /// The identifier case is not specified.
        Unspecified = 0,
        /// Identifiers' cases will be kept as the original cases.
        Original = 1,
        /// Identifiers will be in upper cases.
        Upper = 2,
        /// Identifiers will be in lower cases.
        Lower = 3,
    }
    /// The SQL identifier rewrite mode.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum IdentifierRewriteMode {
        /// SQL Identifier rewrite mode is unspecified.
        Unspecified = 0,
        /// SQL identifiers won't be rewrite.
        None = 1,
        /// All SQL identifiers will be rewrite.
        RewriteAll = 2,
    }
}
/// Teradata SQL specific translation task related settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TeradataOptions {
}
/// BTEQ translation task related settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BteqOptions {
    /// Specifies the project and dataset in BigQuery that will be used for
    /// external table creation during the translation.
    #[prost(message, optional, tag="1")]
    pub project_dataset: ::core::option::Option<DatasetReference>,
    /// The Cloud Storage location to be used as the default path for files that
    /// are not otherwise specified in the file replacement map.
    #[prost(string, tag="2")]
    pub default_path_uri: ::prost::alloc::string::String,
    /// Maps the local paths that are used in BTEQ scripts (the keys) to the paths
    /// in Cloud Storage that should be used in their stead in the translation (the
    /// value).
    #[prost(btree_map="string, string", tag="3")]
    pub file_replacement_map: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Reference to a BigQuery dataset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatasetReference {
    /// A unique ID for this dataset, without the project name. The ID
    /// must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_).
    /// The maximum length is 1,024 characters.
    #[prost(string, tag="1")]
    pub dataset_id: ::prost::alloc::string::String,
    /// The ID of the project containing this dataset.
    #[prost(string, tag="2")]
    pub project_id: ::prost::alloc::string::String,
}
/// Assessment task config.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssessmentTaskDetails {
    /// Required. The Cloud Storage path for assessment input files.
    #[prost(string, tag="1")]
    pub input_path: ::prost::alloc::string::String,
    /// Required. The BigQuery dataset for output.
    #[prost(string, tag="2")]
    pub output_dataset: ::prost::alloc::string::String,
    /// Optional. An optional Cloud Storage path to write the query logs (which is
    /// then used as an input path on the translation task)
    #[prost(string, tag="3")]
    pub querylogs_path: ::prost::alloc::string::String,
    /// Required. The data source or data warehouse type (eg: TERADATA/REDSHIFT)
    /// from which the input data is extracted.
    #[prost(string, tag="4")]
    pub data_source: ::prost::alloc::string::String,
}
/// Details for an assessment task orchestration result.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssessmentOrchestrationResultDetails {
    /// Optional. The version used for the output table schemas.
    #[prost(string, tag="1")]
    pub output_tables_schema_version: ::prost::alloc::string::String,
}
/// Provides details for errors and the corresponding resources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceErrorDetail {
    /// Required. Information about the resource where the error is located.
    #[prost(message, optional, tag="1")]
    pub resource_info: ::core::option::Option<super::super::super::super::rpc::ResourceInfo>,
    /// Required. The error details for the resource.
    #[prost(message, repeated, tag="2")]
    pub error_details: ::prost::alloc::vec::Vec<ErrorDetail>,
    /// Required. How many errors there are in total for the resource. Truncation can be
    /// indicated by having an `error_count` that is higher than the size of
    /// `error_details`.
    #[prost(int32, tag="3")]
    pub error_count: i32,
}
/// Provides details for errors, e.g. issues that where encountered when
/// processing a subtask.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorDetail {
    /// Optional. The exact location within the resource (if applicable).
    #[prost(message, optional, tag="1")]
    pub location: ::core::option::Option<ErrorLocation>,
    /// Required. Describes the cause of the error with structured detail.
    #[prost(message, optional, tag="2")]
    pub error_info: ::core::option::Option<super::super::super::super::rpc::ErrorInfo>,
}
/// Holds information about where the error is located.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorLocation {
    /// Optional. If applicable, denotes the line where the error occurred. A zero value
    /// means that there is no line information.
    #[prost(int32, tag="1")]
    pub line: i32,
    /// Optional. If applicable, denotes the column where the error occurred. A zero value
    /// means that there is no columns information.
    #[prost(int32, tag="2")]
    pub column: i32,
}
/// The metrics object for a SubTask.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeSeries {
    /// Required. The name of the metric.
    ///
    /// If the metric is not known by the service yet, it will be auto-created.
    #[prost(string, tag="1")]
    pub metric: ::prost::alloc::string::String,
    /// Required. The value type of the time series.
    #[prost(enumeration="super::super::super::super::api::metric_descriptor::ValueType", tag="2")]
    pub value_type: i32,
    /// Optional. The metric kind of the time series.
    ///
    /// If present, it must be the same as the metric kind of the associated
    /// metric. If the associated metric's descriptor must be auto-created, then
    /// this field specifies the metric kind of the new descriptor and must be
    /// either `GAUGE` (the default) or `CUMULATIVE`.
    #[prost(enumeration="super::super::super::super::api::metric_descriptor::MetricKind", tag="3")]
    pub metric_kind: i32,
    /// Required. The data points of this time series. When listing time series, points are
    /// returned in reverse time order.
    ///
    /// When creating a time series, this field must contain exactly one point and
    /// the point's type must be the same as the value type of the associated
    /// metric. If the associated metric's descriptor must be auto-created, then
    /// the value type of the descriptor is determined by the point's type, which
    /// must be `BOOL`, `INT64`, `DOUBLE`, or `DISTRIBUTION`.
    #[prost(message, repeated, tag="4")]
    pub points: ::prost::alloc::vec::Vec<Point>,
}
/// A single data point in a time series.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Point {
    /// The time interval to which the data point applies.  For `GAUGE` metrics,
    /// the start time does not need to be supplied, but if it is supplied, it must
    /// equal the end time.  For `DELTA` metrics, the start and end time should
    /// specify a non-zero interval, with subsequent points specifying contiguous
    /// and non-overlapping intervals.  For `CUMULATIVE` metrics, the start and end
    /// time should specify a non-zero interval, with subsequent points specifying
    /// the same start time and increasing end times, until an event resets the
    /// cumulative value to zero and sets a new start time for the following
    /// points.
    #[prost(message, optional, tag="1")]
    pub interval: ::core::option::Option<TimeInterval>,
    /// The value of the data point.
    #[prost(message, optional, tag="2")]
    pub value: ::core::option::Option<TypedValue>,
}
/// A time interval extending just after a start time through an end time.
/// If the start time is the same as the end time, then the interval
/// represents a single point in time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeInterval {
    /// Optional. The beginning of the time interval.  The default value
    /// for the start time is the end time. The start time must not be
    /// later than the end time.
    #[prost(message, optional, tag="1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. The end of the time interval.
    #[prost(message, optional, tag="2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// A single strongly-typed value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypedValue {
    /// The typed value field.
    #[prost(oneof="typed_value::Value", tags="1, 2, 3, 4, 5")]
    pub value: ::core::option::Option<typed_value::Value>,
}
/// Nested message and enum types in `TypedValue`.
pub mod typed_value {
    /// The typed value field.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// A Boolean value: `true` or `false`.
        #[prost(bool, tag="1")]
        BoolValue(bool),
        /// A 64-bit integer. Its range is approximately +/-9.2x10^18.
        #[prost(int64, tag="2")]
        Int64Value(i64),
        /// A 64-bit double-precision floating-point number. Its magnitude
        /// is approximately +/-10^(+/-300) and it has 16 significant digits of
        /// precision.
        #[prost(double, tag="3")]
        DoubleValue(f64),
        /// A variable-length string value.
        #[prost(string, tag="4")]
        StringValue(::prost::alloc::string::String),
        /// A distribution value.
        #[prost(message, tag="5")]
        DistributionValue(super::super::super::super::super::api::Distribution),
    }
}
/// A migration workflow which specifies what needs to be done for an EDW
/// migration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrationWorkflow {
    /// Output only. Immutable. The unique identifier for the migration workflow. The ID is
    /// server-generated.
    ///
    /// Example: `projects/123/locations/us/workflows/345`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The display name of the workflow. This can be set to give a workflow
    /// a descriptive name. There is no guarantee or enforcement of uniqueness.
    #[prost(string, tag="6")]
    pub display_name: ::prost::alloc::string::String,
    /// The tasks in a workflow in a named map. The name (i.e. key) has no
    /// meaning and is merely a convenient way to address a specific task
    /// in a workflow.
    #[prost(btree_map="string, message", tag="2")]
    pub tasks: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, MigrationTask>,
    /// Output only. That status of the workflow.
    #[prost(enumeration="migration_workflow::State", tag="3")]
    pub state: i32,
    /// Time when the workflow was created.
    #[prost(message, optional, tag="4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Time when the workflow was last updated.
    #[prost(message, optional, tag="5")]
    pub last_update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `MigrationWorkflow`.
pub mod migration_workflow {
    /// Possible migration workflow states.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Workflow state is unspecified.
        Unspecified = 0,
        /// Workflow is in draft status, i.e. tasks are not yet eligible for
        /// execution.
        Draft = 1,
        /// Workflow is running (i.e. tasks are eligible for execution).
        Running = 2,
        /// Workflow is paused. Tasks currently in progress may continue, but no
        /// further tasks will be scheduled.
        Paused = 3,
        /// Workflow is complete. There should not be any task in a non-terminal
        /// state, but if they are (e.g. forced termination), they will not be
        /// scheduled.
        Completed = 4,
    }
}
/// A single task for a migration which has details about the configuration of
/// the task.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrationTask {
    /// Output only. Immutable. The unique identifier for the migration task. The ID is server-generated.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The type of the task. This must be a supported task type.
    #[prost(string, tag="2")]
    pub r#type: ::prost::alloc::string::String,
    /// DEPRECATED! Use one of the task_details below.
    /// The details of the task. The type URL must be one of the supported task
    /// details messages and correspond to the Task's type.
    #[prost(message, optional, tag="3")]
    pub details: ::core::option::Option<::prost_types::Any>,
    /// Output only. The current state of the task.
    #[prost(enumeration="migration_task::State", tag="4")]
    pub state: i32,
    /// Output only. An explanation that may be populated when the task is in FAILED state.
    #[prost(message, optional, tag="5")]
    pub processing_error: ::core::option::Option<super::super::super::super::rpc::ErrorInfo>,
    /// Time when the task was created.
    #[prost(message, optional, tag="6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Time when the task was last updated.
    #[prost(message, optional, tag="7")]
    pub last_update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Additional information about the orchestration.
    #[prost(message, optional, tag="10")]
    pub orchestration_result: ::core::option::Option<MigrationTaskOrchestrationResult>,
    /// The details of the task.
    #[prost(oneof="migration_task::TaskDetails", tags="12, 13")]
    pub task_details: ::core::option::Option<migration_task::TaskDetails>,
}
/// Nested message and enum types in `MigrationTask`.
pub mod migration_task {
    /// Possible states of a migration task.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The state is unspecified.
        Unspecified = 0,
        /// The task is waiting for orchestration.
        Pending = 1,
        /// The task is assigned to an orchestrator.
        Orchestrating = 2,
        /// The task is running, i.e. its subtasks are ready for execution.
        Running = 3,
        /// Tha task is paused. Assigned subtasks can continue, but no new subtasks
        /// will be scheduled.
        Paused = 4,
        /// The task finished successfully.
        Succeeded = 5,
        /// The task finished unsuccessfully.
        Failed = 6,
    }
    /// The details of the task.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TaskDetails {
        /// Task configuration for Assessment.
        #[prost(message, tag="12")]
        AssessmentTaskDetails(super::AssessmentTaskDetails),
        /// Task configuration for Batch/Offline SQL Translation.
        #[prost(message, tag="13")]
        TranslationTaskDetails(super::TranslationTaskDetails),
    }
}
/// A subtask for a migration which carries details about the configuration of
/// the subtask. The content of the details should not matter to the end user,
/// but is a contract between the subtask creator and subtask worker.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrationSubtask {
    /// Output only. Immutable. The resource name for the migration subtask. The ID is
    /// server-generated.
    ///
    /// Example: `projects/123/locations/us/workflows/345/subtasks/678`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The unique ID of the task to which this subtask belongs.
    #[prost(string, tag="2")]
    pub task_id: ::prost::alloc::string::String,
    /// The type of the Subtask. The migration service does not check whether this
    /// is a known type. It is up to the task creator (i.e. orchestrator or worker)
    /// to ensure it only creates subtasks for which there are compatible workers
    /// polling for Subtasks.
    #[prost(string, tag="3")]
    pub r#type: ::prost::alloc::string::String,
    /// Output only. The current state of the subtask.
    #[prost(enumeration="migration_subtask::State", tag="5")]
    pub state: i32,
    /// Output only. An explanation that may be populated when the task is in FAILED state.
    #[prost(message, optional, tag="6")]
    pub processing_error: ::core::option::Option<super::super::super::super::rpc::ErrorInfo>,
    /// Output only. Provides details to errors and issues encountered while processing the
    /// subtask. Presence of error details does not mean that the subtask failed.
    #[prost(message, repeated, tag="12")]
    pub resource_error_details: ::prost::alloc::vec::Vec<ResourceErrorDetail>,
    /// The number or resources with errors. Note: This is not the total
    /// number of errors as each resource can have more than one error.
    /// This is used to indicate truncation by having a `resource_error_count`
    /// that is higher than the size of `resource_error_details`.
    #[prost(int32, tag="13")]
    pub resource_error_count: i32,
    /// Time when the subtask was created.
    #[prost(message, optional, tag="7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Time when the subtask was last updated.
    #[prost(message, optional, tag="8")]
    pub last_update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The metrics for the subtask.
    #[prost(message, repeated, tag="11")]
    pub metrics: ::prost::alloc::vec::Vec<TimeSeries>,
}
/// Nested message and enum types in `MigrationSubtask`.
pub mod migration_subtask {
    /// Possible states of a migration subtask.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The state is unspecified.
        Unspecified = 0,
        /// The subtask is ready, i.e. it is ready for execution.
        Active = 1,
        /// The subtask is running, i.e. it is assigned to a worker for execution.
        Running = 2,
        /// The subtask finished successfully.
        Succeeded = 3,
        /// The subtask finished unsuccessfully.
        Failed = 4,
        /// The subtask is paused, i.e., it will not be scheduled. If it was already
        /// assigned,it might still finish but no new lease renewals will be granted.
        Paused = 5,
    }
}
/// Additional information from the orchestrator when it is done with the
/// task orchestration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrationTaskOrchestrationResult {
    /// Details specific to the task type.
    #[prost(oneof="migration_task_orchestration_result::Details", tags="1")]
    pub details: ::core::option::Option<migration_task_orchestration_result::Details>,
}
/// Nested message and enum types in `MigrationTaskOrchestrationResult`.
pub mod migration_task_orchestration_result {
    /// Details specific to the task type.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Details {
        /// Details specific to assessment task types.
        #[prost(message, tag="1")]
        AssessmentDetails(super::AssessmentOrchestrationResultDetails),
    }
}
/// Request to create a migration workflow resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMigrationWorkflowRequest {
    /// Required. The name of the project to which this migration workflow belongs.
    /// Example: `projects/foo/locations/bar`
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The migration workflow to create.
    #[prost(message, optional, tag="2")]
    pub migration_workflow: ::core::option::Option<MigrationWorkflow>,
}
/// A request to get a previously created migration workflow.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMigrationWorkflowRequest {
    /// Required. The unique identifier for the migration workflow.
    /// Example: `projects/123/locations/us/workflows/1234`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The list of fields to be retrieved.
    #[prost(message, optional, tag="2")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// A request to list previously created migration workflows.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMigrationWorkflowsRequest {
    /// Required. The project and location of the migration workflows to list.
    /// Example: `projects/123/locations/us`
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The list of fields to be retrieved.
    #[prost(message, optional, tag="2")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The maximum number of migration workflows to return. The service may return
    /// fewer than this number.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    /// A page token, received from previous `ListMigrationWorkflows` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListMigrationWorkflows`
    /// must match the call that provided the page token.
    #[prost(string, tag="4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response object for a `ListMigrationWorkflows` call.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMigrationWorkflowsResponse {
    /// The migration workflows for the specified project / location.
    #[prost(message, repeated, tag="1")]
    pub migration_workflows: ::prost::alloc::vec::Vec<MigrationWorkflow>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A request to delete a previously created migration workflow.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMigrationWorkflowRequest {
    /// Required. The unique identifier for the migration workflow.
    /// Example: `projects/123/locations/us/workflows/1234`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// A request to start a previously created migration workflow.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartMigrationWorkflowRequest {
    /// Required. The unique identifier for the migration workflow.
    /// Example: `projects/123/locations/us/workflows/1234`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// A request to get a previously created migration subtasks.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMigrationSubtaskRequest {
    /// Required. The unique identifier for the migration subtask.
    /// Example: `projects/123/locations/us/workflows/1234/subtasks/543`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The list of fields to be retrieved.
    #[prost(message, optional, tag="2")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// A request to list previously created migration subtasks.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMigrationSubtasksRequest {
    /// Required. The migration task of the subtasks to list.
    /// Example: `projects/123/locations/us/workflows/1234`
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The list of fields to be retrieved.
    #[prost(message, optional, tag="2")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Optional. The maximum number of migration tasks to return. The service may return
    /// fewer than this number.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    /// Optional. A page token, received from previous `ListMigrationSubtasks` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListMigrationSubtasks`
    /// must match the call that provided the page token.
    #[prost(string, tag="4")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The filter to apply. This can be used to get the subtasks of a specific
    /// tasks in a workflow, e.g. `migration_task = "ab012"` where `"ab012"` is the
    /// task ID (not the name in the named map).
    #[prost(string, tag="5")]
    pub filter: ::prost::alloc::string::String,
}
/// Response object for a `ListMigrationSubtasks` call.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMigrationSubtasksResponse {
    /// The migration subtasks for the specified task.
    #[prost(message, repeated, tag="1")]
    pub migration_subtasks: ::prost::alloc::vec::Vec<MigrationSubtask>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod migration_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Service to handle EDW migrations.
    #[derive(Debug, Clone)]
    pub struct MigrationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> MigrationServiceClient<T>
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
        ) -> MigrationServiceClient<InterceptedService<T, F>>
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
            MigrationServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a migration workflow.
        pub async fn create_migration_workflow(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateMigrationWorkflowRequest>,
        ) -> Result<tonic::Response<super::MigrationWorkflow>, tonic::Status> {
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
                "/google.cloud.bigquery.migration.v2alpha.MigrationService/CreateMigrationWorkflow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a previously created migration workflow.
        pub async fn get_migration_workflow(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMigrationWorkflowRequest>,
        ) -> Result<tonic::Response<super::MigrationWorkflow>, tonic::Status> {
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
                "/google.cloud.bigquery.migration.v2alpha.MigrationService/GetMigrationWorkflow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists previously created migration workflow.
        pub async fn list_migration_workflows(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMigrationWorkflowsRequest>,
        ) -> Result<
            tonic::Response<super::ListMigrationWorkflowsResponse>,
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
                "/google.cloud.bigquery.migration.v2alpha.MigrationService/ListMigrationWorkflows",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a migration workflow by name.
        pub async fn delete_migration_workflow(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteMigrationWorkflowRequest>,
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
                "/google.cloud.bigquery.migration.v2alpha.MigrationService/DeleteMigrationWorkflow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Starts a previously created migration workflow. I.e., the state transitions
        /// from DRAFT to RUNNING. This is a no-op if the state is already RUNNING.
        /// An error will be signaled if the state is anything other than DRAFT or
        /// RUNNING.
        pub async fn start_migration_workflow(
            &mut self,
            request: impl tonic::IntoRequest<super::StartMigrationWorkflowRequest>,
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
                "/google.cloud.bigquery.migration.v2alpha.MigrationService/StartMigrationWorkflow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a previously created migration subtask.
        pub async fn get_migration_subtask(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMigrationSubtaskRequest>,
        ) -> Result<tonic::Response<super::MigrationSubtask>, tonic::Status> {
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
                "/google.cloud.bigquery.migration.v2alpha.MigrationService/GetMigrationSubtask",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists previously created migration subtasks.
        pub async fn list_migration_subtasks(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMigrationSubtasksRequest>,
        ) -> Result<
            tonic::Response<super::ListMigrationSubtasksResponse>,
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
                "/google.cloud.bigquery.migration.v2alpha.MigrationService/ListMigrationSubtasks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// The request of translating a SQL query to Standard SQL.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TranslateQueryRequest {
    /// Required. The name of the project to which this translation request belongs.
    /// Example: `projects/foo/locations/bar`
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The source SQL dialect of `queries`.
    #[prost(enumeration="translate_query_request::SqlTranslationSourceDialect", tag="2")]
    pub source_dialect: i32,
    /// Required. The query to be translated.
    #[prost(string, tag="3")]
    pub query: ::prost::alloc::string::String,
}
/// Nested message and enum types in `TranslateQueryRequest`.
pub mod translate_query_request {
    /// Supported SQL translation source dialects.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SqlTranslationSourceDialect {
        /// SqlTranslationSourceDialect not specified.
        Unspecified = 0,
        /// Teradata SQL.
        Teradata = 1,
    }
}
/// The response of translating a SQL query to Standard SQL.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TranslateQueryResponse {
    /// Output only. Immutable. The unique identifier for the SQL translation job.
    /// Example: `projects/123/locations/us/translation/1234`
    #[prost(string, tag="4")]
    pub translation_job: ::prost::alloc::string::String,
    /// The translated result. This will be empty if the translation fails.
    #[prost(string, tag="1")]
    pub translated_query: ::prost::alloc::string::String,
    /// The list of errors encountered during the translation, if present.
    #[prost(message, repeated, tag="2")]
    pub errors: ::prost::alloc::vec::Vec<SqlTranslationError>,
    /// The list of warnings encountered during the translation, if present,
    /// indicates non-semantically correct translation.
    #[prost(message, repeated, tag="3")]
    pub warnings: ::prost::alloc::vec::Vec<SqlTranslationWarning>,
}
/// Structured error object capturing the error message and the location in the
/// source text where the error occurs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlTranslationErrorDetail {
    /// Specifies the row from the source text where the error occurred.
    #[prost(int64, tag="1")]
    pub row: i64,
    /// Specifie the column from the source texts where the error occurred.
    #[prost(int64, tag="2")]
    pub column: i64,
    /// A human-readable description of the error.
    #[prost(string, tag="3")]
    pub message: ::prost::alloc::string::String,
}
/// The detailed error object if the SQL translation job fails.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlTranslationError {
    /// The type of SQL translation error.
    #[prost(enumeration="sql_translation_error::SqlTranslationErrorType", tag="1")]
    pub error_type: i32,
    /// Specifies the details of the error, including the error message and
    /// location from the source text.
    #[prost(message, optional, tag="2")]
    pub error_detail: ::core::option::Option<SqlTranslationErrorDetail>,
}
/// Nested message and enum types in `SqlTranslationError`.
pub mod sql_translation_error {
    /// The error type of the SQL translation job.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SqlTranslationErrorType {
        /// SqlTranslationErrorType not specified.
        Unspecified = 0,
        /// Failed to parse the input text as a SQL query.
        SqlParseError = 1,
        /// Found unsupported functions in the input SQL query that are not able to
        /// translate.
        UnsupportedSqlFunction = 2,
    }
}
/// The detailed warning object if the SQL translation job is completed but not
/// semantically correct.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlTranslationWarning {
    /// Specifies the details of the warning, including the warning message and
    /// location from the source text.
    #[prost(message, optional, tag="1")]
    pub warning_detail: ::core::option::Option<SqlTranslationErrorDetail>,
}
/// Generated client implementations.
pub mod sql_translation_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Provides other SQL dialects to GoogleSQL translation operations.
    #[derive(Debug, Clone)]
    pub struct SqlTranslationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SqlTranslationServiceClient<T>
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
        ) -> SqlTranslationServiceClient<InterceptedService<T, F>>
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
            SqlTranslationServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Translates input queries from source dialects to GoogleSQL.
        pub async fn translate_query(
            &mut self,
            request: impl tonic::IntoRequest<super::TranslateQueryRequest>,
        ) -> Result<tonic::Response<super::TranslateQueryResponse>, tonic::Status> {
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
                "/google.cloud.bigquery.migration.v2alpha.SqlTranslationService/TranslateQuery",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
