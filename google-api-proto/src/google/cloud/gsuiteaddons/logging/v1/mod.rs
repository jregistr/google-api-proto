/// JSON payload of error messages that are logged to Cloud Logging. An error
/// message (in English) is written to Cloud Logging (if not disabled) when an
/// error is encountered while using an add-on.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GSuiteAddOnsLogEntry {
    /// The deployment that caused the error. For add-ons built in Apps Script,
    /// this is the deployment ID defined by Apps Script. For add-ons built in
    /// other languages, this is the deployment ID defined in Google Cloud.
    #[prost(string, tag = "1")]
    pub deployment: ::prost::alloc::string::String,
    /// The error code and message.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// The function name that was running when the error occurred. This field
    /// might not always be set, for example, if an error happens when fetching the
    /// list of installed add-ons for a user.
    #[prost(string, tag = "3")]
    pub deployment_function: ::prost::alloc::string::String,
}
