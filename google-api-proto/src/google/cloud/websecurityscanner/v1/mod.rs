/// A CrawledUrl resource represents a URL that was crawled during a ScanRun. Web
/// Security Scanner Service crawls the web applications, following all links
/// within the scope of sites, to find the URLs to test against.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrawledUrl {
    /// Output only. The http method of the request that was used to visit the URL, in
    /// uppercase.
    #[prost(string, tag="1")]
    pub http_method: ::prost::alloc::string::String,
    /// Output only. The URL that was crawled.
    #[prost(string, tag="2")]
    pub url: ::prost::alloc::string::String,
    /// Output only. The body of the request that was used to visit the URL.
    #[prost(string, tag="3")]
    pub body: ::prost::alloc::string::String,
}
/// Output only.
/// Defines a warning trace message for ScanRun. Warning traces provide customers
/// with useful information that helps make the scanning process more effective.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScanRunWarningTrace {
    /// Output only. Indicates the warning code.
    #[prost(enumeration="scan_run_warning_trace::Code", tag="1")]
    pub code: i32,
}
/// Nested message and enum types in `ScanRunWarningTrace`.
pub mod scan_run_warning_trace {
    /// Output only.
    /// Defines a warning message code.
    /// Next id: 6
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Code {
        /// Default value is never used.
        Unspecified = 0,
        /// Indicates that a scan discovered an unexpectedly low number of URLs. This
        /// is sometimes caused by complex navigation features or by using a single
        /// URL for numerous pages.
        InsufficientCrawlResults = 1,
        /// Indicates that a scan discovered too many URLs to test, or excessive
        /// redundant URLs.
        TooManyCrawlResults = 2,
        /// Indicates that too many tests have been generated for the scan. Customer
        /// should try reducing the number of starting URLs, increasing the QPS rate,
        /// or narrowing down the scope of the scan using the excluded patterns.
        TooManyFuzzTasks = 3,
        /// Indicates that a scan is blocked by IAP.
        BlockedByIap = 4,
        /// Indicates that no seeds is found for a scan
        NoStartingUrlFoundForManagedScan = 5,
    }
}
/// ! Information about a vulnerability with an HTML.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Form {
    /// ! The URI where to send the form when it's submitted.
    #[prost(string, tag="1")]
    pub action_uri: ::prost::alloc::string::String,
    /// ! The names of form fields related to the vulnerability.
    #[prost(string, repeated, tag="2")]
    pub fields: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Information reported for an outdated library.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutdatedLibrary {
    /// The name of the outdated library.
    #[prost(string, tag="1")]
    pub library_name: ::prost::alloc::string::String,
    /// The version number.
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
    /// URLs to learn more information about the vulnerabilities in the library.
    #[prost(string, repeated, tag="3")]
    pub learn_more_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Information regarding any resource causing the vulnerability such
/// as JavaScript sources, image, audio files, etc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViolatingResource {
    /// The MIME type of this resource.
    #[prost(string, tag="1")]
    pub content_type: ::prost::alloc::string::String,
    /// URL of this violating resource.
    #[prost(string, tag="2")]
    pub resource_url: ::prost::alloc::string::String,
}
/// Information about vulnerable request parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VulnerableParameters {
    /// The vulnerable parameter names.
    #[prost(string, repeated, tag="1")]
    pub parameter_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Information about vulnerable or missing HTTP Headers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VulnerableHeaders {
    /// List of vulnerable headers.
    #[prost(message, repeated, tag="1")]
    pub headers: ::prost::alloc::vec::Vec<vulnerable_headers::Header>,
    /// List of missing headers.
    #[prost(message, repeated, tag="2")]
    pub missing_headers: ::prost::alloc::vec::Vec<vulnerable_headers::Header>,
}
/// Nested message and enum types in `VulnerableHeaders`.
pub mod vulnerable_headers {
    /// Describes a HTTP Header.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Header {
        /// Header name.
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        /// Header value.
        #[prost(string, tag="2")]
        pub value: ::prost::alloc::string::String,
    }
}
/// Information reported for an XSS.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Xss {
    /// Stack traces leading to the point where the XSS occurred.
    #[prost(string, repeated, tag="1")]
    pub stack_traces: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// An error message generated by a javascript breakage.
    #[prost(string, tag="2")]
    pub error_message: ::prost::alloc::string::String,
    /// The attack vector of the payload triggering this XSS.
    #[prost(enumeration="xss::AttackVector", tag="3")]
    pub attack_vector: i32,
    /// The reproduction url for the seeding POST request of a Stored XSS.
    #[prost(string, tag="4")]
    pub stored_xss_seeding_url: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Xss`.
pub mod xss {
    /// Types of XSS attack vector.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AttackVector {
        /// Unknown attack vector.
        Unspecified = 0,
        /// The attack comes from fuzzing the browser's localStorage.
        LocalStorage = 1,
        /// The attack comes from fuzzing the browser's sessionStorage.
        SessionStorage = 2,
        /// The attack comes from fuzzing the window's name property.
        WindowName = 3,
        /// The attack comes from fuzzing the referrer property.
        Referrer = 4,
        /// The attack comes from fuzzing an input element.
        FormInput = 5,
        /// The attack comes from fuzzing the browser's cookies.
        Cookie = 6,
        /// The attack comes from hijacking the post messaging mechanism.
        PostMessage = 7,
        /// The attack comes from fuzzing parameters in the url.
        GetParameters = 8,
        /// The attack comes from fuzzing the fragment in the url.
        UrlFragment = 9,
        /// The attack comes from fuzzing the HTML comments.
        HtmlComment = 10,
        /// The attack comes from fuzzing the POST parameters.
        PostParameters = 11,
        /// The attack comes from fuzzing the protocol.
        Protocol = 12,
        /// The attack comes from the server side and is stored.
        StoredXss = 13,
        /// The attack is a Same-Origin Method Execution attack via a GET parameter.
        SameOrigin = 14,
        /// The attack payload is received from a third-party host via a URL that is
        /// user-controllable
        UserControllableUrl = 15,
    }
}
/// Information reported for an XXE.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Xxe {
    /// The XML string that triggered the XXE vulnerability. Non-payload values
    /// might be redacted.
    #[prost(string, tag="1")]
    pub payload_value: ::prost::alloc::string::String,
    /// Location within the request where the payload was placed.
    #[prost(enumeration="xxe::Location", tag="2")]
    pub payload_location: i32,
}
/// Nested message and enum types in `Xxe`.
pub mod xxe {
    /// Locations within a request where XML was substituted.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Location {
        /// Unknown Location.
        Unspecified = 0,
        /// The XML payload replaced the complete request body.
        CompleteRequestBody = 1,
    }
}
/// A Finding resource represents a vulnerability instance identified during a
/// ScanRun.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Finding {
    /// Output only. The resource name of the Finding. The name follows the format of
    /// 'projects/{projectId}/scanConfigs/{scanConfigId}/scanruns/{scanRunId}/findings/{findingId}'.
    /// The finding IDs are generated by the system.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The type of the Finding.
    /// Detailed and up-to-date information on findings can be found here:
    /// <https://cloud.google.com/security-command-center/docs/how-to-remediate-web-security-scanner-findings>
    #[prost(string, tag="2")]
    pub finding_type: ::prost::alloc::string::String,
    /// Output only. The severity level of the reported vulnerability.
    #[prost(enumeration="finding::Severity", tag="17")]
    pub severity: i32,
    /// Output only. The http method of the request that triggered the vulnerability, in
    /// uppercase.
    #[prost(string, tag="3")]
    pub http_method: ::prost::alloc::string::String,
    /// Output only. The URL produced by the server-side fuzzer and used in the request that
    /// triggered the vulnerability.
    #[prost(string, tag="4")]
    pub fuzzed_url: ::prost::alloc::string::String,
    /// Output only. The body of the request that triggered the vulnerability.
    #[prost(string, tag="5")]
    pub body: ::prost::alloc::string::String,
    /// Output only. The description of the vulnerability.
    #[prost(string, tag="6")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The URL containing human-readable payload that user can leverage to
    /// reproduce the vulnerability.
    #[prost(string, tag="7")]
    pub reproduction_url: ::prost::alloc::string::String,
    /// Output only. If the vulnerability was originated from nested IFrame, the immediate
    /// parent IFrame is reported.
    #[prost(string, tag="8")]
    pub frame_url: ::prost::alloc::string::String,
    /// Output only. The URL where the browser lands when the vulnerability is detected.
    #[prost(string, tag="9")]
    pub final_url: ::prost::alloc::string::String,
    /// Output only. The tracking ID uniquely identifies a vulnerability instance across
    /// multiple ScanRuns.
    #[prost(string, tag="10")]
    pub tracking_id: ::prost::alloc::string::String,
    /// Output only. An addon containing information reported for a vulnerability with an HTML
    /// form, if any.
    #[prost(message, optional, tag="16")]
    pub form: ::core::option::Option<Form>,
    /// Output only. An addon containing information about outdated libraries.
    #[prost(message, optional, tag="11")]
    pub outdated_library: ::core::option::Option<OutdatedLibrary>,
    /// Output only. An addon containing detailed information regarding any resource causing the
    /// vulnerability such as JavaScript sources, image, audio files, etc.
    #[prost(message, optional, tag="12")]
    pub violating_resource: ::core::option::Option<ViolatingResource>,
    /// Output only. An addon containing information about vulnerable or missing HTTP headers.
    #[prost(message, optional, tag="15")]
    pub vulnerable_headers: ::core::option::Option<VulnerableHeaders>,
    /// Output only. An addon containing information about request parameters which were found
    /// to be vulnerable.
    #[prost(message, optional, tag="13")]
    pub vulnerable_parameters: ::core::option::Option<VulnerableParameters>,
    /// Output only. An addon containing information reported for an XSS, if any.
    #[prost(message, optional, tag="14")]
    pub xss: ::core::option::Option<Xss>,
    /// Output only. An addon containing information reported for an XXE, if any.
    #[prost(message, optional, tag="18")]
    pub xxe: ::core::option::Option<Xxe>,
}
/// Nested message and enum types in `Finding`.
pub mod finding {
    /// The severity level of a vulnerability.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Severity {
        /// No severity specified. The default value.
        Unspecified = 0,
        /// Critical severity.
        Critical = 1,
        /// High severity.
        High = 2,
        /// Medium severity.
        Medium = 3,
        /// Low severity.
        Low = 4,
    }
}
/// A FindingTypeStats resource represents stats regarding a specific FindingType
/// of Findings under a given ScanRun.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindingTypeStats {
    /// Output only. The finding type associated with the stats.
    #[prost(string, tag="1")]
    pub finding_type: ::prost::alloc::string::String,
    /// Output only. The count of findings belonging to this finding type.
    #[prost(int32, tag="2")]
    pub finding_count: i32,
}
/// A ScanConfig resource contains the configurations to launch a scan.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScanConfig {
    /// The resource name of the ScanConfig. The name follows the format of
    /// 'projects/{projectId}/scanConfigs/{scanConfigId}'. The ScanConfig IDs are
    /// generated by the system.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The user provided display name of the ScanConfig.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    /// The maximum QPS during scanning. A valid value ranges from 5 to 20
    /// inclusively. If the field is unspecified or its value is set 0, server will
    /// default to 15. Other values outside of [5, 20] range will be rejected with
    /// INVALID_ARGUMENT error.
    #[prost(int32, tag="3")]
    pub max_qps: i32,
    /// Required. The starting URLs from which the scanner finds site pages.
    #[prost(string, repeated, tag="4")]
    pub starting_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The authentication configuration. If specified, service will use the
    /// authentication configuration during scanning.
    #[prost(message, optional, tag="5")]
    pub authentication: ::core::option::Option<scan_config::Authentication>,
    /// The user agent used during scanning.
    #[prost(enumeration="scan_config::UserAgent", tag="6")]
    pub user_agent: i32,
    /// The excluded URL patterns as described in
    /// <https://cloud.google.com/security-command-center/docs/how-to-use-web-security-scanner#excluding_urls>
    #[prost(string, repeated, tag="7")]
    pub blacklist_patterns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The schedule of the ScanConfig.
    #[prost(message, optional, tag="8")]
    pub schedule: ::core::option::Option<scan_config::Schedule>,
    /// Controls export of scan configurations and results to Security
    /// Command Center.
    #[prost(enumeration="scan_config::ExportToSecurityCommandCenter", tag="10")]
    pub export_to_security_command_center: i32,
    /// The risk level selected for the scan
    #[prost(enumeration="scan_config::RiskLevel", tag="12")]
    pub risk_level: i32,
    /// Whether the scan config is managed by Web Security Scanner, output
    /// only.
    #[prost(bool, tag="13")]
    pub managed_scan: bool,
    /// Whether the scan configuration has enabled static IP address scan feature.
    /// If enabled, the scanner will access applications from static IP addresses.
    #[prost(bool, tag="14")]
    pub static_ip_scan: bool,
    /// Whether to keep scanning even if most requests return HTTP error codes.
    #[prost(bool, tag="15")]
    pub ignore_http_status_errors: bool,
}
/// Nested message and enum types in `ScanConfig`.
pub mod scan_config {
    /// Scan authentication configuration.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Authentication {
        /// Required.
        /// Authentication configuration
        #[prost(oneof="authentication::Authentication", tags="1, 2, 4")]
        pub authentication: ::core::option::Option<authentication::Authentication>,
    }
    /// Nested message and enum types in `Authentication`.
    pub mod authentication {
        /// Describes authentication configuration that uses a Google account.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct GoogleAccount {
            /// Required. The user name of the Google account.
            #[prost(string, tag="1")]
            pub username: ::prost::alloc::string::String,
            /// Required. Input only. The password of the Google account. The credential is stored encrypted
            /// and not returned in any response nor included in audit logs.
            #[prost(string, tag="2")]
            pub password: ::prost::alloc::string::String,
        }
        /// Describes authentication configuration that uses a custom account.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CustomAccount {
            /// Required. The user name of the custom account.
            #[prost(string, tag="1")]
            pub username: ::prost::alloc::string::String,
            /// Required. Input only. The password of the custom account. The credential is stored encrypted
            /// and not returned in any response nor included in audit logs.
            #[prost(string, tag="2")]
            pub password: ::prost::alloc::string::String,
            /// Required. The login form URL of the website.
            #[prost(string, tag="3")]
            pub login_url: ::prost::alloc::string::String,
        }
        /// Describes authentication configuration for Identity-Aware-Proxy (IAP).
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct IapCredential {
            /// Identity-Aware-Proxy (IAP) Authentication Configuration
            #[prost(oneof="iap_credential::IapCredentials", tags="1")]
            pub iap_credentials: ::core::option::Option<iap_credential::IapCredentials>,
        }
        /// Nested message and enum types in `IapCredential`.
        pub mod iap_credential {
            /// Describes authentication configuration when Web-Security-Scanner
            /// service account is added in Identity-Aware-Proxy (IAP) access policies.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct IapTestServiceAccountInfo {
                /// Required. Describes OAuth2 client id of resources protected by
                /// Identity-Aware-Proxy (IAP).
                #[prost(string, tag="1")]
                pub target_audience_client_id: ::prost::alloc::string::String,
            }
            /// Identity-Aware-Proxy (IAP) Authentication Configuration
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum IapCredentials {
                /// Authentication configuration when Web-Security-Scanner service
                /// account is added in Identity-Aware-Proxy (IAP) access policies.
                #[prost(message, tag="1")]
                IapTestServiceAccountInfo(IapTestServiceAccountInfo),
            }
        }
        /// Required.
        /// Authentication configuration
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Authentication {
            /// Authentication using a Google account.
            #[prost(message, tag="1")]
            GoogleAccount(GoogleAccount),
            /// Authentication using a custom account.
            #[prost(message, tag="2")]
            CustomAccount(CustomAccount),
            /// Authentication using Identity-Aware-Proxy (IAP).
            #[prost(message, tag="4")]
            IapCredential(IapCredential),
        }
    }
    /// Scan schedule configuration.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Schedule {
        /// A timestamp indicates when the next run will be scheduled. The value is
        /// refreshed by the server after each run. If unspecified, it will default
        /// to current server time, which means the scan will be scheduled to start
        /// immediately.
        #[prost(message, optional, tag="1")]
        pub schedule_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Required. The duration of time between executions in days.
        #[prost(int32, tag="2")]
        pub interval_duration_days: i32,
    }
    /// Type of user agents used for scanning.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum UserAgent {
        /// The user agent is unknown. Service will default to CHROME_LINUX.
        Unspecified = 0,
        /// Chrome on Linux. This is the service default if unspecified.
        ChromeLinux = 1,
        /// Chrome on Android.
        ChromeAndroid = 2,
        /// Safari on IPhone.
        SafariIphone = 3,
    }
    /// Scan risk levels supported by Web Security Scanner. LOW impact
    /// scanning will minimize requests with the potential to modify data. To
    /// achieve the maximum scan coverage, NORMAL risk level is recommended.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RiskLevel {
        /// Use default, which is NORMAL.
        Unspecified = 0,
        /// Normal scanning (Recommended)
        Normal = 1,
        /// Lower impact scanning
        Low = 2,
    }
    /// Controls export of scan configurations and results to Security
    /// Command Center.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ExportToSecurityCommandCenter {
        /// Use default, which is ENABLED.
        Unspecified = 0,
        /// Export results of this scan to Security Command Center.
        Enabled = 1,
        /// Do not export results of this scan to Security Command Center.
        Disabled = 2,
    }
}
/// Defines a custom error message used by CreateScanConfig and UpdateScanConfig
/// APIs when scan configuration validation fails. It is also reported as part of
/// a ScanRunErrorTrace message if scan validation fails due to a scan
/// configuration error.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScanConfigError {
    /// Output only. Indicates the reason code for a configuration failure.
    #[prost(enumeration="scan_config_error::Code", tag="1")]
    pub code: i32,
    /// Output only. Indicates the full name of the ScanConfig field that triggers this error,
    /// for example "scan_config.max_qps". This field is provided for
    /// troubleshooting purposes only and its actual value can change in the
    /// future.
    #[prost(string, tag="2")]
    pub field_name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ScanConfigError`.
