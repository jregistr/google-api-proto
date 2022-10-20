/// Elements that will be displayed on the canvas once a particular type's entity
/// is extracted from a query. Only relevant for canvas enabled apps.
/// **This message is localizable.**
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityDisplay {
    /// Optional. Title of the icon.
    #[prost(string, tag="1")]
    pub icon_title: ::prost::alloc::string::String,
    /// Required. Url of the icon.
    #[prost(string, tag="2")]
    pub icon_url: ::prost::alloc::string::String,
}
/// Type that matches text by set of synonyms.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SynonymType {
    /// Optional. The match type for the synonym.
    #[prost(enumeration="synonym_type::MatchType", tag="1")]
    pub match_type: i32,
    /// Optional. When set to true this will match unknown words or phrases based on
    /// surrounding input and intent training data, such as items that might be
    /// added to a grocery list.
    #[prost(bool, tag="3")]
    pub accept_unknown_values: bool,
    /// Required. Named map of synonym entities.
    #[prost(btree_map="string, message", tag="2")]
    pub entities: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, synonym_type::Entity>,
}
/// Nested message and enum types in `SynonymType`.
pub mod synonym_type {
    /// Represents a synonym entity field that contains the details of a single
    /// entry inside the type.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Entity {
        /// Optional. The entity display details.
        #[prost(message, optional, tag="1")]
        pub display: ::core::option::Option<super::EntityDisplay>,
        /// Optional. The list of synonyms for the entity.
        /// **This field is localizable.**
        #[prost(string, repeated, tag="2")]
        pub synonyms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// The type of matching that entries in this type will use. This will ensure
    /// all of the types use the same matching method and allow variation of
    /// matching for synonym matching (i.e. fuzzy versus exact). If the value is
    /// `UNSPECIFIED` it will be defaulted to `EXACT_MATCH`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MatchType {
        /// Defaults to `EXACT_MATCH`.
        Unspecified = 0,
        /// Looks for an exact match of the synonym or name.
        ExactMatch = 1,
        /// Looser than `EXACT_MATCH`. Looks for similar matches as well as exact
        /// matches.
        FuzzyMatch = 2,
    }
    impl MatchType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MatchType::Unspecified => "UNSPECIFIED",
                MatchType::ExactMatch => "EXACT_MATCH",
                MatchType::FuzzyMatch => "FUZZY_MATCH",
            }
        }
    }
}
/// Type that matches text by regular expressions.
/// **This message is localizable.**
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegularExpressionType {
    /// Required. Named map of entities which each contain Regex strings.
    #[prost(btree_map="string, message", tag="1")]
    pub entities: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, regular_expression_type::Entity>,
}
/// Nested message and enum types in `RegularExpressionType`.
pub mod regular_expression_type {
    /// Represents an entity object that contains the regular expression that is
    /// used for comparison.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Entity {
        /// Optional. Elements that will be displayed on the canvas once an entity is
        /// extracted from a query. Only relevant for canvas enabled apps.
        #[prost(message, optional, tag="1")]
        pub display: ::core::option::Option<super::EntityDisplay>,
        /// Required. Uses RE2 regex syntax (See
        /// <https://github.com/google/re2/wiki/Syntax> for more details)
        #[prost(string, repeated, tag="2")]
        pub regular_expressions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// A reference to a class which is used to declare the type of a field or return
/// value. Enums are also a type of class that can be referenced using
/// ClassReference.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassReference {
    /// Required. Name of a built-in type or custom type of the parameter. Examples:
    /// `PizzaToppings`, `actions.type.Number`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Indicates whether the data type represents a list of values.
    #[prost(bool, tag="2")]
    pub list: bool,
}
/// Type that matches any text if surrounding words context is close to provided
/// training examples.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FreeTextType {
    /// Optional. Elements that will be displayed on the canvas once an entity is extracted
    /// from a query. Only relevant for canvas enabled apps.
    #[prost(message, optional, tag="2")]
    pub display: ::core::option::Option<EntityDisplay>,
}
/// Declaration of a custom type, as opposed to built-in types. Types can be
/// assigned to slots in a scene or parameters of an intent's training phrases.
/// Practically, Types can be thought of as enums.
/// Note, type name is specified in the name of the file.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Type {
    /// Set of exceptional words/phrases that shouldn't be matched by type.
    /// Note: If word/phrase is matched by the type but listed as an exclusion it
    /// won't be returned in parameter extraction result.
    /// **This field is localizable.**
    #[prost(string, repeated, tag="4")]
    pub exclusions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Selection of sub type based on the type of matching to be done.
    #[prost(oneof="r#type::SubType", tags="1, 2, 3")]
    pub sub_type: ::core::option::Option<r#type::SubType>,
}
/// Nested message and enum types in `Type`.
pub mod r#type {
    /// Selection of sub type based on the type of matching to be done.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SubType {
        /// Synonyms type, which is essentially an enum.
        #[prost(message, tag="1")]
        Synonym(super::SynonymType),
        /// Regex type, allows regular expression matching.
        #[prost(message, tag="2")]
        RegularExpression(super::RegularExpressionType),
        /// FreeText type.
        #[prost(message, tag="3")]
        FreeText(super::FreeTextType),
    }
}
