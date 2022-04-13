/// A Firestore document.
///
/// Must not exceed 1 MiB - 4 bytes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Document {
    /// The resource name of the document, for example
    /// `projects/{project_id}/databases/{database_id}/documents/{document_path}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The document's fields.
    ///
    /// The map keys represent field names.
    ///
    /// A simple field name contains only characters `a` to `z`, `A` to `Z`,
    /// `0` to `9`, or `_`, and must not start with `0` to `9`. For example,
    /// `foo_bar_17`.
    ///
    /// Field names matching the regular expression `__.*__` are reserved. Reserved
    /// field names are forbidden except in certain documented contexts. The map
    /// keys, represented as UTF-8, must not exceed 1,500 bytes and cannot be
    /// empty.
    ///
    /// Field paths may be used in other contexts to refer to structured fields
    /// defined here. For `map_value`, the field path is represented by the simple
    /// or quoted field names of the containing fields, delimited by `.`. For
    /// example, the structured field
    /// `"foo" : { map_value: { "x&y" : { string_value: "hello" }}}` would be
    /// represented by the field path `foo.x&y`.
    ///
    /// Within a field path, a quoted field name starts and ends with `` ` `` and
    /// may contain any character. Some characters, including `` ` ``, must be
    /// escaped using a `\`. For example, `` `x&y` `` represents `x&y` and
    /// `` `bak\`tik` `` represents `` bak`tik ``.
    #[prost(btree_map = "string, message", tag = "2")]
    pub fields: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, Value>,
    /// Output only. The time at which the document was created.
    ///
    /// This value increases monotonically when a document is deleted then
    /// recreated. It can also be compared to values from other documents and
    /// the `read_time` of a query.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which the document was last changed.
    ///
    /// This value is initially set to the `create_time` then increases
    /// monotonically with each change to the document. It can also be
    /// compared to values from other documents and the `read_time` of a query.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// A message that can hold any of the supported value types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    /// Must have a value set.
    #[prost(
        oneof = "value::ValueType",
        tags = "11, 1, 2, 3, 10, 17, 18, 5, 8, 9, 6"
    )]
    pub value_type: ::core::option::Option<value::ValueType>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    /// Must have a value set.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ValueType {
        /// A null value.
        #[prost(enumeration = "::prost_types::NullValue", tag = "11")]
        NullValue(i32),
        /// A boolean value.
        #[prost(bool, tag = "1")]
        BooleanValue(bool),
        /// An integer value.
        #[prost(int64, tag = "2")]
        IntegerValue(i64),
        /// A double value.
        #[prost(double, tag = "3")]
        DoubleValue(f64),
        /// A timestamp value.
        ///
        /// Precise only to microseconds. When stored, any additional precision is
        /// rounded down.
        #[prost(message, tag = "10")]
        TimestampValue(::prost_types::Timestamp),
        /// A string value.
        ///
        /// The string, represented as UTF-8, must not exceed 1 MiB - 89 bytes.
        /// Only the first 1,500 bytes of the UTF-8 representation are considered by
        /// queries.
        #[prost(string, tag = "17")]
        StringValue(::prost::alloc::string::String),
        /// A bytes value.
        ///
        /// Must not exceed 1 MiB - 89 bytes.
        /// Only the first 1,500 bytes are considered by queries.
        #[prost(bytes, tag = "18")]
        BytesValue(::prost::bytes::Bytes),
        /// A reference to a document. For example:
        /// `projects/{project_id}/databases/{database_id}/documents/{document_path}`.
        #[prost(string, tag = "5")]
        ReferenceValue(::prost::alloc::string::String),
        /// A geo point value representing a point on the surface of Earth.
        #[prost(message, tag = "8")]
        GeoPointValue(super::super::super::r#type::LatLng),
        /// An array value.
        ///
        /// Cannot directly contain another array value, though can contain an
        /// map which contains another array.
        #[prost(message, tag = "9")]
        ArrayValue(super::ArrayValue),
        /// A map value.
        #[prost(message, tag = "6")]
        MapValue(super::MapValue),
    }
}
/// An array value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArrayValue {
    /// Values in the array.
    #[prost(message, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<Value>,
}
/// A map value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MapValue {
    /// The map's fields.
    ///
    /// The map keys represent field names. Field names matching the regular
    /// expression `__.*__` are reserved. Reserved field names are forbidden except
    /// in certain documented contexts. The map keys, represented as UTF-8, must
    /// not exceed 1,500 bytes and cannot be empty.
    #[prost(btree_map = "string, message", tag = "1")]
    pub fields: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, Value>,
}
/// A Firestore query.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StructuredQuery {
    /// The projection to return.
    #[prost(message, optional, tag = "1")]
    pub select: ::core::option::Option<structured_query::Projection>,
    /// The collections to query.
    #[prost(message, repeated, tag = "2")]
    pub from: ::prost::alloc::vec::Vec<structured_query::CollectionSelector>,
    /// The filter to apply.
    #[prost(message, optional, tag = "3")]
    pub r#where: ::core::option::Option<structured_query::Filter>,
    /// The order to apply to the query results.
    ///
    /// Firestore guarantees a stable ordering through the following rules:
    ///
    ///  * Any field required to appear in `order_by`, that is not already
    ///    specified in `order_by`, is appended to the order in field name order
    ///    by default.
    ///  * If an order on `__name__` is not specified, it is appended by default.
    ///
    /// Fields are appended with the same sort direction as the last order
    /// specified, or 'ASCENDING' if no order was specified. For example:
    ///
    ///  * `SELECT * FROM Foo ORDER BY A` becomes
    ///    `SELECT * FROM Foo ORDER BY A, __name__`
    ///  * `SELECT * FROM Foo ORDER BY A DESC` becomes
    ///    `SELECT * FROM Foo ORDER BY A DESC, __name__ DESC`
    ///  * `SELECT * FROM Foo WHERE A > 1` becomes
    ///    `SELECT * FROM Foo WHERE A > 1 ORDER BY A, __name__`
    #[prost(message, repeated, tag = "4")]
    pub order_by: ::prost::alloc::vec::Vec<structured_query::Order>,
    /// A starting point for the query results.
    #[prost(message, optional, tag = "7")]
    pub start_at: ::core::option::Option<Cursor>,
    /// A end point for the query results.
    #[prost(message, optional, tag = "8")]
    pub end_at: ::core::option::Option<Cursor>,
    /// The number of results to skip.
    ///
    /// Applies before limit, but after all other constraints. Must be >= 0 if
    /// specified.
    #[prost(int32, tag = "6")]
    pub offset: i32,
    /// The maximum number of results to return.
    ///
    /// Applies after all other constraints.
    /// Must be >= 0 if specified.
    #[prost(message, optional, tag = "5")]
    pub limit: ::core::option::Option<i32>,
}
/// Nested message and enum types in `StructuredQuery`.
pub mod structured_query {
    /// A selection of a collection, such as `messages as m1`.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CollectionSelector {
        /// The collection ID.
        /// When set, selects only collections with this ID.
        #[prost(string, tag = "2")]
        pub collection_id: ::prost::alloc::string::String,
        /// When false, selects only collections that are immediate children of
        /// the `parent` specified in the containing `RunQueryRequest`.
        /// When true, selects all descendant collections.
        #[prost(bool, tag = "3")]
        pub all_descendants: bool,
    }
    /// A filter.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Filter {
        /// The type of filter.
        #[prost(oneof = "filter::FilterType", tags = "1, 2, 3")]
        pub filter_type: ::core::option::Option<filter::FilterType>,
    }
    /// Nested message and enum types in `Filter`.
    pub mod filter {
        /// The type of filter.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum FilterType {
            /// A composite filter.
            #[prost(message, tag = "1")]
            CompositeFilter(super::CompositeFilter),
            /// A filter on a document field.
            #[prost(message, tag = "2")]
            FieldFilter(super::FieldFilter),
            /// A filter that takes exactly one argument.
            #[prost(message, tag = "3")]
            UnaryFilter(super::UnaryFilter),
        }
    }
    /// A filter that merges multiple other filters using the given operator.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CompositeFilter {
        /// The operator for combining multiple filters.
        #[prost(enumeration = "composite_filter::Operator", tag = "1")]
        pub op: i32,
        /// The list of filters to combine.
        ///
        /// Requires:
        ///
        /// * At least one filter is present.
        #[prost(message, repeated, tag = "2")]
        pub filters: ::prost::alloc::vec::Vec<Filter>,
    }
    /// Nested message and enum types in `CompositeFilter`.
    pub mod composite_filter {
        /// A composite filter operator.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Operator {
            /// Unspecified. This value must not be used.
            Unspecified = 0,
            /// Documents are required to satisfy all of the combined filters.
            And = 1,
        }
    }
    /// A filter on a specific field.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FieldFilter {
        /// The field to filter by.
        #[prost(message, optional, tag = "1")]
        pub field: ::core::option::Option<FieldReference>,
        /// The operator to filter by.
        #[prost(enumeration = "field_filter::Operator", tag = "2")]
        pub op: i32,
        /// The value to compare to.
        #[prost(message, optional, tag = "3")]
        pub value: ::core::option::Option<super::Value>,
    }
    /// Nested message and enum types in `FieldFilter`.
    pub mod field_filter {
        /// A field filter operator.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Operator {
            /// Unspecified. This value must not be used.
            Unspecified = 0,
            /// The given `field` is less than the given `value`.
            ///
            /// Requires:
            ///
            /// * That `field` come first in `order_by`.
            LessThan = 1,
            /// The given `field` is less than or equal to the given `value`.
            ///
            /// Requires:
            ///
            /// * That `field` come first in `order_by`.
            LessThanOrEqual = 2,
            /// The given `field` is greater than the given `value`.
            ///
            /// Requires:
            ///
            /// * That `field` come first in `order_by`.
            GreaterThan = 3,
            /// The given `field` is greater than or equal to the given `value`.
            ///
            /// Requires:
            ///
            /// * That `field` come first in `order_by`.
            GreaterThanOrEqual = 4,
            /// The given `field` is equal to the given `value`.
            Equal = 5,
            /// The given `field` is not equal to the given `value`.
            ///
            /// Requires:
            ///
            /// * No other `NOT_EQUAL`, `NOT_IN`, `IS_NOT_NULL`, or `IS_NOT_NAN`.
            /// * That `field` comes first in the `order_by`.
            NotEqual = 6,
            /// The given `field` is an array that contains the given `value`.
            ArrayContains = 7,
            /// The given `field` is equal to at least one value in the given array.
            ///
            /// Requires:
            ///
            /// * That `value` is a non-empty `ArrayValue` with at most 10 values.
            /// * No other `IN` or `ARRAY_CONTAINS_ANY` or `NOT_IN`.
            In = 8,
            /// The given `field` is an array that contains any of the values in the
            /// given array.
            ///
            /// Requires:
            ///
            /// * That `value` is a non-empty `ArrayValue` with at most 10 values.
            /// * No other `IN` or `ARRAY_CONTAINS_ANY` or `NOT_IN`.
            ArrayContainsAny = 9,
            /// The value of the `field` is not in the given array.
            ///
            /// Requires:
            ///
            /// * That `value` is a non-empty `ArrayValue` with at most 10 values.
            /// * No other `IN`, `ARRAY_CONTAINS_ANY`, `NOT_IN`, `NOT_EQUAL`,
            ///   `IS_NOT_NULL`, or `IS_NOT_NAN`.
            /// * That `field` comes first in the `order_by`.
            NotIn = 10,
        }
    }
    /// A filter with a single operand.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UnaryFilter {
        /// The unary operator to apply.
        #[prost(enumeration = "unary_filter::Operator", tag = "1")]
        pub op: i32,
        /// The argument to the filter.
        #[prost(oneof = "unary_filter::OperandType", tags = "2")]
        pub operand_type: ::core::option::Option<unary_filter::OperandType>,
    }
    /// Nested message and enum types in `UnaryFilter`.
    pub mod unary_filter {
        /// A unary operator.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Operator {
            /// Unspecified. This value must not be used.
            Unspecified = 0,
            /// The given `field` is equal to `NaN`.
            IsNan = 2,
            /// The given `field` is equal to `NULL`.
            IsNull = 3,
            /// The given `field` is not equal to `NaN`.
            ///
            /// Requires:
            ///
            /// * No other `NOT_EQUAL`, `NOT_IN`, `IS_NOT_NULL`, or `IS_NOT_NAN`.
            /// * That `field` comes first in the `order_by`.
            IsNotNan = 4,
            /// The given `field` is not equal to `NULL`.
            ///
            /// Requires:
            ///
            /// * A single `NOT_EQUAL`, `NOT_IN`, `IS_NOT_NULL`, or `IS_NOT_NAN`.
            /// * That `field` comes first in the `order_by`.
            IsNotNull = 5,
        }
        /// The argument to the filter.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum OperandType {
            /// The field to which to apply the operator.
            #[prost(message, tag = "2")]
            Field(super::FieldReference),
        }
    }
    /// An order on a field.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Order {
        /// The field to order by.
        #[prost(message, optional, tag = "1")]
        pub field: ::core::option::Option<FieldReference>,
        /// The direction to order by. Defaults to `ASCENDING`.
        #[prost(enumeration = "Direction", tag = "2")]
        pub direction: i32,
    }
    /// A reference to a field, such as `max(messages.time) as max_time`.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FieldReference {
        #[prost(string, tag = "2")]
        pub field_path: ::prost::alloc::string::String,
    }
    /// The projection of document's fields to return.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Projection {
        /// The fields to return.
        ///
        /// If empty, all fields are returned. To only return the name
        /// of the document, use `\['__name__'\]`.
        #[prost(message, repeated, tag = "2")]
        pub fields: ::prost::alloc::vec::Vec<FieldReference>,
    }
    /// A sort direction.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Direction {
        /// Unspecified.
        Unspecified = 0,
        /// Ascending.
        Ascending = 1,
        /// Descending.
        Descending = 2,
    }
}
/// A position in a query result set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cursor {
    /// The values that represent a position, in the order they appear in
    /// the order by clause of a query.
    ///
    /// Can contain fewer values than specified in the order by clause.
    #[prost(message, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<Value>,
    /// If the position is just before or just after the given values, relative
    /// to the sort order defined by the query.
    #[prost(bool, tag = "2")]
    pub before: bool,
}
/// A set of field paths on a document.
/// Used to restrict a get or update operation on a document to a subset of its
/// fields.
/// This is different from standard field masks, as this is always scoped to a
/// \[Document][google.firestore.v1.Document\], and takes in account the dynamic nature of \[Value][google.firestore.v1.Value\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentMask {
    /// The list of field paths in the mask. See \[Document.fields][google.firestore.v1.Document.fields\] for a field
    /// path syntax reference.
    #[prost(string, repeated, tag = "1")]
    pub field_paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A precondition on a document, used for conditional operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Precondition {
    /// The type of precondition.
    #[prost(oneof = "precondition::ConditionType", tags = "1, 2")]
    pub condition_type: ::core::option::Option<precondition::ConditionType>,
}
/// Nested message and enum types in `Precondition`.
pub mod precondition {
    /// The type of precondition.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConditionType {
        /// When set to `true`, the target document must exist.
        /// When set to `false`, the target document must not exist.
        #[prost(bool, tag = "1")]
        Exists(bool),
        /// When set, the target document must exist and have been last updated at
        /// that time. Timestamp must be microsecond aligned.
        #[prost(message, tag = "2")]
        UpdateTime(::prost_types::Timestamp),
    }
}
/// Options for creating a new transaction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionOptions {
    /// The mode of the transaction.
    #[prost(oneof = "transaction_options::Mode", tags = "2, 3")]
    pub mode: ::core::option::Option<transaction_options::Mode>,
}
/// Nested message and enum types in `TransactionOptions`.
pub mod transaction_options {
    /// Options for a transaction that can be used to read and write documents.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ReadWrite {
        /// An optional transaction to retry.
        #[prost(bytes = "bytes", tag = "1")]
        pub retry_transaction: ::prost::bytes::Bytes,
    }
    /// Options for a transaction that can only be used to read documents.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ReadOnly {
        /// The consistency mode for this transaction. If not set, defaults to strong
        /// consistency.
        #[prost(oneof = "read_only::ConsistencySelector", tags = "2")]
        pub consistency_selector: ::core::option::Option<read_only::ConsistencySelector>,
    }
    /// Nested message and enum types in `ReadOnly`.
    pub mod read_only {
        /// The consistency mode for this transaction. If not set, defaults to strong
        /// consistency.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ConsistencySelector {
            /// Reads documents at the given time.
            /// This may not be older than 60 seconds.
            #[prost(message, tag = "2")]
            ReadTime(::prost_types::Timestamp),
        }
    }
    /// The mode of the transaction.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Mode {
        /// The transaction can only be used for read operations.
        #[prost(message, tag = "2")]
        ReadOnly(ReadOnly),
        /// The transaction can be used for both read and write operations.
        #[prost(message, tag = "3")]
        ReadWrite(ReadWrite),
    }
}
/// A write on a document.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Write {
    /// The fields to update in this write.
    ///
    /// This field can be set only when the operation is `update`.
    /// If the mask is not set for an `update` and the document exists, any
    /// existing data will be overwritten.
    /// If the mask is set and the document on the server has fields not covered by
    /// the mask, they are left unchanged.
    /// Fields referenced in the mask, but not present in the input document, are
    /// deleted from the document on the server.
    /// The field paths in this mask must not contain a reserved field name.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<DocumentMask>,
    /// The transforms to perform after update.
    ///
    /// This field can be set only when the operation is `update`. If present, this
    /// write is equivalent to performing `update` and `transform` to the same
    /// document atomically and in order.
    #[prost(message, repeated, tag = "7")]
    pub update_transforms: ::prost::alloc::vec::Vec<document_transform::FieldTransform>,
    /// An optional precondition on the document.
    ///
    /// The write will fail if this is set and not met by the target document.
    #[prost(message, optional, tag = "4")]
    pub current_document: ::core::option::Option<Precondition>,
    /// The operation to execute.
    #[prost(oneof = "write::Operation", tags = "1, 2, 6")]
    pub operation: ::core::option::Option<write::Operation>,
}
/// Nested message and enum types in `Write`.
pub mod write {
    /// The operation to execute.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// A document to write.
        #[prost(message, tag = "1")]
        Update(super::Document),
        /// A document name to delete. In the format:
        /// `projects/{project_id}/databases/{database_id}/documents/{document_path}`.
        #[prost(string, tag = "2")]
        Delete(::prost::alloc::string::String),
        /// Applies a transformation to a document.
        #[prost(message, tag = "6")]
        Transform(super::DocumentTransform),
    }
}
/// A transformation of a document.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentTransform {
    /// The name of the document to transform.
    #[prost(string, tag = "1")]
    pub document: ::prost::alloc::string::String,
    /// The list of transformations to apply to the fields of the document, in
    /// order.
    /// This must not be empty.
    #[prost(message, repeated, tag = "2")]
    pub field_transforms: ::prost::alloc::vec::Vec<document_transform::FieldTransform>,
}
/// Nested message and enum types in `DocumentTransform`.
pub mod document_transform {
    /// A transformation of a field of the document.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FieldTransform {
        /// The path of the field. See \[Document.fields][google.firestore.v1.Document.fields\] for the field path syntax
        /// reference.
        #[prost(string, tag = "1")]
        pub field_path: ::prost::alloc::string::String,
        /// The transformation to apply on the field.
        #[prost(oneof = "field_transform::TransformType", tags = "2, 3, 4, 5, 6, 7")]
        pub transform_type: ::core::option::Option<field_transform::TransformType>,
    }
    /// Nested message and enum types in `FieldTransform`.
    pub mod field_transform {
        /// A value that is calculated by the server.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum ServerValue {
            /// Unspecified. This value must not be used.
            Unspecified = 0,
            /// The time at which the server processed the request, with millisecond
            /// precision. If used on multiple fields (same or different documents) in
            /// a transaction, all the fields will get the same server timestamp.
            RequestTime = 1,
        }
        /// The transformation to apply on the field.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum TransformType {
            /// Sets the field to the given server value.
            #[prost(enumeration = "ServerValue", tag = "2")]
            SetToServerValue(i32),
            /// Adds the given value to the field's current value.
            ///
            /// This must be an integer or a double value.
            /// If the field is not an integer or double, or if the field does not yet
            /// exist, the transformation will set the field to the given value.
            /// If either of the given value or the current field value are doubles,
            /// both values will be interpreted as doubles. Double arithmetic and
            /// representation of double values follow IEEE 754 semantics.
            /// If there is positive/negative integer overflow, the field is resolved
            /// to the largest magnitude positive/negative integer.
            #[prost(message, tag = "3")]
            Increment(super::super::Value),
            /// Sets the field to the maximum of its current value and the given value.
            ///
            /// This must be an integer or a double value.
            /// If the field is not an integer or double, or if the field does not yet
            /// exist, the transformation will set the field to the given value.
            /// If a maximum operation is applied where the field and the input value
            /// are of mixed types (that is - one is an integer and one is a double)
            /// the field takes on the type of the larger operand. If the operands are
            /// equivalent (e.g. 3 and 3.0), the field does not change.
            /// 0, 0.0, and -0.0 are all zero. The maximum of a zero stored value and
            /// zero input value is always the stored value.
            /// The maximum of any numeric value x and NaN is NaN.
            #[prost(message, tag = "4")]
            Maximum(super::super::Value),
            /// Sets the field to the minimum of its current value and the given value.
            ///
            /// This must be an integer or a double value.
            /// If the field is not an integer or double, or if the field does not yet
            /// exist, the transformation will set the field to the input value.
            /// If a minimum operation is applied where the field and the input value
            /// are of mixed types (that is - one is an integer and one is a double)
            /// the field takes on the type of the smaller operand. If the operands are
            /// equivalent (e.g. 3 and 3.0), the field does not change.
            /// 0, 0.0, and -0.0 are all zero. The minimum of a zero stored value and
            /// zero input value is always the stored value.
            /// The minimum of any numeric value x and NaN is NaN.
            #[prost(message, tag = "5")]
            Minimum(super::super::Value),
            /// Append the given elements in order if they are not already present in
            /// the current field value.
            /// If the field is not an array, or if the field does not yet exist, it is
            /// first set to the empty array.
            ///
            /// Equivalent numbers of different types (e.g. 3L and 3.0) are
            /// considered equal when checking if a value is missing.
            /// NaN is equal to NaN, and Null is equal to Null.
            /// If the input contains multiple equivalent values, only the first will
            /// be considered.
            ///
            /// The corresponding transform_result will be the null value.
            #[prost(message, tag = "6")]
            AppendMissingElements(super::super::ArrayValue),
            /// Remove all of the given elements from the array in the field.
            /// If the field is not an array, or if the field does not yet exist, it is
            /// set to the empty array.
            ///
            /// Equivalent numbers of the different types (e.g. 3L and 3.0) are
            /// considered equal when deciding whether an element should be removed.
            /// NaN is equal to NaN, and Null is equal to Null.
            /// This will remove all equivalent values if there are duplicates.
            ///
            /// The corresponding transform_result will be the null value.
            #[prost(message, tag = "7")]
            RemoveAllFromArray(super::super::ArrayValue),
        }
    }
}
/// The result of applying a write.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteResult {
    /// The last update time of the document after applying the write. Not set
    /// after a `delete`.
    ///
    /// If the write did not actually change the document, this will be the
    /// previous update_time.
    #[prost(message, optional, tag = "1")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The results of applying each \[DocumentTransform.FieldTransform][google.firestore.v1.DocumentTransform.FieldTransform\], in the
    /// same order.
    #[prost(message, repeated, tag = "2")]
    pub transform_results: ::prost::alloc::vec::Vec<Value>,
}
/// A \[Document][google.firestore.v1.Document\] has changed.
///
/// May be the result of multiple \[writes][google.firestore.v1.Write\], including deletes, that
/// ultimately resulted in a new value for the \[Document][google.firestore.v1.Document\].
///
/// Multiple \[DocumentChange][google.firestore.v1.DocumentChange\] messages may be returned for the same logical
/// change, if multiple targets are affected.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentChange {
    /// The new state of the \[Document][google.firestore.v1.Document\].
    ///
    /// If `mask` is set, contains only fields that were updated or added.
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<Document>,
    /// A set of target IDs of targets that match this document.
    #[prost(int32, repeated, tag = "5")]
    pub target_ids: ::prost::alloc::vec::Vec<i32>,
    /// A set of target IDs for targets that no longer match this document.
    #[prost(int32, repeated, tag = "6")]
    pub removed_target_ids: ::prost::alloc::vec::Vec<i32>,
}
/// A \[Document][google.firestore.v1.Document\] has been deleted.
///
/// May be the result of multiple \[writes][google.firestore.v1.Write\], including updates, the
/// last of which deleted the \[Document][google.firestore.v1.Document\].
///
/// Multiple \[DocumentDelete][google.firestore.v1.DocumentDelete\] messages may be returned for the same logical
/// delete, if multiple targets are affected.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentDelete {
    /// The resource name of the \[Document][google.firestore.v1.Document\] that was deleted.
    #[prost(string, tag = "1")]
    pub document: ::prost::alloc::string::String,
    /// A set of target IDs for targets that previously matched this entity.
    #[prost(int32, repeated, tag = "6")]
    pub removed_target_ids: ::prost::alloc::vec::Vec<i32>,
    /// The read timestamp at which the delete was observed.
    ///
    /// Greater or equal to the `commit_time` of the delete.
    #[prost(message, optional, tag = "4")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// A \[Document][google.firestore.v1.Document\] has been removed from the view of the targets.
