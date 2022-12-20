/// Represents the input to API methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Document {
    /// Required. If the type is not set or is `TYPE_UNSPECIFIED`,
    /// returns an `INVALID_ARGUMENT` error.
    #[prost(enumeration = "document::Type", tag = "1")]
    pub r#type: i32,
    /// The language of the document (if not specified, the language is
    /// automatically detected). Both ISO and BCP-47 language codes are
    /// accepted.<br>
    /// [Language
    /// Support](<https://cloud.google.com/natural-language/docs/languages>) lists
    /// currently supported languages for each API method. If the language (either
    /// specified by the caller or automatically detected) is not supported by the
    /// called API method, an `INVALID_ARGUMENT` error is returned.
    #[prost(string, tag = "4")]
    pub language: ::prost::alloc::string::String,
    /// The web URI where the document comes from. This URI is not used for
    /// fetching the content, but as a hint for analyzing the document.
    #[prost(string, tag = "5")]
    pub reference_web_uri: ::prost::alloc::string::String,
    /// Indicates how detected boilerplate(e.g. advertisements, copyright
    /// declarations, banners) should be handled for this document. If not
    /// specified, boilerplate will be treated the same as content.
    #[prost(enumeration = "document::BoilerplateHandling", tag = "6")]
    pub boilerplate_handling: i32,
    /// The source of the document: a string containing the content or a
    /// Google Cloud Storage URI.
    #[prost(oneof = "document::Source", tags = "2, 3")]
    pub source: ::core::option::Option<document::Source>,
}
/// Nested message and enum types in `Document`.
pub mod document {
    /// The document types enum.
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
    pub enum Type {
        /// The content type is not specified.
        Unspecified = 0,
        /// Plain text
        PlainText = 1,
        /// HTML
        Html = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::PlainText => "PLAIN_TEXT",
                Type::Html => "HTML",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "PLAIN_TEXT" => Some(Self::PlainText),
                "HTML" => Some(Self::Html),
                _ => None,
            }
        }
    }
    /// Ways of handling boilerplate detected in the document
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
    pub enum BoilerplateHandling {
        /// The boilerplate handling is not specified.
        Unspecified = 0,
        /// Do not analyze detected boilerplate. Reference web URI is required for
        /// detecting boilerplate.
        SkipBoilerplate = 1,
        /// Treat boilerplate the same as content.
        KeepBoilerplate = 2,
    }
    impl BoilerplateHandling {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BoilerplateHandling::Unspecified => "BOILERPLATE_HANDLING_UNSPECIFIED",
                BoilerplateHandling::SkipBoilerplate => "SKIP_BOILERPLATE",
                BoilerplateHandling::KeepBoilerplate => "KEEP_BOILERPLATE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BOILERPLATE_HANDLING_UNSPECIFIED" => Some(Self::Unspecified),
                "SKIP_BOILERPLATE" => Some(Self::SkipBoilerplate),
                "KEEP_BOILERPLATE" => Some(Self::KeepBoilerplate),
                _ => None,
            }
        }
    }
    /// The source of the document: a string containing the content or a
    /// Google Cloud Storage URI.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// The content of the input in string format.
        /// Cloud audit logging exempt since it is based on user data.
        #[prost(string, tag = "2")]
        Content(::prost::alloc::string::String),
        /// The Google Cloud Storage URI where the file content is located.
        /// This URI must be of the form: gs://bucket_name/object_name. For more
        /// details, see <https://cloud.google.com/storage/docs/reference-uris.>
        /// NOTE: Cloud Storage object versioning is not supported.
        #[prost(string, tag = "3")]
        GcsContentUri(::prost::alloc::string::String),
    }
}
/// Represents a sentence in the input document.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sentence {
    /// The sentence text.
    #[prost(message, optional, tag = "1")]
    pub text: ::core::option::Option<TextSpan>,
    /// For calls to \[AnalyzeSentiment][\] or if
    /// \[AnnotateTextRequest.Features.extract_document_sentiment][google.cloud.language.v1beta2.AnnotateTextRequest.Features.extract_document_sentiment\] is set to
    /// true, this field will contain the sentiment for the sentence.
    #[prost(message, optional, tag = "2")]
    pub sentiment: ::core::option::Option<Sentiment>,
}
/// Represents a phrase in the text that is a known entity, such as
/// a person, an organization, or location. The API associates information, such
/// as salience and mentions, with entities.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entity {
    /// The representative name for the entity.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The entity type.
    #[prost(enumeration = "entity::Type", tag = "2")]
    pub r#type: i32,
    /// Metadata associated with the entity.
    ///
    /// For most entity types, the metadata is a Wikipedia URL (`wikipedia_url`)
    /// and Knowledge Graph MID (`mid`), if they are available. For the metadata
    /// associated with other entity types, see the Type table below.
    #[prost(btree_map = "string, string", tag = "3")]
    pub metadata: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The salience score associated with the entity in the [0, 1.0] range.
    ///
    /// The salience score for an entity provides information about the
    /// importance or centrality of that entity to the entire document text.
    /// Scores closer to 0 are less salient, while scores closer to 1.0 are highly
    /// salient.
    #[prost(float, tag = "4")]
    pub salience: f32,
    /// The mentions of this entity in the input document. The API currently
    /// supports proper noun mentions.
    #[prost(message, repeated, tag = "5")]
    pub mentions: ::prost::alloc::vec::Vec<EntityMention>,
    /// For calls to \[AnalyzeEntitySentiment][\] or if
    /// \[AnnotateTextRequest.Features.extract_entity_sentiment][google.cloud.language.v1beta2.AnnotateTextRequest.Features.extract_entity_sentiment\] is set to
    /// true, this field will contain the aggregate sentiment expressed for this
    /// entity in the provided document.
    #[prost(message, optional, tag = "6")]
    pub sentiment: ::core::option::Option<Sentiment>,
}
/// Nested message and enum types in `Entity`.
pub mod entity {
    /// The type of the entity. For most entity types, the associated metadata is a
    /// Wikipedia URL (`wikipedia_url`) and Knowledge Graph MID (`mid`). The table
    /// below lists the associated fields for entities that have different
    /// metadata.
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
    pub enum Type {
        /// Unknown
        Unknown = 0,
        /// Person
        Person = 1,
        /// Location
        Location = 2,
        /// Organization
        Organization = 3,
        /// Event
        Event = 4,
        /// Artwork
        WorkOfArt = 5,
        /// Consumer product
        ConsumerGood = 6,
        /// Other types of entities
        Other = 7,
        /// Phone number
        ///
        /// The metadata lists the phone number, formatted according to local
        /// convention, plus whichever additional elements appear in the text:
        ///
        /// * `number` - the actual number, broken down into sections as per local
        /// convention
        /// * `national_prefix` - country code, if detected
        /// * `area_code` - region or area code, if detected
        /// * `extension` - phone extension (to be dialed after connection), if
        /// detected
        PhoneNumber = 9,
        /// Address
        ///
        /// The metadata identifies the street number and locality plus whichever
        /// additional elements appear in the text:
        ///
        /// * `street_number` - street number
        /// * `locality` - city or town
        /// * `street_name` - street/route name, if detected
        /// * `postal_code` - postal code, if detected
        /// * `country` - country, if detected<
        /// * `broad_region` - administrative area, such as the state, if detected
        /// * `narrow_region` - smaller administrative area, such as county, if
        /// detected
        /// * `sublocality` - used in Asian addresses to demark a district within a
        /// city, if detected
        Address = 10,
        /// Date
        ///
        /// The metadata identifies the components of the date:
        ///
        /// * `year` - four digit year, if detected
        /// * `month` - two digit month number, if detected
        /// * `day` - two digit day number, if detected
        Date = 11,
        /// Number
        ///
        /// The metadata is the number itself.
        Number = 12,
        /// Price
        ///
        /// The metadata identifies the `value` and `currency`.
        Price = 13,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unknown => "UNKNOWN",
                Type::Person => "PERSON",
                Type::Location => "LOCATION",
                Type::Organization => "ORGANIZATION",
                Type::Event => "EVENT",
                Type::WorkOfArt => "WORK_OF_ART",
                Type::ConsumerGood => "CONSUMER_GOOD",
                Type::Other => "OTHER",
                Type::PhoneNumber => "PHONE_NUMBER",
                Type::Address => "ADDRESS",
                Type::Date => "DATE",
                Type::Number => "NUMBER",
                Type::Price => "PRICE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "PERSON" => Some(Self::Person),
                "LOCATION" => Some(Self::Location),
                "ORGANIZATION" => Some(Self::Organization),
                "EVENT" => Some(Self::Event),
                "WORK_OF_ART" => Some(Self::WorkOfArt),
                "CONSUMER_GOOD" => Some(Self::ConsumerGood),
                "OTHER" => Some(Self::Other),
                "PHONE_NUMBER" => Some(Self::PhoneNumber),
                "ADDRESS" => Some(Self::Address),
                "DATE" => Some(Self::Date),
                "NUMBER" => Some(Self::Number),
                "PRICE" => Some(Self::Price),
                _ => None,
            }
        }
    }
}
/// Represents the smallest syntactic building block of the text.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Token {
    /// The token text.
    #[prost(message, optional, tag = "1")]
    pub text: ::core::option::Option<TextSpan>,
    /// Parts of speech tag for this token.
    #[prost(message, optional, tag = "2")]
    pub part_of_speech: ::core::option::Option<PartOfSpeech>,
    /// Dependency tree parse for this token.
    #[prost(message, optional, tag = "3")]
    pub dependency_edge: ::core::option::Option<DependencyEdge>,
    /// \[Lemma\](<https://en.wikipedia.org/wiki/Lemma_%28morphology%29>) of the token.
    #[prost(string, tag = "4")]
    pub lemma: ::prost::alloc::string::String,
}
/// Represents the feeling associated with the entire text or entities in
/// the text.
/// Next ID: 6
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sentiment {
    /// A non-negative number in the [0, +inf) range, which represents
    /// the absolute magnitude of sentiment regardless of score (positive or
    /// negative).
    #[prost(float, tag = "2")]
    pub magnitude: f32,
    /// Sentiment score between -1.0 (negative sentiment) and 1.0
    /// (positive sentiment).
    #[prost(float, tag = "3")]
    pub score: f32,
}
/// Represents part of speech information for a token.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartOfSpeech {
    /// The part of speech tag.
    #[prost(enumeration = "part_of_speech::Tag", tag = "1")]
    pub tag: i32,
    /// The grammatical aspect.
    #[prost(enumeration = "part_of_speech::Aspect", tag = "2")]
    pub aspect: i32,
    /// The grammatical case.
    #[prost(enumeration = "part_of_speech::Case", tag = "3")]
    pub case: i32,
    /// The grammatical form.
    #[prost(enumeration = "part_of_speech::Form", tag = "4")]
    pub form: i32,
    /// The grammatical gender.
    #[prost(enumeration = "part_of_speech::Gender", tag = "5")]
    pub gender: i32,
    /// The grammatical mood.
    #[prost(enumeration = "part_of_speech::Mood", tag = "6")]
    pub mood: i32,
    /// The grammatical number.
    #[prost(enumeration = "part_of_speech::Number", tag = "7")]
    pub number: i32,
    /// The grammatical person.
    #[prost(enumeration = "part_of_speech::Person", tag = "8")]
    pub person: i32,
    /// The grammatical properness.
    #[prost(enumeration = "part_of_speech::Proper", tag = "9")]
    pub proper: i32,
    /// The grammatical reciprocity.
    #[prost(enumeration = "part_of_speech::Reciprocity", tag = "10")]
    pub reciprocity: i32,
    /// The grammatical tense.
    #[prost(enumeration = "part_of_speech::Tense", tag = "11")]
    pub tense: i32,
    /// The grammatical voice.
    #[prost(enumeration = "part_of_speech::Voice", tag = "12")]
    pub voice: i32,
}
/// Nested message and enum types in `PartOfSpeech`.
pub mod part_of_speech {
    /// The part of speech tags enum.
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
    pub enum Tag {
        /// Unknown
        Unknown = 0,
        /// Adjective
        Adj = 1,
        /// Adposition (preposition and postposition)
        Adp = 2,
        /// Adverb
        Adv = 3,
        /// Conjunction
        Conj = 4,
        /// Determiner
        Det = 5,
        /// Noun (common and proper)
        Noun = 6,
        /// Cardinal number
        Num = 7,
        /// Pronoun
        Pron = 8,
        /// Particle or other function word
        Prt = 9,
        /// Punctuation
        Punct = 10,
        /// Verb (all tenses and modes)
        Verb = 11,
        /// Other: foreign words, typos, abbreviations
        X = 12,
        /// Affix
        Affix = 13,
    }
    impl Tag {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Tag::Unknown => "UNKNOWN",
                Tag::Adj => "ADJ",
                Tag::Adp => "ADP",
                Tag::Adv => "ADV",
                Tag::Conj => "CONJ",
                Tag::Det => "DET",
                Tag::Noun => "NOUN",
                Tag::Num => "NUM",
                Tag::Pron => "PRON",
                Tag::Prt => "PRT",
                Tag::Punct => "PUNCT",
                Tag::Verb => "VERB",
                Tag::X => "X",
                Tag::Affix => "AFFIX",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "ADJ" => Some(Self::Adj),
                "ADP" => Some(Self::Adp),
                "ADV" => Some(Self::Adv),
                "CONJ" => Some(Self::Conj),
                "DET" => Some(Self::Det),
                "NOUN" => Some(Self::Noun),
                "NUM" => Some(Self::Num),
                "PRON" => Some(Self::Pron),
                "PRT" => Some(Self::Prt),
                "PUNCT" => Some(Self::Punct),
                "VERB" => Some(Self::Verb),
                "X" => Some(Self::X),
                "AFFIX" => Some(Self::Affix),
                _ => None,
            }
        }
    }
    /// The characteristic of a verb that expresses time flow during an event.
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
    pub enum Aspect {
        /// Aspect is not applicable in the analyzed language or is not predicted.
        Unknown = 0,
        /// Perfective
        Perfective = 1,
        /// Imperfective
        Imperfective = 2,
        /// Progressive
        Progressive = 3,
    }
    impl Aspect {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Aspect::Unknown => "ASPECT_UNKNOWN",
                Aspect::Perfective => "PERFECTIVE",
                Aspect::Imperfective => "IMPERFECTIVE",
                Aspect::Progressive => "PROGRESSIVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ASPECT_UNKNOWN" => Some(Self::Unknown),
                "PERFECTIVE" => Some(Self::Perfective),
                "IMPERFECTIVE" => Some(Self::Imperfective),
                "PROGRESSIVE" => Some(Self::Progressive),
                _ => None,
            }
        }
    }
    /// The grammatical function performed by a noun or pronoun in a phrase,
    /// clause, or sentence. In some languages, other parts of speech, such as
    /// adjective and determiner, take case inflection in agreement with the noun.
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
    pub enum Case {
        /// Case is not applicable in the analyzed language or is not predicted.
        Unknown = 0,
        /// Accusative
        Accusative = 1,
        /// Adverbial
        Adverbial = 2,
        /// Complementive
        Complementive = 3,
        /// Dative
        Dative = 4,
        /// Genitive
        Genitive = 5,
        /// Instrumental
        Instrumental = 6,
        /// Locative
        Locative = 7,
        /// Nominative
        Nominative = 8,
        /// Oblique
        Oblique = 9,
        /// Partitive
        Partitive = 10,
        /// Prepositional
        Prepositional = 11,
        /// Reflexive
        ReflexiveCase = 12,
        /// Relative
        RelativeCase = 13,
        /// Vocative
        Vocative = 14,
    }
    impl Case {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Case::Unknown => "CASE_UNKNOWN",
                Case::Accusative => "ACCUSATIVE",
                Case::Adverbial => "ADVERBIAL",
                Case::Complementive => "COMPLEMENTIVE",
                Case::Dative => "DATIVE",
                Case::Genitive => "GENITIVE",
                Case::Instrumental => "INSTRUMENTAL",
                Case::Locative => "LOCATIVE",
                Case::Nominative => "NOMINATIVE",
                Case::Oblique => "OBLIQUE",
                Case::Partitive => "PARTITIVE",
                Case::Prepositional => "PREPOSITIONAL",
                Case::ReflexiveCase => "REFLEXIVE_CASE",
                Case::RelativeCase => "RELATIVE_CASE",
                Case::Vocative => "VOCATIVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CASE_UNKNOWN" => Some(Self::Unknown),
                "ACCUSATIVE" => Some(Self::Accusative),
                "ADVERBIAL" => Some(Self::Adverbial),
                "COMPLEMENTIVE" => Some(Self::Complementive),
                "DATIVE" => Some(Self::Dative),
                "GENITIVE" => Some(Self::Genitive),
                "INSTRUMENTAL" => Some(Self::Instrumental),
                "LOCATIVE" => Some(Self::Locative),
                "NOMINATIVE" => Some(Self::Nominative),
                "OBLIQUE" => Some(Self::Oblique),
                "PARTITIVE" => Some(Self::Partitive),
                "PREPOSITIONAL" => Some(Self::Prepositional),
                "REFLEXIVE_CASE" => Some(Self::ReflexiveCase),
                "RELATIVE_CASE" => Some(Self::RelativeCase),
                "VOCATIVE" => Some(Self::Vocative),
                _ => None,
            }
        }
    }
    /// Depending on the language, Form can be categorizing different forms of
    /// verbs, adjectives, adverbs, etc. For example, categorizing inflected
    /// endings of verbs and adjectives or distinguishing between short and long
    /// forms of adjectives and participles
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
    pub enum Form {
        /// Form is not applicable in the analyzed language or is not predicted.
        Unknown = 0,
        /// Adnomial
        Adnomial = 1,
        /// Auxiliary
        Auxiliary = 2,
        /// Complementizer
        Complementizer = 3,
        /// Final ending
        FinalEnding = 4,
        /// Gerund
        Gerund = 5,
        /// Realis
        Realis = 6,
        /// Irrealis
        Irrealis = 7,
        /// Short form
        Short = 8,
        /// Long form
        Long = 9,
        /// Order form
        Order = 10,
        /// Specific form
        Specific = 11,
    }
    impl Form {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Form::Unknown => "FORM_UNKNOWN",
                Form::Adnomial => "ADNOMIAL",
                Form::Auxiliary => "AUXILIARY",
                Form::Complementizer => "COMPLEMENTIZER",
                Form::FinalEnding => "FINAL_ENDING",
                Form::Gerund => "GERUND",
                Form::Realis => "REALIS",
                Form::Irrealis => "IRREALIS",
                Form::Short => "SHORT",
                Form::Long => "LONG",
                Form::Order => "ORDER",
                Form::Specific => "SPECIFIC",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FORM_UNKNOWN" => Some(Self::Unknown),
                "ADNOMIAL" => Some(Self::Adnomial),
                "AUXILIARY" => Some(Self::Auxiliary),
                "COMPLEMENTIZER" => Some(Self::Complementizer),
                "FINAL_ENDING" => Some(Self::FinalEnding),
                "GERUND" => Some(Self::Gerund),
                "REALIS" => Some(Self::Realis),
                "IRREALIS" => Some(Self::Irrealis),
                "SHORT" => Some(Self::Short),
                "LONG" => Some(Self::Long),
                "ORDER" => Some(Self::Order),
                "SPECIFIC" => Some(Self::Specific),
                _ => None,
            }
        }
    }
    /// Gender classes of nouns reflected in the behaviour of associated words.
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
    pub enum Gender {
        /// Gender is not applicable in the analyzed language or is not predicted.
        Unknown = 0,
        /// Feminine
        Feminine = 1,
        /// Masculine
        Masculine = 2,
        /// Neuter
        Neuter = 3,
    }
    impl Gender {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Gender::Unknown => "GENDER_UNKNOWN",
                Gender::Feminine => "FEMININE",
                Gender::Masculine => "MASCULINE",
                Gender::Neuter => "NEUTER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "GENDER_UNKNOWN" => Some(Self::Unknown),
                "FEMININE" => Some(Self::Feminine),
                "MASCULINE" => Some(Self::Masculine),
                "NEUTER" => Some(Self::Neuter),
                _ => None,
            }
        }
    }
    /// The grammatical feature of verbs, used for showing modality and attitude.
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
    pub enum Mood {
        /// Mood is not applicable in the analyzed language or is not predicted.
        Unknown = 0,
        /// Conditional
        ConditionalMood = 1,
        /// Imperative
        Imperative = 2,
        /// Indicative
        Indicative = 3,
        /// Interrogative
        Interrogative = 4,
        /// Jussive
        Jussive = 5,
        /// Subjunctive
        Subjunctive = 6,
    }
    impl Mood {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Mood::Unknown => "MOOD_UNKNOWN",
                Mood::ConditionalMood => "CONDITIONAL_MOOD",
                Mood::Imperative => "IMPERATIVE",
                Mood::Indicative => "INDICATIVE",
                Mood::Interrogative => "INTERROGATIVE",
                Mood::Jussive => "JUSSIVE",
                Mood::Subjunctive => "SUBJUNCTIVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MOOD_UNKNOWN" => Some(Self::Unknown),
                "CONDITIONAL_MOOD" => Some(Self::ConditionalMood),
                "IMPERATIVE" => Some(Self::Imperative),
                "INDICATIVE" => Some(Self::Indicative),
                "INTERROGATIVE" => Some(Self::Interrogative),
                "JUSSIVE" => Some(Self::Jussive),
                "SUBJUNCTIVE" => Some(Self::Subjunctive),
                _ => None,
            }
        }
    }
    /// Count distinctions.
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
    pub enum Number {
        /// Number is not applicable in the analyzed language or is not predicted.
        Unknown = 0,
        /// Singular
        Singular = 1,
        /// Plural
        Plural = 2,
        /// Dual
        Dual = 3,
    }
    impl Number {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Number::Unknown => "NUMBER_UNKNOWN",
                Number::Singular => "SINGULAR",
                Number::Plural => "PLURAL",
                Number::Dual => "DUAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NUMBER_UNKNOWN" => Some(Self::Unknown),
                "SINGULAR" => Some(Self::Singular),
                "PLURAL" => Some(Self::Plural),
                "DUAL" => Some(Self::Dual),
                _ => None,
            }
        }
    }
    /// The distinction between the speaker, second person, third person, etc.
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
    pub enum Person {
        /// Person is not applicable in the analyzed language or is not predicted.
        Unknown = 0,
        /// First
        First = 1,
        /// Second
        Second = 2,
        /// Third
        Third = 3,
        /// Reflexive
        ReflexivePerson = 4,
    }
    impl Person {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Person::Unknown => "PERSON_UNKNOWN",
                Person::First => "FIRST",
                Person::Second => "SECOND",
                Person::Third => "THIRD",
                Person::ReflexivePerson => "REFLEXIVE_PERSON",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PERSON_UNKNOWN" => Some(Self::Unknown),
                "FIRST" => Some(Self::First),
                "SECOND" => Some(Self::Second),
                "THIRD" => Some(Self::Third),
                "REFLEXIVE_PERSON" => Some(Self::ReflexivePerson),
                _ => None,
            }
        }
    }
    /// This category shows if the token is part of a proper name.
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
    pub enum Proper {
        /// Proper is not applicable in the analyzed language or is not predicted.
        Unknown = 0,
        /// Proper
        Proper = 1,
        /// Not proper
        NotProper = 2,
    }
    impl Proper {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Proper::Unknown => "PROPER_UNKNOWN",
                Proper::Proper => "PROPER",
                Proper::NotProper => "NOT_PROPER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PROPER_UNKNOWN" => Some(Self::Unknown),
                "PROPER" => Some(Self::Proper),
                "NOT_PROPER" => Some(Self::NotProper),
                _ => None,
            }
        }
    }
    /// Reciprocal features of a pronoun.
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
    pub enum Reciprocity {
        /// Reciprocity is not applicable in the analyzed language or is not
        /// predicted.
        Unknown = 0,
        /// Reciprocal
        Reciprocal = 1,
        /// Non-reciprocal
        NonReciprocal = 2,
    }
    impl Reciprocity {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Reciprocity::Unknown => "RECIPROCITY_UNKNOWN",
                Reciprocity::Reciprocal => "RECIPROCAL",
                Reciprocity::NonReciprocal => "NON_RECIPROCAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RECIPROCITY_UNKNOWN" => Some(Self::Unknown),
                "RECIPROCAL" => Some(Self::Reciprocal),
                "NON_RECIPROCAL" => Some(Self::NonReciprocal),
                _ => None,
            }
        }
    }
    /// Time reference.
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
    pub enum Tense {
        /// Tense is not applicable in the analyzed language or is not predicted.
        Unknown = 0,
        /// Conditional
        ConditionalTense = 1,
        /// Future
        Future = 2,
        /// Past
        Past = 3,
        /// Present
        Present = 4,
        /// Imperfect
        Imperfect = 5,
        /// Pluperfect
        Pluperfect = 6,
    }
    impl Tense {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Tense::Unknown => "TENSE_UNKNOWN",
                Tense::ConditionalTense => "CONDITIONAL_TENSE",
                Tense::Future => "FUTURE",
                Tense::Past => "PAST",
                Tense::Present => "PRESENT",
                Tense::Imperfect => "IMPERFECT",
                Tense::Pluperfect => "PLUPERFECT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TENSE_UNKNOWN" => Some(Self::Unknown),
                "CONDITIONAL_TENSE" => Some(Self::ConditionalTense),
                "FUTURE" => Some(Self::Future),
                "PAST" => Some(Self::Past),
                "PRESENT" => Some(Self::Present),
                "IMPERFECT" => Some(Self::Imperfect),
                "PLUPERFECT" => Some(Self::Pluperfect),
                _ => None,
            }
        }
    }
    /// The relationship between the action that a verb expresses and the
    /// participants identified by its arguments.
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
    pub enum Voice {
        /// Voice is not applicable in the analyzed language or is not predicted.
        Unknown = 0,
        /// Active
        Active = 1,
        /// Causative
        Causative = 2,
        /// Passive
        Passive = 3,
    }
    impl Voice {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Voice::Unknown => "VOICE_UNKNOWN",
                Voice::Active => "ACTIVE",
                Voice::Causative => "CAUSATIVE",
                Voice::Passive => "PASSIVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "VOICE_UNKNOWN" => Some(Self::Unknown),
                "ACTIVE" => Some(Self::Active),
                "CAUSATIVE" => Some(Self::Causative),
                "PASSIVE" => Some(Self::Passive),
                _ => None,
            }
        }
    }
}
/// Represents dependency parse tree information for a token.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DependencyEdge {
    /// Represents the head of this token in the dependency tree.
    /// This is the index of the token which has an arc going to this token.
    /// The index is the position of the token in the array of tokens returned
    /// by the API method. If this token is a root token, then the
    /// `head_token_index` is its own index.
    #[prost(int32, tag = "1")]
    pub head_token_index: i32,
    /// The parse label for the token.
    #[prost(enumeration = "dependency_edge::Label", tag = "2")]
    pub label: i32,
}
/// Nested message and enum types in `DependencyEdge`.
pub mod dependency_edge {
    /// The parse label enum for the token.
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
    pub enum Label {
        /// Unknown
        Unknown = 0,
        /// Abbreviation modifier
        Abbrev = 1,
        /// Adjectival complement
        Acomp = 2,
        /// Adverbial clause modifier
        Advcl = 3,
        /// Adverbial modifier
        Advmod = 4,
        /// Adjectival modifier of an NP
        Amod = 5,
        /// Appositional modifier of an NP
        Appos = 6,
        /// Attribute dependent of a copular verb
        Attr = 7,
        /// Auxiliary (non-main) verb
        Aux = 8,
        /// Passive auxiliary
        Auxpass = 9,
        /// Coordinating conjunction
        Cc = 10,
        /// Clausal complement of a verb or adjective
        Ccomp = 11,
        /// Conjunct
        Conj = 12,
        /// Clausal subject
        Csubj = 13,
        /// Clausal passive subject
        Csubjpass = 14,
        /// Dependency (unable to determine)
        Dep = 15,
        /// Determiner
        Det = 16,
        /// Discourse
        Discourse = 17,
        /// Direct object
        Dobj = 18,
        /// Expletive
        Expl = 19,
        /// Goes with (part of a word in a text not well edited)
        Goeswith = 20,
        /// Indirect object
        Iobj = 21,
        /// Marker (word introducing a subordinate clause)
        Mark = 22,
        /// Multi-word expression
        Mwe = 23,
        /// Multi-word verbal expression
        Mwv = 24,
        /// Negation modifier
        Neg = 25,
        /// Noun compound modifier
        Nn = 26,
        /// Noun phrase used as an adverbial modifier
        Npadvmod = 27,
        /// Nominal subject
        Nsubj = 28,
        /// Passive nominal subject
        Nsubjpass = 29,
        /// Numeric modifier of a noun
        Num = 30,
        /// Element of compound number
        Number = 31,
        /// Punctuation mark
        P = 32,
        /// Parataxis relation
        Parataxis = 33,
        /// Participial modifier
        Partmod = 34,
        /// The complement of a preposition is a clause
        Pcomp = 35,
        /// Object of a preposition
        Pobj = 36,
        /// Possession modifier
        Poss = 37,
        /// Postverbal negative particle
        Postneg = 38,
        /// Predicate complement
        Precomp = 39,
        /// Preconjunt
        Preconj = 40,
        /// Predeterminer
        Predet = 41,
        /// Prefix
        Pref = 42,
        /// Prepositional modifier
        Prep = 43,
        /// The relationship between a verb and verbal morpheme
        Pronl = 44,
        /// Particle
        Prt = 45,
        /// Associative or possessive marker
        Ps = 46,
        /// Quantifier phrase modifier
        Quantmod = 47,
        /// Relative clause modifier
        Rcmod = 48,
        /// Complementizer in relative clause
        Rcmodrel = 49,
        /// Ellipsis without a preceding predicate
        Rdrop = 50,
        /// Referent
        Ref = 51,
        /// Remnant
        Remnant = 52,
        /// Reparandum
        Reparandum = 53,
        /// Root
        Root = 54,
        /// Suffix specifying a unit of number
        Snum = 55,
        /// Suffix
        Suff = 56,
        /// Temporal modifier
        Tmod = 57,
        /// Topic marker
        Topic = 58,
        /// Clause headed by an infinite form of the verb that modifies a noun
        Vmod = 59,
        /// Vocative
        Vocative = 60,
        /// Open clausal complement
        Xcomp = 61,
        /// Name suffix
        Suffix = 62,
        /// Name title
        Title = 63,
        /// Adverbial phrase modifier
        Advphmod = 64,
        /// Causative auxiliary
        Auxcaus = 65,
        /// Helper auxiliary
        Auxvv = 66,
        /// Rentaishi (Prenominal modifier)
        Dtmod = 67,
        /// Foreign words
        Foreign = 68,
        /// Keyword
        Kw = 69,
        /// List for chains of comparable items
        List = 70,
        /// Nominalized clause
        Nomc = 71,
        /// Nominalized clausal subject
        Nomcsubj = 72,
        /// Nominalized clausal passive
        Nomcsubjpass = 73,
        /// Compound of numeric modifier
        Numc = 74,
        /// Copula
        Cop = 75,
        /// Dislocated relation (for fronted/topicalized elements)
        Dislocated = 76,
        /// Aspect marker
        Asp = 77,
        /// Genitive modifier
        Gmod = 78,
        /// Genitive object
        Gobj = 79,
        /// Infinitival modifier
        Infmod = 80,
        /// Measure
        Mes = 81,
        /// Nominal complement of a noun
        Ncomp = 82,
    }
    impl Label {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Label::Unknown => "UNKNOWN",
                Label::Abbrev => "ABBREV",
                Label::Acomp => "ACOMP",
                Label::Advcl => "ADVCL",
                Label::Advmod => "ADVMOD",
                Label::Amod => "AMOD",
                Label::Appos => "APPOS",
                Label::Attr => "ATTR",
                Label::Aux => "AUX",
                Label::Auxpass => "AUXPASS",
                Label::Cc => "CC",
                Label::Ccomp => "CCOMP",
                Label::Conj => "CONJ",
                Label::Csubj => "CSUBJ",
                Label::Csubjpass => "CSUBJPASS",
                Label::Dep => "DEP",
                Label::Det => "DET",
                Label::Discourse => "DISCOURSE",
                Label::Dobj => "DOBJ",
                Label::Expl => "EXPL",
                Label::Goeswith => "GOESWITH",
                Label::Iobj => "IOBJ",
                Label::Mark => "MARK",
                Label::Mwe => "MWE",
                Label::Mwv => "MWV",
                Label::Neg => "NEG",
                Label::Nn => "NN",
                Label::Npadvmod => "NPADVMOD",
                Label::Nsubj => "NSUBJ",
                Label::Nsubjpass => "NSUBJPASS",
                Label::Num => "NUM",
                Label::Number => "NUMBER",
                Label::P => "P",
                Label::Parataxis => "PARATAXIS",
                Label::Partmod => "PARTMOD",
                Label::Pcomp => "PCOMP",
                Label::Pobj => "POBJ",
                Label::Poss => "POSS",
                Label::Postneg => "POSTNEG",
                Label::Precomp => "PRECOMP",
                Label::Preconj => "PRECONJ",
                Label::Predet => "PREDET",
                Label::Pref => "PREF",
                Label::Prep => "PREP",
                Label::Pronl => "PRONL",
                Label::Prt => "PRT",
                Label::Ps => "PS",
                Label::Quantmod => "QUANTMOD",
                Label::Rcmod => "RCMOD",
                Label::Rcmodrel => "RCMODREL",
                Label::Rdrop => "RDROP",
                Label::Ref => "REF",
                Label::Remnant => "REMNANT",
                Label::Reparandum => "REPARANDUM",
                Label::Root => "ROOT",
                Label::Snum => "SNUM",
                Label::Suff => "SUFF",
                Label::Tmod => "TMOD",
                Label::Topic => "TOPIC",
                Label::Vmod => "VMOD",
                Label::Vocative => "VOCATIVE",
                Label::Xcomp => "XCOMP",
                Label::Suffix => "SUFFIX",
                Label::Title => "TITLE",
                Label::Advphmod => "ADVPHMOD",
                Label::Auxcaus => "AUXCAUS",
                Label::Auxvv => "AUXVV",
                Label::Dtmod => "DTMOD",
                Label::Foreign => "FOREIGN",
                Label::Kw => "KW",
                Label::List => "LIST",
                Label::Nomc => "NOMC",
                Label::Nomcsubj => "NOMCSUBJ",
                Label::Nomcsubjpass => "NOMCSUBJPASS",
                Label::Numc => "NUMC",
                Label::Cop => "COP",
                Label::Dislocated => "DISLOCATED",
                Label::Asp => "ASP",
                Label::Gmod => "GMOD",
                Label::Gobj => "GOBJ",
                Label::Infmod => "INFMOD",
                Label::Mes => "MES",
                Label::Ncomp => "NCOMP",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "ABBREV" => Some(Self::Abbrev),
                "ACOMP" => Some(Self::Acomp),
                "ADVCL" => Some(Self::Advcl),
                "ADVMOD" => Some(Self::Advmod),
                "AMOD" => Some(Self::Amod),
                "APPOS" => Some(Self::Appos),
                "ATTR" => Some(Self::Attr),
                "AUX" => Some(Self::Aux),
                "AUXPASS" => Some(Self::Auxpass),
                "CC" => Some(Self::Cc),
                "CCOMP" => Some(Self::Ccomp),
                "CONJ" => Some(Self::Conj),
                "CSUBJ" => Some(Self::Csubj),
                "CSUBJPASS" => Some(Self::Csubjpass),
                "DEP" => Some(Self::Dep),
                "DET" => Some(Self::Det),
                "DISCOURSE" => Some(Self::Discourse),
                "DOBJ" => Some(Self::Dobj),
                "EXPL" => Some(Self::Expl),
                "GOESWITH" => Some(Self::Goeswith),
                "IOBJ" => Some(Self::Iobj),
                "MARK" => Some(Self::Mark),
                "MWE" => Some(Self::Mwe),
                "MWV" => Some(Self::Mwv),
                "NEG" => Some(Self::Neg),
                "NN" => Some(Self::Nn),
                "NPADVMOD" => Some(Self::Npadvmod),
                "NSUBJ" => Some(Self::Nsubj),
                "NSUBJPASS" => Some(Self::Nsubjpass),
                "NUM" => Some(Self::Num),
                "NUMBER" => Some(Self::Number),
                "P" => Some(Self::P),
                "PARATAXIS" => Some(Self::Parataxis),
                "PARTMOD" => Some(Self::Partmod),
                "PCOMP" => Some(Self::Pcomp),
                "POBJ" => Some(Self::Pobj),
                "POSS" => Some(Self::Poss),
                "POSTNEG" => Some(Self::Postneg),
                "PRECOMP" => Some(Self::Precomp),
                "PRECONJ" => Some(Self::Preconj),
                "PREDET" => Some(Self::Predet),
                "PREF" => Some(Self::Pref),
                "PREP" => Some(Self::Prep),
                "PRONL" => Some(Self::Pronl),
                "PRT" => Some(Self::Prt),
                "PS" => Some(Self::Ps),
                "QUANTMOD" => Some(Self::Quantmod),
                "RCMOD" => Some(Self::Rcmod),
                "RCMODREL" => Some(Self::Rcmodrel),
                "RDROP" => Some(Self::Rdrop),
                "REF" => Some(Self::Ref),
                "REMNANT" => Some(Self::Remnant),
                "REPARANDUM" => Some(Self::Reparandum),
                "ROOT" => Some(Self::Root),
                "SNUM" => Some(Self::Snum),
                "SUFF" => Some(Self::Suff),
                "TMOD" => Some(Self::Tmod),
                "TOPIC" => Some(Self::Topic),
                "VMOD" => Some(Self::Vmod),
                "VOCATIVE" => Some(Self::Vocative),
                "XCOMP" => Some(Self::Xcomp),
                "SUFFIX" => Some(Self::Suffix),
                "TITLE" => Some(Self::Title),
                "ADVPHMOD" => Some(Self::Advphmod),
                "AUXCAUS" => Some(Self::Auxcaus),
                "AUXVV" => Some(Self::Auxvv),
                "DTMOD" => Some(Self::Dtmod),
                "FOREIGN" => Some(Self::Foreign),
                "KW" => Some(Self::Kw),
                "LIST" => Some(Self::List),
                "NOMC" => Some(Self::Nomc),
                "NOMCSUBJ" => Some(Self::Nomcsubj),
                "NOMCSUBJPASS" => Some(Self::Nomcsubjpass),
                "NUMC" => Some(Self::Numc),
                "COP" => Some(Self::Cop),
                "DISLOCATED" => Some(Self::Dislocated),
                "ASP" => Some(Self::Asp),
                "GMOD" => Some(Self::Gmod),
                "GOBJ" => Some(Self::Gobj),
                "INFMOD" => Some(Self::Infmod),
                "MES" => Some(Self::Mes),
                "NCOMP" => Some(Self::Ncomp),
                _ => None,
            }
        }
    }
}
/// Represents a mention for an entity in the text. Currently, proper noun
/// mentions are supported.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityMention {
    /// The mention text.
    #[prost(message, optional, tag = "1")]
    pub text: ::core::option::Option<TextSpan>,
    /// The type of the entity mention.
    #[prost(enumeration = "entity_mention::Type", tag = "2")]
    pub r#type: i32,
    /// For calls to \[AnalyzeEntitySentiment][\] or if
    /// \[AnnotateTextRequest.Features.extract_entity_sentiment][google.cloud.language.v1beta2.AnnotateTextRequest.Features.extract_entity_sentiment\] is set to
    /// true, this field will contain the sentiment expressed for this mention of
    /// the entity in the provided document.
    #[prost(message, optional, tag = "3")]
    pub sentiment: ::core::option::Option<Sentiment>,
}
/// Nested message and enum types in `EntityMention`.
pub mod entity_mention {
    /// The supported types of mentions.
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
    pub enum Type {
        /// Unknown
        Unknown = 0,
        /// Proper name
        Proper = 1,
        /// Common noun (or noun compound)
        Common = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unknown => "TYPE_UNKNOWN",
                Type::Proper => "PROPER",
                Type::Common => "COMMON",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNKNOWN" => Some(Self::Unknown),
                "PROPER" => Some(Self::Proper),
                "COMMON" => Some(Self::Common),
                _ => None,
            }
        }
    }
}
/// Represents an output piece of text.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextSpan {
    /// The content of the output text.
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    /// The API calculates the beginning offset of the content in the original
    /// document according to the \[EncodingType][google.cloud.language.v1beta2.EncodingType\] specified in the API request.
    #[prost(int32, tag = "2")]
    pub begin_offset: i32,
}
/// Represents a category returned from the text classifier.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassificationCategory {
    /// The name of the category representing the document, from the [predefined
    /// taxonomy](<https://cloud.google.com/natural-language/docs/categories>).
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The classifier's confidence of the category. Number represents how certain
    /// the classifier is that this category represents the given text.
    #[prost(float, tag = "2")]
    pub confidence: f32,
}
/// Model options available for classification requests.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassificationModelOptions {
    /// If this field is not set, then the `v1_model` will be used by default.
    #[prost(oneof = "classification_model_options::ModelType", tags = "1, 2")]
    pub model_type: ::core::option::Option<classification_model_options::ModelType>,
}
/// Nested message and enum types in `ClassificationModelOptions`.
pub mod classification_model_options {
    /// Options for the V1 model.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct V1Model {}
    /// Options for the V2 model.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct V2Model {
        /// The content categories used for classification.
        #[prost(enumeration = "v2_model::ContentCategoriesVersion", tag = "1")]
        pub content_categories_version: i32,
    }
    /// Nested message and enum types in `V2Model`.
    pub mod v2_model {
        /// The content categories used for classification.
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
        pub enum ContentCategoriesVersion {
            /// If `ContentCategoriesVersion` is not specified, this option will
            /// default to `V1`.
            Unspecified = 0,
            /// Legacy content categories of our initial launch in 2017.
            V1 = 1,
            /// Updated content categories in 2022.
            V2 = 2,
        }
        impl ContentCategoriesVersion {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ContentCategoriesVersion::Unspecified => {
                        "CONTENT_CATEGORIES_VERSION_UNSPECIFIED"
                    }
                    ContentCategoriesVersion::V1 => "V1",
                    ContentCategoriesVersion::V2 => "V2",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "CONTENT_CATEGORIES_VERSION_UNSPECIFIED" => Some(Self::Unspecified),
                    "V1" => Some(Self::V1),
                    "V2" => Some(Self::V2),
                    _ => None,
                }
            }
        }
    }
    /// If this field is not set, then the `v1_model` will be used by default.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ModelType {
        /// Setting this field will use the V1 model and V1 content categories
        /// version. The V1 model is a legacy model; support for this will be
        /// discontinued in the future.
        #[prost(message, tag = "1")]
        V1Model(V1Model),
        /// Setting this field will use the V2 model with the appropriate content
        /// categories version. The V2 model is a better performing model.
        #[prost(message, tag = "2")]
        V2Model(V2Model),
    }
}
/// The sentiment analysis request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeSentimentRequest {
    /// Required. Input document.
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<Document>,
    /// The encoding type used by the API to calculate sentence offsets for the
    /// sentence sentiment.
    #[prost(enumeration = "EncodingType", tag = "2")]
    pub encoding_type: i32,
}
/// The sentiment analysis response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeSentimentResponse {
    /// The overall sentiment of the input document.
    #[prost(message, optional, tag = "1")]
    pub document_sentiment: ::core::option::Option<Sentiment>,
    /// The language of the text, which will be the same as the language specified
    /// in the request or, if not specified, the automatically-detected language.
    /// See \[Document.language][google.cloud.language.v1beta2.Document.language\] field for more details.
    #[prost(string, tag = "2")]
    pub language: ::prost::alloc::string::String,
    /// The sentiment for all the sentences in the document.
    #[prost(message, repeated, tag = "3")]
    pub sentences: ::prost::alloc::vec::Vec<Sentence>,
}
/// The entity-level sentiment analysis request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeEntitySentimentRequest {
    /// Required. Input document.
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<Document>,
    /// The encoding type used by the API to calculate offsets.
    #[prost(enumeration = "EncodingType", tag = "2")]
    pub encoding_type: i32,
}
/// The entity-level sentiment analysis response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeEntitySentimentResponse {
    /// The recognized entities in the input document with associated sentiments.
    #[prost(message, repeated, tag = "1")]
    pub entities: ::prost::alloc::vec::Vec<Entity>,
    /// The language of the text, which will be the same as the language specified
    /// in the request or, if not specified, the automatically-detected language.
    /// See \[Document.language][google.cloud.language.v1beta2.Document.language\] field for more details.
    #[prost(string, tag = "2")]
    pub language: ::prost::alloc::string::String,
}
/// The entity analysis request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeEntitiesRequest {
    /// Required. Input document.
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<Document>,
    /// The encoding type used by the API to calculate offsets.
    #[prost(enumeration = "EncodingType", tag = "2")]
    pub encoding_type: i32,
}
/// The entity analysis response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeEntitiesResponse {
    /// The recognized entities in the input document.
    #[prost(message, repeated, tag = "1")]
    pub entities: ::prost::alloc::vec::Vec<Entity>,
    /// The language of the text, which will be the same as the language specified
    /// in the request or, if not specified, the automatically-detected language.
    /// See \[Document.language][google.cloud.language.v1beta2.Document.language\] field for more details.
    #[prost(string, tag = "2")]
    pub language: ::prost::alloc::string::String,
}
/// The syntax analysis request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeSyntaxRequest {
    /// Required. Input document.
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<Document>,
    /// The encoding type used by the API to calculate offsets.
    #[prost(enumeration = "EncodingType", tag = "2")]
    pub encoding_type: i32,
}
/// The syntax analysis response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeSyntaxResponse {
    /// Sentences in the input document.
    #[prost(message, repeated, tag = "1")]
    pub sentences: ::prost::alloc::vec::Vec<Sentence>,
    /// Tokens, along with their syntactic information, in the input document.
    #[prost(message, repeated, tag = "2")]
    pub tokens: ::prost::alloc::vec::Vec<Token>,
    /// The language of the text, which will be the same as the language specified
    /// in the request or, if not specified, the automatically-detected language.
    /// See \[Document.language][google.cloud.language.v1beta2.Document.language\] field for more details.
    #[prost(string, tag = "3")]
    pub language: ::prost::alloc::string::String,
}
/// The document classification request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassifyTextRequest {
    /// Required. Input document.
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<Document>,
    /// Model options to use for classification. Defaults to v1 options if not
    /// specified.
    #[prost(message, optional, tag = "3")]
    pub classification_model_options: ::core::option::Option<ClassificationModelOptions>,
}
/// The document classification response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassifyTextResponse {
    /// Categories representing the input document.
    #[prost(message, repeated, tag = "1")]
    pub categories: ::prost::alloc::vec::Vec<ClassificationCategory>,
}
/// The request message for the text annotation API, which can perform multiple
/// analysis types (sentiment, entities, and syntax) in one call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateTextRequest {
    /// Required. Input document.
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<Document>,
    /// Required. The enabled features.
    #[prost(message, optional, tag = "2")]
    pub features: ::core::option::Option<annotate_text_request::Features>,
    /// The encoding type used by the API to calculate offsets.
    #[prost(enumeration = "EncodingType", tag = "3")]
    pub encoding_type: i32,
}
/// Nested message and enum types in `AnnotateTextRequest`.
pub mod annotate_text_request {
    /// All available features for sentiment, syntax, and semantic analysis.
    /// Setting each one to true will enable that specific analysis for the input.
    /// Next ID: 11
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Features {
        /// Extract syntax information.
        #[prost(bool, tag = "1")]
        pub extract_syntax: bool,
        /// Extract entities.
        #[prost(bool, tag = "2")]
        pub extract_entities: bool,
        /// Extract document-level sentiment.
        #[prost(bool, tag = "3")]
        pub extract_document_sentiment: bool,
        /// Extract entities and their associated sentiment.
        #[prost(bool, tag = "4")]
        pub extract_entity_sentiment: bool,
        /// Classify the full document into categories. If this is true,
        /// the API will use the default model which classifies into a
        /// [predefined
        /// taxonomy](<https://cloud.google.com/natural-language/docs/categories>).
        #[prost(bool, tag = "6")]
        pub classify_text: bool,
        /// The model options to use for classification. Defaults to v1 options
        /// if not specified. Only used if `classify_text` is set to true.
        #[prost(message, optional, tag = "10")]
        pub classification_model_options: ::core::option::Option<
            super::ClassificationModelOptions,
        >,
    }
}
/// The text annotations response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateTextResponse {
    /// Sentences in the input document. Populated if the user enables
    /// \[AnnotateTextRequest.Features.extract_syntax][google.cloud.language.v1beta2.AnnotateTextRequest.Features.extract_syntax\].
    #[prost(message, repeated, tag = "1")]
    pub sentences: ::prost::alloc::vec::Vec<Sentence>,
    /// Tokens, along with their syntactic information, in the input document.
    /// Populated if the user enables
    /// \[AnnotateTextRequest.Features.extract_syntax][google.cloud.language.v1beta2.AnnotateTextRequest.Features.extract_syntax\].
    #[prost(message, repeated, tag = "2")]
    pub tokens: ::prost::alloc::vec::Vec<Token>,
    /// Entities, along with their semantic information, in the input document.
    /// Populated if the user enables
    /// \[AnnotateTextRequest.Features.extract_entities][google.cloud.language.v1beta2.AnnotateTextRequest.Features.extract_entities\].
    #[prost(message, repeated, tag = "3")]
    pub entities: ::prost::alloc::vec::Vec<Entity>,
    /// The overall sentiment for the document. Populated if the user enables
    /// \[AnnotateTextRequest.Features.extract_document_sentiment][google.cloud.language.v1beta2.AnnotateTextRequest.Features.extract_document_sentiment\].
    #[prost(message, optional, tag = "4")]
    pub document_sentiment: ::core::option::Option<Sentiment>,
    /// The language of the text, which will be the same as the language specified
    /// in the request or, if not specified, the automatically-detected language.
    /// See \[Document.language][google.cloud.language.v1beta2.Document.language\] field for more details.
    #[prost(string, tag = "5")]
    pub language: ::prost::alloc::string::String,
    /// Categories identified in the input document.
    #[prost(message, repeated, tag = "6")]
    pub categories: ::prost::alloc::vec::Vec<ClassificationCategory>,
}
/// Represents the text encoding that the caller uses to process the output.
/// Providing an `EncodingType` is recommended because the API provides the
/// beginning offsets for various outputs, such as tokens and mentions, and
/// languages that natively use different text encodings may access offsets
/// differently.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EncodingType {
    /// If `EncodingType` is not specified, encoding-dependent information (such as
    /// `begin_offset`) will be set at `-1`.
    None = 0,
    /// Encoding-dependent information (such as `begin_offset`) is calculated based
    /// on the UTF-8 encoding of the input. C++ and Go are examples of languages
    /// that use this encoding natively.
    Utf8 = 1,
    /// Encoding-dependent information (such as `begin_offset`) is calculated based
    /// on the UTF-16 encoding of the input. Java and JavaScript are examples of
    /// languages that use this encoding natively.
    Utf16 = 2,
    /// Encoding-dependent information (such as `begin_offset`) is calculated based
    /// on the UTF-32 encoding of the input. Python is an example of a language
    /// that uses this encoding natively.
    Utf32 = 3,
}
impl EncodingType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EncodingType::None => "NONE",
            EncodingType::Utf8 => "UTF8",
            EncodingType::Utf16 => "UTF16",
            EncodingType::Utf32 => "UTF32",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NONE" => Some(Self::None),
            "UTF8" => Some(Self::Utf8),
            "UTF16" => Some(Self::Utf16),
            "UTF32" => Some(Self::Utf32),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod language_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Provides text analysis operations such as sentiment analysis and entity
    /// recognition.
    #[derive(Debug, Clone)]
    pub struct LanguageServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> LanguageServiceClient<T>
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
        ) -> LanguageServiceClient<InterceptedService<T, F>>
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
            LanguageServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Analyzes the sentiment of the provided text.
        pub async fn analyze_sentiment(
            &mut self,
            request: impl tonic::IntoRequest<super::AnalyzeSentimentRequest>,
        ) -> Result<tonic::Response<super::AnalyzeSentimentResponse>, tonic::Status> {
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
                "/google.cloud.language.v1beta2.LanguageService/AnalyzeSentiment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Finds named entities (currently proper names and common nouns) in the text
        /// along with entity types, salience, mentions for each entity, and
        /// other properties.
        pub async fn analyze_entities(
            &mut self,
            request: impl tonic::IntoRequest<super::AnalyzeEntitiesRequest>,
        ) -> Result<tonic::Response<super::AnalyzeEntitiesResponse>, tonic::Status> {
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
                "/google.cloud.language.v1beta2.LanguageService/AnalyzeEntities",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Finds entities, similar to [AnalyzeEntities][google.cloud.language.v1beta2.LanguageService.AnalyzeEntities] in the text and analyzes
        /// sentiment associated with each entity and its mentions.
        pub async fn analyze_entity_sentiment(
            &mut self,
            request: impl tonic::IntoRequest<super::AnalyzeEntitySentimentRequest>,
        ) -> Result<
            tonic::Response<super::AnalyzeEntitySentimentResponse>,
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
                "/google.cloud.language.v1beta2.LanguageService/AnalyzeEntitySentiment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Analyzes the syntax of the text and provides sentence boundaries and
        /// tokenization along with part of speech tags, dependency trees, and other
        /// properties.
        pub async fn analyze_syntax(
            &mut self,
            request: impl tonic::IntoRequest<super::AnalyzeSyntaxRequest>,
        ) -> Result<tonic::Response<super::AnalyzeSyntaxResponse>, tonic::Status> {
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
                "/google.cloud.language.v1beta2.LanguageService/AnalyzeSyntax",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Classifies a document into categories.
        pub async fn classify_text(
            &mut self,
            request: impl tonic::IntoRequest<super::ClassifyTextRequest>,
        ) -> Result<tonic::Response<super::ClassifyTextResponse>, tonic::Status> {
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
                "/google.cloud.language.v1beta2.LanguageService/ClassifyText",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// A convenience method that provides all syntax, sentiment, entity, and
        /// classification features in one call.
        pub async fn annotate_text(
            &mut self,
            request: impl tonic::IntoRequest<super::AnnotateTextRequest>,
        ) -> Result<tonic::Response<super::AnnotateTextResponse>, tonic::Status> {
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
                "/google.cloud.language.v1beta2.LanguageService/AnnotateText",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
