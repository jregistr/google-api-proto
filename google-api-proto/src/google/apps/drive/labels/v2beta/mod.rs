/// Normalized internal-only message that identifies the exact exception that
/// caused the error on the server.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExceptionDetail {
    /// The type of exception that occurred.
    /// required
    #[prost(enumeration = "ExceptionType", tag = "1")]
    pub error_type: i32,
}
/// Every ExceptionType maps to one and only one Exception class. This allows
/// internal clients to identify the exact server exception that caused the
/// error for debugging and logging purposes.
/// Add new ExceptionTypes to EXCEPTION_TYPE_TO_ERROR_CODE_MAP in
/// j/c/g/apps/boq/metadata/model/service/exceptions/CategoryExceptionHelper
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExceptionType {
    /// Unknown ExceptionType.
    Unspecified = 0,
    /// The required field is missing.
    FieldRequired = 1,
    /// Unable to create a metamodel with the given ID because it already exists.
    MetamodelAlreadyExists = 2,
    /// Metamodel was not found
    MetamodelNotFound = 3,
    /// Metamodel state transition isn't allowed.
    IllegalMetamodelStateTransition = 4,
    /// Metamodel deprecation policy is invalid.
    InvalidMetamodelDeprecationPolicy = 5,
    /// Cannot delete a metamodel due to the pending deprecation policy.
    MetamodelDeletionDeniedUntil = 6,
    /// A Field value is invalid.
    InvalidField = 7,
    /// Precondition failed when updating a metamodel
    MetamodelPreconditionFailed = 8,
    /// Multiple fields had the same key.
    DuplicateFieldKey = 9,
    /// Removing a field from a Metamodel (e.g. a published Metamodel) is not
    /// permitted.
    IllegalFieldRemoval = 10,
    /// Cannot specify field options for a different field type.
    IllegalFieldOptionsForField = 11,
    /// Some changes are not supported
    UnsupportedChangeToPublishedMetamodel = 12,
    /// Cannot change the metamodel state in an update
    IllegalMetamodelStateTransitionInUpdate = 13,
    /// The page token is expired
    PageTokenExpired = 14,
    /// The user is not authorized to make the request.
    NotAuthorized = 15,
    /// Illegal field state transition
    IllegalFieldStateTransition = 16,
    /// Illegal choice set option state transition
    IllegalChoiceSetOptionStateTransition = 17,
    /// Invalid choice set options
    InvalidChoiceSetOptions = 18,
    /// Invalid field key
    InvalidFieldKey = 19,
    /// A specified property on a field is outside the allowed range.
    InvalidFieldPropertyRange = 20,
    /// A localized string wasn't valid. This may be because the locale is invalid,
    /// its missing a default value, or the translation is empty for a set locale.
    InvalidLocalizedString = 21,
    /// cannot change a property on a published field
    IllegalChangeToPublishedField = 22,
    /// A field update is not inclusive of the previous value
    InvalidFieldUpdateNotInclusive = 23,
    /// A field update is not inclusive of the previous value
    InvalidChoiceSetState = 24,
    /// An unknown error occurred
    InternalServerError = 500,
}
impl ExceptionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExceptionType::Unspecified => "EXCEPTION_TYPE_UNSPECIFIED",
            ExceptionType::FieldRequired => "FIELD_REQUIRED",
            ExceptionType::MetamodelAlreadyExists => "METAMODEL_ALREADY_EXISTS",
            ExceptionType::MetamodelNotFound => "METAMODEL_NOT_FOUND",
            ExceptionType::IllegalMetamodelStateTransition => {
                "ILLEGAL_METAMODEL_STATE_TRANSITION"
            }
            ExceptionType::InvalidMetamodelDeprecationPolicy => {
                "INVALID_METAMODEL_DEPRECATION_POLICY"
            }
            ExceptionType::MetamodelDeletionDeniedUntil => {
                "METAMODEL_DELETION_DENIED_UNTIL"
            }
            ExceptionType::InvalidField => "INVALID_FIELD",
            ExceptionType::MetamodelPreconditionFailed => "METAMODEL_PRECONDITION_FAILED",
            ExceptionType::DuplicateFieldKey => "DUPLICATE_FIELD_KEY",
            ExceptionType::IllegalFieldRemoval => "ILLEGAL_FIELD_REMOVAL",
            ExceptionType::IllegalFieldOptionsForField => {
                "ILLEGAL_FIELD_OPTIONS_FOR_FIELD"
            }
            ExceptionType::UnsupportedChangeToPublishedMetamodel => {
                "UNSUPPORTED_CHANGE_TO_PUBLISHED_METAMODEL"
            }
            ExceptionType::IllegalMetamodelStateTransitionInUpdate => {
                "ILLEGAL_METAMODEL_STATE_TRANSITION_IN_UPDATE"
            }
            ExceptionType::PageTokenExpired => "PAGE_TOKEN_EXPIRED",
            ExceptionType::NotAuthorized => "NOT_AUTHORIZED",
            ExceptionType::IllegalFieldStateTransition => {
                "ILLEGAL_FIELD_STATE_TRANSITION"
            }
            ExceptionType::IllegalChoiceSetOptionStateTransition => {
                "ILLEGAL_CHOICE_SET_OPTION_STATE_TRANSITION"
            }
            ExceptionType::InvalidChoiceSetOptions => "INVALID_CHOICE_SET_OPTIONS",
            ExceptionType::InvalidFieldKey => "INVALID_FIELD_KEY",
            ExceptionType::InvalidFieldPropertyRange => "INVALID_FIELD_PROPERTY_RANGE",
            ExceptionType::InvalidLocalizedString => "INVALID_LOCALIZED_STRING",
            ExceptionType::IllegalChangeToPublishedField => {
                "ILLEGAL_CHANGE_TO_PUBLISHED_FIELD"
            }
            ExceptionType::InvalidFieldUpdateNotInclusive => {
                "INVALID_FIELD_UPDATE_NOT_INCLUSIVE"
            }
            ExceptionType::InvalidChoiceSetState => "INVALID_CHOICE_SET_STATE",
            ExceptionType::InternalServerError => "INTERNAL_SERVER_ERROR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXCEPTION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "FIELD_REQUIRED" => Some(Self::FieldRequired),
            "METAMODEL_ALREADY_EXISTS" => Some(Self::MetamodelAlreadyExists),
            "METAMODEL_NOT_FOUND" => Some(Self::MetamodelNotFound),
            "ILLEGAL_METAMODEL_STATE_TRANSITION" => {
                Some(Self::IllegalMetamodelStateTransition)
            }
            "INVALID_METAMODEL_DEPRECATION_POLICY" => {
                Some(Self::InvalidMetamodelDeprecationPolicy)
            }
            "METAMODEL_DELETION_DENIED_UNTIL" => Some(Self::MetamodelDeletionDeniedUntil),
            "INVALID_FIELD" => Some(Self::InvalidField),
            "METAMODEL_PRECONDITION_FAILED" => Some(Self::MetamodelPreconditionFailed),
            "DUPLICATE_FIELD_KEY" => Some(Self::DuplicateFieldKey),
            "ILLEGAL_FIELD_REMOVAL" => Some(Self::IllegalFieldRemoval),
            "ILLEGAL_FIELD_OPTIONS_FOR_FIELD" => Some(Self::IllegalFieldOptionsForField),
            "UNSUPPORTED_CHANGE_TO_PUBLISHED_METAMODEL" => {
                Some(Self::UnsupportedChangeToPublishedMetamodel)
            }
            "ILLEGAL_METAMODEL_STATE_TRANSITION_IN_UPDATE" => {
                Some(Self::IllegalMetamodelStateTransitionInUpdate)
            }
            "PAGE_TOKEN_EXPIRED" => Some(Self::PageTokenExpired),
            "NOT_AUTHORIZED" => Some(Self::NotAuthorized),
            "ILLEGAL_FIELD_STATE_TRANSITION" => Some(Self::IllegalFieldStateTransition),
            "ILLEGAL_CHOICE_SET_OPTION_STATE_TRANSITION" => {
                Some(Self::IllegalChoiceSetOptionStateTransition)
            }
            "INVALID_CHOICE_SET_OPTIONS" => Some(Self::InvalidChoiceSetOptions),
            "INVALID_FIELD_KEY" => Some(Self::InvalidFieldKey),
            "INVALID_FIELD_PROPERTY_RANGE" => Some(Self::InvalidFieldPropertyRange),
            "INVALID_LOCALIZED_STRING" => Some(Self::InvalidLocalizedString),
            "ILLEGAL_CHANGE_TO_PUBLISHED_FIELD" => {
                Some(Self::IllegalChangeToPublishedField)
            }
            "INVALID_FIELD_UPDATE_NOT_INCLUSIVE" => {
                Some(Self::InvalidFieldUpdateNotInclusive)
            }
            "INVALID_CHOICE_SET_STATE" => Some(Self::InvalidChoiceSetState),
            "INTERNAL_SERVER_ERROR" => Some(Self::InternalServerError),
            _ => None,
        }
    }
}
/// The lifecycle state of an object, such as label, field, or choice. The
/// lifecycle enforces the following transitions:
///
/// * `UNPUBLISHED_DRAFT` (starting state)
/// * `UNPUBLISHED_DRAFT` -> `PUBLISHED`
/// * `UNPUBLISHED_DRAFT` -> (Deleted)
/// * `PUBLISHED` -> `DISABLED`
/// * `DISABLED` -> `PUBLISHED`
/// * `DISABLED` -> (Deleted)
///
/// The published and disabled states have some distinct characteristics:
///
/// * Published—Some kinds of changes might be made to an object in this state,
///    in which case `has_unpublished_changes` will be true. Also, some kinds of
///    changes are not permitted. Generally, any change that would invalidate or
///    cause new restrictions on existing metadata related to the label are
///    rejected.
/// * Disabled—When disabled, the configured `DisabledPolicy` takes effect.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Lifecycle {
    /// Output only. The state of the object associated with this lifecycle.
    #[prost(enumeration = "lifecycle::State", tag = "1")]
    pub state: i32,
    /// Output only. Whether the object associated with this lifecycle has
    /// unpublished changes.
    #[prost(bool, tag = "2")]
    pub has_unpublished_changes: bool,
    /// The policy that governs how to show a disabled label, field, or selection
    /// choice.
    #[prost(message, optional, tag = "3")]
    pub disabled_policy: ::core::option::Option<lifecycle::DisabledPolicy>,
}
/// Nested message and enum types in `Lifecycle`.
pub mod lifecycle {
    /// The policy that governs how to treat a disabled label, field, or selection
    /// choice in different contexts.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DisabledPolicy {
        /// Whether to hide this disabled object in the search menu for Drive items.
        ///
        /// * When `false`, the object is generally shown in the UI as disabled but
        /// it appears in the search results when searching for Drive items.
        /// * When `true`, the object is generally hidden in the UI when
        ///    searching for Drive items.
        #[prost(bool, tag = "1")]
        pub hide_in_search: bool,
        /// Whether to show this disabled object in the apply menu on Drive items.
        ///
        /// * When `true`, the object is generally shown in the UI as disabled
        ///    and is unselectable.
        /// * When `false`, the object is generally hidden in the UI.
        #[prost(bool, tag = "2")]
        pub show_in_apply: bool,
    }
    /// The state of the object associated with this lifecycle.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum State {
        /// Unknown State.
        Unspecified = 0,
        /// The initial state of an object. Once published, the object can never
        /// return to this state. Once an object is published, certain kinds of
        /// changes are no longer permitted.
        UnpublishedDraft = 1,
        /// The object has been published. The object might have unpublished draft
        /// changes as indicated by `has_unpublished_changes`.
        Published = 2,
        /// The object has been published and has since been disabled. The object
        /// might have unpublished draft changes as indicated by
        /// `has_unpublished_changes`.
        Disabled = 3,
        /// The object has been deleted.
        Deleted = 4,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::UnpublishedDraft => "UNPUBLISHED_DRAFT",
                State::Published => "PUBLISHED",
                State::Disabled => "DISABLED",
                State::Deleted => "DELETED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "UNPUBLISHED_DRAFT" => Some(Self::UnpublishedDraft),
                "PUBLISHED" => Some(Self::Published),
                "DISABLED" => Some(Self::Disabled),
                "DELETED" => Some(Self::Deleted),
                _ => None,
            }
        }
    }
}
/// Information about a user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInfo {
    /// The identifier for this user that can be used with the People API to get
    /// more information.
    /// For example, people/12345678.
    #[prost(string, tag = "1")]
    pub person: ::prost::alloc::string::String,
}
/// Badge status of the label.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BadgeConfig {
    /// The color of the badge. When not specified, no badge is rendered.
    /// The background, foreground, and solo (light and dark mode) colors set here
    /// are changed in the Drive UI into the closest recommended supported color.
    #[prost(message, optional, tag = "1")]
    pub color: ::core::option::Option<super::super::super::super::r#type::Color>,
    /// Override the default global priority of this badge.
    /// When set to 0, the default priority heuristic is used.
    #[prost(int64, tag = "2")]
    pub priority_override: i64,
}
/// The color derived from BadgeConfig and changed to the closest recommended
/// supported color.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BadgeColors {
    /// Output only. Badge background that pairs with the foreground.
    #[prost(message, optional, tag = "1")]
    pub background_color: ::core::option::Option<
        super::super::super::super::r#type::Color,
    >,
    /// Output only. Badge foreground that pairs with the background.
    #[prost(message, optional, tag = "2")]
    pub foreground_color: ::core::option::Option<
        super::super::super::super::r#type::Color,
    >,
    /// Output only. Color that can be used for text without a background.
    #[prost(message, optional, tag = "3")]
    pub solo_color: ::core::option::Option<super::super::super::super::r#type::Color>,
}
/// Contains information about whether a label component should be considered
/// locked.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockStatus {
    /// Output only. Indicates whether this label component is the (direct) target
    /// of a LabelLock.  A label component can be implicitly locked even if it's
    /// not the direct target of a LabelLock, in which case this field is set to
    /// false.
    #[prost(bool, tag = "1")]
    pub locked: bool,
}
/// Defines a field that has a display name, data type, and other
/// configuration options. This field defines the kind of metadata that may be
/// set on a Drive item.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Field {
    /// Output only. The key of a field, unique within a label or library.
    ///
    /// This value is autogenerated. Matches the regex: `(\[a-zA-Z0-9\])+`
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Output only. The key to use when constructing Drive search queries to find
    /// files based on values defined for this field on files.
    /// For example, "`{query_key}` > 2001-01-01".
    #[prost(string, tag = "2")]
    pub query_key: ::prost::alloc::string::String,
    /// The basic properties of the field.
    #[prost(message, optional, tag = "3")]
    pub properties: ::core::option::Option<field::Properties>,
    /// Output only. The lifecycle of this field.
    #[prost(message, optional, tag = "4")]
    pub lifecycle: ::core::option::Option<Lifecycle>,
    /// Output only. UI display hints for rendering a field.
    #[prost(message, optional, tag = "5")]
    pub display_hints: ::core::option::Option<field::DisplayHints>,
    /// Output only. The capabilities this user has when editing this field.
    #[prost(message, optional, tag = "6")]
    pub schema_capabilities: ::core::option::Option<field::SchemaCapabilities>,
    /// Output only. The capabilities this user has on this field and its value
    /// when the label is applied on Drive items.
    #[prost(message, optional, tag = "7")]
    pub applied_capabilities: ::core::option::Option<field::AppliedCapabilities>,
    /// Output only. The user who created this field.
    #[prost(message, optional, tag = "8")]
    pub creator: ::core::option::Option<UserInfo>,
    /// Output only. The time this field was created.
    #[prost(message, optional, tag = "9")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The user who modified this field.
    #[prost(message, optional, tag = "10")]
    pub updater: ::core::option::Option<UserInfo>,
    /// Output only. The time this field was updated.
    #[prost(message, optional, tag = "11")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The user who published this field. This value has no meaning
    /// when the field is not published.
    #[prost(message, optional, tag = "12")]
    pub publisher: ::core::option::Option<UserInfo>,
    /// Output only. The user who disabled this field. This value has no meaning
    /// when the field is not disabled.
    #[prost(message, optional, tag = "13")]
    pub disabler: ::core::option::Option<UserInfo>,
    /// Output only. The time this field was disabled. This value has no meaning
    /// when the field is not disabled.
    #[prost(message, optional, tag = "14")]
    pub disable_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The LockStatus of this field.
    #[prost(message, optional, tag = "15")]
    pub lock_status: ::core::option::Option<LockStatus>,
    /// The data type and options of this field.
    /// Once published, the data type cannot be changed.
    #[prost(oneof = "field::Type", tags = "16, 18, 19, 20, 21")]
    pub r#type: ::core::option::Option<field::Type>,
}
/// Nested message and enum types in `Field`.
pub mod field {
    /// The basic properties of the field.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Properties {
        /// Required. The display text to show in the UI identifying this field.
        #[prost(string, tag = "1")]
        pub display_name: ::prost::alloc::string::String,
        /// Whether the field should be marked as required.
        #[prost(bool, tag = "2")]
        pub required: bool,
        /// Input only. Insert or move this field before the indicated field. If
        /// empty, the field is placed at the end of the list.
        #[prost(string, tag = "3")]
        pub insert_before_field: ::prost::alloc::string::String,
    }
    /// UI display hints for rendering a field.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DisplayHints {
        /// Whether the field should be shown as required in the UI.
        #[prost(bool, tag = "1")]
        pub required: bool,
        /// Whether the field should be shown in the UI as disabled.
        #[prost(bool, tag = "2")]
        pub disabled: bool,
        /// This field should be hidden in the search menu when searching for Drive
        /// items.
        #[prost(bool, tag = "3")]
        pub hidden_in_search: bool,
        /// This field should be shown in the apply menu when applying values to a
        /// Drive item.
        #[prost(bool, tag = "4")]
        pub shown_in_apply: bool,
    }
    /// The capabilities related to this field when editing the field.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SchemaCapabilities {
        /// Whether the user can change this field.
        #[prost(bool, tag = "1")]
        pub can_update: bool,
        /// Whether the user can delete this field.
        /// The user must have permission and the field must be deprecated.
        #[prost(bool, tag = "2")]
        pub can_delete: bool,
        /// Whether the user can disable this field.
        /// The user must have permission and this field must not already be
        /// disabled.
        #[prost(bool, tag = "3")]
        pub can_disable: bool,
        /// Whether the user can enable this field.
        /// The user must have permission and this field must be disabled.
        #[prost(bool, tag = "4")]
        pub can_enable: bool,
    }
    /// The capabilities related to this field on applied metadata.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AppliedCapabilities {
        /// Whether the user can read related applied metadata on items.
        #[prost(bool, tag = "1")]
        pub can_read: bool,
        /// Whether the user can search for Drive items referencing this field.
        #[prost(bool, tag = "2")]
        pub can_search: bool,
        /// Whether the user can set this field on Drive items.
        #[prost(bool, tag = "3")]
        pub can_write: bool,
    }
    /// Options for a multi-valued variant of an associated field type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ListOptions {
        /// Maximum number of entries permitted.
        #[prost(int32, tag = "1")]
        pub max_entries: i32,
    }
    /// Options for the Text field type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TextOptions {
        /// Output only. The minimum valid length of values for the text field.
        #[prost(int32, tag = "1")]
        pub min_length: i32,
        /// Output only. The maximum valid length of values for the text field.
        #[prost(int32, tag = "2")]
        pub max_length: i32,
    }
    /// Options the Long Text field type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LongTextOptions {
        /// Output only. The minimum valid length of values for the text field.
        #[prost(int32, tag = "1")]
        pub min_length: i32,
        /// Output only. The maximum valid length of values for the text field.
        #[prost(int32, tag = "2")]
        pub max_length: i32,
    }
    /// Options for the Integer field type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IntegerOptions {
        /// Output only. The minimum valid value for the integer field.
        #[prost(int64, tag = "1")]
        pub min_value: i64,
        /// Output only. The maximum valid value for the integer field.
        #[prost(int64, tag = "2")]
        pub max_value: i64,
    }
    /// Options for the date field type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DateOptions {
        /// Localized date formatting option. Field values are rendered in
        /// this format according to their locale.
        #[prost(enumeration = "date_options::DateFormat", tag = "1")]
        pub date_format_type: i32,
        /// Output only. ICU date format.
        #[prost(string, tag = "2")]
        pub date_format: ::prost::alloc::string::String,
        /// Output only. Minimum valid value (year, month, day).
        #[prost(message, optional, tag = "3")]
        pub min_value: ::core::option::Option<
            super::super::super::super::super::r#type::Date,
        >,
        /// Output only. Maximum valid value (year, month, day).
        #[prost(message, optional, tag = "4")]
        pub max_value: ::core::option::Option<
            super::super::super::super::super::r#type::Date,
        >,
    }
    /// Nested message and enum types in `DateOptions`.
    pub mod date_options {
        /// Localized date format options.
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum DateFormat {
            /// Date format unspecified.
            Unspecified = 0,
            /// Includes full month name.
            /// For example, January 12, 1999
            /// (MMMM d, y)
            LongDate = 1,
            /// Short, numeric, representation.
            /// For example, 12/13/99
            /// (M/d/yy)
            ShortDate = 2,
        }
        impl DateFormat {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    DateFormat::Unspecified => "DATE_FORMAT_UNSPECIFIED",
                    DateFormat::LongDate => "LONG_DATE",
                    DateFormat::ShortDate => "SHORT_DATE",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "DATE_FORMAT_UNSPECIFIED" => Some(Self::Unspecified),
                    "LONG_DATE" => Some(Self::LongDate),
                    "SHORT_DATE" => Some(Self::ShortDate),
                    _ => None,
                }
            }
        }
    }
    /// Options for the selection field type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SelectionOptions {
        /// When specified, indicates this field supports a list of values.
        /// Once the field is published, this cannot be changed.
        #[prost(message, optional, tag = "1")]
        pub list_options: ::core::option::Option<ListOptions>,
        /// The options available for this selection field.
        /// The list order is consistent, and modified with `insert_before_choice`.
        #[prost(message, repeated, tag = "2")]
        pub choices: ::prost::alloc::vec::Vec<selection_options::Choice>,
    }
    /// Nested message and enum types in `SelectionOptions`.
    pub mod selection_options {
        /// Selection field choice.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Choice {
            /// The unique value of the choice.
            /// This ID is autogenerated. Matches the regex: `(\[a-zA-Z0-9_\])+`.
            #[prost(string, tag = "1")]
            pub id: ::prost::alloc::string::String,
            /// Basic properties of the choice.
            #[prost(message, optional, tag = "2")]
            pub properties: ::core::option::Option<choice::Properties>,
            /// Output only. Lifecycle of the choice.
            #[prost(message, optional, tag = "3")]
            pub lifecycle: ::core::option::Option<super::super::Lifecycle>,
            /// Output only. UI display hints for rendering a choice.
            #[prost(message, optional, tag = "4")]
            pub display_hints: ::core::option::Option<choice::DisplayHints>,
            /// Output only. The capabilities related to this option when editing the
            /// option.
            #[prost(message, optional, tag = "5")]
            pub schema_capabilities: ::core::option::Option<choice::SchemaCapabilities>,
            /// Output only. The capabilities related to this choice on applied
            /// metadata.
            #[prost(message, optional, tag = "6")]
            pub applied_capabilities: ::core::option::Option<
                choice::AppliedCapabilities,
            >,
            /// Output only. The user who created this choice.
            #[prost(message, optional, tag = "7")]
            pub creator: ::core::option::Option<super::super::UserInfo>,
            /// Output only. The time this choice was created.
            #[prost(message, optional, tag = "8")]
            pub create_time: ::core::option::Option<::prost_types::Timestamp>,
            /// Output only. The user who updated this choice last.
            #[prost(message, optional, tag = "9")]
            pub updater: ::core::option::Option<super::super::UserInfo>,
            /// Output only. The time this choice was updated last.
            #[prost(message, optional, tag = "10")]
            pub update_time: ::core::option::Option<::prost_types::Timestamp>,
            /// Output only. The user who published this choice. This value has no
            /// meaning when the choice is not published.
            #[prost(message, optional, tag = "11")]
            pub publisher: ::core::option::Option<super::super::UserInfo>,
            /// Output only. The time this choice was published. This value has no
            /// meaning when the choice is not published.
            #[prost(message, optional, tag = "12")]
            pub publish_time: ::core::option::Option<::prost_types::Timestamp>,
            /// Output only. The user who disabled this choice. This value has no
            /// meaning when the option is not disabled.
            #[prost(message, optional, tag = "13")]
            pub disabler: ::core::option::Option<super::super::UserInfo>,
            /// Output only. The time this choice was disabled. This value has no
            /// meaning when the choice is not disabled.
            #[prost(message, optional, tag = "14")]
            pub disable_time: ::core::option::Option<::prost_types::Timestamp>,
            /// Output only. The LockStatus of this choice.
            #[prost(message, optional, tag = "15")]
            pub lock_status: ::core::option::Option<super::super::LockStatus>,
        }
        /// Nested message and enum types in `Choice`.
        pub mod choice {
            /// Basic properties of the choice.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Properties {
                /// Required. The display text to show in the UI identifying this field.
                #[prost(string, tag = "1")]
                pub display_name: ::prost::alloc::string::String,
                /// The description of this label.
                #[prost(string, tag = "2")]
                pub description: ::prost::alloc::string::String,
                /// The badge configuration for this choice. When set, the
                /// label that owns this choice is considered a "badged label".
                #[prost(message, optional, tag = "3")]
                pub badge_config: ::core::option::Option<
                    super::super::super::BadgeConfig,
                >,
                /// Input only. Insert or move this choice before the indicated choice.
                /// If empty, the choice is placed at the end of the list.
                #[prost(string, tag = "4")]
                pub insert_before_choice: ::prost::alloc::string::String,
            }
            /// UI display hints for rendering an option.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct DisplayHints {
                /// Whether the option should be shown in the UI as disabled.
                #[prost(bool, tag = "1")]
                pub disabled: bool,
                /// This option should be hidden in the search menu when searching for
                /// Drive items.
                #[prost(bool, tag = "2")]
                pub hidden_in_search: bool,
                /// This option should be shown in the apply menu when applying values to
                /// a Drive item.
                #[prost(bool, tag = "3")]
                pub shown_in_apply: bool,
                /// The colors to use for the badge. Changed to Google Material colors
                /// based on the chosen `properties.badge_config.color`.
                #[prost(message, optional, tag = "4")]
                pub badge_colors: ::core::option::Option<
                    super::super::super::BadgeColors,
                >,
                /// The dark-mode color to use for the badge. Changed to Google Material
                /// colors based on the chosen `properties.badge_config.color`.
                #[prost(message, optional, tag = "5")]
                pub dark_badge_colors: ::core::option::Option<
                    super::super::super::BadgeColors,
                >,
                /// The priority of this badge. Used to compare and sort between multiple
                /// badges. A lower number means the badge should be shown first.
                /// When a badging configuration is not present, this will be 0.
                /// Otherwise, this will be set to `BadgeConfig.priority_override` or the
                /// default heuristic which prefers creation date of the label, and field
                /// and option priority.
                #[prost(int64, tag = "6")]
                pub badge_priority: i64,
            }
            /// The capabilities related to this choice when editing the choice.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct SchemaCapabilities {
                /// Whether the user can update this choice.
                #[prost(bool, tag = "1")]
                pub can_update: bool,
                /// Whether the user can delete this choice.
                #[prost(bool, tag = "2")]
                pub can_delete: bool,
                /// Whether the user can disable this choice.
                #[prost(bool, tag = "3")]
                pub can_disable: bool,
                /// Whether the user can enable this choice.
                #[prost(bool, tag = "4")]
                pub can_enable: bool,
            }
            /// The capabilities related to this choice on applied metadata.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct AppliedCapabilities {
                /// Whether the user can read related applied metadata on items.
                #[prost(bool, tag = "1")]
                pub can_read: bool,
                /// Whether the user can use this choice in search queries.
                #[prost(bool, tag = "2")]
                pub can_search: bool,
                /// Whether the user can select this choice on an item.
                #[prost(bool, tag = "3")]
                pub can_select: bool,
            }
        }
    }
    /// Options for the user field type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UserOptions {
        /// When specified, indicates that this field supports a list of values.
        /// Once the field is published, this cannot be changed.
        #[prost(message, optional, tag = "1")]
        pub list_options: ::core::option::Option<ListOptions>,
    }
    /// The data type and options of this field.
    /// Once published, the data type cannot be changed.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// Text field options.
        #[prost(message, tag = "16")]
        TextOptions(TextOptions),
        /// Integer field options.
        #[prost(message, tag = "18")]
        IntegerOptions(IntegerOptions),
        /// Date field options.
        #[prost(message, tag = "19")]
        DateOptions(DateOptions),
        /// Selection field options.
        #[prost(message, tag = "20")]
        SelectionOptions(SelectionOptions),
        /// User field options.
        #[prost(message, tag = "21")]
        UserOptions(UserOptions),
    }
}
/// A label defines a taxonomy that can be applied to Drive items in order to
/// organize and search across items. Labels can be simple strings, or can
/// contain fields that describe additional metadata that can be further used to
/// organize and search Drive items.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Label {
    /// Output only. Resource name of the label. Will be in the form of either:
    /// `labels/{id}` or `labels/{id}@{revision_id}` depending on the request.
    /// See `id` and `revision_id` below.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Globally unique identifier of this label. ID makes up part of
    /// the label `name`, but unlike `name`, ID is consistent between revisions.
    /// Matches the regex: `(\[a-zA-Z0-9\])+`
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// Output only. Revision ID of the label. Revision ID might be part of the
    /// label `name` depending on the request issued. A new revision is created
    /// whenever revisioned properties of a label are changed. Matches the regex:
    /// `(\[a-zA-Z0-9\])+`
    #[prost(string, tag = "3")]
    pub revision_id: ::prost::alloc::string::String,
    /// Required. The type of label.
    #[prost(enumeration = "label::LabelType", tag = "4")]
    pub label_type: i32,
    /// Output only. The user who created this label.
    #[prost(message, optional, tag = "5")]
    pub creator: ::core::option::Option<UserInfo>,
    /// Output only. The time this label was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The user who created this label revision.
    #[prost(message, optional, tag = "7")]
    pub revision_creator: ::core::option::Option<UserInfo>,
    /// Output only. The time this label revision was created.
    #[prost(message, optional, tag = "8")]
    pub revision_create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The user who published this label.  This value has no meaning
    /// when the label is not published.
    #[prost(message, optional, tag = "9")]
    pub publisher: ::core::option::Option<UserInfo>,
    /// Output only. The time this label was published. This value has no meaning
    /// when the label is not published.
    #[prost(message, optional, tag = "10")]
    pub publish_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The user who disabled this label. This value has no meaning
    /// when the label is not disabled.
    #[prost(message, optional, tag = "11")]
    pub disabler: ::core::option::Option<UserInfo>,
    /// Output only. The time this label was disabled. This value has no meaning
    /// when the label is not disabled.
    #[prost(message, optional, tag = "12")]
    pub disable_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. The basic properties of the label.
    #[prost(message, optional, tag = "14")]
    pub properties: ::core::option::Option<label::Properties>,
    /// Output only. The lifecycle state of the label including whether it's
    /// published, deprecated, and has draft changes.
    #[prost(message, optional, tag = "15")]
    pub lifecycle: ::core::option::Option<Lifecycle>,
    /// Output only. UI display hints for rendering the label.
    #[prost(message, optional, tag = "16")]
    pub display_hints: ::core::option::Option<label::DisplayHints>,
    /// Output only. The capabilities related to this label on applied metadata.
    #[prost(message, optional, tag = "17")]
    pub applied_capabilities: ::core::option::Option<label::AppliedCapabilities>,
    /// Output only. The capabilities the user has on this label.
    #[prost(message, optional, tag = "18")]
    pub schema_capabilities: ::core::option::Option<label::SchemaCapabilities>,
    /// Output only. Behavior of this label when it's applied to Drive items.
    #[prost(message, optional, tag = "19")]
    pub applied_label_policy: ::core::option::Option<label::AppliedLabelPolicy>,
    /// List of fields in descending priority order.
    #[prost(message, repeated, tag = "20")]
    pub fields: ::prost::alloc::vec::Vec<Field>,
    /// Custom URL to present to users to allow them to learn more about this label
    /// and how it should be used.
    #[prost(string, tag = "21")]
    pub learn_more_uri: ::prost::alloc::string::String,
    /// Output only. The LockStatus of this label.
    #[prost(message, optional, tag = "22")]
    pub lock_status: ::core::option::Option<LockStatus>,
}
/// Nested message and enum types in `Label`.
pub mod label {
    /// Basic properties of the label.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Properties {
        /// Required. Title of the label.
        #[prost(string, tag = "1")]
        pub title: ::prost::alloc::string::String,
        /// The description of the label.
        #[prost(string, tag = "2")]
        pub description: ::prost::alloc::string::String,
    }
    /// UI display hints for rendering the label.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DisplayHints {
        /// Whether the label should be shown in the UI as disabled.
        #[prost(bool, tag = "1")]
        pub disabled: bool,
        /// This label should be hidden in the search menu when searching for Drive
        /// items.
        #[prost(bool, tag = "2")]
        pub hidden_in_search: bool,
        /// This label should be shown in the apply menu when applying values to a
        /// Drive item.
        #[prost(bool, tag = "3")]
        pub shown_in_apply: bool,
        /// Order to display label in a list.
        #[prost(int64, tag = "4")]
        pub priority: i64,
    }
    /// The capabilities a user has on this label's applied metadata.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AppliedCapabilities {
        /// Whether the user can read applied metadata related to this label.
        #[prost(bool, tag = "1")]
        pub can_read: bool,
        /// Whether the user can apply this label to items.
        #[prost(bool, tag = "2")]
        pub can_apply: bool,
        /// Whether the user can remove this label from items.
        #[prost(bool, tag = "3")]
        pub can_remove: bool,
    }
    /// The capabilities related to this label when editing the label.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SchemaCapabilities {
        /// Whether the user can change this label.
        #[prost(bool, tag = "1")]
        pub can_update: bool,
        /// Whether the user can delete this label.
        /// The user must have permission and the label must be disabled.
        #[prost(bool, tag = "2")]
        pub can_delete: bool,
        /// Whether the user can disable this label.
        /// The user must have permission and this label must not already be
        /// disabled.
        #[prost(bool, tag = "3")]
        pub can_disable: bool,
        /// Whether the user can enable this label.
        /// The user must have permission and this label must be disabled.
        #[prost(bool, tag = "4")]
        pub can_enable: bool,
    }
    /// Behavior of this label when it's applied to Drive items.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AppliedLabelPolicy {
        /// Indicates how the applied label and field values should be copied when
        /// a Drive item is copied.
        #[prost(enumeration = "applied_label_policy::CopyMode", tag = "1")]
        pub copy_mode: i32,
    }
    /// Nested message and enum types in `AppliedLabelPolicy`.
    pub mod applied_label_policy {
        /// Indicates how the applied label and field values should be copied when
        /// a Drive item is copied.
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum CopyMode {
            /// Copy mode unspecified.
            Unspecified = 0,
            /// The applied label and field values are not copied by default when
            /// the Drive item it's applied to is copied.
            DoNotCopy = 1,
            /// The applied label and field values are always copied when the
            /// Drive item it's applied to is copied.
            /// Only admins can use this mode.
            AlwaysCopy = 2,
            /// The applied label and field values are copied if the
            /// label is appliable by the user making the copy.
            CopyAppliable = 3,
        }
        impl CopyMode {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    CopyMode::Unspecified => "COPY_MODE_UNSPECIFIED",
                    CopyMode::DoNotCopy => "DO_NOT_COPY",
                    CopyMode::AlwaysCopy => "ALWAYS_COPY",
                    CopyMode::CopyAppliable => "COPY_APPLIABLE",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "COPY_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                    "DO_NOT_COPY" => Some(Self::DoNotCopy),
                    "ALWAYS_COPY" => Some(Self::AlwaysCopy),
                    "COPY_APPLIABLE" => Some(Self::CopyAppliable),
                    _ => None,
                }
            }
        }
    }
    /// The type of this label.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum LabelType {
        /// Unknown label type.
        Unspecified = 0,
        /// Shared labels may be shared with users to apply to Drive items.
        Shared = 1,
        /// Admin-owned label. Only creatable and editable by admins. Supports some
        /// additional admin-only features.
        Admin = 2,
    }
    impl LabelType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LabelType::Unspecified => "LABEL_TYPE_UNSPECIFIED",
                LabelType::Shared => "SHARED",
                LabelType::Admin => "ADMIN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LABEL_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "SHARED" => Some(Self::Shared),
                "ADMIN" => Some(Self::Admin),
                _ => None,
            }
        }
    }
}
/// Label constraints governing the structure of a Label; such as, the maximum
/// number of Fields allowed and maximum length of the label title.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelLimits {
    /// Resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The maximum number of characters allowed for the title.
    #[prost(int32, tag = "2")]
    pub max_title_length: i32,
    /// The maximum number of characters allowed for the description.
    #[prost(int32, tag = "3")]
    pub max_description_length: i32,
    /// The maximum number of Fields allowed within the label.
    #[prost(int32, tag = "4")]
    pub max_fields: i32,
    /// The maximum number of published Fields that can be deleted.
    #[prost(int32, tag = "5")]
    pub max_deleted_fields: i32,
    /// The maximum number of draft revisions that will be kept before deleting
    /// old drafts.
    #[prost(int32, tag = "6")]
    pub max_draft_revisions: i32,
    /// The limits for Fields.
    #[prost(message, optional, tag = "7")]
    pub field_limits: ::core::option::Option<FieldLimits>,
}
/// Field constants governing the structure of a Field; such as, the maximum
/// title length, minimum and maximum field values or length, etc.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldLimits {
    /// Max length for the id.
    #[prost(int32, tag = "1")]
    pub max_id_length: i32,
    /// Limits for Field title.
    #[prost(int32, tag = "2")]
    pub max_display_name_length: i32,
    /// Limits for Field description, also called help text.
    #[prost(int32, tag = "3")]
    pub max_description_length: i32,
    /// The relevant limits for the specified Field.Type.
    /// Text Field limits.
    #[prost(message, optional, tag = "4")]
    pub text_limits: ::core::option::Option<TextLimits>,
    /// Long text Field limits.
    #[prost(message, optional, tag = "5")]
    pub long_text_limits: ::core::option::Option<LongTextLimits>,
    /// Integer Field limits.
    #[prost(message, optional, tag = "6")]
    pub integer_limits: ::core::option::Option<IntegerLimits>,
    /// Date Field limits.
    #[prost(message, optional, tag = "7")]
    pub date_limits: ::core::option::Option<DateLimits>,
    /// User Field limits.
    #[prost(message, optional, tag = "8")]
    pub user_limits: ::core::option::Option<UserLimits>,
    /// Selection Field limits.
    #[prost(message, optional, tag = "9")]
    pub selection_limits: ::core::option::Option<SelectionLimits>,
}
/// Limits for list-variant of a Field type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLimits {
    /// Maximum number of values allowed for the Field type.
    #[prost(int32, tag = "1")]
    pub max_entries: i32,
}
/// Limits for text Field type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextLimits {
    /// Minimum length allowed for a text Field type.
    #[prost(int32, tag = "1")]
    pub min_length: i32,
    /// Maximum length allowed for a text Field type.
    #[prost(int32, tag = "2")]
    pub max_length: i32,
}
/// Limits for long text Field type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LongTextLimits {
    /// Minimum length allowed for a long text Field type.
    #[prost(int32, tag = "1")]
    pub min_length: i32,
    /// Maximum length allowed for a long text Field type.
    #[prost(int32, tag = "2")]
    pub max_length: i32,
}
/// Limits for integer Field type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntegerLimits {
    /// Minimum value for an integer Field type.
    #[prost(int64, tag = "1")]
    pub min_value: i64,
    /// Maximum value for an integer Field type.
    #[prost(int64, tag = "2")]
    pub max_value: i64,
}
/// Limits for date Field type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateLimits {
    /// Minimum value for the date Field type.
    #[prost(message, optional, tag = "1")]
    pub min_value: ::core::option::Option<super::super::super::super::r#type::Date>,
    /// Maximum value for the date Field type.
    #[prost(message, optional, tag = "2")]
    pub max_value: ::core::option::Option<super::super::super::super::r#type::Date>,
}
/// Limits for selection Field type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelectionLimits {
    /// Limits for list-variant of a Field type.
    #[prost(message, optional, tag = "1")]
    pub list_limits: ::core::option::Option<ListLimits>,
    /// Maximum ID length for a selection options.
    #[prost(int32, tag = "2")]
    pub max_id_length: i32,
    /// Maximum length for display name.
    #[prost(int32, tag = "3")]
    pub max_display_name_length: i32,
    /// The max number of choices.
    #[prost(int32, tag = "4")]
    pub max_choices: i32,
    /// Maximum number of deleted choices.
    #[prost(int32, tag = "5")]
    pub max_deleted_choices: i32,
}
/// Limits for Field.Type.USER.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserLimits {
    /// Limits for list-variant of a Field type.
    #[prost(message, optional, tag = "1")]
    pub list_limits: ::core::option::Option<ListLimits>,
}
/// The permission that applies to a principal (user, group, audience) on a
/// label.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelPermission {
    /// Resource name of this permission.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Specifies the email address for a user or group pricinpal. Not populated
    /// for audience principals. User and Group permissions may only be inserted
    /// using email address. On update requests, if email address is specified,
    /// no principal should be specified.
    #[prost(string, tag = "2")]
    pub email: ::prost::alloc::string::String,
    /// The role the principal should have.
    #[prost(enumeration = "label_permission::LabelRole", tag = "6")]
    pub role: i32,
    /// The principal this permission applies to. Must be either an email, user,
    /// group, or audience.
    /// Example:
    /// * people/12345
    /// * groups/45678
    /// * audiences/default
    #[prost(oneof = "label_permission::Principal", tags = "3, 4, 5")]
    pub principal: ::core::option::Option<label_permission::Principal>,
}
/// Nested message and enum types in `LabelPermission`.
pub mod label_permission {
    /// Roles are concentric with subsequent role.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum LabelRole {
        /// Unknown role.
        Unspecified = 0,
        /// A reader can read the label and associated metadata applied to Drive
        /// items.
        Reader = 1,
        /// An applier can write associated metadata on Drive items in which they
        /// also have write access to. Implies `READER`.
        Applier = 2,
        /// An organizer can pin this label in shared drives they manage
        /// and add new appliers to the label.
        Organizer = 3,
        /// Editors can make any update including deleting the label which
        /// also deletes the associated Drive item metadata. Implies `APPLIER`.
        Editor = 4,
    }
    impl LabelRole {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LabelRole::Unspecified => "LABEL_ROLE_UNSPECIFIED",
                LabelRole::Reader => "READER",
                LabelRole::Applier => "APPLIER",
                LabelRole::Organizer => "ORGANIZER",
                LabelRole::Editor => "EDITOR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LABEL_ROLE_UNSPECIFIED" => Some(Self::Unspecified),
                "READER" => Some(Self::Reader),
                "APPLIER" => Some(Self::Applier),
                "ORGANIZER" => Some(Self::Organizer),
                "EDITOR" => Some(Self::Editor),
                _ => None,
            }
        }
    }
    /// The principal this permission applies to. Must be either an email, user,
    /// group, or audience.
    /// Example:
    /// * people/12345
    /// * groups/45678
    /// * audiences/default
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Principal {
        /// Person resource name.
        #[prost(string, tag = "3")]
        Person(::prost::alloc::string::String),
        /// Group resource name.
        #[prost(string, tag = "4")]
        Group(::prost::alloc::string::String),
        /// Audience to grant a role to. The magic value of `audiences/default` may
        /// be used to apply the role to the default audience in the context of the
        /// organization that owns the Label.
        #[prost(string, tag = "5")]
        Audience(::prost::alloc::string::String),
    }
}
/// A Lock that can be applied to a Label, Field, or Choice.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelLock {
    /// Output only. Resource name of this LabelLock.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The ID of the Field that should be locked.  Empty if the whole
    /// Label should be locked.
    #[prost(string, tag = "2")]
    pub field_id: ::prost::alloc::string::String,
    /// The ID of the Selection Field Choice that should be locked.  If present,
    /// `field_id` must also be present.
    #[prost(string, tag = "3")]
    pub choice_id: ::prost::alloc::string::String,
    /// Output only. The time this LabelLock was created.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The user whose credentials were used to create the LabelLock.
    /// This will not be present if no user was responsible for creating the
    /// LabelLock.
    #[prost(message, optional, tag = "5")]
    pub creator: ::core::option::Option<UserInfo>,
    /// Output only. A timestamp indicating when this LabelLock was scheduled for
    /// deletion. This will be present only if this LabelLock is in the DELETING
    /// state.
    #[prost(message, optional, tag = "6")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The user's capabilities on this LabelLock.
    #[prost(message, optional, tag = "8")]
    pub capabilities: ::core::option::Option<label_lock::Capabilities>,
    /// Output only. This LabelLock's state.
    #[prost(enumeration = "label_lock::State", tag = "9")]
    pub state: i32,
}
/// Nested message and enum types in `LabelLock`.
pub mod label_lock {
    /// A description of a user's capabilities on a LabelLock.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Capabilities {
        /// True if the user is authorized to view the policy.
        #[prost(bool, tag = "1")]
        pub can_view_policy: bool,
    }
    /// A description of a LabelLock's state.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum State {
        /// Unknown state.
        Unspecified = 0,
        /// The LabelLock is active and is being enforced by the server.
        Active = 1,
        /// The LabelLock is being deleted.  The LabelLock will continue to be
        /// enforced by the server until it has been fully removed.
        Deleting = 2,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Active => "ACTIVE",
                State::Deleting => "DELETING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "DELETING" => Some(Self::Deleting),
                _ => None,
            }
        }
    }
}
/// Provides control over how write requests are executed. When not specified,
/// the last write wins.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteControl {
    /// Determines the revision of the label to write to and how the request
    /// should behave if that revision is not the current revision of the
    /// label.
    #[prost(oneof = "write_control::Control", tags = "1")]
    pub control: ::core::option::Option<write_control::Control>,
}
/// Nested message and enum types in `WriteControl`.
pub mod write_control {
    /// Determines the revision of the label to write to and how the request
    /// should behave if that revision is not the current revision of the
    /// label.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Control {
        /// The \[revision_id][google.apps.drive.labels.v1.Label.revision_id\] of the
        /// label that the write request will be applied to. If this is not the
        /// latest revision of the label, the request will not be processed and will
        /// return a 400 Bad Request error.
        #[prost(string, tag = "1")]
        RequiredRevisionId(::prost::alloc::string::String),
    }
}
/// Request to get the capabilities for a user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserCapabilitiesRequest {
    /// Required. The resource name of the user. Only "users/me/capabilities" is
    /// supported.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to create a Label.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateLabelRequest {
    /// Required. The label to create.
    #[prost(message, optional, tag = "1")]
    pub label: ::core::option::Option<Label>,
    /// Set to `true` in order to use the user's admin privileges. The server
    /// will verify the user is an admin before allowing access.
    #[prost(bool, tag = "2")]
    pub use_admin_access: bool,
    /// The BCP-47 language code to use for evaluating localized Field labels in
    /// response. When not specified, values in the default configured language
    /// will be used.
    #[prost(string, tag = "3")]
    pub language_code: ::prost::alloc::string::String,
}
/// Request to get a label by resource name.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLabelRequest {
    /// Required. Label resource name.
    ///
    /// May be any of:
    ///
    /// * `labels/{id}` (equivalent to labels/{id}@latest)
    /// * `labels/{id}@latest`
    /// * `labels/{id}@published`
    /// * `labels/{id}@{revision_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Set to `true` in order to use the user's admin credentials. The server
    /// verifies that the user is an admin for the label before allowing access.
    #[prost(bool, tag = "2")]
    pub use_admin_access: bool,
    /// The BCP-47 language code to use for evaluating localized field labels.
    /// When not specified, values in the default configured language are used.
    #[prost(string, tag = "3")]
    pub language_code: ::prost::alloc::string::String,
    /// When specified, only certain fields belonging to the indicated view are
    /// returned.
    #[prost(enumeration = "LabelView", tag = "4")]
    pub view: i32,
}
/// The set of requests for updating aspects of a Label. If any request is not
/// valid, no requests will be applied.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeltaUpdateLabelRequest {
    /// Required. The resource name of the Label to update.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Provides control over how write requests are executed.
    #[prost(message, optional, tag = "2")]
    pub write_control: ::core::option::Option<WriteControl>,
    /// A list of updates to apply to the Label.
    /// Requests will be applied in the order they are specified.
    #[prost(message, repeated, tag = "3")]
    pub requests: ::prost::alloc::vec::Vec<delta_update_label_request::Request>,
    /// Set to `true` in order to use the user's admin credentials. The server
    /// will verify the user is an admin for the Label before allowing access.
    #[prost(bool, tag = "4")]
    pub use_admin_access: bool,
    /// When specified, only certain fields belonging to the indicated view will be
    /// returned.
    #[prost(enumeration = "LabelView", tag = "5")]
    pub view: i32,
    /// The BCP-47 language code to use for evaluating localized Field labels when
    /// `include_label_in_response` is `true`.
    #[prost(string, tag = "6")]
    pub language_code: ::prost::alloc::string::String,
}
/// Nested message and enum types in `DeltaUpdateLabelRequest`.
pub mod delta_update_label_request {
    /// A single kind of update to apply to a Label.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        /// The kind of update. Exactly one Field is required.
        #[prost(oneof = "request::Kind", tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12")]
        pub kind: ::core::option::Option<request::Kind>,
    }
    /// Nested message and enum types in `Request`.
    pub mod request {
        /// The kind of update. Exactly one Field is required.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Kind {
            /// Updates the Label properties.
            #[prost(message, tag = "1")]
            UpdateLabel(super::UpdateLabelPropertiesRequest),
            /// Creates a new Field.
            #[prost(message, tag = "2")]
            CreateField(super::CreateFieldRequest),
            /// Updates basic properties of a Field.
            #[prost(message, tag = "3")]
            UpdateField(super::UpdateFieldPropertiesRequest),
            /// Update Field type and/or type options.
            #[prost(message, tag = "4")]
            UpdateFieldType(super::UpdateFieldTypeRequest),
            /// Enables the Field.
            #[prost(message, tag = "5")]
            EnableField(super::EnableFieldRequest),
            /// Disables the Field.
            #[prost(message, tag = "6")]
            DisableField(super::DisableFieldRequest),
            /// Deletes a Field from the label.
            #[prost(message, tag = "7")]
            DeleteField(super::DeleteFieldRequest),
            /// Creates Choice within a Selection field.
            #[prost(message, tag = "8")]
            CreateSelectionChoice(super::CreateSelectionChoiceRequest),
            /// Update a Choice properties within a Selection Field.
            #[prost(message, tag = "9")]
            UpdateSelectionChoiceProperties(
                super::UpdateSelectionChoicePropertiesRequest,
            ),
            /// Enable a Choice within a Selection Field.
            #[prost(message, tag = "10")]
            EnableSelectionChoice(super::EnableSelectionChoiceRequest),
            /// Disable a Choice within a Selection Field.
            #[prost(message, tag = "11")]
            DisableSelectionChoice(super::DisableSelectionChoiceRequest),
            /// Delete a Choice within a Selection Field.
            #[prost(message, tag = "12")]
            DeleteSelectionChoice(super::DeleteSelectionChoiceRequest),
        }
    }
    /// Updates basic properties of a Label.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdateLabelPropertiesRequest {
        /// The fields that should be updated. At least one field must be specified.
        /// The root `label_properties` is implied and should not be specified. A
        /// single `*` can be used as short-hand for updating every field.
        #[prost(message, optional, tag = "1")]
        pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
        /// Required. Label properties to update.
        #[prost(message, optional, tag = "2")]
        pub properties: ::core::option::Option<super::label::Properties>,
    }
    /// Request to disable the Field.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DisableFieldRequest {
        /// The fields that should be updated. At least one field must be specified.
        /// The root `disabled_policy` is implied and should not be specified. A
        /// single `*` can be used as short-hand for updating every field.
        #[prost(message, optional, tag = "1")]
        pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
        /// Required. Key of the Field to disable.
        #[prost(string, tag = "2")]
        pub id: ::prost::alloc::string::String,
        /// Required. Field Disabled Policy.
        #[prost(message, optional, tag = "3")]
        pub disabled_policy: ::core::option::Option<super::lifecycle::DisabledPolicy>,
    }
    /// Request to enable the Field.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EnableFieldRequest {
        /// Required. ID of the Field to enable.
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
    }
    /// Request to delete the Field.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeleteFieldRequest {
        /// Required. ID of the Field to delete.
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
    }
    /// Request to create a Field within a Label.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CreateFieldRequest {
        /// Required. Field to create.
        #[prost(message, optional, tag = "1")]
        pub field: ::core::option::Option<super::Field>,
    }
    /// Request to update Field properties.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdateFieldPropertiesRequest {
        /// The fields that should be updated. At least one field must be specified.
        /// The root `properties` is implied and should not be specified. A single
        /// `*` can be used as short-hand for updating every field.
        #[prost(message, optional, tag = "1")]
        pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
        /// Required. The Field to update.
        #[prost(string, tag = "2")]
        pub id: ::prost::alloc::string::String,
        /// Required. Basic Field properties.
        #[prost(message, optional, tag = "3")]
        pub properties: ::core::option::Option<super::field::Properties>,
    }
    /// Request to change the type of a Field.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdateFieldTypeRequest {
        /// The fields that should be updated. At least one field must be specified.
        /// The root of `type_options` is implied and should not be specified. A
        /// single `*` can be used as short-hand for updating every field.
        #[prost(message, optional, tag = "1")]
        pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
        /// Required. The Field to update.
        #[prost(string, tag = "2")]
        pub id: ::prost::alloc::string::String,
        #[prost(
            oneof = "update_field_type_request::TypeOptions",
            tags = "3, 4, 5, 6, 7, 8"
        )]
        pub type_options: ::core::option::Option<update_field_type_request::TypeOptions>,
    }
    /// Nested message and enum types in `UpdateFieldTypeRequest`.
    pub mod update_field_type_request {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum TypeOptions {
            /// Update field to Text.
            #[prost(message, tag = "3")]
            TextOptions(super::super::field::TextOptions),
            /// Update field to Long Text.
            #[prost(message, tag = "4")]
            LongTextOptions(super::super::field::LongTextOptions),
            /// Update field to Integer.
            #[prost(message, tag = "5")]
            IntegerOptions(super::super::field::IntegerOptions),
            /// Update field to Date.
            #[prost(message, tag = "6")]
            DateOptions(super::super::field::DateOptions),
            /// Update field to Selection.
            #[prost(message, tag = "7")]
            SelectionOptions(super::super::field::SelectionOptions),
            /// Update field to User.
            #[prost(message, tag = "8")]
            UserOptions(super::super::field::UserOptions),
        }
    }
    /// Request to create a Selection Choice.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CreateSelectionChoiceRequest {
        /// Required. The Selection Field in which a Choice will be created.
        #[prost(string, tag = "1")]
        pub field_id: ::prost::alloc::string::String,
        /// Required. The Choice to create.
        #[prost(message, optional, tag = "2")]
        pub choice: ::core::option::Option<super::field::selection_options::Choice>,
    }
    /// Request to update a Choice properties.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdateSelectionChoicePropertiesRequest {
        /// The fields that should be updated. At least one field must be specified.
        /// The root `properties` is implied and should not be specified. A single
        /// `*` can be used as short-hand for updating every field.
        #[prost(message, optional, tag = "1")]
        pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
        /// Required. The Selection Field to update.
        #[prost(string, tag = "2")]
        pub field_id: ::prost::alloc::string::String,
        /// Required. The Choice to update.
        #[prost(string, tag = "3")]
        pub id: ::prost::alloc::string::String,
        /// Required. The Choice properties to update.
        #[prost(message, optional, tag = "4")]
        pub properties: ::core::option::Option<
            super::field::selection_options::choice::Properties,
        >,
    }
    /// Request to delete a Choice.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeleteSelectionChoiceRequest {
        /// Required. The Selection Field from which a Choice will be deleted.
        #[prost(string, tag = "1")]
        pub field_id: ::prost::alloc::string::String,
        /// Required. Choice to delete.
        #[prost(string, tag = "2")]
        pub id: ::prost::alloc::string::String,
    }
    /// Request to disable a Choice.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DisableSelectionChoiceRequest {
        /// The fields that should be updated. At least one field must be specified.
        /// The root `disabled_policy` is implied and should not be specified. A
        /// single `*` can be used as short-hand for updating every field.
        #[prost(message, optional, tag = "1")]
        pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
        /// Required. The Selection Field in which a Choice will be disabled.
        #[prost(string, tag = "2")]
        pub field_id: ::prost::alloc::string::String,
        /// Required. Choice to disable.
        #[prost(string, tag = "3")]
        pub id: ::prost::alloc::string::String,
        /// Required. The disabled policy to update.
        #[prost(message, optional, tag = "4")]
        pub disabled_policy: ::core::option::Option<super::lifecycle::DisabledPolicy>,
    }
    /// Request to enable a Choice.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EnableSelectionChoiceRequest {
        /// Required. The Selection Field in which a Choice will be enabled.
        #[prost(string, tag = "1")]
        pub field_id: ::prost::alloc::string::String,
        /// Required. Choice to enable.
        #[prost(string, tag = "2")]
        pub id: ::prost::alloc::string::String,
    }
}
/// Response for Label update.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeltaUpdateLabelResponse {
    /// The reply of the updates. This maps 1:1 with the updates, although
    /// responses to some requests may be empty.
    #[prost(message, repeated, tag = "1")]
    pub responses: ::prost::alloc::vec::Vec<delta_update_label_response::Response>,
    /// The label after updates were applied. This is only set if
    /// \[BatchUpdateLabelResponse2.include_label_in_response\] is `true` and there
    /// were no errors.
    #[prost(message, optional, tag = "6")]
    pub updated_label: ::core::option::Option<Label>,
}
/// Nested message and enum types in `DeltaUpdateLabelResponse`.
pub mod delta_update_label_response {
    /// A single response from an update.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        /// The response for the corresponding request.
        #[prost(
            oneof = "response::Response",
            tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12"
        )]
        pub response: ::core::option::Option<response::Response>,
    }
    /// Nested message and enum types in `Response`.
    pub mod response {
        /// The response for the corresponding request.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Response {
            /// Updated basic properties of a Label.
            #[prost(message, tag = "1")]
            UpdateLabel(super::UpdateLabelPropertiesResponse),
            /// Creates a new Field.
            #[prost(message, tag = "2")]
            CreateField(super::CreateFieldResponse),
            /// Updates basic properties of a Field.
            #[prost(message, tag = "3")]
            UpdateField(super::UpdateFieldPropertiesResponse),
            /// Update Field type and/or type options.
            #[prost(message, tag = "4")]
            UpdateFieldType(super::UpdateFieldTypeResponse),
            /// Enables Field.
            #[prost(message, tag = "5")]
            EnableField(super::EnableFieldResponse),
            /// Disables Field.
            #[prost(message, tag = "6")]
            DisableField(super::DisableFieldResponse),
            /// Deletes a Field from the label.
            #[prost(message, tag = "7")]
            DeleteField(super::DeleteFieldResponse),
            /// Creates a new selection list option to add to a Selection Field.
            #[prost(message, tag = "8")]
            CreateSelectionChoice(super::CreateSelectionChoiceResponse),
            /// Updates a Choice within a Selection Field.
            #[prost(message, tag = "9")]
            UpdateSelectionChoiceProperties(
                super::UpdateSelectionChoicePropertiesResponse,
            ),
            /// Enables a Choice within a Selection Field.
            #[prost(message, tag = "10")]
            EnableSelectionChoice(super::EnableSelectionChoiceResponse),
            /// Disables a Choice within a Selection Field.
            #[prost(message, tag = "11")]
            DisableSelectionChoice(super::DisableSelectionChoiceResponse),
            /// Deletes a Choice from a Selection Field.
            #[prost(message, tag = "12")]
            DeleteSelectionChoice(super::DeleteSelectionChoiceResponse),
        }
    }
    /// Response following update to Label properties.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdateLabelPropertiesResponse {}
    /// Response following Field create.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CreateFieldResponse {
        /// The field of the created field. When left blank in a create request,
        /// a key will be autogenerated and can be identified here.
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// The priority of the created field. The priority may change from what
        /// was specified to assure contiguous priorities between fields (1-n).
        #[prost(int32, tag = "2")]
        pub priority: i32,
    }
    /// Response following update to Field properties.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdateFieldPropertiesResponse {
        /// The priority of the updated field. The priority may change from what
        /// was specified to assure contiguous priorities between fields (1-n).
        #[prost(int32, tag = "1")]
        pub priority: i32,
    }
    /// Response following update to Field type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdateFieldTypeResponse {}
    /// Response following Field enable.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EnableFieldResponse {}
    /// Response following Field disable.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DisableFieldResponse {}
    /// Response following Field delete.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeleteFieldResponse {}
    /// Response following Selection Choice create.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CreateSelectionChoiceResponse {
        /// The server-generated id of the field.
        #[prost(string, tag = "1")]
        pub field_id: ::prost::alloc::string::String,
        /// The server-generated ID of the created choice within the Field
        #[prost(string, tag = "2")]
        pub id: ::prost::alloc::string::String,
    }
    /// Response following update to Selection Choice properties.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdateSelectionChoicePropertiesResponse {
        /// The priority of the updated choice. The priority may change from what
        /// was specified to assure contiguous priorities between choices (1-n).
        #[prost(int32, tag = "1")]
        pub priority: i32,
    }
    /// Response following Choice enable.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EnableSelectionChoiceResponse {}
    /// Response following Choice disable.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DisableSelectionChoiceResponse {}
    /// Response following Choice delete.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeleteSelectionChoiceResponse {}
}
/// Request to update the `CopyMode` of the given Label. Changes to this policy
/// are not revisioned, do not require publishing, and take effect immediately.
/// \
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateLabelCopyModeRequest {
    /// Required. The resource name of the Label to update.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Indicates how the applied Label, and Field values should be copied
    /// when a Drive item is copied.
    #[prost(enumeration = "label::applied_label_policy::CopyMode", tag = "2")]
    pub copy_mode: i32,
    /// Set to `true` in order to use the user's admin credentials. The server
    /// will verify the user is an admin for the Label before allowing access.
    #[prost(bool, tag = "3")]
    pub use_admin_access: bool,
    /// The BCP-47 language code to use for evaluating localized field labels.
    /// When not specified, values in the default configured language will be used.
    #[prost(string, tag = "4")]
    pub language_code: ::prost::alloc::string::String,
    /// When specified, only certain fields belonging to the indicated view will be
    /// returned.
    #[prost(enumeration = "LabelView", tag = "5")]
    pub view: i32,
}
/// Request to get the limits for a Label.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLabelLimitsRequest {
    /// Required. Label revision resource name
    /// Must be: "limits/label"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to list labels available to the current user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLabelsRequest {
    /// Whether to include only published labels in the results.
    ///
    /// * When `true`, only the current published label revisions are returned.
    ///    Disabled labels are included. Returned label resource names
    ///    reference the published revision (`labels/{id}/{revision_id}`).
    /// * When `false`, the current label revisions are returned, which might not
    ///    be published. Returned label resource names don't reference a specific
    ///    revision (`labels/{id}`).
    #[prost(bool, tag = "1")]
    pub published_only: bool,
    /// The BCP-47 language code to use for evaluating localized field labels.
    /// When not specified, values in the default configured language are used.
    #[prost(string, tag = "5")]
    pub language_code: ::prost::alloc::string::String,
    /// Maximum number of labels to return per page. Default: 50. Max: 200.
    #[prost(int32, tag = "6")]
    pub page_size: i32,
    /// The token of the page to return.
    #[prost(string, tag = "7")]
    pub page_token: ::prost::alloc::string::String,
    /// When specified, only certain fields belonging to the indicated view are
    /// returned.
    #[prost(enumeration = "LabelView", tag = "8")]
    pub view: i32,
    #[prost(oneof = "list_labels_request::Access", tags = "3, 4")]
    pub access: ::core::option::Option<list_labels_request::Access>,
}
/// Nested message and enum types in `ListLabelsRequest`.
pub mod list_labels_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Access {
        /// Set to `true` in order to use the user's admin credentials. This will
        /// return all Labels within the customer.
        #[prost(bool, tag = "3")]
        UseAdminAccess(bool),
        /// Specifies the level of access the user must have on the returned Labels.
        /// The minimum role a user must have on a label.
        /// Defaults to `READER`.
        #[prost(enumeration = "super::label_permission::LabelRole", tag = "4")]
        MinimumRole(i32),
    }
}
/// Response for listing Labels.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLabelsResponse {
    /// Labels.
    #[prost(message, repeated, tag = "1")]
    pub labels: ::prost::alloc::vec::Vec<Label>,
    /// The token of the next page in the response.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Creates or updates a permission on the Label. Permissions affect the Label
