#[cfg(any(feature = "google-actions-sdk-v2-conversation"))]
pub mod conversation;
#[cfg(
    any(
        feature = "google-actions-sdk-v2-interactionmodel",
        feature = "google-actions-sdk-v2-interactionmodel-prompt",
        feature = "google-actions-sdk-v2-interactionmodel-type",
    )
)]
pub mod interactionmodel;
/// Information about the encrypted OAuth client secret used in account linking
/// flows (for AUTH_CODE grant type).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLinkingSecret {
    /// Encrypted account linking client secret ciphertext.
    #[prost(bytes="bytes", tag="1")]
    pub encrypted_client_secret: ::prost::bytes::Bytes,
    /// The version of the crypto key used to encrypt the account linking client
    /// secret.
    /// Note that this field is ignored in push, preview, and version creation
    /// flows.
    #[prost(string, tag="2")]
    pub encryption_key_version: ::prost::alloc::string::String,
}
/// Represents the list of Actions defined in a project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Actions {
    /// Map from intents to custom Actions to configure invocation for the project.
    /// The invocation intents could either be system or custom intents defined
    /// in the "custom/intents/" package. All intents defined here (system
    /// intents & custom intents) must have a corresponding intent file in the
    /// "custom/global/" package.
    #[prost(btree_map="string, message", tag="3")]
    pub custom: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, actions::CustomAction>,
}
/// Nested message and enum types in `Actions`.
pub mod actions {
    /// Defines the engagement mechanisms associated with this action. This
    /// allows end users to subscribe to push notification and daily update.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Engagement {
        /// The title of the engagement that will be sent to end users asking for
        /// their permission to receive updates. The prompt sent to end users for
        /// daily updates will look like "What time would you like me to send your
        /// daily {title}" and for push notifications will look like
        /// "Is it ok if I send push notifications for {title}".
        /// **This field is localizable.**
        #[prost(string, tag="1")]
        pub title: ::prost::alloc::string::String,
        /// Push notification settings that this engagement supports.
        #[prost(message, optional, tag="2")]
        pub push_notification: ::core::option::Option<engagement::PushNotification>,
        /// Link config for an action which determines whether sharing links is
        /// enabled for the action and if so, contains the user friendly display name
        /// for the link.
        /// ActionLink is deprecated. Use AssistantLink instead.
        #[deprecated]
        #[prost(message, optional, tag="4")]
        pub action_link: ::core::option::Option<engagement::ActionLink>,
        /// Link config for an action which determines whether sharing links is
        /// enabled for the action and if so, contains the user friendly display name
        /// for the link.
        #[prost(message, optional, tag="6")]
        pub assistant_link: ::core::option::Option<engagement::AssistantLink>,
        /// Recurring update settings that this engagement supports.
        #[prost(oneof="engagement::RecurringUpdate", tags="3")]
        pub recurring_update: ::core::option::Option<engagement::RecurringUpdate>,
    }
    /// Nested message and enum types in `Engagement`.
    pub mod engagement {
        /// Defines push notification settings that this engagement supports.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PushNotification {
        }
        /// Defines daily update settings that this engagement supports.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DailyUpdate {
        }
        /// Indicates whether sharing links is enabled for this action and the
        /// corresponding settings. Action links are used to deep link a user into a
        /// specific action.
        /// ActionLink is deprecated. Use AssistantLink instead.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ActionLink {
            /// User friendly display title for the link.
            #[prost(string, tag="1")]
            pub title: ::prost::alloc::string::String,
        }
        /// Indicates whether sharing links is enabled for this action and the
        /// corresponding settings. Assistant links are used to deep link a user into
        /// a specific action.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct AssistantLink {
            /// User friendly display title for the link.
            #[prost(string, tag="1")]
            pub title: ::prost::alloc::string::String,
        }
        /// Recurring update settings that this engagement supports.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum RecurringUpdate {
            /// Daily update settings that this engagement supports.
            #[prost(message, tag="3")]
            DailyUpdate(DailyUpdate),
        }
    }
    /// Details regarding a custom action.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CustomAction {
        /// Engagement mechanisms associated with the action to help end users
        /// subscribe to push notifications and daily updates.
        /// Note that the intent name specified in daily updates/push notifications
        /// slot config needs to match the intent corresponding to this action for
        /// end users to subscribe to these updates.
        #[prost(message, optional, tag="2")]
        pub engagement: ::core::option::Option<Engagement>,
    }
}
/// Contains information that's "transportable" i.e. not specific to any given
/// project and can be moved between projects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Manifest {
    /// Version of the file format. The current file format version is 1.0
    /// Example: "1.0"
    #[prost(string, tag="1")]
    pub version: ::prost::alloc::string::String,
}
/// AccountLinking allows Google to guide the user to sign-in to the App's web
/// services.
///
/// For Google Sign In and OAuth + Google Sign In linking types, Google generates
/// a client ID identifying your App to Google ("Client ID issued by Google to
/// your Actions" on Console UI). This field is read-only and can be checked by
/// navigating to the Console UI's Account Linking page.
/// See: <https://developers.google.com/assistant/identity/google-sign-in>
///
/// Note: For all account linking setting types (except for Google Sign In), you
/// must provide a username and password for a test account in
/// Settings.testing_instructions for the review team to review the app (they
/// will not be visible to users).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLinking {
    /// Required. If `true`, users are allowed to sign up for new accounts via voice.
    /// If `false`, account creation is only allowed on your website. Select this
    /// option if you want to display your terms of service or obtain user consents
    /// during sign-up.
    /// linking_type cannot be GOOGLE_SIGN_IN when this is `false`.
    /// linking_type cannot be OAUTH when this is `true`.
    #[prost(bool, tag="1")]
    pub enable_account_creation: bool,
    /// Required. The linking type to use.
    /// See <https://developers.google.com/assistant/identity> for further details on
    /// the linking types.
    #[prost(enumeration="account_linking::LinkingType", tag="2")]
    pub linking_type: i32,
    /// Optional. Indicates the type of authentication for OAUTH linking_type.
    #[prost(enumeration="account_linking::AuthGrantType", tag="3")]
    pub auth_grant_type: i32,
    /// Optional. Client ID issued by your App to Google.
    /// This is the OAuth2 Client ID identifying Google to your service.
    /// Only set when using OAuth.
    #[prost(string, tag="4")]
    pub app_client_id: ::prost::alloc::string::String,
    /// Optional. Endpoint for your sign-in web page that supports OAuth2 code or
    /// implicit flows.
    /// URL must use HTTPS.
    /// Only set when using OAuth.
    #[prost(string, tag="5")]
    pub authorization_url: ::prost::alloc::string::String,
    /// Optional. OAuth2 endpoint for token exchange.
    /// URL must use HTTPS.
    /// This is not set when only using OAuth with IMPLICIT grant as the
    /// linking type.
    /// Only set when using OAuth.
    #[prost(string, tag="6")]
    pub token_url: ::prost::alloc::string::String,
    /// Optional. List of permissions the user must consent to in order to use
    /// your service.
    /// Only set when using OAuth.
    /// Make sure to provide a Terms of Service in the directory information in
    /// LocalizedSettings.terms_of_service_url section if specifying this field.
    #[prost(string, repeated, tag="7")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. This is the web page on your service which describes the
    /// permissions the user is granting to Google.
    /// Only set if using OAuth and Google Sign In.
    /// Make sure to provide a Terms of Service in the directory information in
    /// LocalizedSettings.terms_of_service_url section if specifying this field.
    #[prost(string, tag="8")]
    pub learn_more_url: ::prost::alloc::string::String,
    /// Optional. If true, allow Google to transmit client ID and secret via HTTP
    /// basic auth header. Otherwise, Google uses the client ID and secret inside
    /// the post body.
    /// Only set when using OAuth.
    /// Make sure to provide a Terms of Service in the directory information in
    /// LocalizedSettings.terms_of_service_url section if specifying this field.
    #[prost(bool, tag="9")]
    pub use_basic_auth_header: bool,
}
/// Nested message and enum types in `AccountLinking`.
pub mod account_linking {
    /// The type of Account Linking to perform.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LinkingType {
        /// Unspecified.
        Unspecified = 0,
        /// Google Sign In linking type.
        /// If using this linking type, no OAuth-related fields need to be set below.
        GoogleSignIn = 1,
        /// OAuth and Google Sign In linking type.
        OauthAndGoogleSignIn = 2,
        /// OAuth linking type.
        Oauth = 3,
    }
    impl LinkingType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LinkingType::Unspecified => "LINKING_TYPE_UNSPECIFIED",
                LinkingType::GoogleSignIn => "GOOGLE_SIGN_IN",
                LinkingType::OauthAndGoogleSignIn => "OAUTH_AND_GOOGLE_SIGN_IN",
                LinkingType::Oauth => "OAUTH",
            }
        }
    }
    /// The OAuth2 grant type Google uses to guide the user to sign in to your
    /// App's web service.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AuthGrantType {
        /// Unspecified.
        Unspecified = 0,
        /// Authorization code grant. Requires you to provide both
        /// authentication URL and access token URL.
        AuthCode = 1,
        /// Implicit code grant. Only requires you to provide authentication
        /// URL.
        Implicit = 2,
    }
    impl AuthGrantType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AuthGrantType::Unspecified => "AUTH_GRANT_TYPE_UNSPECIFIED",
                AuthGrantType::AuthCode => "AUTH_CODE",
                AuthGrantType::Implicit => "IMPLICIT",
            }
        }
    }
}
/// Styles applied to cards that are presented to users
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThemeCustomization {
    /// Background color of cards. Acts as a fallback if `background_image` is
    /// not provided by developers or `background_image` doesn't fit for certain
    /// surfaces.
    /// Example usage: #FAFAFA
    #[prost(string, tag="1")]
    pub background_color: ::prost::alloc::string::String,
    /// Primary theme color of the Action will be used to set text color of title,
    /// action item background color for Actions on Google cards.
    /// Example usage: #FAFAFA
    #[prost(string, tag="2")]
    pub primary_color: ::prost::alloc::string::String,
    /// The font family that will be used for title of cards.
    /// Supported fonts:
    /// - Sans Serif
    /// - Sans Serif Medium
    /// - Sans Serif Bold
    /// - Sans Serif Black
    /// - Sans Serif Condensed
    /// - Sans Serif Condensed Medium
    /// - Serif
    /// - Serif Bold
    /// - Monospace
    /// - Cursive
    /// - Sans Serif Smallcaps
    #[prost(string, tag="3")]
    pub font_family: ::prost::alloc::string::String,
    /// Border style of foreground image of cards. For example, can be applied on
    /// the foreground image of a basic card or carousel card.
    #[prost(enumeration="theme_customization::ImageCornerStyle", tag="4")]
    pub image_corner_style: i32,
    /// Landscape mode (minimum 1920x1200 pixels).
    /// This should be specified as a reference to the corresponding image in the
    /// `resources/images/` directory. Eg: `$resources.images.foo` (without the
    /// extension) for image in `resources/images/foo.jpg`
    /// When working on a project pulled from Console the Google managed url pulled
    /// could be used.
    #[prost(string, tag="5")]
    pub landscape_background_image: ::prost::alloc::string::String,
    /// Portrait mode (minimum 1200x1920 pixels).
    /// This should be specified as a reference to the corresponding image in the
    /// `resources/images/` directory. Eg: `$resources.images.foo` (without the
    /// extension) for image in `resources/images/foo.jpg`
    /// When working on a project pulled from Console the Google managed url pulled
    /// could be used.
    #[prost(string, tag="6")]
    pub portrait_background_image: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ThemeCustomization`.
pub mod theme_customization {
    /// Describes how the borders of images should be rendered.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ImageCornerStyle {
        /// Undefined / Unspecified.
        Unspecified = 0,
        /// Round corner for image.
        Curved = 1,
        /// Rectangular corner for image.
        Angled = 2,
    }
    impl ImageCornerStyle {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ImageCornerStyle::Unspecified => "IMAGE_CORNER_STYLE_UNSPECIFIED",
                ImageCornerStyle::Curved => "CURVED",
                ImageCornerStyle::Angled => "ANGLED",
            }
        }
    }
}
/// Represents settings of an Actions project that are specific to a user locale.
/// In this instance, user means the end user who invokes your Actions.
/// **This message is localizable.**
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalizedSettings {
    /// Required. The default display name for this Actions project (if there is no
    /// translation available)
    #[prost(string, tag="1")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. The pronunciation of the display name to invoke it within a voice
    /// (spoken) context.
    #[prost(string, tag="2")]
    pub pronunciation: ::prost::alloc::string::String,
    /// Required. The default short description for the Actions project (if there is no
    /// translation available). 80 character limit.
    #[prost(string, tag="3")]
    pub short_description: ::prost::alloc::string::String,
    /// Required. The default long description for the Actions project (if there is no
    /// translation available). 4000 character limit.
    #[prost(string, tag="4")]
    pub full_description: ::prost::alloc::string::String,
    /// Required. Small square image, 192 x 192 px.
    /// This should be specified as a reference to the corresponding image in the
    /// `resources/images/` directory. For example, `$resources.images.foo` (without the
    /// extension) for image in `resources/images/foo.jpg`
    /// When working on a project pulled from Console, the Google-managed URL
    /// pulled could be used. URLs from external sources are not allowed.
    #[prost(string, tag="5")]
    pub small_logo_image: ::prost::alloc::string::String,
    /// Optional. Large landscape image, 1920 x 1080 px.
    /// This should be specified as a reference to the corresponding image in the
    /// `resources/images/` directory. For example, `$resources.images.foo` (without the
    /// extension) for image in `resources/images/foo.jpg`
    /// When working on a project pulled from Console, the Google-managed URL
    /// pulled could be used. URLs from external sources are not allowed.
    #[prost(string, tag="6")]
    pub large_banner_image: ::prost::alloc::string::String,
    /// Required. The name of the developer to be displayed to users.
    #[prost(string, tag="7")]
    pub developer_name: ::prost::alloc::string::String,
    /// Required. The contact email address for the developer.
    #[prost(string, tag="8")]
    pub developer_email: ::prost::alloc::string::String,
    /// Optional. The terms of service URL.
    #[prost(string, tag="9")]
    pub terms_of_service_url: ::prost::alloc::string::String,
    /// Required. The Google Assistant voice type that users hear when they interact with
    /// your Actions. The supported values are "male_1", "male_2", "female_1", and
    /// "female_2".
    #[prost(string, tag="10")]
    pub voice: ::prost::alloc::string::String,
    /// Optional. The locale for the specified voice. If not specified, this resolves
    /// to the user's Assistant locale. If specified, the voice locale must have
    /// the same root language as the locale specified in LocalizedSettings.
    #[prost(string, tag="14")]
    pub voice_locale: ::prost::alloc::string::String,
    /// Required. The privacy policy URL.
    #[prost(string, tag="11")]
    pub privacy_policy_url: ::prost::alloc::string::String,
    /// Optional. Sample invocation phrases displayed as part of your Actions project's
    /// description in the Assistant directory. This will help users learn how to
    /// use it.
    #[prost(string, repeated, tag="12")]
    pub sample_invocations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Theme customizations for visual components of your Actions.
    #[prost(message, optional, tag="13")]
    pub theme_customization: ::core::option::Option<ThemeCustomization>,
}
/// Contains a set of requirements that the client surface must support to invoke
/// Actions in your project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SurfaceRequirements {
    /// The minimum set of capabilities needed to invoke the Actions in your
    /// project. If the surface is missing any of these, the Action will not be
    /// triggered.
    #[prost(message, repeated, tag="1")]
    pub minimum_requirements: ::prost::alloc::vec::Vec<CapabilityRequirement>,
}
/// Represents a requirement about the availability of a given capability.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapabilityRequirement {
    /// The type of capability.
    #[prost(enumeration="capability_requirement::SurfaceCapability", tag="1")]
    pub capability: i32,
}
/// Nested message and enum types in `CapabilityRequirement`.
pub mod capability_requirement {
    /// Possible set of surface capabilities.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SurfaceCapability {
        /// Unknown / Unspecified.
        Unspecified = 0,
        /// Surface supports audio output.
        AudioOutput = 1,
        /// Surface supports screen/visual output.
        ScreenOutput = 2,
        /// Surface supports media response audio.
        MediaResponseAudio = 3,
        /// Surface supports web browsers.
        WebBrowser = 4,
        /// Surface supports account linking.
        AccountLinking = 7,
        /// Surface supports Interactive Canvas.
        InteractiveCanvas = 8,
        /// Surface supports home storage.
        HomeStorage = 9,
    }
    impl SurfaceCapability {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SurfaceCapability::Unspecified => "SURFACE_CAPABILITY_UNSPECIFIED",
                SurfaceCapability::AudioOutput => "AUDIO_OUTPUT",
                SurfaceCapability::ScreenOutput => "SCREEN_OUTPUT",
                SurfaceCapability::MediaResponseAudio => "MEDIA_RESPONSE_AUDIO",
                SurfaceCapability::WebBrowser => "WEB_BROWSER",
                SurfaceCapability::AccountLinking => "ACCOUNT_LINKING",
                SurfaceCapability::InteractiveCanvas => "INTERACTIVE_CANVAS",
                SurfaceCapability::HomeStorage => "HOME_STORAGE",
            }
        }
    }
}
/// Represents settings of an Actions project that are not locale specific.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Settings {
    /// Actions project id.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// Locale which is default for the project. For all files except under
    /// `resources/` with no locale in the path, the localized data is attributed
    /// to this `default_locale`. For files under `resources/` no locale means that
    /// the resource is applicable to all locales.
    #[prost(string, tag="2")]
    pub default_locale: ::prost::alloc::string::String,
    /// Represents the regions where users can invoke your Actions, which is
    /// based on the user's location of presence. Cannot be set if
    /// `disabled_regions` is set. If both `enabled_regions` and `disabled_regions`
    /// are not specified, users can invoke your Actions in all regions. Each
    /// region is represented using the Canonical Name of Adwords geotargets. See
    /// <https://developers.google.com/adwords/api/docs/appendix/geotargeting>
    /// Examples include:
    /// - "Germany"
    /// - "Ghana"
    /// - "Greece"
    /// - "Grenada"
    /// - "United Kingdom"
    /// - "United States"
    /// - "United States Minor Outlying Islands"
    /// - "Uruguay"
    #[prost(string, repeated, tag="3")]
    pub enabled_regions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Represents the regions where your Actions are blocked, based on the user's
    /// location of presence. Cannot be set if `enabled_regions` is set.
    /// Each region is represented using the Canonical Name of Adwords geotargets.
    /// See <https://developers.google.com/adwords/api/docs/appendix/geotargeting>
    /// Examples include:
    /// - "Germany"
    /// - "Ghana"
    /// - "Greece"
    /// - "Grenada"
    /// - "United Kingdom"
    /// - "United States"
    /// - "United States Minor Outlying Islands"
    /// - "Uruguay"
    #[prost(string, repeated, tag="4")]
    pub disabled_regions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The category for this Actions project.
    #[prost(enumeration="settings::Category", tag="5")]
    pub category: i32,
    /// Whether Actions can use transactions (for example, making
    /// reservations, taking orders, etc.). If false, then attempts to use the
    /// Transactions APIs fail.
    #[prost(bool, tag="6")]
    pub uses_transactions_api: bool,
    /// Whether Actions can perform transactions for digital goods.
    #[prost(bool, tag="7")]
    pub uses_digital_purchase_api: bool,
    /// Whether Actions use Interactive Canvas.
    #[prost(bool, tag="8")]
    pub uses_interactive_canvas: bool,
    /// Whether Actions use the home storage feature.
    #[prost(bool, tag="17")]
    pub uses_home_storage: bool,
    /// Whether Actions content is designed for family (DFF).
    #[prost(bool, tag="9")]
    pub designed_for_family: bool,
    /// Whether Actions contains alcohol or tobacco related content.
    #[prost(bool, tag="11")]
    pub contains_alcohol_or_tobacco_content: bool,
    /// Whether Actions may leave mic open without an explicit prompt during
    /// conversation.
    #[prost(bool, tag="12")]
    pub keeps_mic_open: bool,
    /// The surface requirements that a client surface must support to invoke
    /// Actions in this project.
    #[prost(message, optional, tag="13")]
    pub surface_requirements: ::core::option::Option<SurfaceRequirements>,
    /// Free-form testing instructions for Actions reviewer (for example, account
    /// linking instructions).
    #[prost(string, tag="14")]
    pub testing_instructions: ::prost::alloc::string::String,
    /// Localized settings for the project's default locale. Every additional
    /// locale should have its own settings file in its own directory.
    #[prost(message, optional, tag="15")]
    pub localized_settings: ::core::option::Option<LocalizedSettings>,
    /// Allow users to create or link accounts through Google sign-in and/or your
    /// own OAuth service.
    #[prost(message, optional, tag="16")]
    pub account_linking: ::core::option::Option<AccountLinking>,
    /// Android apps selected to acccess Google Play purchases for transactions.
    /// This is a selection from the Android apps connected to the actions project
    /// to verify brand ownership and enable additional features. See
    /// <https://developers.google.com/assistant/console/brand-verification> for more
    /// information.
    #[prost(string, repeated, tag="20")]
    pub selected_android_apps: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `Settings`.
pub mod settings {
    /// The category choices for an Actions project.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Category {
        /// Unknown / Unspecified.
        Unspecified = 0,
        /// Business and Finance category.
        BusinessAndFinance = 2,
        /// Education and Reference category.
        EducationAndReference = 3,
        /// Food and Drink category.
        FoodAndDrink = 4,
        /// Games and Trivia category.
        GamesAndTrivia = 5,
        /// Health and Fitness category.
        HealthAndFitness = 6,
        /// Kids and Family category.
        KidsAndFamily = 20,
        /// Lifestyle category.
        Lifestyle = 7,
        /// Local category.
        Local = 8,
        /// Movies and TV category.
        MoviesAndTv = 9,
        /// Music and Audio category.
        MusicAndAudio = 10,
        /// News category,
        News = 1,
        /// Novelty and Humor category.
        NoveltyAndHumor = 11,
        /// Productivity category.
        Productivity = 12,
        /// Shopping category.
        Shopping = 13,
        /// Social category.
        Social = 14,
        /// Sports category.
        Sports = 15,
        /// Travel and Transportation category.
        TravelAndTransportation = 16,
        /// Utilities category.
        Utilities = 17,
        /// Weather category.
        Weather = 18,
        /// Home Control category.
        HomeControl = 19,
    }
    impl Category {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Category::Unspecified => "CATEGORY_UNSPECIFIED",
                Category::BusinessAndFinance => "BUSINESS_AND_FINANCE",
                Category::EducationAndReference => "EDUCATION_AND_REFERENCE",
                Category::FoodAndDrink => "FOOD_AND_DRINK",
                Category::GamesAndTrivia => "GAMES_AND_TRIVIA",
                Category::HealthAndFitness => "HEALTH_AND_FITNESS",
                Category::KidsAndFamily => "KIDS_AND_FAMILY",
                Category::Lifestyle => "LIFESTYLE",
                Category::Local => "LOCAL",
                Category::MoviesAndTv => "MOVIES_AND_TV",
                Category::MusicAndAudio => "MUSIC_AND_AUDIO",
                Category::News => "NEWS",
                Category::NoveltyAndHumor => "NOVELTY_AND_HUMOR",
                Category::Productivity => "PRODUCTIVITY",
                Category::Shopping => "SHOPPING",
                Category::Social => "SOCIAL",
                Category::Sports => "SPORTS",
                Category::TravelAndTransportation => "TRAVEL_AND_TRANSPORTATION",
                Category::Utilities => "UTILITIES",
                Category::Weather => "WEATHER",
                Category::HomeControl => "HOME_CONTROL",
            }
        }
    }
}
/// Metadata for different types of webhooks. If you're using
/// `inlineCloudFunction`, your source code must be in a directory with the same
/// name as the value for the `executeFunction` key.
/// For example, a value of `my_webhook` for the`executeFunction` key would have
/// a code structure like this:
///   - `/webhooks/my_webhook.yaml`
///   - `/webhooks/my_webhook/index.js`
///   - `/webhooks/my_webhook/package.json`
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Webhook {
    /// List of handlers for this webhook.
    #[prost(message, repeated, tag="1")]
    pub handlers: ::prost::alloc::vec::Vec<webhook::Handler>,
    /// Only one webhook type is supported.
    #[prost(oneof="webhook::WebhookType", tags="2, 3")]
    pub webhook_type: ::core::option::Option<webhook::WebhookType>,
}
/// Nested message and enum types in `Webhook`.
pub mod webhook {
    /// Declares the name of the webhoook handler. A webhook can have
    /// multiple handlers registered. These handlers can be called from multiple
    /// places in your Actions project.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Handler {
        /// Required. Name of the handler. Must be unique across all handlers the Actions
        /// project. You can check the name of this handler to invoke the correct
        /// function in your fulfillment source code.
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
    }
    /// REST endpoint to notify if you're not using the inline editor.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HttpsEndpoint {
        /// The HTTPS base URL for your fulfillment endpoint (HTTP is not supported).
        /// Handler names are appended to the base URL path after a colon
        /// (following the style guide in
        /// <https://cloud.google.com/apis/design/custom_methods>).
        /// For example a base URL of '<https://gactions.service.com/api'> would
        /// receive requests with URL '<https://gactions.service.com/api:{method}'.>
        #[prost(string, tag="1")]
        pub base_url: ::prost::alloc::string::String,
        /// Map of HTTP parameters to be included in the POST request.
        #[prost(btree_map="string, string", tag="2")]
        pub http_headers: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
        /// Version of the protocol used by the endpoint. This is the protocol shared
        /// by all fulfillment types and not specific to Google fulfillment type.
        #[prost(int32, tag="3")]
        pub endpoint_api_version: i32,
    }
    /// Holds the metadata of an inline Cloud Function deployed from the
    /// webhooks folder.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InlineCloudFunction {
        /// The name of the Cloud Function entry point. The value of this field
        /// should match the name of the method exported from the source code.
        #[prost(string, tag="1")]
        pub execute_function: ::prost::alloc::string::String,
    }
    /// Only one webhook type is supported.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum WebhookType {
        /// Custom webhook HTTPS endpoint.
        #[prost(message, tag="2")]
        HttpsEndpoint(HttpsEndpoint),
        /// Metadata for cloud function deployed from code in the webhooks folder.
        #[prost(message, tag="3")]
        InlineCloudFunction(InlineCloudFunction),
    }
}
/// Wrapper for repeated config files. Repeated fields cannot exist in a oneof.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigFiles {
    /// Multiple config files.
    #[prost(message, repeated, tag="1")]
    pub config_files: ::prost::alloc::vec::Vec<ConfigFile>,
}
/// Represents a single file which contains structured data. Developers can
/// define most of their project using structured config including Actions,
/// Settings, Fulfillment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigFile {
    /// Relative path of the config file from the project root in the SDK file
    /// structure. Each file types below have an allowed file path.
    /// Eg: settings/settings.yaml
    #[prost(string, tag="1")]
    pub file_path: ::prost::alloc::string::String,
    /// Each type of config file should have a corresponding field in the oneof.
    #[prost(oneof="config_file::File", tags="2, 3, 4, 6, 7, 8, 15, 9, 10, 11, 13, 12")]
    pub file: ::core::option::Option<config_file::File>,
}
/// Nested message and enum types in `ConfigFile`.
pub mod config_file {
    /// Each type of config file should have a corresponding field in the oneof.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum File {
        /// Single manifest file.
        /// Allowed file path: `manifest.yaml`
        #[prost(message, tag="2")]
        Manifest(super::Manifest),
        /// Single actions file with all the actions defined.
        /// Allowed file paths: `actions/{language}?/actions.yaml`
        #[prost(message, tag="3")]
        Actions(super::Actions),
        /// Single settings config which includes non-localizable settings and
        /// settings for the project's default locale (if specified).
        /// For a locale override file, only localized_settings field will be
        /// populated.
        /// Allowed file paths: `settings/{language}?/settings.yaml`
        /// Note that the non-localized settings file `settings/settings.yaml` must
        /// be present in the write flow requests.
        #[prost(message, tag="4")]
        Settings(super::Settings),
        /// Single webhook definition.
        /// Allowed file path: `webhooks/{WebhookName}.yaml`
        #[prost(message, tag="6")]
        Webhook(super::Webhook),
        /// Single intent definition.
        /// Allowed file paths: `custom/intents/{language}?/{IntentName}.yaml`
        #[prost(message, tag="7")]
        Intent(super::interactionmodel::Intent),
        /// Single type definition.
        /// Allowed file paths: `custom/types/{language}?/{TypeName}.yaml`
        #[prost(message, tag="8")]
        Type(super::interactionmodel::r#type::Type),
        /// Single entity set definition.
        /// Allowed file paths: `custom/entitySets/{language}?/{EntitySetName}.yaml`
        #[prost(message, tag="15")]
        EntitySet(super::interactionmodel::EntitySet),
        /// Single global intent event definition.
        /// Allowed file paths: `custom/global/{GlobalIntentEventName}.yaml`
        /// The file name (GlobalIntentEventName) should be the name of the intent
        /// that this global intent event corresponds to.
        #[prost(message, tag="9")]
        GlobalIntentEvent(super::interactionmodel::GlobalIntentEvent),
        /// Single scene definition.
        /// Allowed file paths: `custom/scenes/{SceneName}.yaml`
        #[prost(message, tag="10")]
        Scene(super::interactionmodel::Scene),
        /// Single static prompt definition.
        /// Allowed file paths: `custom/prompts/{language}?/{StaticPromptName}.yaml`
        #[prost(message, tag="11")]
        StaticPrompt(super::interactionmodel::prompt::StaticPrompt),
        /// Metadata corresponding to the client secret used in account linking.
        /// Allowed file path: `settings/accountLinkingSecret.yaml`
        #[prost(message, tag="13")]
        AccountLinkingSecret(super::AccountLinkingSecret),
        /// Single resource bundle, which is a map from a string to a string or list
        /// of strings. Resource bundles could be used for localizing strings in
        /// static prompts.
        /// Allowed file paths: `resources/strings/{language}?/{multiple
        /// directories}?/{BundleName}.yaml`
        #[prost(message, tag="12")]
        ResourceBundle(::prost_types::Struct),
    }
}
/// Wrapper for repeated data file. Repeated fields cannot exist in a oneof.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataFiles {
    /// Multiple data files.
    #[prost(message, repeated, tag="1")]
    pub data_files: ::prost::alloc::vec::Vec<DataFile>,
}
/// Represents a single file which contains unstructured data. Examples include
/// image files, audio files, and cloud function source code.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataFile {
    /// Relative path of the data file from the project root in the SDK file
    /// structure.
    /// Allowed file paths:
    ///      - Images: `resources/images/{multiple
    ///      directories}?/{ImageName}.{extension}`
    ///      - Audio: `resources/audio/{multiple
    ///      directories}?/{AudioFileName}.{extension}`
    ///      - Inline Cloud Function Code: `webhooks/{WebhookName}.zip`
    /// Allowed extensions:
    ///      - Images: `png`, `jpg`, `jpeg`
    ///      - Audio: `mp3`, `mpeg`
    ///      - Inline Cloud Functions: `zip`
    #[prost(string, tag="1")]
    pub file_path: ::prost::alloc::string::String,
    /// Required. The content type of this asset. Example: `text/html`. The content
    /// type must comply with the specification
    /// (<http://www.w3.org/Protocols/rfc1341/4_Content-Type.html>).
    /// Cloud functions must be in zip format and the content type should
    /// be `application/zip;zip_type=cloud_function`. The zip_type parameter
    /// indicates that the zip is for a cloud function.
    #[prost(string, tag="2")]
    pub content_type: ::prost::alloc::string::String,
    /// Content of the data file. Examples would be raw bytes of images, audio
    /// files, or cloud function zip format.
    /// There is 10 MB strict limit on the payload size.
    #[prost(bytes="bytes", tag="3")]
    pub payload: ::prost::bytes::Bytes,
}
/// Wrapper for a list of files.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Files {
    /// Only one type of files can be sent to the server at a time, config files or
    /// data files.
    #[prost(oneof="files::FileType", tags="1, 2")]
    pub file_type: ::core::option::Option<files::FileType>,
}
/// Nested message and enum types in `Files`.
pub mod files {
    /// Only one type of files can be sent to the server at a time, config files or
    /// data files.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum FileType {
        /// List of config files. This includes manifest, settings, interaction model
        /// resource bundles and more.
        #[prost(message, tag="1")]
        ConfigFiles(super::ConfigFiles),
        /// List of data files. This includes image, audio file, cloud function
        /// source code.
        #[prost(message, tag="2")]
        DataFiles(super::DataFiles),
    }
}
/// Definition of release channel resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReleaseChannel {
    /// The unique name of the release channel in the following format.
    /// `projects/{project}/releaseChannels/{release_channel}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Version currently deployed to this release channel in the following format:
    /// `projects/{project}/versions/{version}`.
    #[prost(string, tag="2")]
    pub current_version: ::prost::alloc::string::String,
    /// Version to be deployed to this release channel in the following format:
    /// `projects/{project}/versions/{version}`.
    #[prost(string, tag="3")]
    pub pending_version: ::prost::alloc::string::String,
}
/// Wrapper for repeated validation result.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidationResults {
    /// Multiple validation results.
    #[prost(message, repeated, tag="1")]
    pub results: ::prost::alloc::vec::Vec<ValidationResult>,
}
/// Represents a validation result associated with the app content.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidationResult {
    /// Holds the validation message.
    #[prost(string, tag="1")]
    pub validation_message: ::prost::alloc::string::String,
    /// Context to identify the resource the validation message relates to.
    #[prost(message, optional, tag="2")]
    pub validation_context: ::core::option::Option<validation_result::ValidationContext>,
}
/// Nested message and enum types in `ValidationResult`.
pub mod validation_result {
    /// Context to identify the resource the validation message relates to.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ValidationContext {
        /// Language code of the lozalized resource.
        /// Empty if the error is for non-localized resource.
        /// See the list of supported languages in
        /// <https://developers.google.com/assistant/console/languages-locales>
        #[prost(string, tag="1")]
        pub language_code: ::prost::alloc::string::String,
    }
}
/// Definition of version resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Version {
    /// The unique identifier of the version in the following format.
    /// `projects/{project}/versions/{version}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The current state of the version.
    #[prost(message, optional, tag="2")]
    pub version_state: ::core::option::Option<version::VersionState>,
    /// Email of the user who created this version.
    #[prost(string, tag="3")]
    pub creator: ::prost::alloc::string::String,
    /// Timestamp of the last change to this version.
    #[prost(message, optional, tag="4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `Version`.