///
/// Sent if the document is no longer relevant to a target and is out of view.
/// Can be sent instead of a DocumentDelete or a DocumentChange if the server
/// can not send the new value of the document.
///
/// Multiple \[DocumentRemove][google.firestore.v1.DocumentRemove\] messages may be returned for the same logical
/// write or delete, if multiple targets are affected.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentRemove {
    /// The resource name of the \[Document][google.firestore.v1.Document\] that has gone out of view.
    #[prost(string, tag = "1")]
    pub document: ::prost::alloc::string::String,
    /// A set of target IDs for targets that previously matched this document.
    #[prost(int32, repeated, tag = "2")]
    pub removed_target_ids: ::prost::alloc::vec::Vec<i32>,
    /// The read timestamp at which the remove was observed.
    ///
    /// Greater or equal to the `commit_time` of the change/delete/remove.
    #[prost(message, optional, tag = "4")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// A digest of all the documents that match a given target.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExistenceFilter {
    /// The target ID to which this filter applies.
    #[prost(int32, tag = "1")]
    pub target_id: i32,
    /// The total count of documents that match \[target_id][google.firestore.v1.ExistenceFilter.target_id\].
    ///
    /// If different from the count of documents in the client that match, the
    /// client must manually determine which documents no longer match the target.
    #[prost(int32, tag = "2")]
    pub count: i32,
}
/// The request for \[Firestore.GetDocument][google.firestore.v1.Firestore.GetDocument\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDocumentRequest {
    /// Required. The resource name of the Document to get. In the format:
    /// `projects/{project_id}/databases/{database_id}/documents/{document_path}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The fields to return. If not set, returns all fields.
    ///
    /// If the document has a field that is not present in this mask, that field
    /// will not be returned in the response.
    #[prost(message, optional, tag = "2")]
    pub mask: ::core::option::Option<DocumentMask>,
    /// The consistency mode for this transaction.
    /// If not set, defaults to strong consistency.
    #[prost(oneof = "get_document_request::ConsistencySelector", tags = "3, 5")]
    pub consistency_selector: ::core::option::Option<get_document_request::ConsistencySelector>,
}
/// Nested message and enum types in `GetDocumentRequest`.
pub mod get_document_request {
    /// The consistency mode for this transaction.
    /// If not set, defaults to strong consistency.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConsistencySelector {
        /// Reads the document in a transaction.
        #[prost(bytes, tag = "3")]
        Transaction(::prost::bytes::Bytes),
        /// Reads the version of the document at the given time.
        /// This may not be older than 270 seconds.
        #[prost(message, tag = "5")]
        ReadTime(::prost_types::Timestamp),
    }
}
/// The request for \[Firestore.ListDocuments][google.firestore.v1.Firestore.ListDocuments\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDocumentsRequest {
    /// Required. The parent resource name. In the format:
    /// `projects/{project_id}/databases/{database_id}/documents` or
    /// `projects/{project_id}/databases/{database_id}/documents/{document_path}`.
    /// For example:
    /// `projects/my-project/databases/my-database/documents` or
    /// `projects/my-project/databases/my-database/documents/chatrooms/my-chatroom`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The collection ID, relative to `parent`, to list. For example: `chatrooms`
    /// or `messages`.
    #[prost(string, tag = "2")]
    pub collection_id: ::prost::alloc::string::String,
    /// The maximum number of documents to return.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The `next_page_token` value returned from a previous List request, if any.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
    /// The order to sort results by. For example: `priority desc, name`.
    #[prost(string, tag = "6")]
    pub order_by: ::prost::alloc::string::String,
    /// The fields to return. If not set, returns all fields.
    ///
    /// If a document has a field that is not present in this mask, that field
    /// will not be returned in the response.
    #[prost(message, optional, tag = "7")]
    pub mask: ::core::option::Option<DocumentMask>,
    /// If the list should show missing documents. A missing document is a
    /// document that does not exist but has sub-documents. These documents will
    /// be returned with a key but will not have fields, \[Document.create_time][google.firestore.v1.Document.create_time\],
    /// or \[Document.update_time][google.firestore.v1.Document.update_time\] set.
    ///
    /// Requests with `show_missing` may not specify `where` or
    /// `order_by`.
    #[prost(bool, tag = "12")]
    pub show_missing: bool,
    /// The consistency mode for this transaction.
    /// If not set, defaults to strong consistency.
    #[prost(oneof = "list_documents_request::ConsistencySelector", tags = "8, 10")]
    pub consistency_selector: ::core::option::Option<list_documents_request::ConsistencySelector>,
}
/// Nested message and enum types in `ListDocumentsRequest`.
pub mod list_documents_request {
    /// The consistency mode for this transaction.
    /// If not set, defaults to strong consistency.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConsistencySelector {
        /// Reads documents in a transaction.
        #[prost(bytes, tag = "8")]
        Transaction(::prost::bytes::Bytes),
        /// Reads documents as they were at the given time.
        /// This may not be older than 270 seconds.
        #[prost(message, tag = "10")]
        ReadTime(::prost_types::Timestamp),
    }
}
/// The response for \[Firestore.ListDocuments][google.firestore.v1.Firestore.ListDocuments\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDocumentsResponse {
    /// The Documents found.
    #[prost(message, repeated, tag = "1")]
    pub documents: ::prost::alloc::vec::Vec<Document>,
    /// The next page token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request for \[Firestore.CreateDocument][google.firestore.v1.Firestore.CreateDocument\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDocumentRequest {
    /// Required. The parent resource. For example:
    /// `projects/{project_id}/databases/{database_id}/documents` or
    /// `projects/{project_id}/databases/{database_id}/documents/chatrooms/{chatroom_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The collection ID, relative to `parent`, to list. For example: `chatrooms`.
    #[prost(string, tag = "2")]
    pub collection_id: ::prost::alloc::string::String,
    /// The client-assigned document ID to use for this document.
    ///
    /// Optional. If not specified, an ID will be assigned by the service.
    #[prost(string, tag = "3")]
    pub document_id: ::prost::alloc::string::String,
    /// Required. The document to create. `name` must not be set.
    #[prost(message, optional, tag = "4")]
    pub document: ::core::option::Option<Document>,
    /// The fields to return. If not set, returns all fields.
    ///
    /// If the document has a field that is not present in this mask, that field
    /// will not be returned in the response.
    #[prost(message, optional, tag = "5")]
    pub mask: ::core::option::Option<DocumentMask>,
}
/// The request for \[Firestore.UpdateDocument][google.firestore.v1.Firestore.UpdateDocument\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDocumentRequest {
    /// Required. The updated document.
    /// Creates the document if it does not already exist.
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<Document>,
    /// The fields to update.
    /// None of the field paths in the mask may contain a reserved name.
    ///
    /// If the document exists on the server and has fields not referenced in the
    /// mask, they are left unchanged.
    /// Fields referenced in the mask, but not present in the input document, are
    /// deleted from the document on the server.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<DocumentMask>,
    /// The fields to return. If not set, returns all fields.
    ///
    /// If the document has a field that is not present in this mask, that field
    /// will not be returned in the response.
    #[prost(message, optional, tag = "3")]
    pub mask: ::core::option::Option<DocumentMask>,
    /// An optional precondition on the document.
    /// The request will fail if this is set and not met by the target document.
    #[prost(message, optional, tag = "4")]
    pub current_document: ::core::option::Option<Precondition>,
}
/// The request for \[Firestore.DeleteDocument][google.firestore.v1.Firestore.DeleteDocument\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDocumentRequest {
    /// Required. The resource name of the Document to delete. In the format:
    /// `projects/{project_id}/databases/{database_id}/documents/{document_path}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// An optional precondition on the document.
    /// The request will fail if this is set and not met by the target document.
    #[prost(message, optional, tag = "2")]
    pub current_document: ::core::option::Option<Precondition>,
}
/// The request for \[Firestore.BatchGetDocuments][google.firestore.v1.Firestore.BatchGetDocuments\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetDocumentsRequest {
    /// Required. The database name. In the format:
    /// `projects/{project_id}/databases/{database_id}`.
    #[prost(string, tag = "1")]
    pub database: ::prost::alloc::string::String,
    /// The names of the documents to retrieve. In the format:
    /// `projects/{project_id}/databases/{database_id}/documents/{document_path}`.
    /// The request will fail if any of the document is not a child resource of the
    /// given `database`. Duplicate names will be elided.
    #[prost(string, repeated, tag = "2")]
    pub documents: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The fields to return. If not set, returns all fields.
    ///
    /// If a document has a field that is not present in this mask, that field will
    /// not be returned in the response.
    #[prost(message, optional, tag = "3")]
    pub mask: ::core::option::Option<DocumentMask>,
    /// The consistency mode for this transaction.
    /// If not set, defaults to strong consistency.
    #[prost(
        oneof = "batch_get_documents_request::ConsistencySelector",
        tags = "4, 5, 7"
    )]
    pub consistency_selector:
        ::core::option::Option<batch_get_documents_request::ConsistencySelector>,
}
/// Nested message and enum types in `BatchGetDocumentsRequest`.
pub mod batch_get_documents_request {
    /// The consistency mode for this transaction.
    /// If not set, defaults to strong consistency.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConsistencySelector {
        /// Reads documents in a transaction.
        #[prost(bytes, tag = "4")]
        Transaction(::prost::bytes::Bytes),
        /// Starts a new transaction and reads the documents.
        /// Defaults to a read-only transaction.
        /// The new transaction ID will be returned as the first response in the
        /// stream.
        #[prost(message, tag = "5")]
        NewTransaction(super::TransactionOptions),
        /// Reads documents as they were at the given time.
        /// This may not be older than 270 seconds.
        #[prost(message, tag = "7")]
        ReadTime(::prost_types::Timestamp),
    }
}
/// The streamed response for \[Firestore.BatchGetDocuments][google.firestore.v1.Firestore.BatchGetDocuments\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetDocumentsResponse {
    /// The transaction that was started as part of this request.
    /// Will only be set in the first response, and only if
    /// \[BatchGetDocumentsRequest.new_transaction][google.firestore.v1.BatchGetDocumentsRequest.new_transaction\] was set in the request.
    #[prost(bytes = "bytes", tag = "3")]
    pub transaction: ::prost::bytes::Bytes,
    /// The time at which the document was read.
    /// This may be monotically increasing, in this case the previous documents in
    /// the result stream are guaranteed not to have changed between their
    /// read_time and this one.
    #[prost(message, optional, tag = "4")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
    /// A single result.
    /// This can be empty if the server is just returning a transaction.
    #[prost(oneof = "batch_get_documents_response::Result", tags = "1, 2")]
    pub result: ::core::option::Option<batch_get_documents_response::Result>,
}
/// Nested message and enum types in `BatchGetDocumentsResponse`.
pub mod batch_get_documents_response {
    /// A single result.
    /// This can be empty if the server is just returning a transaction.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        /// A document that was requested.
        #[prost(message, tag = "1")]
        Found(super::Document),
        /// A document name that was requested but does not exist. In the format:
        /// `projects/{project_id}/databases/{database_id}/documents/{document_path}`.
        #[prost(string, tag = "2")]
        Missing(::prost::alloc::string::String),
    }
}
/// The request for \[Firestore.BeginTransaction][google.firestore.v1.Firestore.BeginTransaction\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BeginTransactionRequest {
    /// Required. The database name. In the format:
    /// `projects/{project_id}/databases/{database_id}`.
    #[prost(string, tag = "1")]
    pub database: ::prost::alloc::string::String,
    /// The options for the transaction.
    /// Defaults to a read-write transaction.
    #[prost(message, optional, tag = "2")]
    pub options: ::core::option::Option<TransactionOptions>,
}
/// The response for \[Firestore.BeginTransaction][google.firestore.v1.Firestore.BeginTransaction\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BeginTransactionResponse {
    /// The transaction that was started.
    #[prost(bytes = "bytes", tag = "1")]
    pub transaction: ::prost::bytes::Bytes,
}
/// The request for \[Firestore.Commit][google.firestore.v1.Firestore.Commit\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitRequest {
    /// Required. The database name. In the format:
    /// `projects/{project_id}/databases/{database_id}`.
    #[prost(string, tag = "1")]
    pub database: ::prost::alloc::string::String,
    /// The writes to apply.
    ///
    /// Always executed atomically and in order.
    #[prost(message, repeated, tag = "2")]
    pub writes: ::prost::alloc::vec::Vec<Write>,
    /// If set, applies all writes in this transaction, and commits it.
    #[prost(bytes = "bytes", tag = "3")]
    pub transaction: ::prost::bytes::Bytes,
}
/// The response for \[Firestore.Commit][google.firestore.v1.Firestore.Commit\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitResponse {
    /// The result of applying the writes.
    ///
    /// This i-th write result corresponds to the i-th write in the
    /// request.
    #[prost(message, repeated, tag = "1")]
    pub write_results: ::prost::alloc::vec::Vec<WriteResult>,
    /// The time at which the commit occurred. Any read with an equal or greater
    /// `read_time` is guaranteed to see the effects of the commit.
    #[prost(message, optional, tag = "2")]
    pub commit_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// The request for \[Firestore.Rollback][google.firestore.v1.Firestore.Rollback\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollbackRequest {
    /// Required. The database name. In the format:
    /// `projects/{project_id}/databases/{database_id}`.
    #[prost(string, tag = "1")]
    pub database: ::prost::alloc::string::String,
    /// Required. The transaction to roll back.
    #[prost(bytes = "bytes", tag = "2")]
    pub transaction: ::prost::bytes::Bytes,
}
/// The request for \[Firestore.RunQuery][google.firestore.v1.Firestore.RunQuery\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunQueryRequest {
    /// Required. The parent resource name. In the format:
    /// `projects/{project_id}/databases/{database_id}/documents` or
    /// `projects/{project_id}/databases/{database_id}/documents/{document_path}`.
    /// For example:
    /// `projects/my-project/databases/my-database/documents` or
    /// `projects/my-project/databases/my-database/documents/chatrooms/my-chatroom`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The query to run.
    #[prost(oneof = "run_query_request::QueryType", tags = "2")]
    pub query_type: ::core::option::Option<run_query_request::QueryType>,
    /// The consistency mode for this transaction.
    /// If not set, defaults to strong consistency.
    #[prost(oneof = "run_query_request::ConsistencySelector", tags = "5, 6, 7")]
    pub consistency_selector: ::core::option::Option<run_query_request::ConsistencySelector>,
}
/// Nested message and enum types in `RunQueryRequest`.
pub mod run_query_request {
    /// The query to run.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum QueryType {
        /// A structured query.
        #[prost(message, tag = "2")]
        StructuredQuery(super::StructuredQuery),
    }
    /// The consistency mode for this transaction.
    /// If not set, defaults to strong consistency.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConsistencySelector {
        /// Run the query within an already active transaction.
        ///
        /// The value here is the opaque transaction ID to execute the query in.
        #[prost(bytes, tag = "5")]
        Transaction(::prost::bytes::Bytes),
        /// Starts a new transaction and reads the documents.
        /// Defaults to a read-only transaction.
        /// The new transaction ID will be returned as the first response in the
        /// stream.
        #[prost(message, tag = "6")]
        NewTransaction(super::TransactionOptions),
        /// Reads documents as they were at the given time.
        /// This may not be older than 270 seconds.
        #[prost(message, tag = "7")]
        ReadTime(::prost_types::Timestamp),
    }
}
/// The response for \[Firestore.RunQuery][google.firestore.v1.Firestore.RunQuery\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunQueryResponse {
    /// The transaction that was started as part of this request.
    /// Can only be set in the first response, and only if
    /// \[RunQueryRequest.new_transaction][google.firestore.v1.RunQueryRequest.new_transaction\] was set in the request.
    /// If set, no other fields will be set in this response.
    #[prost(bytes = "bytes", tag = "2")]
    pub transaction: ::prost::bytes::Bytes,
    /// A query result, not set when reporting partial progress.
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<Document>,
    /// The time at which the document was read. This may be monotonically
    /// increasing; in this case, the previous documents in the result stream are
    /// guaranteed not to have changed between their `read_time` and this one.
    ///
    /// If the query returns no results, a response with `read_time` and no
    /// `document` will be sent, and this represents the time at which the query
    /// was run.
    #[prost(message, optional, tag = "3")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The number of results that have been skipped due to an offset between
    /// the last response and the current response.
    #[prost(int32, tag = "4")]
    pub skipped_results: i32,
}
/// The request for \[Firestore.PartitionQuery][google.firestore.v1.Firestore.PartitionQuery\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartitionQueryRequest {
    /// Required. The parent resource name. In the format:
    /// `projects/{project_id}/databases/{database_id}/documents`.
    /// Document resource names are not supported; only database resource names
    /// can be specified.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The desired maximum number of partition points.
    /// The partitions may be returned across multiple pages of results.
    /// The number must be positive. The actual number of partitions
    /// returned may be fewer.
    ///
    /// For example, this may be set to one fewer than the number of parallel
    /// queries to be run, or in running a data pipeline job, one fewer than the
    /// number of workers or compute instances available.
    #[prost(int64, tag = "3")]
    pub partition_count: i64,
    /// The `next_page_token` value returned from a previous call to
    /// PartitionQuery that may be used to get an additional set of results.
    /// There are no ordering guarantees between sets of results. Thus, using
    /// multiple sets of results will require merging the different result sets.
    ///
    /// For example, two subsequent calls using a page_token may return:
    ///
    ///  * cursor B, cursor M, cursor Q
    ///  * cursor A, cursor U, cursor W
    ///
    /// To obtain a complete result set ordered with respect to the results of the
    /// query supplied to PartitionQuery, the results sets should be merged:
    /// cursor A, cursor B, cursor M, cursor Q, cursor U, cursor W
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of partitions to return in this call, subject to
    /// `partition_count`.
    ///
    /// For example, if `partition_count` = 10 and `page_size` = 8, the first call
    /// to PartitionQuery will return up to 8 partitions and a `next_page_token`
    /// if more results exist. A second call to PartitionQuery will return up to
    /// 2 partitions, to complete the total of 10 specified in `partition_count`.
    #[prost(int32, tag = "5")]
    pub page_size: i32,
    /// The query to partition.
    #[prost(oneof = "partition_query_request::QueryType", tags = "2")]
    pub query_type: ::core::option::Option<partition_query_request::QueryType>,
}
/// Nested message and enum types in `PartitionQueryRequest`.
pub mod partition_query_request {
    /// The query to partition.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum QueryType {
        /// A structured query.
        /// Query must specify collection with all descendants and be ordered by name
        /// ascending. Other filters, order bys, limits, offsets, and start/end
        /// cursors are not supported.
        #[prost(message, tag = "2")]
        StructuredQuery(super::StructuredQuery),
    }
}
/// The response for \[Firestore.PartitionQuery][google.firestore.v1.Firestore.PartitionQuery\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartitionQueryResponse {
    /// Partition results.
    /// Each partition is a split point that can be used by RunQuery as a starting
    /// or end point for the query results. The RunQuery requests must be made with
    /// the same query supplied to this PartitionQuery request. The partition
    /// cursors will be ordered according to same ordering as the results of the
    /// query supplied to PartitionQuery.
    ///
    /// For example, if a PartitionQuery request returns partition cursors A and B,
    /// running the following three queries will return the entire result set of
    /// the original query:
    ///
    ///  * query, end_at A
    ///  * query, start_at A, end_at B
    ///  * query, start_at B
    ///
    /// An empty result may indicate that the query has too few results to be
    /// partitioned.
    #[prost(message, repeated, tag = "1")]
    pub partitions: ::prost::alloc::vec::Vec<Cursor>,
    /// A page token that may be used to request an additional set of results, up
    /// to the number specified by `partition_count` in the PartitionQuery request.
    /// If blank, there are no more results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request for \[Firestore.Write][google.firestore.v1.Firestore.Write\].