/// resource as a whole, are not revisioned, and do not require publishing.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateLabelPermissionRequest {
    /// Required. The parent Label resource name on the Label Permission is
    /// created. Format: labels/{label}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The permission to create or update on the Label.
    #[prost(message, optional, tag = "2")]
    pub label_permission: ::core::option::Option<LabelPermission>,
    /// Set to `true` in order to use the user's admin credentials. The server
    /// will verify the user is an admin for the Label before allowing access.
    #[prost(bool, tag = "3")]
    pub use_admin_access: bool,
}
/// Request to list the permissions on a Label.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLabelPermissionsRequest {
    /// Required. The parent Label resource name on which Label Permission are
    /// listed. Format: labels/{label}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Set to `true` in order to use the user's admin credentials. The server will
    /// verify the user is an admin for the Label before allowing access.
    #[prost(bool, tag = "2")]
    pub use_admin_access: bool,
    /// Maximum number of permissions to return per page. Default: 50. Max: 200.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The token of the page to return.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for listing the permissions on a Label.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLabelPermissionsResponse {
    /// Label permissions.
    #[prost(message, repeated, tag = "1")]
    pub label_permissions: ::prost::alloc::vec::Vec<LabelPermission>,
    /// The token of the next page in the response.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Updates a Label Permission. Permissions affect the Label resource as a whole,
