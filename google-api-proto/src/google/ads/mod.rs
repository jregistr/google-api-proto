#[cfg(any(feature = "google-ads-admob-v1"))]
pub mod admob;
#[cfg(
    any(
        feature = "google-ads-googleads-v12-common",
        feature = "google-ads-googleads-v12-enums",
        feature = "google-ads-googleads-v12-errors",
        feature = "google-ads-googleads-v12-resources",
        feature = "google-ads-googleads-v12-services",
        feature = "google-ads-googleads-v13-common",
        feature = "google-ads-googleads-v13-enums",
        feature = "google-ads-googleads-v13-errors",
        feature = "google-ads-googleads-v13-resources",
        feature = "google-ads-googleads-v13-services",
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
