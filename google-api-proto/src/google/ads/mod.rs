#[cfg(any(feature = "google-ads-admob-v1"))]
pub mod admob;
#[cfg(
    any(
        feature = "google-ads-googleads-v10-common",
        feature = "google-ads-googleads-v10-enums",
        feature = "google-ads-googleads-v10-errors",
        feature = "google-ads-googleads-v10-resources",
        feature = "google-ads-googleads-v10-services",
        feature = "google-ads-googleads-v11-common",
        feature = "google-ads-googleads-v11-enums",
        feature = "google-ads-googleads-v11-errors",
        feature = "google-ads-googleads-v11-resources",
        feature = "google-ads-googleads-v11-services",
        feature = "google-ads-googleads-v12-common",
        feature = "google-ads-googleads-v12-enums",
        feature = "google-ads-googleads-v12-errors",
        feature = "google-ads-googleads-v12-resources",
        feature = "google-ads-googleads-v12-services",
        feature = "google-ads-googleads-v9-common",
        feature = "google-ads-googleads-v9-enums",
        feature = "google-ads-googleads-v9-errors",
        feature = "google-ads-googleads-v9-resources",
        feature = "google-ads-googleads-v9-services",
    )
)]
pub mod googleads;
#[cfg(
    any(
        feature = "google-ads-searchads360-v0-common",
        feature = "google-ads-searchads360-v0-enums",
        feature = "google-ads-searchads360-v0-resources",
        feature = "google-ads-searchads360-v0-services",
    )
)]
pub mod searchads360;