pub mod scan_config_error {
    /// Output only.
    /// Defines an error reason code.
    /// Next id: 44
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Code {
        /// There is no error.
        Unspecified = 0,
        /// Indicates an internal server error.
        /// Please DO NOT USE THIS ERROR CODE unless the root cause is truly unknown.
        InternalError = 1,
        /// One of the seed URLs is an App Engine URL but we cannot validate the scan
        /// settings due to an App Engine API backend error.
        AppengineApiBackendError = 2,
        /// One of the seed URLs is an App Engine URL but we cannot access the
        /// App Engine API to validate scan settings.
        AppengineApiNotAccessible = 3,
        /// One of the seed URLs is an App Engine URL but the Default Host of the
        /// App Engine is not set.
        AppengineDefaultHostMissing = 4,
        /// Google corporate accounts can not be used for scanning.
        CannotUseGoogleComAccount = 6,
        /// The account of the scan creator can not be used for scanning.
        CannotUseOwnerAccount = 7,
        /// This scan targets Compute Engine, but we cannot validate scan settings
        /// due to a Compute Engine API backend error.
        ComputeApiBackendError = 8,
        /// This scan targets Compute Engine, but we cannot access the Compute Engine
        /// API to validate the scan settings.
        ComputeApiNotAccessible = 9,
        /// The Custom Login URL does not belong to the current project.
        CustomLoginUrlDoesNotBelongToCurrentProject = 10,
        /// The Custom Login URL is malformed (can not be parsed).
        CustomLoginUrlMalformed = 11,
        /// The Custom Login URL is mapped to a non-routable IP address in DNS.
        CustomLoginUrlMappedToNonRoutableAddress = 12,
        /// The Custom Login URL is mapped to an IP address which is not reserved for
        /// the current project.
        CustomLoginUrlMappedToUnreservedAddress = 13,
        /// The Custom Login URL has a non-routable IP address.
        CustomLoginUrlHasNonRoutableIpAddress = 14,
        /// The Custom Login URL has an IP address which is not reserved for the
        /// current project.
        CustomLoginUrlHasUnreservedIpAddress = 15,
        /// Another scan with the same name (case-sensitive) already exists.
        DuplicateScanName = 16,
        /// A field is set to an invalid value.
        InvalidFieldValue = 18,
        /// There was an error trying to authenticate to the scan target.
        FailedToAuthenticateToTarget = 19,
        /// Finding type value is not specified in the list findings request.
        FindingTypeUnspecified = 20,
        /// Scan targets Compute Engine, yet current project was not whitelisted for
        /// Google Compute Engine Scanning Alpha access.
        ForbiddenToScanCompute = 21,
        /// User tries to update managed scan
        ForbiddenUpdateToManagedScan = 43,
        /// The supplied filter is malformed. For example, it can not be parsed, does
        /// not have a filter type in expression, or the same filter type appears
        /// more than once.
        MalformedFilter = 22,
        /// The supplied resource name is malformed (can not be parsed).
        MalformedResourceName = 23,
        /// The current project is not in an active state.
        ProjectInactive = 24,
        /// A required field is not set.
        RequiredField = 25,
        /// Project id, scanconfig id, scanrun id, or finding id are not consistent
        /// with each other in resource name.
        ResourceNameInconsistent = 26,
        /// The scan being requested to start is already running.
        ScanAlreadyRunning = 27,
        /// The scan that was requested to be stopped is not running.
        ScanNotRunning = 28,
        /// One of the seed URLs does not belong to the current project.
        SeedUrlDoesNotBelongToCurrentProject = 29,
        /// One of the seed URLs is malformed (can not be parsed).
        SeedUrlMalformed = 30,
        /// One of the seed URLs is mapped to a non-routable IP address in DNS.
        SeedUrlMappedToNonRoutableAddress = 31,
        /// One of the seed URLs is mapped to an IP address which is not reserved
        /// for the current project.
        SeedUrlMappedToUnreservedAddress = 32,
        /// One of the seed URLs has on-routable IP address.
        SeedUrlHasNonRoutableIpAddress = 33,
        /// One of the seed URLs has an IP address that is not reserved
        /// for the current project.
        SeedUrlHasUnreservedIpAddress = 35,
        /// The Web Security Scanner service account is not configured under the
        /// project.
        ServiceAccountNotConfigured = 36,
        /// A project has reached the maximum number of scans.
        TooManyScans = 37,
        /// Resolving the details of the current project fails.
        UnableToResolveProjectInfo = 38,
        /// One or more blacklist patterns were in the wrong format.
        UnsupportedBlacklistPatternFormat = 39,
        /// The supplied filter is not supported.
        UnsupportedFilter = 40,
        /// The supplied finding type is not supported. For example, we do not
        /// provide findings of the given finding type.
        UnsupportedFindingType = 41,
        /// The URL scheme of one or more of the supplied URLs is not supported.
        UnsupportedUrlScheme = 42,
    }
}
/// Output only.
/// Defines an error trace message for a ScanRun.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScanRunErrorTrace {
    /// Output only. Indicates the error reason code.
    #[prost(enumeration="scan_run_error_trace::Code", tag="1")]
    pub code: i32,
    /// Output only. If the scan encounters SCAN_CONFIG_ISSUE error, this field has the error
    /// message encountered during scan configuration validation that is performed
    /// before each scan run.
    #[prost(message, optional, tag="2")]
    pub scan_config_error: ::core::option::Option<ScanConfigError>,
    /// Output only. If the scan encounters TOO_MANY_HTTP_ERRORS, this field indicates the most
    /// common HTTP error code, if such is available. For example, if this code is
    /// 404, the scan has encountered too many NOT_FOUND responses.
    #[prost(int32, tag="3")]
    pub most_common_http_error_code: i32,
}
/// Nested message and enum types in `ScanRunErrorTrace`.
pub mod scan_run_error_trace {
    /// Output only.
    /// Defines an error reason code.
    /// Next id: 8
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Code {
        /// Default value is never used.
        Unspecified = 0,
        /// Indicates that the scan run failed due to an internal server error.
        InternalError = 1,
        /// Indicates a scan configuration error, usually due to outdated ScanConfig
        /// settings, such as starting_urls or the DNS configuration.
        ScanConfigIssue = 2,
        /// Indicates an authentication error, usually due to outdated ScanConfig
        /// authentication settings.
        AuthenticationConfigIssue = 3,
        /// Indicates a scan operation timeout, usually caused by a very large site.
        TimedOutWhileScanning = 4,
        /// Indicates that a scan encountered excessive redirects, either to
        /// authentication or some other page outside of the scan scope.
        TooManyRedirects = 5,
        /// Indicates that a scan encountered numerous errors from the web site
        /// pages. When available, most_common_http_error_code field indicates the
        /// most common HTTP error code encountered during the scan.
        TooManyHttpErrors = 6,
    }
}
/// A ScanRun is a output-only resource representing an actual run of the scan.
/// Next id: 12
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScanRun {
    /// Output only. The resource name of the ScanRun. The name follows the format of
    /// 'projects/{projectId}/scanConfigs/{scanConfigId}/scanRuns/{scanRunId}'.
    /// The ScanRun IDs are generated by the system.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The execution state of the ScanRun.
    #[prost(enumeration="scan_run::ExecutionState", tag="2")]
    pub execution_state: i32,
    /// Output only. The result state of the ScanRun. This field is only available after the
    /// execution state reaches "FINISHED".
    #[prost(enumeration="scan_run::ResultState", tag="3")]
    pub result_state: i32,
    /// Output only. The time at which the ScanRun started.
    #[prost(message, optional, tag="4")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which the ScanRun reached termination state - that the ScanRun
    /// is either finished or stopped by user.
    #[prost(message, optional, tag="5")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The number of URLs crawled during this ScanRun. If the scan is in progress,
    /// the value represents the number of URLs crawled up to now.
    #[prost(int64, tag="6")]
    pub urls_crawled_count: i64,
    /// Output only. The number of URLs tested during this ScanRun. If the scan is in progress,
    /// the value represents the number of URLs tested up to now. The number of
    /// URLs tested is usually larger than the number URLS crawled because
    /// typically a crawled URL is tested with multiple test payloads.
    #[prost(int64, tag="7")]
    pub urls_tested_count: i64,
    /// Output only. Whether the scan run has found any vulnerabilities.
    #[prost(bool, tag="8")]
    pub has_vulnerabilities: bool,
    /// Output only. The percentage of total completion ranging from 0 to 100.
    /// If the scan is in queue, the value is 0.
    /// If the scan is running, the value ranges from 0 to 100.
    /// If the scan is finished, the value is 100.
    #[prost(int32, tag="9")]
    pub progress_percent: i32,
    /// Output only. If result_state is an ERROR, this field provides the primary reason for
    /// scan's termination and more details, if such are available.
    #[prost(message, optional, tag="10")]
    pub error_trace: ::core::option::Option<ScanRunErrorTrace>,
    /// Output only. A list of warnings, if such are encountered during this scan run.
    #[prost(message, repeated, tag="11")]
    pub warning_traces: ::prost::alloc::vec::Vec<ScanRunWarningTrace>,
}
/// Nested message and enum types in `ScanRun`.
pub mod scan_run {
    /// Types of ScanRun execution state.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ExecutionState {
        /// Represents an invalid state caused by internal server error. This value
        /// should never be returned.
        Unspecified = 0,
        /// The scan is waiting in the queue.
        Queued = 1,
        /// The scan is in progress.
        Scanning = 2,
        /// The scan is either finished or stopped by user.
        Finished = 3,
    }
    /// Types of ScanRun result state.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ResultState {
        /// Default value. This value is returned when the ScanRun is not yet
        /// finished.
        Unspecified = 0,
        /// The scan finished without errors.
        Success = 1,
        /// The scan finished with errors.
        Error = 2,
        /// The scan was terminated by user.
        Killed = 3,
    }
}
/// Request for the `CreateScanConfig` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateScanConfigRequest {
    /// Required. The parent resource name where the scan is created, which should be a
    /// project resource name in the format 'projects/{projectId}'.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ScanConfig to be created.
    #[prost(message, optional, tag="2")]
    pub scan_config: ::core::option::Option<ScanConfig>,
}
/// Request for the `DeleteScanConfig` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteScanConfigRequest {
    /// Required. The resource name of the ScanConfig to be deleted. The name follows the
    /// format of 'projects/{projectId}/scanConfigs/{scanConfigId}'.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for the `GetScanConfig` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetScanConfigRequest {
    /// Required. The resource name of the ScanConfig to be returned. The name follows the
    /// format of 'projects/{projectId}/scanConfigs/{scanConfigId}'.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for the `ListScanConfigs` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListScanConfigsRequest {
    /// Required. The parent resource name, which should be a project resource name in the
    /// format 'projects/{projectId}'.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// A token identifying a page of results to be returned. This should be a
    /// `next_page_token` value returned from a previous List request.
    /// If unspecified, the first page of results is returned.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of ScanConfigs to return, can be limited by server.
    /// If not specified or not positive, the implementation will select a
    /// reasonable value.
    #[prost(int32, tag="3")]
    pub page_size: i32,
}
/// Request for the `UpdateScanConfigRequest` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateScanConfigRequest {
    /// Required. The ScanConfig to be updated. The name field must be set to identify the
    /// resource to be updated. The values of fields not covered by the mask
    /// will be ignored.
    #[prost(message, optional, tag="2")]
    pub scan_config: ::core::option::Option<ScanConfig>,
    /// Required. The update mask applies to the resource. For the `FieldMask` definition,
    /// see
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>
    #[prost(message, optional, tag="3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Response for the `ListScanConfigs` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListScanConfigsResponse {
    /// The list of ScanConfigs returned.
    #[prost(message, repeated, tag="1")]
    pub scan_configs: ::prost::alloc::vec::Vec<ScanConfig>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for the `StartScanRun` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartScanRunRequest {
    /// Required. The resource name of the ScanConfig to be used. The name follows the
    /// format of 'projects/{projectId}/scanConfigs/{scanConfigId}'.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for the `GetScanRun` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetScanRunRequest {
    /// Required. The resource name of the ScanRun to be returned. The name follows the
    /// format of
    /// 'projects/{projectId}/scanConfigs/{scanConfigId}/scanRuns/{scanRunId}'.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for the `ListScanRuns` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListScanRunsRequest {
    /// Required. The parent resource name, which should be a scan resource name in the
    /// format 'projects/{projectId}/scanConfigs/{scanConfigId}'.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// A token identifying a page of results to be returned. This should be a
    /// `next_page_token` value returned from a previous List request.
    /// If unspecified, the first page of results is returned.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of ScanRuns to return, can be limited by server.
    /// If not specified or not positive, the implementation will select a
    /// reasonable value.
    #[prost(int32, tag="3")]
    pub page_size: i32,
}
/// Response for the `ListScanRuns` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListScanRunsResponse {
    /// The list of ScanRuns returned.
    #[prost(message, repeated, tag="1")]
    pub scan_runs: ::prost::alloc::vec::Vec<ScanRun>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for the `StopScanRun` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopScanRunRequest {
    /// Required. The resource name of the ScanRun to be stopped. The name follows the
    /// format of
    /// 'projects/{projectId}/scanConfigs/{scanConfigId}/scanRuns/{scanRunId}'.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for the `ListCrawledUrls` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCrawledUrlsRequest {
    /// Required. The parent resource name, which should be a scan run resource name in the
    /// format
    /// 'projects/{projectId}/scanConfigs/{scanConfigId}/scanRuns/{scanRunId}'.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// A token identifying a page of results to be returned. This should be a
    /// `next_page_token` value returned from a previous List request.
    /// If unspecified, the first page of results is returned.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of CrawledUrls to return, can be limited by server.
    /// If not specified or not positive, the implementation will select a
    /// reasonable value.
    #[prost(int32, tag="3")]
    pub page_size: i32,
}
/// Response for the `ListCrawledUrls` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCrawledUrlsResponse {
    /// The list of CrawledUrls returned.
    #[prost(message, repeated, tag="1")]
    pub crawled_urls: ::prost::alloc::vec::Vec<CrawledUrl>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for the `GetFinding` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFindingRequest {
    /// Required. The resource name of the Finding to be returned. The name follows the
    /// format of
    /// 'projects/{projectId}/scanConfigs/{scanConfigId}/scanRuns/{scanRunId}/findings/{findingId}'.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for the `ListFindings` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFindingsRequest {
    /// Required. The parent resource name, which should be a scan run resource name in the
    /// format
    /// 'projects/{projectId}/scanConfigs/{scanConfigId}/scanRuns/{scanRunId}'.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The filter expression. The expression must be in the format: <field>
    /// <operator> <value>.
    /// Supported field: 'finding_type'.
    /// Supported operator: '='.
    #[prost(string, tag="2")]
    pub filter: ::prost::alloc::string::String,
    /// A token identifying a page of results to be returned. This should be a
    /// `next_page_token` value returned from a previous List request.
    /// If unspecified, the first page of results is returned.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of Findings to return, can be limited by server.
    /// If not specified or not positive, the implementation will select a
    /// reasonable value.
    #[prost(int32, tag="4")]
    pub page_size: i32,
}
/// Response for the `ListFindings` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFindingsResponse {
    /// The list of Findings returned.
    #[prost(message, repeated, tag="1")]
    pub findings: ::prost::alloc::vec::Vec<Finding>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for the `ListFindingTypeStats` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFindingTypeStatsRequest {
    /// Required. The parent resource name, which should be a scan run resource name in the
    /// format
    /// 'projects/{projectId}/scanConfigs/{scanConfigId}/scanRuns/{scanRunId}'.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
}
/// Response for the `ListFindingTypeStats` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFindingTypeStatsResponse {
    /// The list of FindingTypeStats returned.
    #[prost(message, repeated, tag="1")]
    pub finding_type_stats: ::prost::alloc::vec::Vec<FindingTypeStats>,
}
/// Generated client implementations.
pub mod web_security_scanner_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Web Security Scanner Service identifies security vulnerabilities in web
    /// applications hosted on Google Cloud. It crawls your application, and
    /// attempts to exercise as many user inputs and event handlers as possible.
    #[derive(Debug, Clone)]
    pub struct WebSecurityScannerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> WebSecurityScannerClient<T>
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
        ) -> WebSecurityScannerClient<InterceptedService<T, F>>
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
            WebSecurityScannerClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a new ScanConfig.
        pub async fn create_scan_config(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateScanConfigRequest>,
        ) -> Result<tonic::Response<super::ScanConfig>, tonic::Status> {
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
                "/google.cloud.websecurityscanner.v1.WebSecurityScanner/CreateScanConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes an existing ScanConfig and its child resources.
        pub async fn delete_scan_config(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteScanConfigRequest>,
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
                "/google.cloud.websecurityscanner.v1.WebSecurityScanner/DeleteScanConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a ScanConfig.
        pub async fn get_scan_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetScanConfigRequest>,
        ) -> Result<tonic::Response<super::ScanConfig>, tonic::Status> {
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
                "/google.cloud.websecurityscanner.v1.WebSecurityScanner/GetScanConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists ScanConfigs under a given project.
        pub async fn list_scan_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListScanConfigsRequest>,
        ) -> Result<tonic::Response<super::ListScanConfigsResponse>, tonic::Status> {
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
                "/google.cloud.websecurityscanner.v1.WebSecurityScanner/ListScanConfigs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a ScanConfig. This method support partial update of a ScanConfig.
        pub async fn update_scan_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateScanConfigRequest>,
        ) -> Result<tonic::Response<super::ScanConfig>, tonic::Status> {
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
                "/google.cloud.websecurityscanner.v1.WebSecurityScanner/UpdateScanConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Start a ScanRun according to the given ScanConfig.
        pub async fn start_scan_run(
            &mut self,
            request: impl tonic::IntoRequest<super::StartScanRunRequest>,
        ) -> Result<tonic::Response<super::ScanRun>, tonic::Status> {
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
                "/google.cloud.websecurityscanner.v1.WebSecurityScanner/StartScanRun",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a ScanRun.
        pub async fn get_scan_run(
            &mut self,
            request: impl tonic::IntoRequest<super::GetScanRunRequest>,
        ) -> Result<tonic::Response<super::ScanRun>, tonic::Status> {
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
                "/google.cloud.websecurityscanner.v1.WebSecurityScanner/GetScanRun",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists ScanRuns under a given ScanConfig, in descending order of ScanRun
        /// stop time.
        pub async fn list_scan_runs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListScanRunsRequest>,
        ) -> Result<tonic::Response<super::ListScanRunsResponse>, tonic::Status> {
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
                "/google.cloud.websecurityscanner.v1.WebSecurityScanner/ListScanRuns",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Stops a ScanRun. The stopped ScanRun is returned.
        pub async fn stop_scan_run(
            &mut self,
            request: impl tonic::IntoRequest<super::StopScanRunRequest>,
        ) -> Result<tonic::Response<super::ScanRun>, tonic::Status> {
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
                "/google.cloud.websecurityscanner.v1.WebSecurityScanner/StopScanRun",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List CrawledUrls under a given ScanRun.
        pub async fn list_crawled_urls(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCrawledUrlsRequest>,
        ) -> Result<tonic::Response<super::ListCrawledUrlsResponse>, tonic::Status> {
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
                "/google.cloud.websecurityscanner.v1.WebSecurityScanner/ListCrawledUrls",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a Finding.
        pub async fn get_finding(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFindingRequest>,
        ) -> Result<tonic::Response<super::Finding>, tonic::Status> {
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
                "/google.cloud.websecurityscanner.v1.WebSecurityScanner/GetFinding",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List Findings under a given ScanRun.
        pub async fn list_findings(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFindingsRequest>,
        ) -> Result<tonic::Response<super::ListFindingsResponse>, tonic::Status> {
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
                "/google.cloud.websecurityscanner.v1.WebSecurityScanner/ListFindings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List all FindingTypeStats under a given ScanRun.
        pub async fn list_finding_type_stats(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFindingTypeStatsRequest>,
        ) -> Result<
            tonic::Response<super::ListFindingTypeStatsResponse>,
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
                "/google.cloud.websecurityscanner.v1.WebSecurityScanner/ListFindingTypeStats",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// A ScanRunLog is an output-only proto used for Stackdriver customer logging.
/// It is used for logs covering the start and end of scan pipelines.
/// Other than an added summary, this is a subset of the ScanRun.
/// Representation in logs is either a proto Struct, or converted to JSON.
/// Next id: 9
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScanRunLog {
    /// Human friendly message about the event.
    #[prost(string, tag="1")]
    pub summary: ::prost::alloc::string::String,
    /// The resource name of the ScanRun being logged.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// The execution state of the ScanRun.
    #[prost(enumeration="scan_run::ExecutionState", tag="3")]
    pub execution_state: i32,
    /// The result state of the ScanRun.
    #[prost(enumeration="scan_run::ResultState", tag="4")]
    pub result_state: i32,
    #[prost(int64, tag="5")]
    pub urls_crawled_count: i64,
    #[prost(int64, tag="6")]
    pub urls_tested_count: i64,
    #[prost(bool, tag="7")]
    pub has_findings: bool,
    #[prost(message, optional, tag="8")]
    pub error_trace: ::core::option::Option<ScanRunErrorTrace>,
}
