#[cfg(any(feature = "google-actions-type",))]
pub mod r#type;

#[cfg(any(
    feature = "google-actions-sdk-v2",
    feature = "google-actions-sdk-v2-conversation",
    feature = "google-actions-sdk-v2-interactionmodel",
    feature = "google-actions-sdk-v2-interactionmodel-prompt",
    feature = "google-actions-sdk-v2-interactionmodel-type",
))]
pub mod sdk;
