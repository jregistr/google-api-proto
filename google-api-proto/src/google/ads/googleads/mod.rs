#[cfg(
    any(
        feature = "google-ads-googleads-v11-common",
        feature = "google-ads-googleads-v11-enums",
        feature = "google-ads-googleads-v11-errors",
        feature = "google-ads-googleads-v11-resources",
        feature = "google-ads-googleads-v11-services",
    )
)]
pub mod v11;
#[cfg(
    any(
        feature = "google-ads-googleads-v12-common",
        feature = "google-ads-googleads-v12-enums",
        feature = "google-ads-googleads-v12-errors",
        feature = "google-ads-googleads-v12-resources",
        feature = "google-ads-googleads-v12-services",
    )
)]
pub mod v12;
#[cfg(
    any(
        feature = "google-ads-googleads-v13-common",
        feature = "google-ads-googleads-v13-enums",
        feature = "google-ads-googleads-v13-errors",
        feature = "google-ads-googleads-v13-resources",
        feature = "google-ads-googleads-v13-services",
    )
)]
pub mod v13;