///
/// The first request creates a stream, or resumes an existing one from a token.
///
/// When creating a new stream, the server replies with a response containing
/// only an ID and a token, to use in the next request.
///
/// When resuming a stream, the server first streams any responses later than the
/// given token, then a response containing only an up-to-date token, to use in
/// the next request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteRequest {
    /// Required. The database name. In the format:
    /// `projects/{project_id}/databases/{database_id}`.
    /// This is only required in the first message.
    #[prost(string, tag = "1")]
    pub database: ::prost::alloc::string::String,
    /// The ID of the write stream to resume.
    /// This may only be set in the first message. When left empty, a new write
    /// stream will be created.
    #[prost(string, tag = "2")]
    pub stream_id: ::prost::alloc::string::String,
    /// The writes to apply.
    ///
    /// Always executed atomically and in order.
    /// This must be empty on the first request.
    /// This may be empty on the last request.
    /// This must not be empty on all other requests.
    #[prost(message, repeated, tag = "3")]
    pub writes: ::prost::alloc::vec::Vec<Write>,
    /// A stream token that was previously sent by the server.
    ///
    /// The client should set this field to the token from the most recent
    /// \[WriteResponse][google.firestore.v1.WriteResponse\] it has received. This acknowledges that the client has
    /// received responses up to this token. After sending this token, earlier
    /// tokens may not be used anymore.
    ///
    /// The server may close the stream if there are too many unacknowledged
    /// responses.
    ///
    /// Leave this field unset when creating a new stream. To resume a stream at
    /// a specific point, set this field and the `stream_id` field.
    ///
    /// Leave this field unset when creating a new stream.
    #[prost(bytes = "bytes", tag = "4")]
    pub stream_token: ::prost::bytes::Bytes,
    /// Labels associated with this write request.
    #[prost(btree_map = "string, string", tag = "5")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// The response for \[Firestore.Write][google.firestore.v1.Firestore.Write\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteResponse {
    /// The ID of the stream.
    /// Only set on the first message, when a new stream was created.
    #[prost(string, tag = "1")]
    pub stream_id: ::prost::alloc::string::String,
    /// A token that represents the position of this response in the stream.
    /// This can be used by a client to resume the stream at this point.
    ///
    /// This field is always set.
    #[prost(bytes = "bytes", tag = "2")]
    pub stream_token: ::prost::bytes::Bytes,
    /// The result of applying the writes.
    ///
    /// This i-th write result corresponds to the i-th write in the
    /// request.
    #[prost(message, repeated, tag = "3")]
    pub write_results: ::prost::alloc::vec::Vec<WriteResult>,
    /// The time at which the commit occurred. Any read with an equal or greater
    /// `read_time` is guaranteed to see the effects of the write.
    #[prost(message, optional, tag = "4")]
    pub commit_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// A request for \[Firestore.Listen][google.firestore.v1.Firestore.Listen\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListenRequest {
    /// Required. The database name. In the format:
    /// `projects/{project_id}/databases/{database_id}`.
    #[prost(string, tag = "1")]
    pub database: ::prost::alloc::string::String,
    /// Labels associated with this target change.
    #[prost(btree_map = "string, string", tag = "4")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The supported target changes.
    #[prost(oneof = "listen_request::TargetChange", tags = "2, 3")]
    pub target_change: ::core::option::Option<listen_request::TargetChange>,
}
/// Nested message and enum types in `ListenRequest`.
pub mod listen_request {
    /// The supported target changes.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TargetChange {
        /// A target to add to this stream.
        #[prost(message, tag = "2")]
        AddTarget(super::Target),
        /// The ID of a target to remove from this stream.
        #[prost(int32, tag = "3")]
        RemoveTarget(i32),
    }
}
/// The response for \[Firestore.Listen][google.firestore.v1.Firestore.Listen\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListenResponse {
    /// The supported responses.
    #[prost(oneof = "listen_response::ResponseType", tags = "2, 3, 4, 6, 5")]
    pub response_type: ::core::option::Option<listen_response::ResponseType>,
}
/// Nested message and enum types in `ListenResponse`.
pub mod listen_response {
    /// The supported responses.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ResponseType {
        /// Targets have changed.
        #[prost(message, tag = "2")]
        TargetChange(super::TargetChange),
        /// A \[Document][google.firestore.v1.Document\] has changed.
        #[prost(message, tag = "3")]
        DocumentChange(super::DocumentChange),
        /// A \[Document][google.firestore.v1.Document\] has been deleted.
        #[prost(message, tag = "4")]
        DocumentDelete(super::DocumentDelete),
        /// A \[Document][google.firestore.v1.Document\] has been removed from a target (because it is no longer
        /// relevant to that target).
        #[prost(message, tag = "6")]
        DocumentRemove(super::DocumentRemove),
        /// A filter to apply to the set of documents previously returned for the
        /// given target.
        ///
        /// Returned when documents may have been removed from the given target, but
        /// the exact documents are unknown.
        #[prost(message, tag = "5")]
        Filter(super::ExistenceFilter),
    }
}
/// A specification of a set of documents to listen to.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Target {
    /// The target ID that identifies the target on the stream. Must be a positive
    /// number and non-zero.
    #[prost(int32, tag = "5")]
    pub target_id: i32,
    /// If the target should be removed once it is current and consistent.
    #[prost(bool, tag = "6")]
    pub once: bool,
    /// The type of target to listen to.
    #[prost(oneof = "target::TargetType", tags = "2, 3")]
    pub target_type: ::core::option::Option<target::TargetType>,
    /// When to start listening.
    ///
    /// If not specified, all matching Documents are returned before any
    /// subsequent changes.
    #[prost(oneof = "target::ResumeType", tags = "4, 11")]
    pub resume_type: ::core::option::Option<target::ResumeType>,
}
/// Nested message and enum types in `Target`.
pub mod target {
    /// A target specified by a set of documents names.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DocumentsTarget {
        /// The names of the documents to retrieve. In the format:
        /// `projects/{project_id}/databases/{database_id}/documents/{document_path}`.
        /// The request will fail if any of the document is not a child resource of
        /// the given `database`. Duplicate names will be elided.
        #[prost(string, repeated, tag = "2")]
        pub documents: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// A target specified by a query.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QueryTarget {
        /// The parent resource name. In the format:
        /// `projects/{project_id}/databases/{database_id}/documents` or
        /// `projects/{project_id}/databases/{database_id}/documents/{document_path}`.
        /// For example:
        /// `projects/my-project/databases/my-database/documents` or
        /// `projects/my-project/databases/my-database/documents/chatrooms/my-chatroom`
        #[prost(string, tag = "1")]
        pub parent: ::prost::alloc::string::String,
        /// The query to run.
        #[prost(oneof = "query_target::QueryType", tags = "2")]
        pub query_type: ::core::option::Option<query_target::QueryType>,
    }
    /// Nested message and enum types in `QueryTarget`.
    pub mod query_target {
        /// The query to run.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum QueryType {
            /// A structured query.
            #[prost(message, tag = "2")]
            StructuredQuery(super::super::StructuredQuery),
        }
    }
    /// The type of target to listen to.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TargetType {
        /// A target specified by a query.
        #[prost(message, tag = "2")]
        Query(QueryTarget),
        /// A target specified by a set of document names.
        #[prost(message, tag = "3")]
        Documents(DocumentsTarget),
    }
    /// When to start listening.
    ///
    /// If not specified, all matching Documents are returned before any
    /// subsequent changes.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ResumeType {
        /// A resume token from a prior \[TargetChange][google.firestore.v1.TargetChange\] for an identical target.
        ///
        /// Using a resume token with a different target is unsupported and may fail.
        #[prost(bytes, tag = "4")]
        ResumeToken(::prost::bytes::Bytes),
        /// Start listening after a specific `read_time`.
        ///
        /// The client must know the state of matching documents at this time.
        #[prost(message, tag = "11")]
        ReadTime(::prost_types::Timestamp),
    }
}
/// Targets being watched have changed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetChange {
    /// The type of change that occurred.
    #[prost(enumeration = "target_change::TargetChangeType", tag = "1")]
    pub target_change_type: i32,
    /// The target IDs of targets that have changed.
    ///
    /// If empty, the change applies to all targets.
    ///
    /// The order of the target IDs is not defined.
    #[prost(int32, repeated, tag = "2")]
    pub target_ids: ::prost::alloc::vec::Vec<i32>,
    /// The error that resulted in this change, if applicable.
    #[prost(message, optional, tag = "3")]
    pub cause: ::core::option::Option<super::super::rpc::Status>,
    /// A token that can be used to resume the stream for the given `target_ids`,
    /// or all targets if `target_ids` is empty.
    ///
    /// Not set on every target change.
    #[prost(bytes = "bytes", tag = "4")]
    pub resume_token: ::prost::bytes::Bytes,
    /// The consistent `read_time` for the given `target_ids` (omitted when the
    /// target_ids are not at a consistent snapshot).
    ///
    /// The stream is guaranteed to send a `read_time` with `target_ids` empty
    /// whenever the entire stream reaches a new consistent snapshot. ADD,
    /// CURRENT, and RESET messages are guaranteed to (eventually) result in a
    /// new consistent snapshot (while NO_CHANGE and REMOVE messages are not).
    ///
    /// For a given stream, `read_time` is guaranteed to be monotonically
    /// increasing.
    #[prost(message, optional, tag = "6")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `TargetChange`.