/// are not revisioned, and do not require publishing.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateLabelPermissionRequest {
    /// Required. The parent Label resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The permission to create or update on the Label.
    #[prost(message, optional, tag = "2")]
    pub label_permission: ::core::option::Option<LabelPermission>,
    /// Set to `true` in order to use the user's admin credentials. The server
    /// will verify the user is an admin for the Label before allowing access.
    #[prost(bool, tag = "3")]
    pub use_admin_access: bool,
}
/// Deletes a Label Permission. Permissions affect the Label resource as a whole,
/// are not revisioned, and do not require publishing.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteLabelPermissionRequest {
    /// Required. Label Permission resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Set to `true` in order to use the user's admin credentials. The server
    /// will verify the user is an admin for the Label before allowing access.
    #[prost(bool, tag = "2")]
    pub use_admin_access: bool,
}
/// Updates one or more Label Permissions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateLabelPermissionsRequest {
    /// Required. The parent Label resource name shared by all permissions being
    /// updated. Format: labels/{label} If this is set, the parent field in the
    /// UpdateLabelPermissionRequest messages must either be empty or match this
    /// field.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The request message specifying the resources to update.
    #[prost(message, repeated, tag = "2")]
    pub requests: ::prost::alloc::vec::Vec<UpdateLabelPermissionRequest>,
    /// Set to `true` in order to use the user's admin credentials. The server
    /// will verify the user is an admin for the Label before allowing access.
    /// If this is set, the use_admin_access field in the
    /// UpdateLabelPermissionRequest messages must either be empty or match this
    /// field.
    #[prost(bool, tag = "3")]
    pub use_admin_access: bool,
}
/// Response for updating one or more Label Permissions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateLabelPermissionsResponse {
    /// Required. Permissions updated.
    #[prost(message, repeated, tag = "1")]
    pub permissions: ::prost::alloc::vec::Vec<LabelPermission>,
}
/// Deletes one of more Label Permissions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeleteLabelPermissionsRequest {
    /// Required. The request message specifying the resources to update.
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<DeleteLabelPermissionRequest>,
    /// Set to `true` in order to use the user's admin credentials. The server
    /// will verify the user is an admin for the Label before allowing access.
    /// If this is set, the use_admin_access field in the
    /// DeleteLabelPermissionRequest messages must either be empty or match this
    /// field.
    #[prost(bool, tag = "2")]
    pub use_admin_access: bool,
    /// Required. The parent Label resource name shared by all permissions being
    /// deleted. Format: labels/{label} If this is set, the parent field in the
    /// UpdateLabelPermissionRequest messages must either be empty or match this
    /// field.
    #[prost(string, tag = "3")]
    pub parent: ::prost::alloc::string::String,
}
/// Request to deprecate a published Label.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableLabelRequest {
    /// The fields that should be updated. At least one field must be specified.
    /// The root `disabled_policy` is implied and should not be specified. A
    /// single `*` can be used as short-hand for updating every field.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. Label resource name.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// Set to `true` in order to use the user's admin credentials. The server
    /// will verify the user is an admin for the Label before allowing access.
    #[prost(bool, tag = "3")]
    pub use_admin_access: bool,
    /// Provides control over how write requests are executed. Defaults to unset,
    /// which means last write wins.
    #[prost(message, optional, tag = "4")]
    pub write_control: ::core::option::Option<WriteControl>,
    /// Disabled policy to use.
    #[prost(message, optional, tag = "5")]
    pub disabled_policy: ::core::option::Option<lifecycle::DisabledPolicy>,
    /// The BCP-47 language code to use for evaluating localized field labels.
    /// When not specified, values in the default configured language will be used.
    #[prost(string, tag = "6")]
    pub language_code: ::prost::alloc::string::String,
}
/// Request to publish a label.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishLabelRequest {
    /// Required. Label resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Set to `true` in order to use the user's admin credentials. The server
    /// will verify the user is an admin for the Label before allowing access.
    #[prost(bool, tag = "2")]
    pub use_admin_access: bool,
    /// Provides control over how write requests are executed. Defaults to unset,
    /// which means last write wins.
    #[prost(message, optional, tag = "3")]
    pub write_control: ::core::option::Option<WriteControl>,
    /// The BCP-47 language code to use for evaluating localized field labels.
    /// When not specified, values in the default configured language will be used.
    #[prost(string, tag = "4")]
    pub language_code: ::prost::alloc::string::String,
}
/// Request to enable a label.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableLabelRequest {
    /// Required. Label resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Set to `true` in order to use the user's admin credentials. The server
    /// will verify the user is an admin for the Label before allowing access.
    #[prost(bool, tag = "2")]
    pub use_admin_access: bool,
    /// Provides control over how write requests are executed. Defaults to unset,
    /// which means last write wins.
    #[prost(message, optional, tag = "3")]
    pub write_control: ::core::option::Option<WriteControl>,
    /// The BCP-47 language code to use for evaluating localized field labels.
    /// When not specified, values in the default configured language will be used.
    #[prost(string, tag = "4")]
    pub language_code: ::prost::alloc::string::String,
}
/// Request to delete a label.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteLabelRequest {
    /// Required. Label resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Set to `true` in order to use the user's admin credentials. The server
    /// will verify the user is an admin for the Label before allowing access.
    #[prost(bool, tag = "2")]
    pub use_admin_access: bool,
    /// Provides control over how write requests are executed. Defaults to unset,
    /// which means last write wins.
    #[prost(message, optional, tag = "3")]
    pub write_control: ::core::option::Option<WriteControl>,
}
/// A request to list the LabelLocks on a Label.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLabelLocksRequest {
    /// Required. Label on which Locks are applied.
    /// Format: labels/{label}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of Locks to return per page. Default: 100. Max: 200.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The token of the page to return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response to a ListLabelLocksRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLabelLocksResponse {
    /// LabelLocks.
    #[prost(message, repeated, tag = "1")]
    pub label_locks: ::prost::alloc::vec::Vec<LabelLock>,
    /// The token of the next page in the response.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Resource view that can be applied to label responses. The default value