pub mod version {
    /// Represents the current state of the version.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VersionState {
        /// The current state of the version.
        #[prost(enumeration="version_state::State", tag="1")]
        pub state: i32,
        /// User-friendly message for the current state of the version.
        #[prost(string, tag="2")]
        pub message: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `VersionState`.
    pub mod version_state {
        /// Enum indicating the states that a Version can take. This enum is not yet
        /// frozen and values maybe added later.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum State {
            /// Default value of State.
            Unspecified = 0,
            /// The version creation is in progress.
            CreationInProgress = 1,
            /// The version creation failed.
            CreationFailed = 2,
            /// The version has been successfully created.
            Created = 3,
            /// The version is under policy review (aka Approval).
            ReviewInProgress = 4,
            /// The version has been approved for policy review and can be deployed.
            Approved = 5,
            /// The version has been conditionally approved but is pending final
            /// review. It may be rolled back if final review is denied.
            ConditionallyApproved = 6,
            /// The version has been denied for policy review.
            Denied = 7,
            /// The version is taken down as entire agent and all versions are taken
            /// down.
            UnderTakedown = 8,
            /// The version has been deleted.
            Deleted = 9,
        }
        impl State {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    State::Unspecified => "STATE_UNSPECIFIED",
                    State::CreationInProgress => "CREATION_IN_PROGRESS",
                    State::CreationFailed => "CREATION_FAILED",
                    State::Created => "CREATED",
                    State::ReviewInProgress => "REVIEW_IN_PROGRESS",
                    State::Approved => "APPROVED",
                    State::ConditionallyApproved => "CONDITIONALLY_APPROVED",
                    State::Denied => "DENIED",
                    State::UnderTakedown => "UNDER_TAKEDOWN",
                    State::Deleted => "DELETED",
                }
            }
        }
    }
}
/// Streaming RPC request for WriteDraft.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteDraftRequest {
    /// Required. The parent resource name in the format `projects/{project}`. The
    /// `{project}` is the cloud project ID associated with the project.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. List of files sent to the server at a time. This is a list of config files
    /// or data files.
    /// 1. The first request must be a ConfigFiles.
    /// 2. The first request must have a ConfigFile with 'settings'.
    /// 3. The first request must have a ConfigFile with 'manifest'.
    /// 4. The webhook ConfigFile corresponding to inline cloud function must be
    ///     streamed before the DataFile corresponding to its source code.
    #[prost(message, optional, tag="4")]
    pub files: ::core::option::Option<Files>,
}
/// Definition of draft resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Draft {
    /// The unique identifier of the draft in the following format.
    /// `projects/{project}/draft`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Validation results associated with the project draft content. Note that
    /// WriteDraft updates the draft despite the warnings as warnings are not draft
    /// blocking.
    #[prost(message, optional, tag="2")]
    pub validation_results: ::core::option::Option<ValidationResults>,
}
/// Streaming RPC request for WritePreview.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WritePreviewRequest {
    /// Required. The parent resource name in the format `projects/{project}`. The
    /// `{project}` is the cloud project ID associated with the project.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The settings for updating the user's preview.
    #[prost(message, optional, tag="4")]
    pub preview_settings: ::core::option::Option<write_preview_request::PreviewSettings>,
    /// Data source used to created the preview.
    #[prost(oneof="write_preview_request::Source", tags="5, 6, 7")]
    pub source: ::core::option::Option<write_preview_request::Source>,
}
/// Nested message and enum types in `WritePreviewRequest`.
pub mod write_preview_request {
    /// Indicates the preview content will be coming from the Draft.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ContentFromDraft {
    }
    /// Indicates the preview content will be coming from an exiting version.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ContentFromSubmittedVersion {
        /// Required. Submitted version of the project to be used to create a preview.
        /// Format: `projects/{project}/versions/{version}`
        #[prost(string, tag="1")]
        pub version: ::prost::alloc::string::String,
    }
    /// Settings for updating the preview.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PreviewSettings {
        /// Indicates whether or not to run certain operations, such as transactions,
        /// in sandbox mode. By default, preview requests run these operations in
        /// sandbox mode. In other words, the default value for `sandbox` is `true`.
        #[prost(message, optional, tag="1")]
        pub sandbox: ::core::option::Option<bool>,
    }
    /// Data source used to created the preview.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// List of files sent to the server at a time. This is a list of config
        /// files or data files.
        /// 1. The first request must be a ConfigFiles.
        /// 2. The first request must have a ConfigFile with 'settings'.
        /// 3. The first request must have a ConfigFile with 'manifest'.
        /// 4. The webhook ConfigFile corresponding to inline cloud function must be
        ///     streamed before the DataFile corresponding to its source code.
        #[prost(message, tag="5")]
        Files(super::Files),
        /// Content sourced from the project draft.
        #[prost(message, tag="6")]
        Draft(ContentFromDraft),
        /// Content sourced from the an exiting version.
        #[prost(message, tag="7")]
        SubmittedVersion(ContentFromSubmittedVersion),
    }
}
/// Definition of preview resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Preview {
    /// The unique identifier of the preview.
    /// Format: `projects/{project}/preview`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Validation results associated with the user project preview content.
    #[prost(message, optional, tag="2")]
    pub validation_results: ::core::option::Option<ValidationResults>,
    /// The simulator URL to test the user preview.
    #[prost(string, tag="3")]
    pub simulator_url: ::prost::alloc::string::String,
}
/// Streaming RPC request for CreateVersion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateVersionRequest {
    /// Required. The parent resource name in the format `projects/{project}`. The
    /// `{project}` is the cloud project ID associated with the project.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. List of files sent to the server at a time. This is a list of config files
    /// or data files.
    /// 1. The first request must be a ConfigFiles.
    /// 2. The first request must have a ConfigFile with 'settings'.
    /// 3. The first request must have a ConfigFile with 'manifest'.
    /// 4. The webhook ConfigFile corresponding to inline cloud function must be
    ///     streamed before the DataFile corresponding to its source code.
    #[prost(message, optional, tag="5")]
    pub files: ::core::option::Option<Files>,
    /// Optional. The release channel to deploy the version, if specified. The supported
    /// built in release channels are actions.channels.Production,
    /// actions.channels.ClosedBeta, actions.channels.Alpha.
    /// .
    #[prost(string, tag="4")]
    pub release_channel: ::prost::alloc::string::String,
}
/// RPC request for ReadDraft.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadDraftRequest {
    /// Required. The name of the resource in the format `projects/{project}/draft`. The
    /// `{project}` is the cloud project ID associated with the project.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The version of the crypto key used to encrypt the account linking OAuth
    /// client secret. If not specified, the primary key version is used for
    /// encryption. Only relevant for projects with account linking with client
    /// secret.
    #[prost(string, tag="2")]
    pub client_secret_encryption_key_version: ::prost::alloc::string::String,
}
/// Streaming RPC response for ReadDraft.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadDraftResponse {
    /// List of files sent from the server at a time.
    #[prost(message, optional, tag="3")]
    pub files: ::core::option::Option<Files>,
}
/// RPC request for ReadVersion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadVersionRequest {
    /// Required. The name of the version resource in the format
    /// `projects/{project}/versions/{version}`. `{project}` is the
    /// cloud project ID associated with the project, `{version}` is the
    /// identifier of the version being read.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The version of the crypto key used to encrypt the account linking OAuth
    /// client secret. If not specified, the primary key version is used for
    /// encryption. Only relevant for projects with account linking with client
    /// secret.
    #[prost(string, tag="2")]
    pub client_secret_encryption_key_version: ::prost::alloc::string::String,
}
/// Streaming RPC response for ReadVersion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadVersionResponse {
    /// List of files sent from the server at a time.
    #[prost(message, optional, tag="1")]
    pub files: ::core::option::Option<Files>,
}
/// RPC request for EncryptSecret.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptSecretRequest {
    /// Required. The account linking client secret plaintext.
    #[prost(string, tag="1")]
    pub client_secret: ::prost::alloc::string::String,
}
/// RPC response for EncryptSecret.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptSecretResponse {
    /// Contains the encrypted account linking client secret and the key version
    /// used to encrypt the secret.
    #[prost(message, optional, tag="1")]
    pub account_linking_secret: ::core::option::Option<AccountLinkingSecret>,
}
/// RPC request for DecryptSecret.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecryptSecretRequest {
    /// Required. The account linking client secret ciphertext.
    #[prost(bytes="bytes", tag="1")]
    pub encrypted_client_secret: ::prost::bytes::Bytes,
}
/// RPC response for DecryptSecret.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecryptSecretResponse {
    /// The account linking client secret plaintext.
    #[prost(string, tag="1")]
    pub client_secret: ::prost::alloc::string::String,
}
/// RPC request for ListSampleProjects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSampleProjectsRequest {
    /// Optional. The maximum number of sample projects to return. The service may return
    /// fewer than this value.
    /// If unspecified, at most 1000 sample projects will be returned. Values above
    /// 1000 will be coerced to 1000.
    #[prost(int32, tag="1")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous 'ListSampleProjects' call.
    /// Provide this to retrieve the subsequent page.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
}
/// RPC response for ListSampleProjects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSampleProjectsResponse {
    /// The list of sample projects supported.
    #[prost(message, repeated, tag="1")]
    pub sample_projects: ::prost::alloc::vec::Vec<SampleProject>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Definition of sample project resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SampleProject {
    /// The name of the sample project.
    /// Format: `sampleProjects/{sample_project}`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The URL to the zip file where the sample is hosted.
    #[prost(string, tag="2")]
    pub hosted_url: ::prost::alloc::string::String,
    /// The description of the sample project.
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
}
/// RPC request for listing release channels
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReleaseChannelsRequest {
    /// Required. The name of the resource in the format `projects/{project}`. The
    /// `{project}` is the cloud project ID associated with the project.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of release channels to return. The service may return
    /// fewer than this value. If unspecified, at most 50 release channels will be
    /// returned.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListReleaseChannels` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListReleaseChannels`
    /// must match the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// RPC response for listing release channels
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReleaseChannelsResponse {
    /// List of the release channels for the given project id.
    #[prost(message, repeated, tag="1")]
    pub release_channels: ::prost::alloc::vec::Vec<ReleaseChannel>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// RPC request for listing versions
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVersionsRequest {
    /// Required. The name of the resource in the format `projects/{project}`. The
    /// `{project}` is the cloud project ID associated with the project.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of versions to return. The service may return
    /// fewer than this value. If unspecified, at most 50 versions will be
    /// returned.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListVersions` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListVersions`
    /// must match the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// RPC response for listing versions
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVersionsResponse {
    /// List of the versions for the given project id.
    #[prost(message, repeated, tag="1")]
    pub versions: ::prost::alloc::vec::Vec<Version>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod actions_sdk_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Actions SDK API which allows developers to build projects using the SDK.
    #[derive(Debug, Clone)]
    pub struct ActionsSdkClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ActionsSdkClient<T>
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
        ) -> ActionsSdkClient<InterceptedService<T, F>>
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
            ActionsSdkClient::new(InterceptedService::new(inner, interceptor))
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
        /// Updates the project draft based on the model.
        pub async fn write_draft(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::WriteDraftRequest>,
        ) -> Result<tonic::Response<super::Draft>, tonic::Status> {
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
                "/google.actions.sdk.v2.ActionsSdk/WriteDraft",
            );
            self.inner
                .client_streaming(request.into_streaming_request(), path, codec)
                .await
        }
        /// Updates the user's project preview based on the model.
        pub async fn write_preview(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::WritePreviewRequest,
            >,
        ) -> Result<tonic::Response<super::Preview>, tonic::Status> {
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
                "/google.actions.sdk.v2.ActionsSdk/WritePreview",
            );
            self.inner
                .client_streaming(request.into_streaming_request(), path, codec)
                .await
        }
        /// Creates a project version based on the model and triggers deployment to the
        /// specified release channel, if specified.
        pub async fn create_version(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::CreateVersionRequest,
            >,
        ) -> Result<tonic::Response<super::Version>, tonic::Status> {
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
                "/google.actions.sdk.v2.ActionsSdk/CreateVersion",
            );
            self.inner
                .client_streaming(request.into_streaming_request(), path, codec)
                .await
        }
        /// Reads the entire content of the project draft.
        pub async fn read_draft(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadDraftRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ReadDraftResponse>>,
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
                "/google.actions.sdk.v2.ActionsSdk/ReadDraft",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Reads the entire content of a project version.
        pub async fn read_version(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadVersionRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ReadVersionResponse>>,
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
                "/google.actions.sdk.v2.ActionsSdk/ReadVersion",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Encrypts the OAuth client secret used in account linking flows.
        /// This can be used to encrypt the client secret for the first time (e.g.
        /// before the first push or after changing the client secret) or to re-encrypt
        /// a client secret using the latest primary key version (considering key
        /// rotation).
        pub async fn encrypt_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::EncryptSecretRequest>,
        ) -> Result<tonic::Response<super::EncryptSecretResponse>, tonic::Status> {
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
                "/google.actions.sdk.v2.ActionsSdk/EncryptSecret",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Decrypts the OAuth client secret used in account linking flows.
        /// This can be used to view the client secret (e.g. after pulling a project).
        pub async fn decrypt_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::DecryptSecretRequest>,
        ) -> Result<tonic::Response<super::DecryptSecretResponse>, tonic::Status> {
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
                "/google.actions.sdk.v2.ActionsSdk/DecryptSecret",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all the sample projects supported by the gactions CLI.
        pub async fn list_sample_projects(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSampleProjectsRequest>,
        ) -> Result<tonic::Response<super::ListSampleProjectsResponse>, tonic::Status> {
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
                "/google.actions.sdk.v2.ActionsSdk/ListSampleProjects",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all release channels and corresponding versions, if any.
        pub async fn list_release_channels(
            &mut self,
            request: impl tonic::IntoRequest<super::ListReleaseChannelsRequest>,
        ) -> Result<tonic::Response<super::ListReleaseChannelsResponse>, tonic::Status> {
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
                "/google.actions.sdk.v2.ActionsSdk/ListReleaseChannels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all versions and their current states.
        pub async fn list_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVersionsRequest>,
        ) -> Result<tonic::Response<super::ListVersionsResponse>, tonic::Status> {
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
                "/google.actions.sdk.v2.ActionsSdk/ListVersions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Contains information about execution event which happened during processing
/// Actions Builder conversation request. For an overview of the stages involved
/// in a conversation request, see
/// <https://developers.google.com/assistant/conversational/actions.>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionEvent {
    /// Timestamp when the event happened.
    #[prost(message, optional, tag="1")]
    pub event_time: ::core::option::Option<::prost_types::Timestamp>,
    /// State of the execution during this event.
    #[prost(message, optional, tag="2")]
    pub execution_state: ::core::option::Option<ExecutionState>,
    /// Resulting status of particular execution step.
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<super::super::super::rpc::Status>,
    /// List of warnings generated during execution of this Event. Warnings are
    /// tips for the developer discovered during the conversation request. These
    /// are usually non-critical and do not halt the execution of the request. For
    /// example, a warnings might be generated when webhook tries to override a
    /// custom Type which does not exist. Errors are reported as a failed status
    /// code, but warnings can be present even when the status is OK.
    #[prost(string, repeated, tag="17")]
    pub warning_messages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Detailed information specific to different of events that may be involved
    /// in processing a conversation round. The field set here defines the type of
    /// this event.
    #[prost(oneof="execution_event::EventData", tags="4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16")]
    pub event_data: ::core::option::Option<execution_event::EventData>,
}
/// Nested message and enum types in `ExecutionEvent`.
pub mod execution_event {
    /// Detailed information specific to different of events that may be involved
    /// in processing a conversation round. The field set here defines the type of
    /// this event.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EventData {
        /// User input handling event.
        #[prost(message, tag="4")]
        UserInput(super::UserConversationInput),
        /// Intent matching event.
        #[prost(message, tag="5")]
        IntentMatch(super::IntentMatch),
        /// Condition evaluation event.
        #[prost(message, tag="6")]
        ConditionsEvaluated(super::ConditionsEvaluated),
        /// OnSceneEnter execution event.
        #[prost(message, tag="7")]
        OnSceneEnter(super::OnSceneEnter),
        /// Webhook request dispatch event.
        #[prost(message, tag="8")]
        WebhookRequest(super::WebhookRequest),
        /// Webhook response receipt event.
        #[prost(message, tag="9")]
        WebhookResponse(super::WebhookResponse),
        /// Webhook-initiated transition event.
        #[prost(message, tag="10")]
        WebhookInitiatedTransition(super::WebhookInitiatedTransition),
        /// Slot matching event.
        #[prost(message, tag="11")]
        SlotMatch(super::SlotMatch),
        /// Slot requesting event.
        #[prost(message, tag="12")]
        SlotRequested(super::SlotRequested),
        /// Slot validation event.
        #[prost(message, tag="13")]
        SlotValidated(super::SlotValidated),
        /// Form filling event.
        #[prost(message, tag="14")]
        FormFilled(super::FormFilled),
        /// Waiting-for-user-input event.
        #[prost(message, tag="15")]
        WaitingUserInput(super::WaitingForUserInput),
        /// End-of-conversation event.
        #[prost(message, tag="16")]
        EndConversation(super::EndConversation),
    }
}
/// Current state of the execution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionState {
    /// ID of the scene which is currently  active.
    #[prost(string, tag="1")]
    pub current_scene_id: ::prost::alloc::string::String,
    /// State of the session storage:
    /// <https://developers.google.com/assistant/conversational/storage-session>
    #[prost(message, optional, tag="2")]
    pub session_storage: ::core::option::Option<::prost_types::Struct>,
    /// State of the slots filling, if applicable:
    /// <https://developers.google.com/assistant/conversational/scenes#slot_filling>
    #[prost(message, optional, tag="5")]
    pub slots: ::core::option::Option<Slots>,
    /// Prompt queue:
    /// <https://developers.google.com/assistant/conversational/prompts>
    #[prost(message, repeated, tag="7")]
    pub prompt_queue: ::prost::alloc::vec::Vec<conversation::Prompt>,
    /// State of the user storage:
    /// <https://developers.google.com/assistant/conversational/storage-user>
    #[prost(message, optional, tag="6")]
    pub user_storage: ::core::option::Option<::prost_types::Struct>,
    /// State of the home storage:
    /// <https://developers.google.com/assistant/conversational/storage-home>
    #[prost(message, optional, tag="8")]
    pub household_storage: ::core::option::Option<::prost_types::Struct>,
}
/// Represents the current state of a the scene's slots.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Slots {
    /// The current status of slot filling.
    #[prost(enumeration="conversation::SlotFillingStatus", tag="2")]
    pub status: i32,
    /// The slots associated with the current scene.
    #[prost(btree_map="string, message", tag="3")]
    pub slots: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, conversation::Slot>,
}
/// Information related to user input.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserConversationInput {
    /// Type of user input. E.g. keyboard, voice, touch, etc.
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    /// Original text input from the user.
    #[prost(string, tag="2")]
    pub original_query: ::prost::alloc::string::String,
}
/// Information about triggered intent match (global or within a scene):
/// <https://developers.google.com/assistant/conversational/intents>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntentMatch {
    /// Intent id which triggered this interaction.
    #[prost(string, tag="1")]
    pub intent_id: ::prost::alloc::string::String,
    /// Parameters of intent which triggered this interaction.
    #[prost(btree_map="string, message", tag="5")]
    pub intent_parameters: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, conversation::IntentParameterValue>,
    /// Name of the handler attached to this interaction.
    #[prost(string, tag="3")]
    pub handler: ::prost::alloc::string::String,
    /// Scene to which this interaction leads to.
    #[prost(string, tag="4")]
    pub next_scene_id: ::prost::alloc::string::String,
}
/// Results of conditions evaluation:
/// <https://developers.google.com/assistant/conversational/scenes#conditions>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConditionsEvaluated {
    /// List of conditions which were evaluated to 'false'.
    #[prost(message, repeated, tag="1")]
    pub failed_conditions: ::prost::alloc::vec::Vec<Condition>,
    /// The first condition which was evaluated to 'true', if any.
    #[prost(message, optional, tag="2")]
    pub success_condition: ::core::option::Option<Condition>,
}
/// Evaluated condition.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Condition {
    /// Expression specified in this condition.
    #[prost(string, tag="1")]
    pub expression: ::prost::alloc::string::String,
    /// Handler name specified in evaluated condition.
    #[prost(string, tag="2")]
    pub handler: ::prost::alloc::string::String,
    /// Destination scene specified in evaluated condition.
    #[prost(string, tag="3")]
    pub next_scene_id: ::prost::alloc::string::String,
}
/// Information about execution of onSceneEnter stage:
/// <https://developers.google.com/assistant/conversational/scenes#on_enter>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnSceneEnter {
    /// Handler name specified in onSceneEnter event.
    #[prost(string, tag="1")]
    pub handler: ::prost::alloc::string::String,
}
/// Event triggered by destination scene returned from webhook:
/// <https://developers.google.com/assistant/conversational/webhooks#transition_scenes>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebhookInitiatedTransition {
    /// ID of the scene the transition is leading to.
    #[prost(string, tag="1")]
    pub next_scene_id: ::prost::alloc::string::String,
}
/// Information about a request dispatched to the Action webhook:
/// <https://developers.google.com/assistant/conversational/webhooks#payloads>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebhookRequest {
    /// Payload of the webhook request.
    #[prost(string, tag="1")]
    pub request_json: ::prost::alloc::string::String,
}
/// Information about a response received from the Action webhook:
/// <https://developers.google.com/assistant/conversational/webhooks#payloads>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebhookResponse {
    /// Payload of the webhook response.
    #[prost(string, tag="1")]
    pub response_json: ::prost::alloc::string::String,
}
/// Information about matched slot(s):
/// <https://developers.google.com/assistant/conversational/scenes#slot_filling>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SlotMatch {
    /// Parameters extracted by NLU from user input.
    #[prost(btree_map="string, message", tag="2")]
    pub nlu_parameters: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, conversation::IntentParameterValue>,
}
/// Information about currently requested slot:
/// <https://developers.google.com/assistant/conversational/scenes#slot_filling>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SlotRequested {
    /// Name of the requested slot.
    #[prost(string, tag="1")]
    pub slot: ::prost::alloc::string::String,
    /// Slot prompt.
    #[prost(message, optional, tag="3")]
    pub prompt: ::core::option::Option<conversation::Prompt>,
}
/// Event which happens after webhook validation was finished for slot(s):
/// <https://developers.google.com/assistant/conversational/scenes#slot_filling>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SlotValidated {
}
/// Event which happens when form is fully filled:
/// <https://developers.google.com/assistant/conversational/scenes#slot_filling>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FormFilled {
}
/// Event which happens when system needs user input:
/// <https://developers.google.com/assistant/conversational/scenes#input>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitingForUserInput {
}
/// Event which informs that conversation with agent was ended.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndConversation {
}
/// Request for playing a round of the conversation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendInteractionRequest {
    /// Required. The project being tested, indicated by the Project ID.
    /// Format: projects/{project}
    #[prost(string, tag="1")]
    pub project: ::prost::alloc::string::String,
    /// Required. Input provided by the user.
    #[prost(message, optional, tag="2")]
    pub input: ::core::option::Option<UserInput>,
    /// Required. Properties of the device used for interacting with the Action.
    #[prost(message, optional, tag="3")]
    pub device_properties: ::core::option::Option<DeviceProperties>,
    /// Opaque token that must be passed as received from SendInteractionResponse
    /// on the previous interaction. This can be left unset in order to start a new
    /// conversation, either as the first interaction of a testing session or to
    /// abandon a previous conversation and start a new one.
    #[prost(string, tag="4")]
    pub conversation_token: ::prost::alloc::string::String,
}
/// User input provided on a conversation round.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInput {
    /// Content of the input sent by the user.
    #[prost(string, tag="1")]
    pub query: ::prost::alloc::string::String,
    /// Type of the input.
    #[prost(enumeration="user_input::InputType", tag="2")]
    pub r#type: i32,
}
/// Nested message and enum types in `UserInput`.
pub mod user_input {
    /// Indicates the input source, typed query or voice query.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum InputType {
        /// Unspecified input source.
        Unspecified = 0,
        /// Query from a GUI interaction.
        Touch = 1,
        /// Voice query.
        Voice = 2,
        /// Typed query.
        Keyboard = 3,
        /// The action was triggered by a URL link.
        Url = 4,
    }
    impl InputType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                InputType::Unspecified => "INPUT_TYPE_UNSPECIFIED",
                InputType::Touch => "TOUCH",
                InputType::Voice => "VOICE",
                InputType::Keyboard => "KEYBOARD",
                InputType::Url => "URL",
            }
        }
    }
}
/// Properties of device relevant to a conversation round.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceProperties {
    /// Surface used for interacting with the Action.
    #[prost(enumeration="device_properties::Surface", tag="1")]
    pub surface: i32,
    /// Device location such as latitude, longitude, and formatted address.
    #[prost(message, optional, tag="2")]
    pub location: ::core::option::Option<Location>,
    /// Locale as set on the device.
    /// The format should follow BCP 47: <https://tools.ietf.org/html/bcp47>
    /// Examples: en, en-US, es-419 (more examples at
    /// <https://tools.ietf.org/html/bcp47#appendix-A>).
    #[prost(string, tag="3")]
    pub locale: ::prost::alloc::string::String,
    /// Time zone as set on the device.
    /// The format should follow the IANA Time Zone Database, e.g.
    /// "America/New_York": <https://www.iana.org/time-zones>
    #[prost(string, tag="4")]
    pub time_zone: ::prost::alloc::string::String,
}
/// Nested message and enum types in `DeviceProperties`.
pub mod device_properties {
    /// Possible surfaces used to interact with the Action.
    /// Additional values may be included in the future.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Surface {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// Speaker (e.g. Google Home).
        Speaker = 1,
        /// Phone.
        Phone = 2,
        /// Allo Chat.
        Allo = 3,
        /// Smart Display Device.
        SmartDisplay = 4,
        /// KaiOS.
        KaiOs = 5,
    }
    impl Surface {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Surface::Unspecified => "SURFACE_UNSPECIFIED",
                Surface::Speaker => "SPEAKER",
                Surface::Phone => "PHONE",
                Surface::Allo => "ALLO",
                Surface::SmartDisplay => "SMART_DISPLAY",
                Surface::KaiOs => "KAI_OS",
            }
        }
    }
}
/// Container that represents a location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Location {
    /// Geo coordinates.
    /// Requires the \[DEVICE_PRECISE_LOCATION\]
    /// \[google.actions.v2.Permission.DEVICE_PRECISE_LOCATION\] permission.
    #[prost(message, optional, tag="1")]
    pub coordinates: ::core::option::Option<super::super::super::r#type::LatLng>,
    /// Display address, e.g., "1600 Amphitheatre Pkwy, Mountain View, CA 94043".
    /// Requires the \[DEVICE_PRECISE_LOCATION\]
    /// \[google.actions.v2.Permission.DEVICE_PRECISE_LOCATION\] permission.
    #[prost(string, tag="2")]
    pub formatted_address: ::prost::alloc::string::String,
    /// Zip code.
    /// Requires the \[DEVICE_PRECISE_LOCATION\]
    /// \[google.actions.v2.Permission.DEVICE_PRECISE_LOCATION\] or
    /// \[DEVICE_COARSE_LOCATION\]
    /// \[google.actions.v2.Permission.DEVICE_COARSE_LOCATION\] permission.
    #[prost(string, tag="3")]
    pub zip_code: ::prost::alloc::string::String,
    /// City.
    /// Requires the \[DEVICE_PRECISE_LOCATION\]
    /// \[google.actions.v2.Permission.DEVICE_PRECISE_LOCATION\] or
    /// \[DEVICE_COARSE_LOCATION\]
    /// \[google.actions.v2.Permission.DEVICE_COARSE_LOCATION\] permission.
    #[prost(string, tag="4")]
    pub city: ::prost::alloc::string::String,
}
/// Response to a round of the conversation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendInteractionResponse {
    /// Output provided to the user.
    #[prost(message, optional, tag="1")]
    pub output: ::core::option::Option<Output>,
    /// Diagnostics information that explains how the request was handled.
    #[prost(message, optional, tag="2")]
    pub diagnostics: ::core::option::Option<Diagnostics>,
    /// Opaque token to be set on SendInteractionRequest on the next RPC call in
    /// order to continue the same conversation.
    #[prost(string, tag="3")]
    pub conversation_token: ::prost::alloc::string::String,
}
/// User-visible output to the conversation round.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    /// Spoken response sent to user as a plain string.
    #[prost(string, tag="1")]
    pub text: ::prost::alloc::string::String,
    /// Speech content produced by the Action. This may include markup elements
    /// such as SSML.
    #[prost(string, repeated, tag="2")]
    pub speech: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Interactive Canvas content.
    #[prost(message, optional, tag="3")]
    pub canvas: ::core::option::Option<conversation::Canvas>,
    /// State of the prompt at the end of the conversation round.
    /// More information about the prompt:
    /// <https://developers.google.com/assistant/conversational/prompts>
    #[prost(message, optional, tag="4")]
    pub actions_builder_prompt: ::core::option::Option<conversation::Prompt>,
}
/// Diagnostics information related to the conversation round.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Diagnostics {
    /// List of events with details about processing of the conversation round
    /// throughout the stages of the Actions Builder interaction model.
    /// Populated for Actions Builder & Actions SDK apps only.
    #[prost(message, repeated, tag="1")]
    pub actions_builder_events: ::prost::alloc::vec::Vec<ExecutionEvent>,
}
/// Request for finding matching intents.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchIntentsRequest {
    /// Required. The project being tested, indicated by the Project ID.
    /// Format: projects/{project}
    #[prost(string, tag="1")]
    pub project: ::prost::alloc::string::String,
    /// Required. User query as plain text.
    #[prost(string, tag="2")]
    pub query: ::prost::alloc::string::String,
    /// Required. Locale to use to evaluate the query, such as "en".
    /// The format should follow BCP 47: <https://tools.ietf.org/html/bcp47>
    /// See the list of supported languages in
    /// <https://developers.google.com/assistant/console/languages-locales>
    #[prost(string, tag="3")]
    pub locale: ::prost::alloc::string::String,
}
/// Response for finding matching intents.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchIntentsResponse {
    /// Intents matched, ordered from most to least relevant. Only the first
    /// 50 matches are returned.
    #[prost(message, repeated, tag="1")]
    pub matched_intents: ::prost::alloc::vec::Vec<conversation::Intent>,
}
/// Request for setting Web & App Activity preferences.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetWebAndAppActivityControlRequest {
    /// Whether the setting should be set to an enabled or disabled state.
    #[prost(bool, tag="1")]
    pub enabled: bool,
}
/// Generated client implementations.
pub mod actions_testing_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Actions Testing API which allows developers to run automated tests.
    #[derive(Debug, Clone)]
    pub struct ActionsTestingClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ActionsTestingClient<T>
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
        ) -> ActionsTestingClient<InterceptedService<T, F>>
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
            ActionsTestingClient::new(InterceptedService::new(inner, interceptor))
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
        /// Plays one round of the conversation.
        pub async fn send_interaction(
            &mut self,
            request: impl tonic::IntoRequest<super::SendInteractionRequest>,
        ) -> Result<tonic::Response<super::SendInteractionResponse>, tonic::Status> {
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
                "/google.actions.sdk.v2.ActionsTesting/SendInteraction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Finds the intents that match a given query.
        pub async fn match_intents(
            &mut self,
            request: impl tonic::IntoRequest<super::MatchIntentsRequest>,
        ) -> Result<tonic::Response<super::MatchIntentsResponse>, tonic::Status> {
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
                "/google.actions.sdk.v2.ActionsTesting/MatchIntents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the Web & App Activity control on a service account.
        ///
        /// It is necessary to have this setting enabled in order to use call Actions.
        /// The setting is originally disabled for service accounts, and it is
        /// preserved until set to a different value. This means it only needs to be
        /// enabled once per account (and not necessarily once per test), unless it is
        /// later disabled.
        ///
        /// Returns an error if the caller is not a service account. User accounts can
        /// change this setting via the Activity Controls page. See
        /// https://support.google.com/websearch/answer/54068.
        pub async fn set_web_and_app_activity_control(
            &mut self,
            request: impl tonic::IntoRequest<super::SetWebAndAppActivityControlRequest>,
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
                "/google.actions.sdk.v2.ActionsTesting/SetWebAndAppActivityControl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
