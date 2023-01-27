/// A notification object for notifying customers about security and privacy
/// issues.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Notification {
    /// The resource name of the notification.
    /// Format:
    /// organizations/{organization}/locations/{location}/notifications/{notification}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The subject line of the notification.
    #[prost(message, optional, tag = "2")]
    pub subject: ::core::option::Option<Subject>,
    /// A list of messages in the notification.
    #[prost(message, repeated, tag = "3")]
    pub messages: ::prost::alloc::vec::Vec<Message>,
    /// Output only. Time the notification was created.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// A text object containing the English text and its localized copies.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Text {
    /// The English copy.
    #[prost(string, tag = "1")]
    pub en_text: ::prost::alloc::string::String,
    /// The requested localized copy (if applicable).
    #[prost(string, tag = "2")]
    pub localized_text: ::prost::alloc::string::String,
    /// Status of the localization.
    #[prost(enumeration = "LocalizationState", tag = "3")]
    pub localization_state: i32,
}
/// A subject line of a notification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subject {
    /// The text content.
    #[prost(message, optional, tag = "1")]
    pub text: ::core::option::Option<Text>,
}
/// A message which contains notification details.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    /// The message content.
    #[prost(message, optional, tag = "1")]
    pub body: ::core::option::Option<message::Body>,
    /// The attachments to download.
    #[prost(message, repeated, tag = "2")]
    pub attachments: ::prost::alloc::vec::Vec<Attachment>,
    /// The Message creation timestamp.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Time when Message was localized
    #[prost(message, optional, tag = "4")]
    pub localization_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `Message`.
pub mod message {
    /// A message body containing text.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Body {
        /// The text content of the message body.
        #[prost(message, optional, tag = "1")]
        pub text: ::core::option::Option<super::Text>,
    }
}
/// Attachment with specific information about the issue.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attachment {
    /// The title of the attachment.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// Data type of the attachment.
    #[prost(oneof = "attachment::Data", tags = "2")]
    pub data: ::core::option::Option<attachment::Data>,
}
/// Nested message and enum types in `Attachment`.
pub mod attachment {
    /// Data type of the attachment.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        /// A CSV file attachment. Max size is 10 MB.
        #[prost(message, tag = "2")]
        Csv(super::Csv),
    }
}
/// A representation of a CSV file attachment, as a list of column headers and
/// a list of data rows.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Csv {
    /// The list of headers for data columns in a CSV file.
    #[prost(string, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The list of data rows in a CSV file, as string arrays rather than as a
    /// single comma-separated string.
    #[prost(message, repeated, tag = "2")]
    pub data_rows: ::prost::alloc::vec::Vec<csv::CsvRow>,
}
/// Nested message and enum types in `Csv`.
pub mod csv {
    /// A representation of a single data row in a CSV file.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CsvRow {
        /// The data entries in a CSV file row, as a string array rather than a
        /// single comma-separated string.
        #[prost(string, repeated, tag = "1")]
        pub entries: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// Request for fetching all notifications for a given parent.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNotificationsRequest {
    /// Required. The parent, which owns this collection of notifications.
    /// Must be of the form "organizations/{organization}/locations/{location}".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of notifications to return. The service may return
    /// fewer than this value. If unspecified or equal to 0, at most 50
    /// notifications will be returned. The maximum value is 50; values above 50
    /// will be coerced to 50.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token returned from a previous request.
    /// When paginating, all other parameters provided in the request
    /// must match the call that returned the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Specifies which parts of the notification resource should be returned
    /// in the response.
    #[prost(enumeration = "NotificationView", tag = "4")]
    pub view: i32,
    /// ISO code for requested localization language.  If unset, will be
    /// interpereted as "en". If the requested language is valid, but not supported
    /// for this notification, English will be returned with an "Not applicable"
    /// LocalizationState. If the ISO code is invalid (i.e. not a real language),
    /// this RPC will throw an error.
    #[prost(string, tag = "5")]
    pub language_code: ::prost::alloc::string::String,
}
/// Response of ListNotifications endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNotificationsResponse {
    /// List of notifications under a given parent.
    #[prost(message, repeated, tag = "1")]
    pub notifications: ::prost::alloc::vec::Vec<Notification>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Estimation of a total number of notifications.
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Request for fetching a notification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNotificationRequest {
    /// Required. A name of the notification to retrieve.
    /// Format:
    /// organizations/{organization}/locations/{location}/notifications/{notification}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// ISO code for requested localization language. If unset, will be
    /// interpereted as "en". If the requested language is valid, but not supported
    /// for this notification, English will be returned with an "Not applicable"
    /// LocalizationState. If the ISO code is invalid (i.e. not a real language),
    /// this RPC will throw an error.
    #[prost(string, tag = "5")]
    pub language_code: ::prost::alloc::string::String,
}
/// Notification view.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NotificationView {
    /// Not specified, equivalent to BASIC.
    Unspecified = 0,
    /// Server responses only include title, creation time and Notification ID.
    /// Note: for internal use responses also include the last update time,
    /// the latest message text and whether notification has attachments.
    Basic = 1,
    /// Include everything.
    Full = 2,
}
impl NotificationView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NotificationView::Unspecified => "NOTIFICATION_VIEW_UNSPECIFIED",
            NotificationView::Basic => "BASIC",
            NotificationView::Full => "FULL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NOTIFICATION_VIEW_UNSPECIFIED" => Some(Self::Unspecified),
            "BASIC" => Some(Self::Basic),
            "FULL" => Some(Self::Full),
            _ => None,
        }
    }
}
/// Status of localized text.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LocalizationState {
    /// Not used.
    Unspecified = 0,
    /// Localization is not applicable for requested language. This can happen
    /// when:
    /// - The requested language was not supported by Advisory Notifications at the
    /// time of localization (including notifications created before the
    /// localization feature was launched).
    /// - The requested language is English, so only the English text is returned.
    NotApplicable = 1,
    /// Localization for requested language is in progress, and not ready yet.
    Pending = 2,
    /// Localization for requested language is completed.
    Completed = 3,
}
impl LocalizationState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LocalizationState::Unspecified => "LOCALIZATION_STATE_UNSPECIFIED",
            LocalizationState::NotApplicable => "LOCALIZATION_STATE_NOT_APPLICABLE",
            LocalizationState::Pending => "LOCALIZATION_STATE_PENDING",
            LocalizationState::Completed => "LOCALIZATION_STATE_COMPLETED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LOCALIZATION_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "LOCALIZATION_STATE_NOT_APPLICABLE" => Some(Self::NotApplicable),
            "LOCALIZATION_STATE_PENDING" => Some(Self::Pending),
            "LOCALIZATION_STATE_COMPLETED" => Some(Self::Completed),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod advisory_notifications_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service to manage Security and Privacy Notifications.
    #[derive(Debug, Clone)]
    pub struct AdvisoryNotificationsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdvisoryNotificationsServiceClient<T>
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
        ) -> AdvisoryNotificationsServiceClient<InterceptedService<T, F>>
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
            AdvisoryNotificationsServiceClient::new(
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
        /// Lists notifications under a given parent.
        pub async fn list_notifications(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNotificationsRequest>,
        ) -> Result<tonic::Response<super::ListNotificationsResponse>, tonic::Status> {
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
                "/google.cloud.advisorynotifications.v1.AdvisoryNotificationsService/ListNotifications",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a notification.
        pub async fn get_notification(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNotificationRequest>,
        ) -> Result<tonic::Response<super::Notification>, tonic::Status> {
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
                "/google.cloud.advisorynotifications.v1.AdvisoryNotificationsService/GetNotification",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}