/// `LABEL_VIEW_BASIC` implies the field mask:
/// `name,id,revision_id,label_type,properties.*`\
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LabelView {
    /// Implies the field mask:
    /// `name,id,revision_id,label_type,properties.*`
    Basic = 0,
    /// All possible fields.
    Full = 1,
}
impl LabelView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LabelView::Basic => "LABEL_VIEW_BASIC",
            LabelView::Full => "LABEL_VIEW_FULL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LABEL_VIEW_BASIC" => Some(Self::Basic),
            "LABEL_VIEW_FULL" => Some(Self::Full),
            _ => None,
        }
    }
}
/// The capabilities of a user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserCapabilities {
    /// Output only. Resource name for the user capabilities.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Whether the user is allowed access to the label manager.
    #[prost(bool, tag = "2")]
    pub can_access_label_manager: bool,
    /// Output only. Whether the user is an administrator for the shared labels
    /// feature.
    #[prost(bool, tag = "3")]
    pub can_administrate_labels: bool,
    /// Output only. Whether the user is allowed to create new shared labels.
    #[prost(bool, tag = "4")]
    pub can_create_shared_labels: bool,
    /// Output only. Whether the user is allowed to create new admin labels.
    #[prost(bool, tag = "5")]
    pub can_create_admin_labels: bool,
}
/// Generated client implementations.
pub mod label_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Manage metadata taxonomies based on Labels and Fields that may be used within
    /// Google Drive to organize and find files using custom metadata.
    #[derive(Debug, Clone)]
    pub struct LabelServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> LabelServiceClient<T>
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
        ) -> LabelServiceClient<InterceptedService<T, F>>
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
            LabelServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets the user capabilities.
        pub async fn get_user_capabilities(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserCapabilitiesRequest>,
        ) -> Result<tonic::Response<super::UserCapabilities>, tonic::Status> {
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
                "/google.apps.drive.labels.v2beta.LabelService/GetUserCapabilities",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List labels.
        pub async fn list_labels(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLabelsRequest>,
        ) -> Result<tonic::Response<super::ListLabelsResponse>, tonic::Status> {
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
                "/google.apps.drive.labels.v2beta.LabelService/ListLabels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get a label by its resource name.
        /// Resource name may be any of:
        ///
        /// * `labels/{id}` - See `labels/{id}@latest`
        /// * `labels/{id}@latest` - Gets the latest revision of the label.
        /// * `labels/{id}@published` - Gets the current published revision of the
        ///   label.
        /// * `labels/{id}@{revision_id}` - Gets the label at the specified revision
        ///   ID.
        pub async fn get_label(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLabelRequest>,
        ) -> Result<tonic::Response<super::Label>, tonic::Status> {
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
                "/google.apps.drive.labels.v2beta.LabelService/GetLabel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get the constraints on the structure of a Label; such as, the maximum
        /// number of Fields allowed and maximum length of the label title.
        pub async fn get_label_limits(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLabelLimitsRequest>,
        ) -> Result<tonic::Response<super::LabelLimits>, tonic::Status> {
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
                "/google.apps.drive.labels.v2beta.LabelService/GetLabelLimits",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new Label.
        pub async fn create_label(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateLabelRequest>,
        ) -> Result<tonic::Response<super::Label>, tonic::Status> {
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
                "/google.apps.drive.labels.v2beta.LabelService/CreateLabel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a single Label by applying a set of update requests resulting in a
        /// new draft revision. The batch update is all-or-nothing: If any of the
        /// update requests are invalid, no changes are applied. The resulting draft
        /// revision must be published before the changes may be used with Drive Items.
        pub async fn delta_update_label(
            &mut self,
            request: impl tonic::IntoRequest<super::DeltaUpdateLabelRequest>,
        ) -> Result<tonic::Response<super::DeltaUpdateLabelResponse>, tonic::Status> {
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
                "/google.apps.drive.labels.v2beta.LabelService/DeltaUpdateLabel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a Label's `CopyMode`. Changes to this policy are not revisioned, do
        /// not require publishing, and take effect immediately.
        pub async fn update_label_copy_mode(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateLabelCopyModeRequest>,
        ) -> Result<tonic::Response<super::Label>, tonic::Status> {
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
                "/google.apps.drive.labels.v2beta.LabelService/UpdateLabelCopyMode",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Publish all draft changes to the Label. Once published, the Label may not
        /// return to its draft state. See
        /// `google.apps.drive.labels.v2.Lifecycle` for more information.
        ///
        /// Publishing a Label will result in a new published revision. All previous
        /// draft revisions will be deleted. Previous published revisions will be kept
        /// but are subject to automated deletion as needed.
        ///
        /// Once published, some changes are no longer permitted. Generally, any change
        /// that would invalidate or cause new restrictions on existing metadata
        /// related to the Label will be rejected. For example, the following changes
        /// to a Label will be rejected after the Label is published:
        /// * The label cannot be directly deleted. It must be disabled first, then
        ///   deleted.
        /// * Field.FieldType cannot be changed.
        /// * Changes to Field validation options cannot reject something that was
        ///   previously accepted.
        /// * Reducing the max entries.
        pub async fn publish_label(
            &mut self,
            request: impl tonic::IntoRequest<super::PublishLabelRequest>,
        ) -> Result<tonic::Response<super::Label>, tonic::Status> {
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
                "/google.apps.drive.labels.v2beta.LabelService/PublishLabel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Disable a published Label.
        /// Disabling a Label will result in a new disabled published revision based on
        /// the current published revision. If there is a draft revision, a new
        /// disabled draft revision will be created based on the latest draft revision.
        /// Older draft revisions will be deleted.
        ///
        /// Once disabled, a label may be deleted with `DeleteLabel`.
        pub async fn disable_label(
            &mut self,
            request: impl tonic::IntoRequest<super::DisableLabelRequest>,
        ) -> Result<tonic::Response<super::Label>, tonic::Status> {
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
                "/google.apps.drive.labels.v2beta.LabelService/DisableLabel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Enable a disabled Label and restore it to its published state.
        /// This will result in a new published revision based on the current disabled
        /// published revision. If there is an existing disabled draft revision, a new
        /// revision will be created based on that draft and will be enabled.
        pub async fn enable_label(
            &mut self,
            request: impl tonic::IntoRequest<super::EnableLabelRequest>,
        ) -> Result<tonic::Response<super::Label>, tonic::Status> {
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
                "/google.apps.drive.labels.v2beta.LabelService/EnableLabel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Permanently deletes a Label and related metadata on Drive Items.
        ///
        /// Once deleted, the Label and related Drive item metadata will be deleted.
        /// Only draft Labels, and disabled Labels may be deleted.
        pub async fn delete_label(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteLabelRequest>,
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
                "/google.apps.drive.labels.v2beta.LabelService/DeleteLabel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists a Label's permissions.
        pub async fn list_label_permissions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLabelPermissionsRequest>,
        ) -> Result<
            tonic::Response<super::ListLabelPermissionsResponse>,
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
                "/google.apps.drive.labels.v2beta.LabelService/ListLabelPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a Label's permissions. If a permission for the indicated principal
        /// doesn't exist, a new Label Permission is created, otherwise the existing
        /// permission is updated. Permissions affect the Label resource as a whole,
        /// are not revisioned, and do not require publishing.
        pub async fn create_label_permission(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateLabelPermissionRequest>,
        ) -> Result<tonic::Response<super::LabelPermission>, tonic::Status> {
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
                "/google.apps.drive.labels.v2beta.LabelService/CreateLabelPermission",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a Label's permissions. If a permission for the indicated principal
        /// doesn't exist, a new Label Permission is created, otherwise the existing
        /// permission is updated. Permissions affect the Label resource as a whole,
        /// are not revisioned, and do not require publishing.
        pub async fn update_label_permission(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateLabelPermissionRequest>,
        ) -> Result<tonic::Response<super::LabelPermission>, tonic::Status> {
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
                "/google.apps.drive.labels.v2beta.LabelService/UpdateLabelPermission",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a Label's permission. Permissions affect the Label resource as a
        /// whole, are not revisioned, and do not require publishing.
        pub async fn delete_label_permission(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteLabelPermissionRequest>,
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
                "/google.apps.drive.labels.v2beta.LabelService/DeleteLabelPermission",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates Label permissions. If a permission for the
        /// indicated principal doesn't exist, a new Label Permission is created,
        /// otherwise the existing permission is updated. Permissions affect the Label
        /// resource as a whole, are not revisioned, and do not require publishing.
        pub async fn batch_update_label_permissions(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchUpdateLabelPermissionsRequest>,
        ) -> Result<
            tonic::Response<super::BatchUpdateLabelPermissionsResponse>,
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
                "/google.apps.drive.labels.v2beta.LabelService/BatchUpdateLabelPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes Label permissions. Permissions affect the Label resource as a
        /// whole, are not revisioned, and do not require publishing.
        pub async fn batch_delete_label_permissions(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchDeleteLabelPermissionsRequest>,
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
                "/google.apps.drive.labels.v2beta.LabelService/BatchDeleteLabelPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists the LabelLocks on a Label.
        pub async fn list_label_locks(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLabelLocksRequest>,
        ) -> Result<tonic::Response<super::ListLabelLocksResponse>, tonic::Status> {
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
                "/google.apps.drive.labels.v2beta.LabelService/ListLabelLocks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Describes violations in a request to create or update a Label or its
/// Fields.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvalidArgument {
    /// Describes all violations in a client request.
    #[prost(message, repeated, tag = "1")]
    pub field_violations: ::prost::alloc::vec::Vec<invalid_argument::FieldViolation>,
}
/// Nested message and enum types in `InvalidArgument`.
pub mod invalid_argument {
    /// Describes the Field in which the violation occurred.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FieldViolation {
        /// The path to the field where this violation occurred. This path is
        /// specified using `FieldMask` format:
        /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>
        #[prost(string, tag = "1")]
        pub field: ::prost::alloc::string::String,
        /// The detailed reason for this FieldViolation.
        #[prost(enumeration = "field_violation::Reason", tag = "2")]
        pub reason: i32,
        /// A message that describes the violation. This message is intended to
        /// be shown to end users, and is localized into the requesting user's
        /// preferred language.
        #[prost(string, tag = "3")]
        pub display_message: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `FieldViolation`.
    pub mod field_violation {
        /// Possible reasons a field is invalid.
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum Reason {
            /// Unknown reason.
            Unspecified = 0,
            /// The referenced field is required.
            FieldRequired = 1,
            /// The referenced value was invalid.
            InvalidValue = 2,
            /// The specified numeric value is out of the allowed range.
            ValueOutOfRange = 3,
            /// The specified string value was too long.
            StringValueTooLong = 4,
            /// The number of entries exceeded the maximum.
            MaxEntriesExceeded = 5,
            /// The specified field is not found in the Label.
            FieldNotFound = 6,
            /// The specified choice is not found in the Field.
            ChoiceNotFound = 7,
        }
        impl Reason {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Reason::Unspecified => "REASON_UNSPECIFIED",
                    Reason::FieldRequired => "FIELD_REQUIRED",
                    Reason::InvalidValue => "INVALID_VALUE",
                    Reason::ValueOutOfRange => "VALUE_OUT_OF_RANGE",
                    Reason::StringValueTooLong => "STRING_VALUE_TOO_LONG",
                    Reason::MaxEntriesExceeded => "MAX_ENTRIES_EXCEEDED",
                    Reason::FieldNotFound => "FIELD_NOT_FOUND",
                    Reason::ChoiceNotFound => "CHOICE_NOT_FOUND",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "REASON_UNSPECIFIED" => Some(Self::Unspecified),
                    "FIELD_REQUIRED" => Some(Self::FieldRequired),
                    "INVALID_VALUE" => Some(Self::InvalidValue),
                    "VALUE_OUT_OF_RANGE" => Some(Self::ValueOutOfRange),
                    "STRING_VALUE_TOO_LONG" => Some(Self::StringValueTooLong),
                    "MAX_ENTRIES_EXCEEDED" => Some(Self::MaxEntriesExceeded),
                    "FIELD_NOT_FOUND" => Some(Self::FieldNotFound),
                    "CHOICE_NOT_FOUND" => Some(Self::ChoiceNotFound),
                    _ => None,
                }
            }
        }
    }
}
/// Describes what preconditions have failed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreconditionFailure {
    /// Describes all violations in a client request.
    #[prost(message, repeated, tag = "1")]
    pub violation: ::prost::alloc::vec::Vec<precondition_failure::Violation>,
}
/// Nested message and enum types in `PreconditionFailure`.
pub mod precondition_failure {
    /// Specific failure reason.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Violation {
        /// The path to the field where this violation occurred. This path is
        /// specified using `FieldMask` format:
        /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>
        #[prost(string, tag = "1")]
        pub field: ::prost::alloc::string::String,
        /// The type of this violation.
        #[prost(enumeration = "violation::Reason", tag = "2")]
        pub reason: i32,
        /// A message that describes the violation. This message is intended to
        /// be shown to end users, and is localized into the requesting user's
        /// preferred language.
        #[prost(string, tag = "3")]
        pub display_message: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `Violation`.
    pub mod violation {
        /// The possible reasons a the violation occurred.
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum Reason {
            /// Unknown violation type.
            Unspecified = 0,
            /// This Resource cannot be Disabled. Only Published resources can be
            /// Disabled.
            CannotDisable = 1,
            /// This Resource cannot be Enabled. Only Disabled resources can be
            /// Enabled.
            CannotEnable = 2,
            /// This Resource cannot be Published. Only Draft or Disabled resources
            /// can be Published.
            CannotPublish = 3,
            /// This Resource cannot be Unpublished. Once published, resources may
            /// not be set in "Draft" state.
            CannotUnpublish = 4,
            /// This Resource cannot be Deleted. Only Disabled resources
            /// can be Deleted.
            CannotDelete = 5,
            /// The request modified a range in a Field, but the new range does
            /// not include the previous range. When this error happens, `field` points
            /// at the Field where the violation occurred.
            CannotRestrictRange = 6,
            /// The specified change cannot be made to published Resources.
            CannotChangePublishedField = 7,
            /// The customer cannot create new labels because the maximum number
            /// of labels for the customer has been reached.
            CannotCreateMoreLabels = 8,
            /// The Field type cannot be changed because the Field has been published.
            CannotChangePublishedFieldType = 9,
            /// The Label component is locked and cannot be modified
            CannotModifyLockedComponent = 10,
        }
        impl Reason {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Reason::Unspecified => "REASON_UNSPECIFIED",
                    Reason::CannotDisable => "CANNOT_DISABLE",
                    Reason::CannotEnable => "CANNOT_ENABLE",
                    Reason::CannotPublish => "CANNOT_PUBLISH",
                    Reason::CannotUnpublish => "CANNOT_UNPUBLISH",
                    Reason::CannotDelete => "CANNOT_DELETE",
                    Reason::CannotRestrictRange => "CANNOT_RESTRICT_RANGE",
                    Reason::CannotChangePublishedField => "CANNOT_CHANGE_PUBLISHED_FIELD",
                    Reason::CannotCreateMoreLabels => "CANNOT_CREATE_MORE_LABELS",
                    Reason::CannotChangePublishedFieldType => {
                        "CANNOT_CHANGE_PUBLISHED_FIELD_TYPE"
                    }
                    Reason::CannotModifyLockedComponent => {
                        "CANNOT_MODIFY_LOCKED_COMPONENT"
                    }
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "REASON_UNSPECIFIED" => Some(Self::Unspecified),
                    "CANNOT_DISABLE" => Some(Self::CannotDisable),
                    "CANNOT_ENABLE" => Some(Self::CannotEnable),
                    "CANNOT_PUBLISH" => Some(Self::CannotPublish),
                    "CANNOT_UNPUBLISH" => Some(Self::CannotUnpublish),
                    "CANNOT_DELETE" => Some(Self::CannotDelete),
                    "CANNOT_RESTRICT_RANGE" => Some(Self::CannotRestrictRange),
                    "CANNOT_CHANGE_PUBLISHED_FIELD" => {
                        Some(Self::CannotChangePublishedField)
                    }
                    "CANNOT_CREATE_MORE_LABELS" => Some(Self::CannotCreateMoreLabels),
                    "CANNOT_CHANGE_PUBLISHED_FIELD_TYPE" => {
                        Some(Self::CannotChangePublishedFieldType)
                    }
                    "CANNOT_MODIFY_LOCKED_COMPONENT" => {
                        Some(Self::CannotModifyLockedComponent)
                    }
                    _ => None,
                }
            }
        }
    }
}
