/// A log entry for a DICOM import long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDicomLogEntry {
    /// The source file, in the format `gs://{bucket-id}/{path/to/file}`.
    #[prost(string, tag = "1")]
    pub source: ::prost::alloc::string::String,
    /// The error code and message.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
}
/// A log entry for a DICOM export long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportDicomLogEntry {
    /// The DICOM resource being exported.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The error code and message.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
}
/// A log entry for a DICOM store Pub/Sub notification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DicomNotificationLogEntry {
    /// The DICOM resource being created.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The Pub/Sub topic that the notification is published on.
    #[prost(string, tag = "2")]
    pub pubsub_topic: ::prost::alloc::string::String,
    /// The error code and message.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
}
/// A log entry for a DICOM streaming export notification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DicomStreamLogEntry {
    /// The DICOM resource being exported.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The destination in BigQuery, in the format
    /// `bq://{projectId}.{bqDatasetId}.{bqTableId}`.
    #[prost(string, tag = "2")]
    pub destination: ::prost::alloc::string::String,
    /// The error code and message.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
}
/// A log entry for a Consent store QueryAccessibleData long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccessibleDataLogEntry {
    /// The resource being processed.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The error code and message.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
}
/// A log entry for a User Data Mapping indexing notification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsentUserDataMappingLogEntry {
    /// The User Data Mapping being indexed (for example,
    /// `projects/{projectId}/locations/{locationId}/datasets/{datasetId}/consentStores/{storeId}/userDataMappings/{messageId}`).
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The error code and message.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
}
/// A log entry for a FHIR import long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportFhirLogEntry {
    /// The source in Cloud Storage (for example,
    /// `gs://{bucket_id}/{path/to/file}`) or BigQuery (for example,
    /// `bq://{projectId}.{bqDatasetId}`).
    #[prost(string, tag = "1")]
    pub source: ::prost::alloc::string::String,
    /// The ID in the source file of the FHIR resource being imported.
    #[prost(string, tag = "2")]
    pub resource_id: ::prost::alloc::string::String,
    /// The error code and message.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
}
/// A log entry for a FHIR export long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportFhirLogEntry {
    /// The destination in Cloud Storage (for example,
    /// `gs://{bucket_id}/{path/to/file}`) or BigQuery (for example,
    /// `bq://{projectId}.{bqDatasetId}`).
    #[prost(string, tag = "1")]
    pub destination: ::prost::alloc::string::String,
    /// The resource being exported (e.g.
    /// `projects/{projectId}/locations/{locationId}/datasets/{datasetId}/fhirStores/{fhirStoreId}/fhir/Patient/{patientId}`).
    #[prost(string, tag = "3")]
    pub resource_name: ::prost::alloc::string::String,
    /// The error code and message.
    #[prost(message, optional, tag = "4")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
}
/// A log entry for a FHIR configure search long-running operation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FhirConfigureSearchLogEntry {
    /// The ID of the resource being reindexed.
    #[prost(string, tag = "1")]
    pub resource_id: ::prost::alloc::string::String,
    /// The error code and message.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
}
/// A log entry for a FHIR store Pub/Sub notification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FhirNotificationLogEntry {
    /// The resource being changed (for example,
    /// `projects/{projectId}/locations/{locationId}/datasets/{datasetId}/fhirStores/{fhirStoreId}/fhir/Patient/{patientId}`).
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The Pub/Sub topic that the notification is published on.
    #[prost(string, tag = "2")]
    pub pubsub_topic: ::prost::alloc::string::String,
    /// The error code and message.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
}
/// A log entry for a FHIR streaming export notification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FhirStreamLogEntry {
    /// The resource being changed (for example,
    /// `projects/{projectId}/locations/{locationId}/datasets/{datasetId}/fhirStores/{fhirStoreId}/fhir/Patient/{patientId}`).
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The destination in BigQuery (for example,
    /// `bq://{projectId}.{bqDatasetId}.{bqTableId}`).
    #[prost(string, tag = "2")]
    pub destination: ::prost::alloc::string::String,
    /// The error code and message.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
}
/// A log entry for a FHIR streaming deidentification notification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FhirDeidentifyStreamToStoreLogEntry {
    /// The resource that changed (for example,
    /// `projects/{projectId}/locations/{locationId}/datasets/{datasetId}/fhirStores/{fhirStoreId}/fhir/Patient/{patientId}`).
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The destination FHIR store name. (for example,
    /// `projects/{projectId}/locations/{locationId}/datasets/{datasetId}/fhirStores/{fhirStoreId}`).
    #[prost(string, tag = "2")]
    pub destination: ::prost::alloc::string::String,
    /// The error code and message.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
}
/// A log entry for a HL7v2 import long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportHl7V2LogEntry {
    /// The source in Cloud Storage (for example,
    /// `gs://{bucket_id}/{path/to/file}`).
    #[prost(string, tag = "1")]
    pub source: ::prost::alloc::string::String,
    /// The error code and message.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
}
/// A log entry for a HL7v2 store Pub/Sub notification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hl7V2NotificationLogEntry {
    /// The HL7v2 message being created (for example,
    /// `projects/{projectId}/locations/{locationId}/datasets/{datasetId}/hl7V2Stores/{hl7v2StoreId}/messages/{hl7v2MessageId}`).
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The Pub/Sub topic that the notification is published on.
    #[prost(string, tag = "2")]
    pub pubsub_topic: ::prost::alloc::string::String,
    /// The error code and message.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
}
/// A log entry for a de-identification long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeidentifyLogEntry {
    /// The resource being de-identified.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The error code and message.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
}
/// A log entry for an Annotation import long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportAnnotationLogEntry {
    /// The source in Cloud Storage. For example,
    /// `gs://{bucket_id}/{path/to/file}`.
    #[prost(string, tag = "1")]
    pub source: ::prost::alloc::string::String,
    /// The error code and message.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
}
/// A log entry for an Annotation export long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportAnnotationLogEntry {
    /// The destination in Cloud Storage or BigQuery.
    #[prost(string, tag = "1")]
    pub destination: ::prost::alloc::string::String,
    /// The annotation record being exported. For example:
    /// `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/annotationStores/{store_id}/annotations/{annotation_id}`.
    #[prost(string, tag = "2")]
    pub annotation_name: ::prost::alloc::string::String,
    /// The error code and message.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
}
/// A log entry for an Annotation evaluate long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluateAnnotationLogEntry {
    /// The report destination in BigQuery.
    #[prost(string, tag = "1")]
    pub destination: ::prost::alloc::string::String,
    /// The eval annotation record being evaluated. For example:
    /// `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/annotationStores/{eval_store_id}/annotations/{eval_annotation_id}`.
    #[prost(string, tag = "2")]
    pub eval_annotation_name: ::prost::alloc::string::String,
    /// The golden annotation record being evaluated. For example:
    /// `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/annotationStores/{golden_store_id}/annotations/{golden_annotation_id}`.
    #[prost(string, tag = "3")]
    pub golden_annotation_name: ::prost::alloc::string::String,
    /// The error code and message.
    #[prost(message, optional, tag = "4")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
}