pub mod target_change {
    /// The type of change.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TargetChangeType {
        /// No change has occurred. Used only to send an updated `resume_token`.
        NoChange = 0,
        /// The targets have been added.
        Add = 1,
        /// The targets have been removed.
        Remove = 2,
        /// The targets reflect all changes committed before the targets were added
        /// to the stream.
        ///
        /// This will be sent after or with a `read_time` that is greater than or
        /// equal to the time at which the targets were added.
        ///
        /// Listeners can wait for this change if read-after-write semantics
        /// are desired.
        Current = 3,
        /// The targets have been reset, and a new initial state for the targets
        /// will be returned in subsequent changes.
        ///
        /// After the initial state is complete, `CURRENT` will be returned even
        /// if the target was previously indicated to be `CURRENT`.
        Reset = 4,
    }
}
/// The request for \[Firestore.ListCollectionIds][google.firestore.v1.Firestore.ListCollectionIds\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCollectionIdsRequest {
    /// Required. The parent document. In the format:
    /// `projects/{project_id}/databases/{database_id}/documents/{document_path}`.
    /// For example:
    /// `projects/my-project/databases/my-database/documents/chatrooms/my-chatroom`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of results to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token. Must be a value from
    /// \[ListCollectionIdsResponse][google.firestore.v1.ListCollectionIdsResponse\].
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response from \[Firestore.ListCollectionIds][google.firestore.v1.Firestore.ListCollectionIds\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCollectionIdsResponse {
    /// The collection ids.
    #[prost(string, repeated, tag = "1")]
    pub collection_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A page token that may be used to continue the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request for \[Firestore.BatchWrite][google.firestore.v1.Firestore.BatchWrite\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchWriteRequest {
    /// Required. The database name. In the format:
    /// `projects/{project_id}/databases/{database_id}`.
    #[prost(string, tag = "1")]
    pub database: ::prost::alloc::string::String,
    /// The writes to apply.
    ///
    /// Method does not apply writes atomically and does not guarantee ordering.
    /// Each write succeeds or fails independently. You cannot write to the same
    /// document more than once per request.
    #[prost(message, repeated, tag = "2")]
    pub writes: ::prost::alloc::vec::Vec<Write>,
    /// Labels associated with this batch write.
    #[prost(btree_map = "string, string", tag = "3")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// The response from \[Firestore.BatchWrite][google.firestore.v1.Firestore.BatchWrite\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchWriteResponse {
    /// The result of applying the writes.
    ///
    /// This i-th write result corresponds to the i-th write in the
    /// request.
    #[prost(message, repeated, tag = "1")]
    pub write_results: ::prost::alloc::vec::Vec<WriteResult>,
    /// The status of applying the writes.
    ///
    /// This i-th write status corresponds to the i-th write in the
    /// request.
    #[prost(message, repeated, tag = "2")]
    pub status: ::prost::alloc::vec::Vec<super::super::rpc::Status>,
}
#[doc = r" Generated client implementations."]
pub mod firestore_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " The Cloud Firestore service."]
    #[doc = ""]
    #[doc = " Cloud Firestore is a fast, fully managed, serverless, cloud-native NoSQL"]
    #[doc = " document database that simplifies storing, syncing, and querying data for"]
    #[doc = " your mobile, web, and IoT apps at global scale. Its client libraries provide"]
    #[doc = " live synchronization and offline support, while its security features and"]
    #[doc = " integrations with Firebase and Google Cloud Platform (GCP) accelerate"]
    #[doc = " building truly serverless apps."]
    #[derive(Debug, Clone)]
    pub struct FirestoreClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> FirestoreClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> FirestoreClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            FirestoreClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Gets a single document."]
        pub async fn get_document(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDocumentRequest>,
        ) -> Result<tonic::Response<super::Document>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.firestore.v1.Firestore/GetDocument");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists documents."]
        pub async fn list_documents(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDocumentsRequest>,
        ) -> Result<tonic::Response<super::ListDocumentsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.firestore.v1.Firestore/ListDocuments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates or inserts a document."]
        pub async fn update_document(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDocumentRequest>,
        ) -> Result<tonic::Response<super::Document>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.firestore.v1.Firestore/UpdateDocument",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a document."]
        pub async fn delete_document(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDocumentRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.firestore.v1.Firestore/DeleteDocument",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets multiple documents."]
        #[doc = ""]
        #[doc = " Documents returned by this method are not guaranteed to be returned in the"]
        #[doc = " same order that they were requested."]
        pub async fn batch_get_documents(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchGetDocumentsRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::BatchGetDocumentsResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.firestore.v1.Firestore/BatchGetDocuments",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Starts a new transaction."]
        pub async fn begin_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::BeginTransactionRequest>,
        ) -> Result<tonic::Response<super::BeginTransactionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.firestore.v1.Firestore/BeginTransaction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Commits a transaction, while optionally updating documents."]
        pub async fn commit(
            &mut self,
            request: impl tonic::IntoRequest<super::CommitRequest>,
        ) -> Result<tonic::Response<super::CommitResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.firestore.v1.Firestore/Commit");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Rolls back a transaction."]
        pub async fn rollback(
            &mut self,
            request: impl tonic::IntoRequest<super::RollbackRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.firestore.v1.Firestore/Rollback");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Runs a query."]
        pub async fn run_query(
            &mut self,
            request: impl tonic::IntoRequest<super::RunQueryRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::RunQueryResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.firestore.v1.Firestore/RunQuery");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Partitions a query by returning partition cursors that can be used to run"]
        #[doc = " the query in parallel. The returned partition cursors are split points that"]
        #[doc = " can be used by RunQuery as starting/end points for the query results."]
        pub async fn partition_query(
            &mut self,
            request: impl tonic::IntoRequest<super::PartitionQueryRequest>,
        ) -> Result<tonic::Response<super::PartitionQueryResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.firestore.v1.Firestore/PartitionQuery",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Streams batches of document updates and deletes, in order."]
        pub async fn write(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::WriteRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::WriteResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/google.firestore.v1.Firestore/Write");
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = " Listens to changes."]
        pub async fn listen(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::ListenRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::ListenResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.firestore.v1.Firestore/Listen");
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = " Lists all the collection IDs underneath a document."]
        pub async fn list_collection_ids(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCollectionIdsRequest>,
        ) -> Result<tonic::Response<super::ListCollectionIdsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.firestore.v1.Firestore/ListCollectionIds",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Applies a batch of write operations."]
        #[doc = ""]
        #[doc = " The BatchWrite method does not apply the write operations atomically"]
        #[doc = " and can apply them out of order. Method does not allow more than one write"]
        #[doc = " per document. Each write succeeds or fails independently. See the"]
        #[doc = " [BatchWriteResponse][google.firestore.v1.BatchWriteResponse] for the success status of each write."]
        #[doc = ""]
        #[doc = " If you require an atomically applied set of writes, use"]
        #[doc = " [Commit][google.firestore.v1.Firestore.Commit] instead."]
        pub async fn batch_write(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchWriteRequest>,
        ) -> Result<tonic::Response<super::BatchWriteResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.firestore.v1.Firestore/BatchWrite");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new document."]
        pub async fn create_document(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDocumentRequest>,
        ) -> Result<tonic::Response<super::Document>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.firestore.v1.Firestore/CreateDocument",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
