/// The desired input location information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputConfig {
    /// The input data format that used to store the model in Cloud Storage.
    #[prost(enumeration = "DataFormat", tag = "2")]
    pub data_format: i32,
    /// The location of the input model in cloud storage.
    /// Required.
    #[prost(oneof = "input_config::Source", tags = "1")]
    pub source: ::core::option::Option<input_config::Source>,
}
/// Nested message and enum types in `InputConfig`.
pub mod input_config {
    /// The location of the input model in cloud storage.
    /// Required.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// The Google Cloud Storage location to read the input from. This must be a
        /// single file.
        #[prost(message, tag = "1")]
        GcsSource(super::GcsSource),
    }
}
/// The desired output location.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputConfig {
    /// The output data format that used to store the results in Cloud Storage.
    #[prost(enumeration = "DataFormat", tag = "2")]
    pub data_format: i32,
    /// The location of the output result in cloud storage.
    /// Required.
    #[prost(oneof = "output_config::Destination", tags = "1")]
    pub destination: ::core::option::Option<output_config::Destination>,
}
/// Nested message and enum types in `OutputConfig`.
pub mod output_config {
    /// The location of the output result in cloud storage.
    /// Required.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// The Google Cloud Storage location to write the output to.
        #[prost(message, tag = "1")]
        GcsDestination(super::GcsDestination),
    }
}
/// The Google Cloud Storage location where the input file will be read from.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsSource {
    /// Required. URI of the Google Cloud Storage location.
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
}
/// The Google Cloud Storage location where the output file will be written to.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsDestination {
    /// Required. URI of the Google Cloud Storage location.
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
}
/// The long running operation metadata for async model related methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncModelMetadata {
    /// The state of the current operation.
    #[prost(enumeration = "async_model_metadata::State", tag = "1")]
    pub state: i32,
    /// A message providing more details about the current state of the operation.
    /// For example, the error message if the operation is failed.
    #[prost(string, tag = "2")]
    pub state_message: ::prost::alloc::string::String,
    /// The creation time of the operation.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The last update time of the operation.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `AsyncModelMetadata`.
pub mod async_model_metadata {
    /// Possible states of the operation.
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
        /// The default value. This value is used if the state is omitted.
        Unspecified = 0,
        /// Request is being processed.
        Running = 1,
        /// The operation completed successfully.
        Succeeded = 2,
        /// The operation was cancelled.
        Cancelled = 3,
        /// The operation has failed.
        Failed = 4,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Running => "RUNNING",
                State::Succeeded => "SUCCEEDED",
                State::Cancelled => "CANCELLED",
                State::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "RUNNING" => Some(Self::Running),
                "SUCCEEDED" => Some(Self::Succeeded),
                "CANCELLED" => Some(Self::Cancelled),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
/// Data formats for input and output files.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DataFormat {
    /// Default value.
    Unspecified = 0,
    /// Input data in json format.
    Json = 1,
    /// Input data in string format.
    String = 2,
}
impl DataFormat {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DataFormat::Unspecified => "DATA_FORMAT_UNSPECIFIED",
            DataFormat::Json => "JSON",
            DataFormat::String => "STRING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DATA_FORMAT_UNSPECIFIED" => Some(Self::Unspecified),
            "JSON" => Some(Self::Json),
            "STRING" => Some(Self::String),
            _ => None,
        }
    }
}
/// Request to be given to a tour optimization solver which defines the
/// shipment model to solve as well as optimization parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptimizeToursRequest {
    /// Required. Target project and location to make a call.
    ///
    /// Format: `projects/{project-id}/locations/{location-id}`.
    ///
    /// If no location is specified, a region will be chosen automatically.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// If this timeout is set, the server returns a response before the timeout
    /// period has elapsed or the server deadline for synchronous requests is
    /// reached, whichever is sooner.
    ///
    /// For asynchronous requests, the server will generate a solution (if
    /// possible) before the timeout has elapsed.
    #[prost(message, optional, tag = "2")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
    /// Shipment model to solve.
    #[prost(message, optional, tag = "3")]
    pub model: ::core::option::Option<ShipmentModel>,
    /// By default, the solving mode is `DEFAULT_SOLVE` (0).
    #[prost(enumeration = "optimize_tours_request::SolvingMode", tag = "4")]
    pub solving_mode: i32,
    /// Truncates the number of validation errors returned. These errors are
    /// typically attached to an INVALID_ARGUMENT error payload as a BadRequest
    /// error detail (<https://cloud.google.com/apis/design/errors#error_details>),
    /// unless solving_mode=VALIDATE_ONLY: see the
    /// \[OptimizeToursResponse.validation_errors][google.cloud.optimization.v1.OptimizeToursResponse.validation_errors\]
    /// field.
    /// This defaults to 100 and is capped at 10,000.
    #[prost(int32, optional, tag = "5")]
    pub max_validation_errors: ::core::option::Option<i32>,
    /// Search mode used to solve the request.
    #[prost(enumeration = "optimize_tours_request::SearchMode", tag = "6")]
    pub search_mode: i32,
    /// Guide the optimization algorithm in finding a first solution that is
    /// similar to a previous solution.
    ///
    /// The model is constrained when the first solution is built.
    /// Any shipments not performed on a route are implicitly skipped in the first
    /// solution, but they may be performed in successive solutions.
    ///
    /// The solution must satisfy some basic validity assumptions:
    ///
    ///    * for all routes, `vehicle_index` must be in range and not be duplicated.
    ///    * for all visits, `shipment_index` and `visit_request_index` must be
    ///      in range.
    ///    * a shipment may only be referenced on one route.
    ///    * the pickup of a pickup-delivery shipment must be performed before
    ///      the delivery.
    ///    * no more than one pickup alternative or delivery alternative of
    ///      a shipment may be performed.
    ///    * for all routes, times are increasing (i.e., `vehicle_start_time
    ///      <= visits\[0\].start_time <= visits\[1\].start_time ...
    ///      <= vehicle_end_time`).
    ///    * a shipment may only be performed on a vehicle that is allowed. A
    ///      vehicle is allowed if
    ///      \[Shipment.allowed_vehicle_indices][google.cloud.optimization.v1.Shipment.allowed_vehicle_indices\]
    ///      is empty or its `vehicle_index` is included in
    ///      \[Shipment.allowed_vehicle_indices][google.cloud.optimization.v1.Shipment.allowed_vehicle_indices\].
    ///
    /// If the injected solution is not feasible, a validation error is not
    /// necessarily returned and an error indicating infeasibility may be returned
    /// instead.
    #[prost(message, repeated, tag = "7")]
    pub injected_first_solution_routes: ::prost::alloc::vec::Vec<ShipmentRoute>,
    /// Constrain the optimization algorithm to find a final solution that is
    /// similar to a previous solution. For example, this may be used to freeze
    /// portions of routes which have already been completed or which are to be
    /// completed but must not be modified.
    ///
    /// If the injected solution is not feasible, a validation error is not
    /// necessarily returned and an error indicating infeasibility may be returned
    /// instead.
    #[prost(message, optional, tag = "8")]
    pub injected_solution_constraint: ::core::option::Option<InjectedSolutionConstraint>,
    /// If non-empty, the given routes will be refreshed, without modifying their
    /// underlying sequence of visits or travel times: only other details will be
    /// updated. This does not solve the model.
    ///
    /// As of 2020/11, this only populates the polylines of non-empty routes and
    /// requires that `populate_polylines` is true.
    ///
    /// The `route_polyline` fields of the passed-in routes may be inconsistent
    /// with route `transitions`.
    ///
    /// This field must not be used together with `injected_first_solution_routes`
    /// or `injected_solution_constraint`.
    ///
    /// `Shipment.ignore` and `Vehicle.ignore` have no effect on the behavior.
    /// Polylines are still populated between all visits in all non-empty routes
    /// regardless of whether the related shipments or vehicles are ignored.
    #[prost(message, repeated, tag = "9")]
    pub refresh_details_routes: ::prost::alloc::vec::Vec<ShipmentRoute>,
    /// If true:
    ///
    ///    * uses
    ///    \[ShipmentRoute.vehicle_label][google.cloud.optimization.v1.ShipmentRoute.vehicle_label\]
    ///    instead of `vehicle_index` to
    ///      match routes in an injected solution with vehicles in the request;
    ///      reuses the mapping of original
    ///      \[ShipmentRoute.vehicle_index][google.cloud.optimization.v1.ShipmentRoute.vehicle_index\]
    ///      to new
    ///      \[ShipmentRoute.vehicle_index][google.cloud.optimization.v1.ShipmentRoute.vehicle_index\]
    ///      to update
    ///      \[ConstraintRelaxation.vehicle_indices][google.cloud.optimization.v1.InjectedSolutionConstraint.ConstraintRelaxation.vehicle_indices\]
    ///      if non-empty, but the mapping must be unambiguous (i.e., multiple
    ///      `ShipmentRoute`s must not share the same original `vehicle_index`).
    ///    * uses
    ///    \[ShipmentRoute.Visit.shipment_label][google.cloud.optimization.v1.ShipmentRoute.Visit.shipment_label\]
    ///    instead of `shipment_index`
    ///      to match visits in an injected solution with shipments in the request;
    ///    * uses
    ///    \[SkippedShipment.label][google.cloud.optimization.v1.SkippedShipment.label\]
    ///    instead of
    ///    \[SkippedShipment.index][google.cloud.optimization.v1.SkippedShipment.index\]
    ///    to
    ///      match skipped shipments in the injected solution with request
    ///      shipments.
    ///
    /// This interpretation applies to the `injected_first_solution_routes`,
    /// `injected_solution_constraint`, and `refresh_details_routes` fields.
    /// It can be used when shipment or vehicle indices in the request have
    /// changed since the solution was created, perhaps because shipments or
    /// vehicles have been removed from or added to the request.
    ///
    /// If true, labels in the following categories must appear at most once in
    /// their category:
    ///
    ///    * \[Vehicle.label][google.cloud.optimization.v1.Vehicle.label\] in the
    ///    request;
    ///    * \[Shipment.label][google.cloud.optimization.v1.Shipment.label\] in the
    ///    request;
    ///    * \[ShipmentRoute.vehicle_label][google.cloud.optimization.v1.ShipmentRoute.vehicle_label\] in the injected solution;
    ///    * \[SkippedShipment.label][google.cloud.optimization.v1.SkippedShipment.label\] and \[ShipmentRoute.Visit.shipment_label][google.cloud.optimization.v1.ShipmentRoute.Visit.shipment_label\] in
    ///      the injected solution (except pickup/delivery visit pairs, whose
    ///      `shipment_label` must appear twice).
    ///
    /// If a `vehicle_label` in the injected solution does not correspond to a
    /// request vehicle, the corresponding route is removed from the solution
    /// along with its visits. If a `shipment_label` in the injected solution does
    /// not correspond to a request shipment, the corresponding visit is removed
    /// from the solution. If a
    /// \[SkippedShipment.label][google.cloud.optimization.v1.SkippedShipment.label\]
    /// in the injected solution does not correspond to a request shipment, the
    /// `SkippedShipment` is removed from the solution.
    ///
    /// Removing route visits or entire routes from an injected solution may
    /// have an effect on the implied constraints, which may lead to change in
    /// solution, validation errors, or infeasibility.
    ///
    /// NOTE: The caller must ensure that each
    /// \[Vehicle.label][google.cloud.optimization.v1.Vehicle.label\] (resp.
    /// \[Shipment.label][google.cloud.optimization.v1.Shipment.label\]) uniquely
    /// identifies a vehicle (resp. shipment) entity used across the two relevant
    /// requests: the past request that produced the `OptimizeToursResponse` used
    /// in the injected solution and the current request that includes the injected
    /// solution. The uniqueness checks described above are not enough to guarantee
    /// this requirement.
    #[prost(bool, tag = "10")]
    pub interpret_injected_solutions_using_labels: bool,
    /// Consider traffic estimation in calculating `ShipmentRoute` fields
    /// \[Transition.travel_duration][google.cloud.optimization.v1.ShipmentRoute.Transition.travel_duration\],
    /// \[Visit.start_time][google.cloud.optimization.v1.ShipmentRoute.Visit.start_time\],
    /// and `vehicle_end_time`; in setting the
    /// \[ShipmentRoute.has_traffic_infeasibilities][google.cloud.optimization.v1.ShipmentRoute.has_traffic_infeasibilities\]
    /// field, and in calculating the
    /// \[OptimizeToursResponse.total_cost][google.cloud.optimization.v1.OptimizeToursResponse.total_cost\]
    /// field.
    #[prost(bool, tag = "11")]
    pub consider_road_traffic: bool,
    /// If true, polylines will be populated in response `ShipmentRoute`s.
    #[prost(bool, tag = "12")]
    pub populate_polylines: bool,
    /// If true, polylines will be populated in response
    /// \[ShipmentRoute.transitions][google.cloud.optimization.v1.ShipmentRoute.transitions\].
    /// Note that in this case, the polylines will also be populated in the
    /// deprecated `travel_steps`.
    #[prost(bool, tag = "13")]
    pub populate_transition_polylines: bool,
    /// If this is set, then the request can have a deadline
    /// (see <https://grpc.io/blog/deadlines>) of up to 60 minutes.
    /// Otherwise, the maximum deadline is only 30 minutes.
    /// Note that long-lived requests have a significantly larger (but still small)
    /// risk of interruption.
    #[prost(bool, tag = "14")]
    pub allow_large_deadline_despite_interruption_risk: bool,
    /// If true, travel distances will be computed using geodesic distances instead
    /// of Google Maps distances, and travel times will be computed using geodesic
    /// distances with a speed defined by `geodesic_meters_per_second`.
    #[prost(bool, tag = "15")]
    pub use_geodesic_distances: bool,
    /// When `use_geodesic_distances` is true, this field must be set and defines
    /// the speed applied to compute travel times. Its value must be at least 1.0
    /// meters/seconds.
    #[prost(double, optional, tag = "16")]
    pub geodesic_meters_per_second: ::core::option::Option<f64>,
    /// Label that may be used to identify this request, reported back in the
    /// \[OptimizeToursResponse.request_label][google.cloud.optimization.v1.OptimizeToursResponse.request_label\].
    #[prost(string, tag = "17")]
    pub label: ::prost::alloc::string::String,
    /// Deprecated: Use
    /// \[OptimizeToursRequest.populate_transition_polylines][google.cloud.optimization.v1.OptimizeToursRequest.populate_transition_polylines\]
    /// instead. If true, polylines will be populated in response
    /// \[ShipmentRoute.transitions][google.cloud.optimization.v1.ShipmentRoute.transitions\].
    /// Note that in this case, the polylines will also be populated in the
    /// deprecated `travel_steps`.
    #[deprecated]
    #[prost(bool, tag = "20")]
    pub populate_travel_step_polylines: bool,
}
/// Nested message and enum types in `OptimizeToursRequest`.
pub mod optimize_tours_request {
    /// Defines how the solver should handle the request. In all modes but
    /// `VALIDATE_ONLY`, if the request is invalid, you will receive an
    /// `INVALID_REQUEST` error. See
    /// \[max_validation_errors][google.cloud.optimization.v1.OptimizeToursRequest.max_validation_errors\]
    /// to cap the number of errors returned.
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
    pub enum SolvingMode {
        /// Solve the model.
        DefaultSolve = 0,
        /// Only validates the model without solving it: populates as many
        /// \[OptimizeToursResponse.validation_errors][google.cloud.optimization.v1.OptimizeToursResponse.validation_errors\]
        /// as possible.
        ValidateOnly = 1,
        /// Only populates
        /// \[OptimizeToursResponse.skipped_shipments][google.cloud.optimization.v1.OptimizeToursResponse.skipped_shipments\],
        /// and doesn't actually solve the rest of the request (`status` and `routes`
        /// are unset in the response).
        ///
        /// *IMPORTANT*: not all infeasible shipments are returned here, but only the
        /// ones that are detected as infeasible as a preprocessing.
        DetectSomeInfeasibleShipments = 2,
    }
    impl SolvingMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SolvingMode::DefaultSolve => "DEFAULT_SOLVE",
                SolvingMode::ValidateOnly => "VALIDATE_ONLY",
                SolvingMode::DetectSomeInfeasibleShipments => {
                    "DETECT_SOME_INFEASIBLE_SHIPMENTS"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DEFAULT_SOLVE" => Some(Self::DefaultSolve),
                "VALIDATE_ONLY" => Some(Self::ValidateOnly),
                "DETECT_SOME_INFEASIBLE_SHIPMENTS" => {
                    Some(Self::DetectSomeInfeasibleShipments)
                }
                _ => None,
            }
        }
    }
    /// Mode defining the behavior of the search, trading off latency versus
    /// solution quality. In all modes, the global request deadline is enforced.
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
    pub enum SearchMode {
        /// Unspecified search mode, equivalent to `RETURN_FAST`.
        Unspecified = 0,
        /// Stop the search after finding the first good solution.
        ReturnFast = 1,
        /// Spend all the available time to search for better solutions.
        ConsumeAllAvailableTime = 2,
    }
    impl SearchMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SearchMode::Unspecified => "SEARCH_MODE_UNSPECIFIED",
                SearchMode::ReturnFast => "RETURN_FAST",
                SearchMode::ConsumeAllAvailableTime => "CONSUME_ALL_AVAILABLE_TIME",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SEARCH_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "RETURN_FAST" => Some(Self::ReturnFast),
                "CONSUME_ALL_AVAILABLE_TIME" => Some(Self::ConsumeAllAvailableTime),
                _ => None,
            }
        }
    }
}
/// Response after solving a tour optimization problem containing the routes
/// followed by each vehicle, the shipments which have been skipped and the
/// overall cost of the solution.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptimizeToursResponse {
    /// Routes computed for each vehicle; the i-th route corresponds to the i-th
    /// vehicle in the model.
    #[prost(message, repeated, tag = "1")]
    pub routes: ::prost::alloc::vec::Vec<ShipmentRoute>,
    /// Copy of the
    /// \[OptimizeToursRequest.label][google.cloud.optimization.v1.OptimizeToursRequest.label\],
    /// if a label was specified in the request.
    #[prost(string, tag = "3")]
    pub request_label: ::prost::alloc::string::String,
    /// The list of all shipments skipped.
    #[prost(message, repeated, tag = "4")]
    pub skipped_shipments: ::prost::alloc::vec::Vec<SkippedShipment>,
    /// List of all the validation errors that we were able to detect
    /// independently. See the "MULTIPLE ERRORS" explanation for the
    /// \[OptimizeToursValidationError][google.cloud.optimization.v1.OptimizeToursValidationError\]
    /// message.
    #[prost(message, repeated, tag = "5")]
    pub validation_errors: ::prost::alloc::vec::Vec<OptimizeToursValidationError>,
    /// Duration, distance and usage metrics for this solution.
    #[prost(message, optional, tag = "6")]
    pub metrics: ::core::option::Option<optimize_tours_response::Metrics>,
    /// Deprecated: Use
    /// \[Metrics.total_cost][google.cloud.optimization.v1.OptimizeToursResponse.Metrics.total_cost\]
    /// instead. Total cost of the solution. This takes into account all costs:
    /// costs per per hour and travel hour, fixed vehicle costs, unperformed
    /// shipment penalty costs, global duration cost, etc.
    #[deprecated]
    #[prost(double, tag = "2")]
    pub total_cost: f64,
}
/// Nested message and enum types in `OptimizeToursResponse`.
pub mod optimize_tours_response {
    /// Overall metrics, aggregated over all routes.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Metrics {
        /// Aggregated over the routes. Each metric is the sum (or max, for loads)
        /// over all
        /// \[ShipmentRoute.metrics][google.cloud.optimization.v1.ShipmentRoute.metrics\]
        /// fields of the same name.
        #[prost(message, optional, tag = "1")]
        pub aggregated_route_metrics: ::core::option::Option<super::AggregatedMetrics>,
        /// Number of mandatory shipments skipped.
        #[prost(int32, tag = "2")]
        pub skipped_mandatory_shipment_count: i32,
        /// Number of vehicles used. Note: if a vehicle route is empty and
        /// \[Vehicle.used_if_route_is_empty][google.cloud.optimization.v1.Vehicle.used_if_route_is_empty\]
        /// is true, the vehicle is considered used.
        #[prost(int32, tag = "3")]
        pub used_vehicle_count: i32,
        /// The earliest start time for a used vehicle, computed as the minimum over
        /// all used vehicles of
        /// \[ShipmentRoute.vehicle_start_time][google.cloud.optimization.v1.ShipmentRoute.vehicle_start_time\].
        #[prost(message, optional, tag = "4")]
        pub earliest_vehicle_start_time: ::core::option::Option<
            ::prost_types::Timestamp,
        >,
        /// The latest end time for a used vehicle, computed as the maximum over all
        /// used vehicles of
        /// \[ShipmentRoute.vehicle_end_time][google.cloud.optimization.v1.ShipmentRoute.vehicle_end_time\].
        #[prost(message, optional, tag = "5")]
        pub latest_vehicle_end_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Cost of the solution, broken down by cost-related request fields.
        /// The keys are proto paths, relative to the input OptimizeToursRequest,
        /// e.g. "model.shipments.pickups.cost", and the values are the total cost
        /// generated by the corresponding cost field, aggregated over the whole
        /// solution. In other words, costs\["model.shipments.pickups.cost"\] is the
        /// sum of all pickup costs over the solution. All costs defined in the model
        /// are reported in detail here with the exception of costs related to
        /// TransitionAttributes that are only reported in an aggregated way as of
        /// 2022/01.
        #[prost(btree_map = "string, double", tag = "10")]
        pub costs: ::prost::alloc::collections::BTreeMap<
            ::prost::alloc::string::String,
            f64,
        >,
        /// Total cost of the solution. The sum of all values in the costs map.
        #[prost(double, tag = "6")]
        pub total_cost: f64,
    }
}
/// Request to batch optimize tours as an asynchronous operation.
/// Each input file should contain one `OptimizeToursRequest`, and each output
/// file will contain one `OptimizeToursResponse`. The request contains
/// information to read/write and parse the files. All the input and output files
/// should be under the same project.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchOptimizeToursRequest {
    /// Required. Target project and location to make a call.
    ///
    /// Format: `projects/{project-id}/locations/{location-id}`.
    ///
    /// If no location is specified, a region will be chosen automatically.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Input/Output information each purchase model, such as file paths
    /// and data formats.
    #[prost(message, repeated, tag = "2")]
    pub model_configs: ::prost::alloc::vec::Vec<
        batch_optimize_tours_request::AsyncModelConfig,
    >,
}
/// Nested message and enum types in `BatchOptimizeToursRequest`.
pub mod batch_optimize_tours_request {
    /// Information for solving one optimization model asynchronously.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AsyncModelConfig {
        /// User defined model name, can be used as alias by users to keep track of
        /// models.
        #[prost(string, tag = "1")]
        pub display_name: ::prost::alloc::string::String,
        /// Required. Information about the input model.
        #[prost(message, optional, tag = "2")]
        pub input_config: ::core::option::Option<super::InputConfig>,
        /// Required. The desired output location information.
        #[prost(message, optional, tag = "3")]
        pub output_config: ::core::option::Option<super::OutputConfig>,
        /// If this is set, the model will be solved in the checkpoint mode. In this
        /// mode, the input model can have a deadline longer than 30 mins without the
        /// risk of interruption. The model will be solved in multiple short-running
        /// stages. Each stage generates an intermediate checkpoint
        /// and stores it in the user's Cloud Storage buckets. The checkpoint
        /// mode should be preferred over
        /// allow_large_deadline_despite_interruption_risk since it prevents the risk
        /// of interruption.
        #[prost(bool, tag = "4")]
        pub enable_checkpoints: bool,
    }
}
/// Response to a `BatchOptimizeToursRequest`. This is returned in
/// the LRO Operation after the operation is complete.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchOptimizeToursResponse {}
/// A shipment model contains a set of shipments which must be performed by a
/// set of vehicles, while minimizing the overall cost, which is the sum of:
///
/// * the cost of routing the vehicles (sum of cost per total time, cost per
///    travel time, and fixed cost over all vehicles).
/// * the unperformed shipment penalties.
/// * the cost of the global duration of the shipments
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShipmentModel {
    /// Set of shipments which must be performed in the model.
    #[prost(message, repeated, tag = "1")]
    pub shipments: ::prost::alloc::vec::Vec<Shipment>,
    /// Set of vehicles which can be used to perform visits.
    #[prost(message, repeated, tag = "2")]
    pub vehicles: ::prost::alloc::vec::Vec<Vehicle>,
    /// Constrains the maximum number of active vehicles. A vehicle is active if
    /// its route performs at least one shipment. This can be used to limit the
    /// number of routes in the case where there are fewer drivers than
    /// vehicles and that the fleet of vehicles is heterogeneous. The optimization
    /// will then select the best subset of vehicles to use.
    /// Must be strictly positive.
    #[prost(int32, optional, tag = "4")]
    pub max_active_vehicles: ::core::option::Option<i32>,
    /// Global start and end time of the model: no times outside of this range
    /// can be considered valid.
    ///
    /// The model's time span must be less than a year, i.e. the `global_end_time`
    /// and the `global_start_time` must be within 31536000 seconds of each other.
    ///
    /// When using `cost_per_*hour` fields, you might want to set this window to a
    /// smaller interval to increase performance (eg. if you model a single day,
    /// you should set the global time limits to that day).
    /// If unset, 00:00:00 UTC, January 1, 1970 (i.e. seconds: 0, nanos: 0) is used
    /// as default.
    #[prost(message, optional, tag = "5")]
    pub global_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// If unset, 00:00:00 UTC, January 1, 1971 (i.e. seconds: 31536000, nanos: 0)
    /// is used as default.
    #[prost(message, optional, tag = "6")]
    pub global_end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The "global duration" of the overall plan is the difference between the
    /// earliest effective start time and the latest effective end time of
    /// all vehicles. Users can assign a cost per hour to that quantity to try
    /// and optimize for earliest job completion, for example. This cost must be in
    /// the same unit as
    /// \[Shipment.penalty_cost][google.cloud.optimization.v1.Shipment.penalty_cost\].
    #[prost(double, tag = "7")]
    pub global_duration_cost_per_hour: f64,
    /// Specifies duration and distance matrices used in the model. If this field
    /// is empty, Google Maps or geodesic distances will be used instead, depending
    /// on the value of the `use_geodesic_distances` field. If it is not empty,
    /// `use_geodesic_distances` cannot be true and neither
    /// `duration_distance_matrix_src_tags` nor `duration_distance_matrix_dst_tags`
    /// can be empty.
    ///
    /// Usage examples:
    ///
    /// * There are two locations: locA and locB.
    /// * 1 vehicle starting its route at locA and ending it at locA.
    /// * 1 pickup visit request at locB.
    ///
    /// ```
    /// model {
    ///    vehicles { start_tags: "locA"  end_tags: "locA" }
    ///    shipments { pickups { tags: "locB" } }
    ///    duration_distance_matrix_src_tags: "locA"
    ///    duration_distance_matrix_src_tags: "locB"
    ///    duration_distance_matrix_dst_tags: "locA"
    ///    duration_distance_matrix_dst_tags: "locB"
    ///    duration_distance_matrices {
    ///      rows {  # from: locA
    ///        durations { seconds: 0 }   meters: 0    # to: locA
    ///        durations { seconds: 100 } meters: 1000 # to: locB
    ///      }
    ///      rows {  # from: locB
    ///        durations { seconds: 102 } meters: 990 # to: locA
    ///        durations { seconds: 0 }   meters: 0   # to: locB
    ///      }
    ///    }
    /// }
    /// ```
    ///
    ///
    /// * There are three locations: locA, locB and locC.
    /// * 1 vehicle starting its route at locA and ending it at locB, using
    ///    matrix "fast".
    /// * 1 vehicle starting its route at locB and ending it at locB, using
    ///    matrix "slow".
    /// * 1 vehicle starting its route at locB and ending it at locB, using
    ///    matrix "fast".
    /// * 1 pickup visit request at locC.
    ///
    /// ```
    /// model {
    ///    vehicles { start_tags: "locA" end_tags: "locB" start_tags: "fast" }
    ///    vehicles { start_tags: "locB" end_tags: "locB" start_tags: "slow" }
    ///    vehicles { start_tags: "locB" end_tags: "locB" start_tags: "fast" }
    ///    shipments { pickups { tags: "locC" } }
    ///    duration_distance_matrix_src_tags: "locA"
    ///    duration_distance_matrix_src_tags: "locB"
    ///    duration_distance_matrix_src_tags: "locC"
    ///    duration_distance_matrix_dst_tags: "locB"
    ///    duration_distance_matrix_dst_tags: "locC"
    ///    duration_distance_matrices {
    ///      vehicle_start_tag: "fast"
    ///      rows {  # from: locA
    ///        durations { seconds: 1000 } meters: 2000 # to: locB
    ///        durations { seconds: 600 }  meters: 1000 # to: locC
    ///      }
    ///      rows {  # from: locB
    ///        durations { seconds: 0 }   meters: 0    # to: locB
    ///        durations { seconds: 700 } meters: 1200 # to: locC
    ///      }
    ///      rows {  # from: locC
    ///        durations { seconds: 702 } meters: 1190 # to: locB
    ///        durations { seconds: 0 }   meters: 0    # to: locC
    ///      }
    ///    }
    ///    duration_distance_matrices {
    ///      vehicle_start_tag: "slow"
    ///      rows {  # from: locA
    ///        durations { seconds: 1800 } meters: 2001 # to: locB
    ///        durations { seconds: 900 }  meters: 1002 # to: locC
    ///      }
    ///      rows {  # from: locB
    ///        durations { seconds: 0 }    meters: 0    # to: locB
    ///        durations { seconds: 1000 } meters: 1202 # to: locC
    ///      }
    ///      rows {  # from: locC
    ///        durations { seconds: 1001 } meters: 1195 # to: locB
    ///        durations { seconds: 0 }    meters: 0    # to: locC
    ///      }
    ///    }
    /// }
    /// ```
    #[prost(message, repeated, tag = "8")]
    pub duration_distance_matrices: ::prost::alloc::vec::Vec<
        shipment_model::DurationDistanceMatrix,
    >,
    /// Tags defining the sources of the duration and distance matrices;
    /// `duration_distance_matrices(i).rows(j)` defines durations and distances
    /// from visits with tag `duration_distance_matrix_src_tags(j)` to other visits
    /// in matrix i.
    ///
    /// Tags correspond to
    /// \[VisitRequest.tags][google.cloud.optimization.v1.Shipment.VisitRequest.tags\]
    /// or \[Vehicle.start_tags][google.cloud.optimization.v1.Vehicle.start_tags\].
    /// A given `VisitRequest` or `Vehicle` must match exactly one tag in this
    /// field. Note that a `Vehicle`'s source, destination and matrix tags may be
    /// the same; similarly a `VisitRequest`'s source and destination tags may be
    /// the same. All tags must be different and cannot be empty strings. If this
    /// field is not empty, then `duration_distance_matrices` must not be empty.
    #[prost(string, repeated, tag = "9")]
    pub duration_distance_matrix_src_tags: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Tags defining the destinations of the duration and distance matrices;
    /// `duration_distance_matrices(i).rows(j).durations(k)` (resp.
    /// `duration_distance_matrices(i).rows(j).meters(k))` defines the duration
    /// (resp. the distance) of the travel from visits with tag
    /// `duration_distance_matrix_src_tags(j)` to visits with tag
    /// `duration_distance_matrix_dst_tags(k)` in matrix i.
    ///
    /// Tags correspond to
    /// \[VisitRequest.tags][google.cloud.optimization.v1.Shipment.VisitRequest.tags\]
    /// or \[Vehicle.start_tags][google.cloud.optimization.v1.Vehicle.start_tags\].
    /// A given `VisitRequest` or `Vehicle` must match exactly one tag in this
    /// field. Note that a `Vehicle`'s source, destination and matrix tags may be
    /// the same; similarly a `VisitRequest`'s source and destination tags may be
    /// the same. All tags must be different and cannot be empty strings. If this
    /// field is not empty, then `duration_distance_matrices` must not be empty.
    #[prost(string, repeated, tag = "10")]
    pub duration_distance_matrix_dst_tags: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Transition attributes added to the model.
    #[prost(message, repeated, tag = "11")]
    pub transition_attributes: ::prost::alloc::vec::Vec<TransitionAttributes>,
    /// Sets of incompatible shipment_types (see `ShipmentTypeIncompatibility`).
    #[prost(message, repeated, tag = "12")]
    pub shipment_type_incompatibilities: ::prost::alloc::vec::Vec<
        ShipmentTypeIncompatibility,
    >,
    /// Sets of `shipment_type` requirements (see `ShipmentTypeRequirement`).
    #[prost(message, repeated, tag = "13")]
    pub shipment_type_requirements: ::prost::alloc::vec::Vec<ShipmentTypeRequirement>,
    /// Set of precedence rules which must be enforced in the model.
    #[prost(message, repeated, tag = "14")]
    pub precedence_rules: ::prost::alloc::vec::Vec<shipment_model::PrecedenceRule>,
    /// Deprecated: No longer used.
    /// Set of break rules used in the model.
    /// Each vehicle specifies the `BreakRule` that applies to it via the
    /// \[Vehicle.break_rule_indices][google.cloud.optimization.v1.Vehicle.break_rule_indices\]
    /// field (which must be a singleton).
    #[deprecated]
    #[prost(message, repeated, tag = "15")]
    pub break_rules: ::prost::alloc::vec::Vec<shipment_model::BreakRule>,
}
/// Nested message and enum types in `ShipmentModel`.
pub mod shipment_model {
    /// Specifies a duration and distance matrix from visit and vehicle start
    /// locations to visit and vehicle end locations.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DurationDistanceMatrix {
        /// Specifies the rows of the duration and distance matrix. It must have as
        /// many elements as
        /// \[ShipmentModel.duration_distance_matrix_src_tags][google.cloud.optimization.v1.ShipmentModel.duration_distance_matrix_src_tags\].
        #[prost(message, repeated, tag = "1")]
        pub rows: ::prost::alloc::vec::Vec<duration_distance_matrix::Row>,
        /// Tag defining to which vehicles this duration and distance matrix applies.
        /// If empty, this applies to all vehicles, and there can only be a single
        /// matrix.
        ///
        /// Each vehicle start must match exactly one matrix, i.e. exactly one of
        /// their `start_tags` field must match the `vehicle_start_tag` of a matrix
        /// (and of that matrix only).
        ///
        /// All matrices must have a different `vehicle_start_tag`.
        #[prost(string, tag = "2")]
        pub vehicle_start_tag: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `DurationDistanceMatrix`.
    pub mod duration_distance_matrix {
        /// Specifies a row of the duration and distance matrix.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Row {
            /// Duration values for a given row. It must have as many elements as
            /// \[ShipmentModel.duration_distance_matrix_dst_tags][google.cloud.optimization.v1.ShipmentModel.duration_distance_matrix_dst_tags\].
            #[prost(message, repeated, tag = "1")]
            pub durations: ::prost::alloc::vec::Vec<::prost_types::Duration>,
            /// Distance values for a given row. If no costs or constraints refer to
            /// distances in the model, this can be left empty; otherwise it must have
            /// as many elements as `durations`.
            #[prost(double, repeated, tag = "2")]
            pub meters: ::prost::alloc::vec::Vec<f64>,
        }
    }
    /// A precedence rule between two events (each event is the pickup or the
    /// delivery of a shipment): the "second" event has to start at least
    /// `offset_duration` after "first" has started.
    ///
    /// Several precedences can refer to the same (or related) events, e.g.,
    /// "pickup of B happens after delivery of A" and "pickup of C happens after
    /// pickup of B".
    ///
    /// Furthermore, precedences only apply when both shipments are performed and
    /// are otherwise ignored.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PrecedenceRule {
        /// Shipment index of the "first" event. This field must be specified.
        #[prost(int32, optional, tag = "1")]
        pub first_index: ::core::option::Option<i32>,
        /// Indicates if the "first" event is a delivery.
        #[prost(bool, tag = "3")]
        pub first_is_delivery: bool,
        /// Shipment index of the "second" event. This field must be specified.
        #[prost(int32, optional, tag = "2")]
        pub second_index: ::core::option::Option<i32>,
        /// Indicates if the "second" event is a delivery.
        #[prost(bool, tag = "4")]
        pub second_is_delivery: bool,
        /// The offset between the "first" and "second" event. It can be negative.
        #[prost(message, optional, tag = "5")]
        pub offset_duration: ::core::option::Option<::prost_types::Duration>,
    }
    /// Deprecated: Use top level
    /// \[BreakRule][google.cloud.optimization.v1.ShipmentModel.BreakRule\] instead.
    /// Rules to generate time breaks for a vehicle (e.g. lunch
    /// breaks). A break is a contiguous period of time during which the vehicle
    /// remains idle at its current position and cannot perform any visit. A break
    /// may occur:
    ///
    /// * during the travel between two visits (which includes the time right
    ///    before or right after a visit, but not in the middle of a visit), in
    ///    which case it extends the corresponding transit time between the visits
    /// * before the vehicle start (the vehicle may not start in the middle of
    ///    a break), in which case it does not affect the vehicle start time.
    /// * after the vehicle end (ditto, with the vehicle end time).
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BreakRule {
        /// Sequence of breaks. See the `BreakRequest` message.
        #[prost(message, repeated, tag = "1")]
        pub break_requests: ::prost::alloc::vec::Vec<break_rule::BreakRequest>,
        /// Several `FrequencyConstraint` may apply. They must all be satisfied by
        /// the `BreakRequest`s of this `BreakRule`. See `FrequencyConstraint`.
        #[prost(message, repeated, tag = "2")]
        pub frequency_constraints: ::prost::alloc::vec::Vec<
            break_rule::FrequencyConstraint,
        >,
    }
    /// Nested message and enum types in `BreakRule`.
    pub mod break_rule {
        /// The sequence of breaks (i.e. their number and order) that apply to each
        /// vehicle must be known beforehand. The repeated `BreakRequest`s define
        /// that sequence, in the order in which they must occur. Their time windows
        /// (`earliest_start_time` / `latest_start_time`) may overlap, but they must
        /// be compatible with the order (this is checked).
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct BreakRequest {
            /// Required. Lower bound (inclusive) on the start of the break.
            #[prost(message, optional, tag = "1")]
            pub earliest_start_time: ::core::option::Option<::prost_types::Timestamp>,
            /// Required. Upper bound (inclusive) on the start of the break.
            #[prost(message, optional, tag = "2")]
            pub latest_start_time: ::core::option::Option<::prost_types::Timestamp>,
            /// Required. Minimum duration of the break. Must be positive.
            #[prost(message, optional, tag = "3")]
            pub min_duration: ::core::option::Option<::prost_types::Duration>,
        }
        /// One may further constrain the frequency and duration of the breaks
        /// specified above, by enforcing a minimum break frequency, such as
        /// "There must be a break of at least 1 hour every 12 hours". Assuming that
        /// this can be interpreted as "Within any sliding time window of 12h, there
        /// must be at least one break of at least one hour", that example would
        /// translate to the following `FrequencyConstraint`:
        /// ```
        /// {
        ///     min_break_duration { seconds: 3600 }         # 1 hour.
        ///     max_inter_break_duration { seconds: 39600 }  # 11 hours (12 - 1 = 11).
        /// }
        /// ```
        ///
        /// The timing and duration of the breaks in the solution will respect all
        /// such constraints, in addition to the time windows and minimum durations
        /// already specified in the `BreakRequest`.
        ///
        /// A `FrequencyConstraint` may in practice apply to non-consecutive breaks.
        /// For example, the following schedule honors the "1h every 12h" example:
        /// ```
        ///    04:00 vehicle start
        ///     .. performing travel and visits ..
        ///    09:00 1 hour break
        ///    10:00 end of the break
        ///     .. performing travel and visits ..
        ///    12:00 20-min lunch break
        ///    12:20 end of the break
        ///     .. performing travel and visits ..
        ///    21:00 1 hour break
        ///    22:00 end of the break
        ///     .. performing travel and visits ..
        ///    23:59 vehicle end
        /// ```
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FrequencyConstraint {
            /// Required. Minimum break duration for this constraint. Nonnegative.
            /// See description of `FrequencyConstraint`.
            #[prost(message, optional, tag = "1")]
            pub min_break_duration: ::core::option::Option<::prost_types::Duration>,
            /// Required. Maximum allowed span of any interval of time in the route
            /// that does not include at least partially a break of `duration >=
            /// min_break_duration`. Must be positive.
            #[prost(message, optional, tag = "2")]
            pub max_inter_break_duration: ::core::option::Option<
                ::prost_types::Duration,
            >,
        }
    }
}
/// The shipment of a single item, from one of its pickups to one of its
/// deliveries. For the shipment to be considered as performed, a unique vehicle
/// must visit one of its pickup locations (and decrease its spare capacities
/// accordingly), then visit one of its delivery locations later on (and
/// therefore re-increase its spare capacities accordingly).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Shipment {
    /// Set of pickup alternatives associated to the shipment. If not specified,
    /// the vehicle only needs to visit a location corresponding to the deliveries.
    #[prost(message, repeated, tag = "1")]
    pub pickups: ::prost::alloc::vec::Vec<shipment::VisitRequest>,
    /// Set of delivery alternatives associated to the shipment. If not specified,
    /// the vehicle only needs to visit a location corresponding to the pickups.
    #[prost(message, repeated, tag = "2")]
    pub deliveries: ::prost::alloc::vec::Vec<shipment::VisitRequest>,
    /// Load demands of the shipment (for example weight, volume, number of
    /// pallets etc). The keys in the map should be identifiers describing the type
    /// of the corresponding load, ideally also including the units.
    /// For example: "weight_kg", "volume_gallons", "pallet_count", etc.
    /// If a given key does not appear in the map, the corresponding load is
    /// considered as null.
    #[prost(btree_map = "string, message", tag = "14")]
    pub load_demands: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        shipment::Load,
    >,
    /// If the shipment is not completed, this penalty is added to the overall
    /// cost of the routes. A shipment is considered completed if one of its pickup
    /// and delivery alternatives is visited. The cost may be expressed in the
    /// same unit used for all other cost-related fields in the model and must be
    /// positive.
    ///
    /// *IMPORTANT*: If this penalty is not specified, it is considered infinite,
    /// i.e. the shipment must be completed.
    #[prost(double, optional, tag = "4")]
    pub penalty_cost: ::core::option::Option<f64>,
    /// The set of vehicles that may perform this shipment. If empty, all vehicles
    /// may perform it. Vehicles are given by their index in the `ShipmentModel`'s
    /// `vehicles` list.
    #[prost(int32, repeated, tag = "5")]
    pub allowed_vehicle_indices: ::prost::alloc::vec::Vec<i32>,
    /// Specifies the cost that is incurred when this shipment is delivered by each
    /// vehicle. If specified, it must have EITHER:
    ///
    ///    * the same number of elements as `costs_per_vehicle_indices`.
    ///      `costs_per_vehicle\[i\]` corresponds to vehicle
    ///      `costs_per_vehicle_indices\[i\]` of the model.
    ///    * the same number of elements as there are vehicles in the model. The
    ///      i-th element corresponds to vehicle #i of the model.
    ///
    /// These costs must be in the same unit as `penalty_cost` and must not be
    /// negative. Leave this field empty, if there are no such costs.
    #[prost(double, repeated, tag = "6")]
    pub costs_per_vehicle: ::prost::alloc::vec::Vec<f64>,
    /// Indices of the vehicles to which `costs_per_vehicle` applies. If non-empty,
    /// it must have the same number of elements as `costs_per_vehicle`. A vehicle
    /// index may not be specified more than once. If a vehicle is excluded from
    /// `costs_per_vehicle_indices`, its cost is zero.
    #[prost(int32, repeated, tag = "7")]
    pub costs_per_vehicle_indices: ::prost::alloc::vec::Vec<i32>,
    /// Specifies the maximum relative detour time compared to the shortest path
    /// from pickup to delivery. If specified, it must be nonnegative, and the
    /// shipment must contain at least a pickup and a delivery.
    ///
    /// For example, let t be the shortest time taken to go from the selected
    /// pickup alternative directly to the selected delivery alternative. Then
    /// setting `pickup_to_delivery_relative_detour_limit` enforces:
    ///
    /// ```
    /// start_time(delivery) - start_time(pickup) <=
    /// std::ceil(t * (1.0 + pickup_to_delivery_relative_detour_limit))
    /// ```
    ///
    /// If both relative and absolute limits are specified on the same shipment,
    /// the more constraining limit is used for each possible pickup/delivery pair.
    /// As of 2017/10, detours are only supported when travel durations do not
    /// depend on vehicles.
    #[prost(double, optional, tag = "8")]
    pub pickup_to_delivery_relative_detour_limit: ::core::option::Option<f64>,
    /// Specifies the maximum absolute detour time compared to the shortest path
    /// from pickup to delivery. If specified, it must be nonnegative, and the
    /// shipment must contain at least a pickup and a delivery.
    ///
    /// For example, let t be the shortest time taken to go from the selected
    /// pickup alternative directly to the selected delivery alternative. Then
    /// setting `pickup_to_delivery_absolute_detour_limit` enforces:
    ///
    /// ```
    /// start_time(delivery) - start_time(pickup) <=
    /// t + pickup_to_delivery_absolute_detour_limit
    /// ```
    ///
    /// If both relative and absolute limits are specified on the same shipment,
    /// the more constraining limit is used for each possible pickup/delivery pair.
    /// As of 2017/10, detours are only supported when travel durations do not
    /// depend on vehicles.
    #[prost(message, optional, tag = "9")]
    pub pickup_to_delivery_absolute_detour_limit: ::core::option::Option<
        ::prost_types::Duration,
    >,
    /// Specifies the maximum duration from start of pickup to start of delivery of
    /// a shipment. If specified, it must be nonnegative, and the shipment must
    /// contain at least a pickup and a delivery. This does not depend on which
    /// alternatives are selected for pickup and delivery, nor on vehicle speed.
    /// This can be specified alongside maximum detour constraints: the solution
    /// will respect both specifications.
    #[prost(message, optional, tag = "10")]
    pub pickup_to_delivery_time_limit: ::core::option::Option<::prost_types::Duration>,
    /// Non-empty string specifying a "type" for this shipment.
    /// This feature can be used to define incompatibilities or requirements
    /// between `shipment_types` (see `shipment_type_incompatibilities` and
    /// `shipment_type_requirements` in `ShipmentModel`).
    ///
    /// Differs from `visit_types` which is specified for a single visit: All
    /// pickup/deliveries belonging to the same shipment share the same
    /// `shipment_type`.
    #[prost(string, tag = "11")]
    pub shipment_type: ::prost::alloc::string::String,
    /// Specifies a label for this shipment. This label is reported in the response
    /// in the `shipment_label` of the corresponding
    /// \[ShipmentRoute.Visit][google.cloud.optimization.v1.ShipmentRoute.Visit\].
    #[prost(string, tag = "12")]
    pub label: ::prost::alloc::string::String,
    /// If true, skip this shipment, but don't apply a `penalty_cost`.
    ///
    /// Ignoring a shipment results in a validation error when there are any
    /// `shipment_type_requirements` in the model.
    ///
    /// Ignoring a shipment that is performed in `injected_first_solution_routes`
    /// or `injected_solution_constraint` is permitted; the solver removes the
    /// related pickup/delivery visits from the performing route.
    /// `precedence_rules` that reference ignored shipments will also be ignored.
    #[prost(bool, tag = "13")]
    pub ignore: bool,
    /// Deprecated: Use
    /// \[Shipment.load_demands][google.cloud.optimization.v1.Shipment.load_demands\]
    /// instead.
    #[deprecated]
    #[prost(message, repeated, tag = "3")]
    pub demands: ::prost::alloc::vec::Vec<CapacityQuantity>,
}
/// Nested message and enum types in `Shipment`.
pub mod shipment {
    /// Request for a visit which can be done by a vehicle: it has a geo-location
    /// (or two, see below), opening and closing times represented by time windows,
    /// and a service duration time (time spent by the vehicle once it has arrived
    /// to pickup or drop off goods).
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VisitRequest {
        /// The geo-location where the vehicle arrives when performing this
        /// `VisitRequest`. If the shipment model has duration distance matrices,
        /// `arrival_location` must not be specified.
        #[prost(message, optional, tag = "1")]
        pub arrival_location: ::core::option::Option<
            super::super::super::super::r#type::LatLng,
        >,
        /// The waypoint where the vehicle arrives when performing this
        /// `VisitRequest`. If the shipment model has duration distance matrices,
        /// `arrival_waypoint` must not be specified.
        #[prost(message, optional, tag = "2")]
        pub arrival_waypoint: ::core::option::Option<super::Waypoint>,
        /// The geo-location where the vehicle departs after completing this
        /// `VisitRequest`. Can be omitted if it is the same as `arrival_location`.
        /// If the shipment model has duration distance matrices,
        /// `departure_location` must not be specified.
        #[prost(message, optional, tag = "3")]
        pub departure_location: ::core::option::Option<
            super::super::super::super::r#type::LatLng,
        >,
        /// The waypoint where the vehicle departs after completing this
        /// `VisitRequest`. Can be omitted if it is the same as `arrival_waypoint`.
        /// If the shipment model has duration distance matrices,
        /// `departure_waypoint` must not be specified.
        #[prost(message, optional, tag = "4")]
        pub departure_waypoint: ::core::option::Option<super::Waypoint>,
        /// Specifies tags attached to the visit request.
        /// Empty or duplicate strings are not allowed.
        #[prost(string, repeated, tag = "5")]
        pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Time windows which constrain the arrival time at a visit.
        /// Note that a vehicle may depart outside of the arrival time window, i.e.
        /// arrival time + duration do not need to be inside a time window. This can
        /// result in waiting time if the vehicle arrives before
        /// \[TimeWindow.start_time][google.cloud.optimization.v1.TimeWindow.start_time\].
        ///
        /// The absence of `TimeWindow` means that the vehicle can perform this visit
        /// at any time.
        ///
        /// Time windows must be disjoint, i.e. no time window must overlap with or
        /// be adjacent to another, and they must be in increasing order.
        ///
        /// `cost_per_hour_after_soft_end_time` and `soft_end_time` can only
        /// be set if there is a single time window.
        #[prost(message, repeated, tag = "6")]
        pub time_windows: ::prost::alloc::vec::Vec<super::TimeWindow>,
        /// Duration of the visit, i.e. time spent by the vehicle between arrival
        /// and departure (to be added to the possible waiting time; see
        /// `time_windows`).
        #[prost(message, optional, tag = "7")]
        pub duration: ::core::option::Option<::prost_types::Duration>,
        /// Cost to service this visit request on a vehicle route. This can be used
        /// to pay different costs for each alternative pickup or delivery of a
        /// shipment. This cost must be in the same unit as `Shipment.penalty_cost`
        /// and must not be negative.
        #[prost(double, tag = "8")]
        pub cost: f64,
        /// Load demands of this visit request. This is just like
        /// \[Shipment.load_demands][google.cloud.optimization.v1.Shipment.load_demands\]
        /// field, except that it only applies to this
        /// \[VisitRequest][google.cloud.optimization.v1.Shipment.VisitRequest\]
        /// instead of the whole \[Shipment][google.cloud.optimization.v1.Shipment\].
        /// The demands listed here are added to the demands listed in
        /// \[Shipment.load_demands][google.cloud.optimization.v1.Shipment.load_demands\].
        #[prost(btree_map = "string, message", tag = "12")]
        pub load_demands: ::prost::alloc::collections::BTreeMap<
            ::prost::alloc::string::String,
            Load,
        >,
        /// Specifies the types of the visit. This may be used to allocate additional
        /// time required for a vehicle to complete this visit (see
        /// \[Vehicle.extra_visit_duration_for_visit_type][google.cloud.optimization.v1.Vehicle.extra_visit_duration_for_visit_type\]).
        ///
        /// A type can only appear once.
        #[prost(string, repeated, tag = "10")]
        pub visit_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Specifies a label for this `VisitRequest`. This label is reported in the
        /// response as `visit_label` in the corresponding
        /// \[ShipmentRoute.Visit][google.cloud.optimization.v1.ShipmentRoute.Visit\].
        #[prost(string, tag = "11")]
        pub label: ::prost::alloc::string::String,
        /// Deprecated: Use
        /// \[VisitRequest.load_demands][google.cloud.optimization.v1.Shipment.VisitRequest.load_demands\]
        /// instead.
        #[deprecated]
        #[prost(message, repeated, tag = "9")]
        pub demands: ::prost::alloc::vec::Vec<super::CapacityQuantity>,
    }
    /// When performing a visit, a predefined amount may be added to the vehicle
    /// load if it's a pickup, or subtracted if it's a delivery. This message
    /// defines such amount. See
    /// \[load_demands][google.cloud.optimization.v1.Shipment.load_demands\].
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Load {
        /// The amount by which the load of the vehicle performing the corresponding
        /// visit will vary. Since it is an integer, users are advised to choose an
        /// appropriate unit to avoid loss of precision. Must be ≥ 0.
        #[prost(int64, tag = "2")]
        pub amount: i64,
    }
}
/// Specifies incompatibilties between shipments depending on their
/// shipment_type. The appearance of incompatible shipments on the same route is
/// restricted based on the incompatibility mode.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShipmentTypeIncompatibility {
    /// List of incompatible types. Two shipments having different `shipment_types`
    /// among those listed are "incompatible".
    #[prost(string, repeated, tag = "1")]
    pub types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Mode applied to the incompatibility.
    #[prost(
        enumeration = "shipment_type_incompatibility::IncompatibilityMode",
        tag = "2"
    )]
    pub incompatibility_mode: i32,
}
/// Nested message and enum types in `ShipmentTypeIncompatibility`.
pub mod shipment_type_incompatibility {
    /// Modes defining how the appearance of incompatible shipments are restricted
    /// on the same route.
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
    pub enum IncompatibilityMode {
        /// Unspecified incompatibility mode. This value should never be used.
        Unspecified = 0,
        /// In this mode, two shipments with incompatible types can never share the
        /// same vehicle.
        NotPerformedBySameVehicle = 1,
        /// For two shipments with incompatible types with the
        /// `NOT_IN_SAME_VEHICLE_SIMULTANEOUSLY` incompatibility mode:
        ///
        /// * If both are pickups only (no deliveries) or deliveries only (no
        ///    pickups), they cannot share the same vehicle at all.
        /// * If one of the shipments has a delivery and the other a pickup, the two
        ///    shipments can share the same vehicle iff the former shipment is
        ///    delivered before the latter is picked up.
        NotInSameVehicleSimultaneously = 2,
    }
    impl IncompatibilityMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                IncompatibilityMode::Unspecified => "INCOMPATIBILITY_MODE_UNSPECIFIED",
                IncompatibilityMode::NotPerformedBySameVehicle => {
                    "NOT_PERFORMED_BY_SAME_VEHICLE"
                }
                IncompatibilityMode::NotInSameVehicleSimultaneously => {
                    "NOT_IN_SAME_VEHICLE_SIMULTANEOUSLY"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INCOMPATIBILITY_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "NOT_PERFORMED_BY_SAME_VEHICLE" => Some(Self::NotPerformedBySameVehicle),
                "NOT_IN_SAME_VEHICLE_SIMULTANEOUSLY" => {
                    Some(Self::NotInSameVehicleSimultaneously)
                }
                _ => None,
            }
        }
    }
}
/// Specifies requirements between shipments based on their shipment_type.
/// The specifics of the requirement are defined by the requirement mode.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShipmentTypeRequirement {
    /// List of alternative shipment types required by the
    /// `dependent_shipment_types`.
    #[prost(string, repeated, tag = "1")]
    pub required_shipment_type_alternatives: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// All shipments with a type in the `dependent_shipment_types` field require
    /// at least one shipment of type `required_shipment_type_alternatives` to be
    /// visited on the same route.
    ///
    /// NOTE: Chains of requirements such that a `shipment_type` depends on itself
    /// are not allowed.
    #[prost(string, repeated, tag = "2")]
    pub dependent_shipment_types: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Mode applied to the requirement.
    #[prost(enumeration = "shipment_type_requirement::RequirementMode", tag = "3")]
    pub requirement_mode: i32,
}
/// Nested message and enum types in `ShipmentTypeRequirement`.
pub mod shipment_type_requirement {
    /// Modes defining the appearance of dependent shipments on a route.
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
    pub enum RequirementMode {
        /// Unspecified requirement mode. This value should never be used.
        Unspecified = 0,
        /// In this mode, all "dependent" shipments must share the same vehicle as at
        /// least one of their "required" shipments.
        PerformedBySameVehicle = 1,
        /// With the `IN_SAME_VEHICLE_AT_PICKUP_TIME` mode, all "dependent"
        /// shipments need to have at least one "required" shipment on their vehicle
        /// at the time of their pickup.
        ///
        /// A "dependent" shipment pickup must therefore have either:
        ///
        /// * A delivery-only "required" shipment delivered on the route after, or
        /// * A "required" shipment picked up on the route before it, and if the
        ///    "required" shipment has a delivery, this delivery must be performed
        ///    after the "dependent" shipment's pickup.
        InSameVehicleAtPickupTime = 2,
        /// Same as before, except the "dependent" shipments need to have a
        /// "required" shipment on their vehicle at the time of their *delivery*.
        InSameVehicleAtDeliveryTime = 3,
    }
    impl RequirementMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RequirementMode::Unspecified => "REQUIREMENT_MODE_UNSPECIFIED",
                RequirementMode::PerformedBySameVehicle => "PERFORMED_BY_SAME_VEHICLE",
                RequirementMode::InSameVehicleAtPickupTime => {
                    "IN_SAME_VEHICLE_AT_PICKUP_TIME"
                }
                RequirementMode::InSameVehicleAtDeliveryTime => {
                    "IN_SAME_VEHICLE_AT_DELIVERY_TIME"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REQUIREMENT_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "PERFORMED_BY_SAME_VEHICLE" => Some(Self::PerformedBySameVehicle),
                "IN_SAME_VEHICLE_AT_PICKUP_TIME" => Some(Self::InSameVehicleAtPickupTime),
                "IN_SAME_VEHICLE_AT_DELIVERY_TIME" => {
                    Some(Self::InSameVehicleAtDeliveryTime)
                }
                _ => None,
            }
        }
    }
}
/// Models a vehicle in a shipment problem. Solving a shipment problem will
/// build a route starting from `start_location` and ending at `end_location`
/// for this vehicle. A route is a sequence of visits (see `ShipmentRoute`).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vehicle {
    /// The travel mode which affects the roads usable by the vehicle and its
    /// speed. See also `travel_duration_multiple`.
    #[prost(enumeration = "vehicle::TravelMode", tag = "1")]
    pub travel_mode: i32,
    /// Geographic location where the vehicle starts before picking up any
    /// shipments. If not specified, the vehicle starts at its first pickup.
    /// If the shipment model has duration and distance matrices, `start_location`
    /// must not be specified.
    #[prost(message, optional, tag = "3")]
    pub start_location: ::core::option::Option<super::super::super::r#type::LatLng>,
    /// Waypoint representing a geographic location where the vehicle starts before
    /// picking up any shipments. If neither `start_waypoint` nor `start_location`
    /// is specified, the vehicle starts at its first pickup.
    /// If the shipment model has duration and distance matrices, `start_waypoint`
    /// must not be specified.
    #[prost(message, optional, tag = "4")]
    pub start_waypoint: ::core::option::Option<Waypoint>,
    /// Geographic location where the vehicle ends after it has completed its last
    /// `VisitRequest`. If not specified the vehicle's `ShipmentRoute` ends
    /// immediately when it completes its last `VisitRequest`.
    /// If the shipment model has duration and distance matrices, `end_location`
    /// must not be specified.
    #[prost(message, optional, tag = "5")]
    pub end_location: ::core::option::Option<super::super::super::r#type::LatLng>,
    /// Waypoint representing a geographic location where the vehicle ends after
    /// it has completed its last `VisitRequest`. If neither `end_waypoint` nor
    /// `end_location` is specified, the vehicle's `ShipmentRoute` ends immediately
    /// when it completes its last `VisitRequest`.
    /// If the shipment model has duration and distance matrices, `end_waypoint`
    /// must not be specified.
    #[prost(message, optional, tag = "6")]
    pub end_waypoint: ::core::option::Option<Waypoint>,
    /// Specifies tags attached to the start of the vehicle's route.
    ///
    /// Empty or duplicate strings are not allowed.
    #[prost(string, repeated, tag = "7")]
    pub start_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Specifies tags attached to the end of the vehicle's route.
    ///
    /// Empty or duplicate strings are not allowed.
    #[prost(string, repeated, tag = "8")]
    pub end_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Time windows during which the vehicle may depart its start location.
    /// They must be within the global time limits (see
    /// \[ShipmentModel.global_*][google.cloud.optimization.v1.ShipmentModel.global_start_time\]
    /// fields). If unspecified, there is no limitation besides those global time
    /// limits.
    ///
    /// Time windows belonging to the same repeated field must be disjoint, i.e. no
    /// time window can overlap with or be adjacent to another, and they must be in
    /// chronological order.
    ///
    /// `cost_per_hour_after_soft_end_time` and `soft_end_time` can only be set if
    /// there is a single time window.
    #[prost(message, repeated, tag = "9")]
    pub start_time_windows: ::prost::alloc::vec::Vec<TimeWindow>,
    /// Time windows during which the vehicle may arrive at its end location.
    /// They must be within the global time limits (see
    /// \[ShipmentModel.global_*][google.cloud.optimization.v1.ShipmentModel.global_start_time\]
    /// fields). If unspecified, there is no limitation besides those global time
    /// limits.
    ///
    /// Time windows belonging to the same repeated field must be disjoint, i.e. no
    /// time window can overlap with or be adjacent to another, and they must be in
    /// chronological order.
    ///
    /// `cost_per_hour_after_soft_end_time` and `soft_end_time` can only be set if
    /// there is a single time window.
    #[prost(message, repeated, tag = "10")]
    pub end_time_windows: ::prost::alloc::vec::Vec<TimeWindow>,
    /// Specifies a multiplicative factor that can be used to increase or decrease
    /// travel times of this vehicle. For example, setting this to 2.0 means
    /// that this vehicle is slower and has travel times that are twice what they
    /// are for standard vehicles. This multiple does not affect visit durations.
    /// It does affect cost if `cost_per_hour` or `cost_per_traveled_hour` are
    /// specified. This must be in the range [0.001, 1000.0]. If unset, the vehicle
    /// is standard, and this multiple is considered 1.0.
    ///
    /// WARNING: Travel times will be rounded to the nearest second after this
    /// multiple is applied but before performing any numerical operations, thus,
    /// a small multiple may result in a loss of precision.
    ///
    /// See also `extra_visit_duration_for_visit_type` below.
    #[prost(double, optional, tag = "11")]
    pub travel_duration_multiple: ::core::option::Option<f64>,
    /// Unloading policy enforced on the vehicle.
    #[prost(enumeration = "vehicle::UnloadingPolicy", tag = "12")]
    pub unloading_policy: i32,
    /// Capacities of the vehicle (weight, volume, # of pallets for example).
    /// The keys in the map are the identifiers of the type of load, consistent
    /// with the keys of the
    /// \[Shipment.load_demands][google.cloud.optimization.v1.Shipment.load_demands\]
    /// field. If a given key is absent from this map, the corresponding capacity
    /// is considered to be limitless.
    #[prost(btree_map = "string, message", tag = "30")]
    pub load_limits: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        vehicle::LoadLimit,
    >,
    /// Vehicle costs: all costs add up and must be in the same unit as
    /// \[Shipment.penalty_cost][google.cloud.optimization.v1.Shipment.penalty_cost\].
    ///
    /// Cost per hour of the vehicle route. This cost is applied to the total time
    /// taken by the route, and includes travel time, waiting time, and visit time.
    /// Using `cost_per_hour` instead of just `cost_per_traveled_hour` may result
    /// in additional latency.
    #[prost(double, tag = "16")]
    pub cost_per_hour: f64,
    /// Cost per traveled hour of the vehicle route. This cost is applied only to
    /// travel time taken by the route (i.e., that reported in
    /// \[ShipmentRoute.transitions][google.cloud.optimization.v1.ShipmentRoute.transitions\]),
    /// and excludes waiting time and visit time.
    #[prost(double, tag = "17")]
    pub cost_per_traveled_hour: f64,
    /// Cost per kilometer of the vehicle route. This cost is applied to the
    /// distance reported in the
    /// \[ShipmentRoute.transitions][google.cloud.optimization.v1.ShipmentRoute.transitions\]
    /// and does not apply to any distance implicitly traveled from the
    /// `arrival_location` to the `departure_location` of a single `VisitRequest`.
    #[prost(double, tag = "18")]
    pub cost_per_kilometer: f64,
    /// Fixed cost applied if this vehicle is used to handle a shipment.
    #[prost(double, tag = "19")]
    pub fixed_cost: f64,
    /// This field only applies to vehicles when their route does not serve any
    /// shipments. It indicates if the vehicle should be considered as used or not
    /// in this case.
    ///
    /// If true, the vehicle goes from its start to its end location even if it
    /// doesn't serve any shipments, and time and distance costs resulting from its
    /// start --> end travel are taken into account.
    ///
    /// Otherwise, it doesn't travel from its start to its end location, and no
    /// `break_rule` or delay (from `TransitionAttributes`) are scheduled for this
    /// vehicle. In this case, the vehicle's `ShipmentRoute` doesn't contain any
    /// information except for the vehicle index and label.
    #[prost(bool, tag = "20")]
    pub used_if_route_is_empty: bool,
    /// Limit applied to the total duration of the vehicle's route. In a given
    /// `OptimizeToursResponse`, the route duration of a vehicle is the
    /// difference between its `vehicle_end_time` and `vehicle_start_time`.
    #[prost(message, optional, tag = "21")]
    pub route_duration_limit: ::core::option::Option<vehicle::DurationLimit>,
    /// Limit applied to the travel duration of the vehicle's route. In a given
    /// `OptimizeToursResponse`, the route travel duration is the sum of all its
    /// \[transitions.travel_duration][google.cloud.optimization.v1.ShipmentRoute.Transition.travel_duration\].
    #[prost(message, optional, tag = "22")]
    pub travel_duration_limit: ::core::option::Option<vehicle::DurationLimit>,
    /// Limit applied to the total distance of the vehicle's route. In a given
    /// `OptimizeToursResponse`, the route distance is the sum of all its
    /// \[transitions.travel_distance_meters][google.cloud.optimization.v1.ShipmentRoute.Transition.travel_distance_meters\].
    #[prost(message, optional, tag = "23")]
    pub route_distance_limit: ::core::option::Option<DistanceLimit>,
    /// Specifies a map from visit_types strings to durations. The duration is time
    /// in addition to
    /// \[VisitRequest.duration][google.cloud.optimization.v1.Shipment.VisitRequest.duration\]
    /// to be taken at visits with the specified `visit_types`. This extra visit
    /// duration adds cost if `cost_per_hour` is specified. Keys (i.e.
    /// `visit_types`) cannot be empty strings.
    ///
    /// If a visit request has multiple types, a duration will be added for each
    /// type in the map.
    #[prost(btree_map = "string, message", tag = "24")]
    pub extra_visit_duration_for_visit_type: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost_types::Duration,
    >,
    /// Describes the break schedule to be enforced on this vehicle.
    /// If empty, no breaks will be scheduled for this vehicle.
    #[prost(message, optional, tag = "25")]
    pub break_rule: ::core::option::Option<BreakRule>,
    /// Specifies a label for this vehicle. This label is reported in the response
    /// as the `vehicle_label` of the corresponding
    /// \[ShipmentRoute][google.cloud.optimization.v1.ShipmentRoute\].
    #[prost(string, tag = "27")]
    pub label: ::prost::alloc::string::String,
    /// If true, `used_if_route_is_empty` must be false, and this vehicle will
    /// remain unused.
    ///
    /// If a shipment is performed by an ignored vehicle in
    /// `injected_first_solution_routes`, it is skipped in the first solution but
    /// is free to be performed in the response.
    ///
    /// If a shipment is performed by an ignored vehicle in
    /// `injected_solution_constraint` and any related pickup/delivery is
    /// constrained to remain on the vehicle (i.e., not relaxed to level
    /// `RELAX_ALL_AFTER_THRESHOLD`), it is skipped in the response.
    /// If a shipment has a non-empty `allowed_vehicle_indices` field and all of
    /// the allowed vehicles are ignored, it is skipped in the response.
    #[prost(bool, tag = "28")]
    pub ignore: bool,
    /// Deprecated: No longer used.
    /// Indices in the `break_rule` field in the source
    /// \[ShipmentModel][google.cloud.optimization.v1.ShipmentModel\]. They
    /// correspond to break rules enforced on the vehicle.
    ///
    /// As of 2018/03, at most one rule index per vehicle can be specified.
    #[deprecated]
    #[prost(int32, repeated, packed = "false", tag = "29")]
    pub break_rule_indices: ::prost::alloc::vec::Vec<i32>,
    /// Deprecated: Use
    /// \[Vehicle.load_limits][google.cloud.optimization.v1.Vehicle.load_limits\]
    /// instead.
    #[deprecated]
    #[prost(message, repeated, tag = "13")]
    pub capacities: ::prost::alloc::vec::Vec<CapacityQuantity>,
    /// Deprecated: Use
    /// \[Vehicle.LoadLimit.start_load_interval][google.cloud.optimization.v1.Vehicle.LoadLimit.start_load_interval\]
    /// instead.
    #[deprecated]
    #[prost(message, repeated, tag = "14")]
    pub start_load_intervals: ::prost::alloc::vec::Vec<CapacityQuantityInterval>,
    /// Deprecated: Use
    /// \[Vehicle.LoadLimit.end_load_interval][google.cloud.optimization.v1.Vehicle.LoadLimit.end_load_interval\]
    /// instead.
    #[deprecated]
    #[prost(message, repeated, tag = "15")]
    pub end_load_intervals: ::prost::alloc::vec::Vec<CapacityQuantityInterval>,
}
/// Nested message and enum types in `Vehicle`.
pub mod vehicle {
    /// Defines a load limit applying to a vehicle, e.g. "this truck may only
    /// carry up to 3500 kg". See
    /// \[load_limits][google.cloud.optimization.v1.Vehicle.load_limits\].
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LoadLimit {
        /// The maximum acceptable amount of load.
        #[prost(int64, optional, tag = "1")]
        pub max_load: ::core::option::Option<i64>,
        /// A soft limit of the load. See
        /// \[cost_per_unit_above_soft_max][google.cloud.optimization.v1.Vehicle.LoadLimit.cost_per_unit_above_soft_max\].
        #[prost(int64, tag = "2")]
        pub soft_max_load: i64,
        /// If the load ever exceeds
        /// \[soft_max_load][google.cloud.optimization.v1.Vehicle.LoadLimit.soft_max_load\]
        /// along this vehicle's route, the following cost penalty applies (only once
        /// per vehicle): (load -
        /// \[soft_max_load][google.cloud.optimization.v1.Vehicle.LoadLimit.soft_max_load\])
        /// * \[cost_per_unit_above_soft_max][google.cloud.optimization.v1.Vehicle.LoadLimit.cost_per_unit_above_soft_max\]. All costs
        /// add up and must be in the same unit as
        /// \[Shipment.penalty_cost][google.cloud.optimization.v1.Shipment.penalty_cost\].
        #[prost(double, tag = "3")]
        pub cost_per_unit_above_soft_max: f64,
        /// The acceptable load interval of the vehicle at the start of the route.
        #[prost(message, optional, tag = "4")]
        pub start_load_interval: ::core::option::Option<load_limit::Interval>,
        /// The acceptable load interval of the vehicle at the end of the route.
        #[prost(message, optional, tag = "5")]
        pub end_load_interval: ::core::option::Option<load_limit::Interval>,
    }
    /// Nested message and enum types in `LoadLimit`.
    pub mod load_limit {
        /// Interval of acceptable load amounts.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Interval {
            /// A minimum acceptable load. Must be ≥ 0.
            /// If they're both specified,
            /// \[min][google.cloud.optimization.v1.Vehicle.LoadLimit.Interval.min\] must
            /// be ≤
            /// \[max][google.cloud.optimization.v1.Vehicle.LoadLimit.Interval.max\].
            #[prost(int64, tag = "1")]
            pub min: i64,
            /// A maximum acceptable load. Must be ≥ 0. If unspecified, the maximum
            /// load is unrestricted by this message.
            /// If they're both specified,
            /// \[min][google.cloud.optimization.v1.Vehicle.LoadLimit.Interval.min\] must
            /// be ≤
            /// \[max][google.cloud.optimization.v1.Vehicle.LoadLimit.Interval.max\].
            #[prost(int64, optional, tag = "2")]
            pub max: ::core::option::Option<i64>,
        }
    }
    /// A limit defining a maximum duration of the route of a vehicle. It can be
    /// either hard or soft.
    ///
    /// When a soft limit field is defined, both the soft max threshold and its
    /// associated cost must be defined together.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DurationLimit {
        /// A hard limit constraining the duration to be at most max_duration.
        #[prost(message, optional, tag = "1")]
        pub max_duration: ::core::option::Option<::prost_types::Duration>,
        /// A soft limit not enforcing a maximum duration limit, but when violated
        /// makes the route incur a cost. This cost adds up to other costs defined in
        /// the model, with the same unit.
        ///
        /// If defined, `soft_max_duration` must be nonnegative. If max_duration is
        /// also defined, `soft_max_duration` must be less than max_duration.
        #[prost(message, optional, tag = "2")]
        pub soft_max_duration: ::core::option::Option<::prost_types::Duration>,
        /// Cost per hour incurred if the `soft_max_duration` threshold is violated.
        /// The additional cost is 0 if the duration is under the threshold,
        /// otherwise the cost depends on the duration as follows:
        /// ```
        ///    cost_per_hour_after_soft_max * (duration - soft_max_duration)
        /// ```
        /// The cost must be nonnegative.
        #[prost(double, optional, tag = "3")]
        pub cost_per_hour_after_soft_max: ::core::option::Option<f64>,
        /// A soft limit not enforcing a maximum duration limit, but when violated
        /// makes the route incur a cost, quadratic in the duration. This cost adds
        /// up to other costs defined in the model, with the same unit.
        ///
        /// If defined, `quadratic_soft_max_duration` must be nonnegative. If
        /// `max_duration` is also defined, `quadratic_soft_max_duration` must be
        /// less than `max_duration`, and the difference must be no larger than one
        /// day:
        ///
        ///     `max_duration - quadratic_soft_max_duration <= 86400 seconds`
        #[prost(message, optional, tag = "4")]
        pub quadratic_soft_max_duration: ::core::option::Option<::prost_types::Duration>,
        /// Cost per square hour incurred if the
        /// `quadratic_soft_max_duration` threshold is violated.
        ///
        /// The additional cost is 0 if the duration is under the threshold,
        /// otherwise the cost depends on the duration as follows:
        ///
        /// ```
        ///    cost_per_square_hour_after_quadratic_soft_max *
        ///    (duration - quadratic_soft_max_duration)^2
        /// ```
        ///
        /// The cost must be nonnegative.
        #[prost(double, optional, tag = "5")]
        pub cost_per_square_hour_after_quadratic_soft_max: ::core::option::Option<f64>,
    }
    /// Travel modes which can be used by vehicles.
    ///
    /// These should be a subset of the Google Maps Platform Routes Preferred API
    /// travel modes, see:
    /// <https://developers.google.com/maps/documentation/routes_preferred/reference/rest/Shared.Types/RouteTravelMode.>
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
    pub enum TravelMode {
        /// Unspecified travel mode, equivalent to `DRIVING`.
        Unspecified = 0,
        /// Travel mode corresponding to driving directions (car, ...).
        Driving = 1,
    }
    impl TravelMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TravelMode::Unspecified => "TRAVEL_MODE_UNSPECIFIED",
                TravelMode::Driving => "DRIVING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TRAVEL_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "DRIVING" => Some(Self::Driving),
                _ => None,
            }
        }
    }
    /// Policy on how a vehicle can be unloaded. Applies only to shipments having
    /// both a pickup and a delivery.
    ///
    /// Other shipments are free to occur anywhere on the route independent of
    /// `unloading_policy`.
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
    pub enum UnloadingPolicy {
        /// Unspecified unloading policy; deliveries must just occur after their
        /// corresponding pickups.
        Unspecified = 0,
        /// Deliveries must occur in reverse order of pickups
        LastInFirstOut = 1,
        /// Deliveries must occur in the same order as pickups
        FirstInFirstOut = 2,
    }
    impl UnloadingPolicy {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                UnloadingPolicy::Unspecified => "UNLOADING_POLICY_UNSPECIFIED",
                UnloadingPolicy::LastInFirstOut => "LAST_IN_FIRST_OUT",
                UnloadingPolicy::FirstInFirstOut => "FIRST_IN_FIRST_OUT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNLOADING_POLICY_UNSPECIFIED" => Some(Self::Unspecified),
                "LAST_IN_FIRST_OUT" => Some(Self::LastInFirstOut),
                "FIRST_IN_FIRST_OUT" => Some(Self::FirstInFirstOut),
                _ => None,
            }
        }
    }
}
/// Time windows constrain the time of an event, such as the arrival time at a
/// visit, or the start and end time of a vehicle.
///
/// Hard time window bounds, `start_time` and `end_time`, enforce the earliest
/// and latest time of the event, such that `start_time <= event_time <=
/// end_time`. The soft time window lower bound, `soft_start_time`, expresses a
/// preference for the event to happen at or after `soft_start_time` by incurring
/// a cost proportional to how long before soft_start_time the event occurs. The
/// soft time window upper bound, `soft_end_time`, expresses a preference for the
/// event to happen at or before `soft_end_time` by incurring a cost proportional
/// to how long after `soft_end_time` the event occurs. `start_time`, `end_time`,
/// `soft_start_time` and `soft_end_time` should be within the global time limits
/// (see
/// \[ShipmentModel.global_start_time][google.cloud.optimization.v1.ShipmentModel.global_start_time\]
/// and
/// \[ShipmentModel.global_end_time][google.cloud.optimization.v1.ShipmentModel.global_end_time\])
/// and should respect:
/// ```
///    0 <= `start_time` <= `soft_start_time` <= `end_time` and
///    0 <= `start_time` <= `soft_end_time` <= `end_time`.
/// ```
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeWindow {
    /// The hard time window start time. If unspecified it will be set to
    /// `ShipmentModel.global_start_time`.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The hard time window end time. If unspecified it will be set to
    /// `ShipmentModel.global_end_time`.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The soft start time of the time window.
    #[prost(message, optional, tag = "3")]
    pub soft_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The soft end time of the time window.
    #[prost(message, optional, tag = "4")]
    pub soft_end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// A cost per hour added to other costs in the model if the event occurs
    /// before soft_start_time, computed as:
    ///
    /// ```
    ///     max(0, soft_start_time - t.seconds)
    ///                            * cost_per_hour_before_soft_start_time / 3600,
    /// t being the time of the event.
    /// ```
    ///
    /// This cost must be positive, and the field can only be set if
    /// soft_start_time has been set.
    #[prost(double, optional, tag = "5")]
    pub cost_per_hour_before_soft_start_time: ::core::option::Option<f64>,
    /// A cost per hour added to other costs in the model if the event occurs after
    /// `soft_end_time`, computed as:
    ///
    /// ```
    ///     max(0, t.seconds - soft_end_time.seconds)
    ///                      * cost_per_hour_after_soft_end_time / 3600,
    /// t being the time of the event.
    /// ```
    ///
    /// This cost must be positive, and the field can only be set if
    /// `soft_end_time` has been set.
    #[prost(double, optional, tag = "6")]
    pub cost_per_hour_after_soft_end_time: ::core::option::Option<f64>,
}
/// Deprecated: Use
/// \[Vehicle.LoadLimit.Interval][google.cloud.optimization.v1.Vehicle.LoadLimit.Interval\]
/// instead.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapacityQuantity {
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub value: i64,
}
/// Deprecated: Use
/// \[Vehicle.LoadLimit.Interval][google.cloud.optimization.v1.Vehicle.LoadLimit.Interval\]
/// instead.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapacityQuantityInterval {
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(int64, optional, tag = "2")]
    pub min_value: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "3")]
    pub max_value: ::core::option::Option<i64>,
}
/// A limit defining a maximum distance which can be traveled. It can be either
/// hard or soft.
///
/// If a soft limit is defined, both `soft_max_meters` and
/// `cost_per_kilometer_above_soft_max` must be defined and be nonnegative.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DistanceLimit {
    /// A hard limit constraining the distance to be at most max_meters. The limit
    /// must be nonnegative.
    #[prost(int64, optional, tag = "1")]
    pub max_meters: ::core::option::Option<i64>,
    /// A soft limit not enforcing a maximum distance limit, but when violated
    /// results in a cost which adds up to other costs defined in the model,
    /// with the same unit.
    ///
    /// If defined soft_max_meters must be less than max_meters and must be
    /// nonnegative.
    #[prost(int64, optional, tag = "2")]
    pub soft_max_meters: ::core::option::Option<i64>,
    /// Cost per kilometer incurred if distance is above `soft_max_meters` limit.
    /// The additional cost is 0 if the distance is under the limit, otherwise the
    /// formula used to compute the cost is the following:
    /// ```
    ///    (distance_meters - soft_max_meters) / 1000.0 *
    ///    cost_per_kilometer_above_soft_max.
    /// ```
    /// The cost must be nonnegative.
    #[prost(double, optional, tag = "3")]
    pub cost_per_kilometer_above_soft_max: ::core::option::Option<f64>,
}
/// Specifies attributes of transitions between two consecutive visits on a
/// route. Several `TransitionAttributes` may apply to the same transition: in
/// that case, all extra costs add up and the strictest constraint or limit
/// applies (following natural "AND" semantics).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransitionAttributes {
    /// Tags defining the set of (src->dst) transitions these attributes apply to.
    ///
    /// A source visit or vehicle start matches iff its
    /// \[VisitRequest.tags][google.cloud.optimization.v1.Shipment.VisitRequest.tags\]
    /// or \[Vehicle.start_tags][google.cloud.optimization.v1.Vehicle.start_tags\]
    /// either contains `src_tag` or does not contain `excluded_src_tag` (depending
    /// on which of these two fields is non-empty).
    #[prost(string, tag = "1")]
    pub src_tag: ::prost::alloc::string::String,
    /// See `src_tag`. Exactly one of `src_tag` and `excluded_src_tag` must be
    /// non-empty.
    #[prost(string, tag = "2")]
    pub excluded_src_tag: ::prost::alloc::string::String,
    /// A destination visit or vehicle end matches iff its
    /// \[VisitRequest.tags][google.cloud.optimization.v1.Shipment.VisitRequest.tags\]
    /// or \[Vehicle.end_tags][google.cloud.optimization.v1.Vehicle.end_tags\] either
    /// contains `dst_tag` or does not contain `excluded_dst_tag` (depending on
    /// which of these two fields is non-empty).
    #[prost(string, tag = "3")]
    pub dst_tag: ::prost::alloc::string::String,
    /// See `dst_tag`. Exactly one of `dst_tag` and `excluded_dst_tag` must be
    /// non-empty.
    #[prost(string, tag = "4")]
    pub excluded_dst_tag: ::prost::alloc::string::String,
    /// Specifies a cost for performing this transition. This is in the same unit
    /// as all other costs in the model and must not be negative. It is applied on
    /// top of all other existing costs.
    #[prost(double, tag = "5")]
    pub cost: f64,
    /// Specifies a cost per kilometer applied to the distance traveled while
    /// performing this transition. It adds up to any
    /// \[Vehicle.cost_per_kilometer][google.cloud.optimization.v1.Vehicle.cost_per_kilometer\]
    /// specified on vehicles.
    #[prost(double, tag = "6")]
    pub cost_per_kilometer: f64,
    /// Specifies a limit on the distance traveled while performing this
    /// transition.
    ///
    /// As of 2021/06, only soft limits are supported.
    #[prost(message, optional, tag = "7")]
    pub distance_limit: ::core::option::Option<DistanceLimit>,
    /// Specifies a delay incurred when performing this transition.
    ///
    /// This delay always occurs *after* finishing the source visit and *before*
    /// starting the destination visit.
    #[prost(message, optional, tag = "8")]
    pub delay: ::core::option::Option<::prost_types::Duration>,
}
/// Encapsulates a waypoint. Waypoints mark arrival and departure locations of
/// VisitRequests, and start and end locations of Vehicles.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Waypoint {
    /// Indicates that the location of this waypoint is meant to have a preference
    /// for the vehicle to stop at a particular side of road. When you set this
    /// value, the route will pass through the location so that the vehicle can
    /// stop at the side of road that the location is biased towards from the
    /// center of the road. This option works only for the 'DRIVING' travel mode,
    /// and when the 'location_type' is set to 'location'.
    #[prost(bool, tag = "3")]
    pub side_of_road: bool,
    /// Different ways to represent a location.
    #[prost(oneof = "waypoint::LocationType", tags = "1, 2")]
    pub location_type: ::core::option::Option<waypoint::LocationType>,
}
/// Nested message and enum types in `Waypoint`.
pub mod waypoint {
    /// Different ways to represent a location.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LocationType {
        /// A point specified using geographic coordinates, including an optional
        /// heading.
        #[prost(message, tag = "1")]
        Location(super::Location),
        /// The POI Place ID associated with the waypoint.
        #[prost(string, tag = "2")]
        PlaceId(::prost::alloc::string::String),
    }
}
/// Encapsulates a location (a geographic point, and an optional heading).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Location {
    /// The waypoint's geographic coordinates.
    #[prost(message, optional, tag = "1")]
    pub lat_lng: ::core::option::Option<super::super::super::r#type::LatLng>,
    /// The compass heading associated with the direction of the flow of traffic.
    /// This value is used to specify the side of the road to use for pickup and
    /// drop-off. Heading values can be from 0 to 360, where 0 specifies a heading
    /// of due North, 90 specifies a heading of due East, etc.
    #[prost(int32, optional, tag = "2")]
    pub heading: ::core::option::Option<i32>,
}
/// Rules to generate time breaks for a vehicle (e.g. lunch breaks). A break
/// is a contiguous period of time during which the vehicle remains idle at its
/// current position and cannot perform any visit. A break may occur:
///
/// * during the travel between two visits (which includes the time right
///    before or right after a visit, but not in the middle of a visit), in
///    which case it extends the corresponding transit time between the visits,
/// * or before the vehicle start (the vehicle may not start in the middle of
///    a break), in which case it does not affect the vehicle start time.
/// * or after the vehicle end (ditto, with the vehicle end time).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BreakRule {
    /// Sequence of breaks. See the `BreakRequest` message.
    #[prost(message, repeated, tag = "1")]
    pub break_requests: ::prost::alloc::vec::Vec<break_rule::BreakRequest>,
    /// Several `FrequencyConstraint` may apply. They must all be satisfied by
    /// the `BreakRequest`s of this `BreakRule`. See `FrequencyConstraint`.
    #[prost(message, repeated, tag = "2")]
    pub frequency_constraints: ::prost::alloc::vec::Vec<break_rule::FrequencyConstraint>,
}
/// Nested message and enum types in `BreakRule`.
pub mod break_rule {
    /// The sequence of breaks (i.e. their number and order) that apply to each
    /// vehicle must be known beforehand. The repeated `BreakRequest`s define
    /// that sequence, in the order in which they must occur. Their time windows
    /// (`earliest_start_time` / `latest_start_time`) may overlap, but they must
    /// be compatible with the order (this is checked).
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BreakRequest {
        /// Required. Lower bound (inclusive) on the start of the break.
        #[prost(message, optional, tag = "1")]
        pub earliest_start_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Required. Upper bound (inclusive) on the start of the break.
        #[prost(message, optional, tag = "2")]
        pub latest_start_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Required. Minimum duration of the break. Must be positive.
        #[prost(message, optional, tag = "3")]
        pub min_duration: ::core::option::Option<::prost_types::Duration>,
    }
    /// One may further constrain the frequency and duration of the breaks
    /// specified above, by enforcing a minimum break frequency, such as
    /// "There must be a break of at least 1 hour every 12 hours". Assuming that
    /// this can be interpreted as "Within any sliding time window of 12h, there
    /// must be at least one break of at least one hour", that example would
    /// translate to the following `FrequencyConstraint`:
    /// ```
    /// {
    ///     min_break_duration { seconds: 3600 }         # 1 hour.
    ///     max_inter_break_duration { seconds: 39600 }  # 11 hours (12 - 1 = 11).
    /// }
    /// ```
    ///
    /// The timing and duration of the breaks in the solution will respect all
    /// such constraints, in addition to the time windows and minimum durations
    /// already specified in the `BreakRequest`.
    ///
    /// A `FrequencyConstraint` may in practice apply to non-consecutive breaks.
    /// For example, the following schedule honors the "1h every 12h" example:
    /// ```
    ///    04:00 vehicle start
    ///     .. performing travel and visits ..
    ///    09:00 1 hour break
    ///    10:00 end of the break
    ///     .. performing travel and visits ..
    ///    12:00 20-min lunch break
    ///    12:20 end of the break
    ///     .. performing travel and visits ..
    ///    21:00 1 hour break
    ///    22:00 end of the break
    ///     .. performing travel and visits ..
    ///    23:59 vehicle end
    /// ```
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FrequencyConstraint {
        /// Required. Minimum break duration for this constraint. Nonnegative.
        /// See description of `FrequencyConstraint`.
        #[prost(message, optional, tag = "1")]
        pub min_break_duration: ::core::option::Option<::prost_types::Duration>,
        /// Required. Maximum allowed span of any interval of time in the route that
        /// does not include at least partially a break of `duration >=
        /// min_break_duration`. Must be positive.
        #[prost(message, optional, tag = "2")]
        pub max_inter_break_duration: ::core::option::Option<::prost_types::Duration>,
    }
}
/// A vehicle's route can be decomposed, along the time axis, like this (we
/// assume there are n visits):
/// ```
///    |            |            |          |       |  T\[2\], |        |      |
///    | Transition |  Visit #0  |          |       |  V\[2\], |        |      |
///    |     #0     |    aka     |   T\[1\]   |  V\[1\] |  ...   | V\[n-1\] | T\[n\] |
///    |  aka T\[0\]  |    V\[0\]    |          |       | V\[n-2\],|        |      |
///    |            |            |          |       | T\[n-1\] |        |      |
///    ^            ^            ^          ^       ^        ^        ^      ^
/// vehicle    V\[0\].start   V\[0\].end     V\[1\].   V\[1\].    V\[n\].    V\[n\]. vehicle
///   start     (arrival)   (departure)   start   end      start    end     end
/// ```
/// Note that we make a difference between:
///
/// * "punctual events", such as the vehicle start and end and each visit's start
///    and end (aka arrival and departure). They happen at a given second.
/// * "time intervals", such as the visits themselves, and the transition between
///    visits. Though time intervals can sometimes have zero duration, i.e. start
///    and end at the same second, they often have a positive duration.
///
/// Invariants:
///
/// * If there are n visits, there are n+1 transitions.
/// * A visit is always surrounded by a transition before it (same index) and a
///    transition after it (index + 1).
/// * The vehicle start is always followed by transition #0.
/// * The vehicle end is always preceded by transition #n.
///
/// Zooming in, here is what happens during a `Transition` and a `Visit`:
/// ```
/// ---+-------------------------------------+-----------------------------+-->
///     |           TRANSITION\[i\]             |           VISIT\[i\]          |
///     |                                     |                             |
///     |  * TRAVEL: the vehicle moves from   |      PERFORM the visit:     |
///     |    VISIT\[i-1\].departure_location to |                             |
///     |    VISIT\[i\].arrival_location, which |  * Spend some time:         |
///     |    takes a given travel duration    |    the "visit duration".    |
///     |    and distance                     |                             |
///     |                                     |  * Load or unload           |
///     |  * BREAKS: the driver may have      |    some quantities from the |
///     |    breaks (e.g. lunch break).       |    vehicle: the "demand".   |
///     |                                     |                             |
///     |  * WAIT: the driver/vehicle does    |                             |
///     |    nothing. This can happen for     |                             |
///     |    many reasons, for example when   |                             |
///     |    the vehicle reaches the next     |                             |
///     |    event's destination before the   |                             |
///     |    start of its time window         |                             |
///     |                                     |                             |
///     |  * DELAY: *right before* the next   |                             |
///     |    arrival. E.g. the vehicle and/or |                             |
///     |    driver spends time unloading.    |                             |
///     |                                     |                             |
/// ---+-------------------------------------+-----------------------------+-->
///     ^                                     ^                             ^
/// V\[i-1\].end                           V\[i\].start                    V\[i\].end
/// ```
/// Lastly, here is how the TRAVEL, BREAKS, DELAY and WAIT can be arranged
/// during a transition.
///
/// * They don't overlap.
/// * The DELAY is unique and *must* be a contiguous period of time right
///    before the next visit (or vehicle end). Thus, it suffice to know the
///    delay duration to know its start and end time.
/// * The BREAKS are contiguous, non-overlapping periods of time. The
///    response specifies the start time and duration of each break.
/// * TRAVEL and WAIT are "preemptable": they can be interrupted several times
///    during this transition. Clients can assume that travel happens "as soon as
///    possible" and that "wait" fills the remaining time.
///
/// A (complex) example:
/// ```
///                                 TRANSITION\[i\]
/// --++-----+-----------------------------------------------------------++-->
///    ||     |       |           |       |           |         |         ||
///    ||  T  |   B   |     T     |       |     B     |         |    D    ||
///    ||  r  |   r   |     r     |   W   |     r     |    W    |    e    ||
///    ||  a  |   e   |     a     |   a   |     e     |    a    |    l    ||
///    ||  v  |   a   |     v     |   i   |     a     |    i    |    a    ||
///    ||  e  |   k   |     e     |   t   |     k     |    t    |    y    ||
///    ||  l  |       |     l     |       |           |         |         ||
///    ||     |       |           |       |           |         |         ||
/// --++-----------------------------------------------------------------++-->
/// ```
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShipmentRoute {
    /// Vehicle performing the route, identified by its index in the source
    /// `ShipmentModel`.
    #[prost(int32, tag = "1")]
    pub vehicle_index: i32,
    /// Label of the vehicle performing this route, equal to
    /// `ShipmentModel.vehicles(vehicle_index).label`, if specified.
    #[prost(string, tag = "2")]
    pub vehicle_label: ::prost::alloc::string::String,
    /// Time at which the vehicle starts its route.
    #[prost(message, optional, tag = "5")]
    pub vehicle_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Time at which the vehicle finishes its route.
    #[prost(message, optional, tag = "6")]
    pub vehicle_end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Ordered sequence of visits representing a route.
    /// visits\[i\] is the i-th visit in the route.
    /// If this field is empty, the vehicle is considered as unused.
    #[prost(message, repeated, tag = "7")]
    pub visits: ::prost::alloc::vec::Vec<shipment_route::Visit>,
    /// Ordered list of transitions for the route.
    #[prost(message, repeated, tag = "8")]
    pub transitions: ::prost::alloc::vec::Vec<shipment_route::Transition>,
    /// When
    /// \[OptimizeToursRequest.consider_road_traffic][google.cloud.optimization.v1.OptimizeToursRequest.consider_road_traffic\],
    /// is set to true, this field indicates that inconsistencies in route timings
    /// are predicted using traffic-based travel duration estimates. There may be
    /// insufficient time to complete traffic-adjusted travel, delays, and breaks
    /// between visits, before the first visit, or after the last visit, while
    /// still satisfying the visit and vehicle time windows. For example,
    ///
    /// ```
    ///    start_time(previous_visit) + duration(previous_visit) +
    ///    travel_duration(previous_visit, next_visit) > start_time(next_visit)
    /// ```
    ///
    /// Arrival at next_visit will likely happen later than its current
    /// time window due the increased estimate of travel time
    /// `travel_duration(previous_visit, next_visit)` due to traffic. Also, a break
    /// may be forced to overlap with a visit due to an increase in travel time
    /// estimates and visit or break time window restrictions.
    #[prost(bool, tag = "9")]
    pub has_traffic_infeasibilities: bool,
    /// The encoded polyline representation of the route.
    /// This field is only populated if
    /// \[OptimizeToursRequest.populate_polylines][google.cloud.optimization.v1.OptimizeToursRequest.populate_polylines\]
    /// is set to true.
    #[prost(message, optional, tag = "10")]
    pub route_polyline: ::core::option::Option<shipment_route::EncodedPolyline>,
    /// Breaks scheduled for the vehicle performing this route.
    /// The `breaks` sequence represents time intervals, each starting at the
    /// corresponding `start_time` and lasting `duration` seconds.
    #[prost(message, repeated, tag = "11")]
    pub breaks: ::prost::alloc::vec::Vec<shipment_route::Break>,
    /// Duration, distance and load metrics for this route. The fields of
    /// \[AggregatedMetrics][google.cloud.optimization.v1.AggregatedMetrics\] are
    /// summed over all
    /// \[ShipmentRoute.transitions][google.cloud.optimization.v1.ShipmentRoute.transitions\]
    /// or
    /// \[ShipmentRoute.visits][google.cloud.optimization.v1.ShipmentRoute.visits\],
    /// depending on the context.
    #[prost(message, optional, tag = "12")]
    pub metrics: ::core::option::Option<AggregatedMetrics>,
    /// Cost of the route, broken down by cost-related request fields.
    /// The keys are proto paths, relative to the input OptimizeToursRequest, e.g.
    /// "model.shipments.pickups.cost", and the values are the total cost
    /// generated by the corresponding cost field, aggregated over the whole route.
    /// In other words, costs\["model.shipments.pickups.cost"\] is the sum of all
    /// pickup costs over the route. All costs defined in the model are reported in
    /// detail here with the exception of costs related to TransitionAttributes
    /// that are only reported in an aggregated way as of 2022/01.
    #[prost(btree_map = "string, double", tag = "17")]
    pub route_costs: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        f64,
    >,
    /// Total cost of the route. The sum of all costs in the cost map.
    #[prost(double, tag = "18")]
    pub route_total_cost: f64,
    /// Deprecated: Use
    /// \[Transition.vehicle_loads][google.cloud.optimization.v1.ShipmentRoute.Transition.vehicle_loads\]
    /// instead. Vehicle loads upon arrival at its end location, for each type
    /// specified in
    /// \[Vehicle.capacities][google.cloud.optimization.v1.Vehicle.capacities\],
    /// `start_load_intervals`, `end_load_intervals` or demands. Exception: we omit
    /// loads for quantity types unconstrained by intervals and that don't have any
    /// non-zero demand on the route.
    #[deprecated]
    #[prost(message, repeated, tag = "13")]
    pub end_loads: ::prost::alloc::vec::Vec<CapacityQuantity>,
    /// Deprecated: Use
    /// \[ShipmentRoute.transitions][google.cloud.optimization.v1.ShipmentRoute.transitions\]
    /// instead. Ordered list of travel steps for the route.
    #[deprecated]
    #[prost(message, repeated, tag = "14")]
    pub travel_steps: ::prost::alloc::vec::Vec<shipment_route::TravelStep>,
    /// Deprecated: No longer used.
    /// This field will only be populated at the
    /// \[ShipmentRoute.Visit][google.cloud.optimization.v1.ShipmentRoute.Visit\]
    /// level.
    ///
    /// This field is the extra detour time due to the shipments visited on the
    /// route.
    ///
    /// It is equal to `vehicle_end_time` - `vehicle_start_time` - travel duration
    /// from the vehicle's start_location to its `end_location`.
    #[deprecated]
    #[prost(message, optional, tag = "15")]
    pub vehicle_detour: ::core::option::Option<::prost_types::Duration>,
    /// Deprecated: Delay occurring before the vehicle end. See
    /// \[TransitionAttributes.delay][google.cloud.optimization.v1.TransitionAttributes.delay\].
    #[deprecated]
    #[prost(message, optional, tag = "16")]
    pub delay_before_vehicle_end: ::core::option::Option<shipment_route::Delay>,
}
/// Nested message and enum types in `ShipmentRoute`.
pub mod shipment_route {
    /// Deprecated: Use
    /// \[ShipmentRoute.Transition.delay_duration][google.cloud.optimization.v1.ShipmentRoute.Transition.delay_duration\]
    /// instead. Time interval spent on the route resulting from a
    /// \[TransitionAttributes.delay][google.cloud.optimization.v1.TransitionAttributes.delay\].
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Delay {
        /// Start of the delay.
        #[prost(message, optional, tag = "1")]
        pub start_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Duration of the delay.
        #[prost(message, optional, tag = "2")]
        pub duration: ::core::option::Option<::prost_types::Duration>,
    }
    /// A visit performed during a route. This visit corresponds to a pickup or a
    /// delivery of a `Shipment`.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Visit {
        /// Index of the `shipments` field in the source
        /// \[ShipmentModel][google.cloud.optimization.v1.ShipmentModel\].
        #[prost(int32, tag = "1")]
        pub shipment_index: i32,
        /// If true the visit corresponds to a pickup of a `Shipment`. Otherwise, it
        /// corresponds to a delivery.
        #[prost(bool, tag = "2")]
        pub is_pickup: bool,
        /// Index of `VisitRequest` in either the pickup or delivery field of the
        /// `Shipment` (see `is_pickup`).
        #[prost(int32, tag = "3")]
        pub visit_request_index: i32,
        /// Time at which the visit starts. Note that the vehicle may arrive earlier
        /// than this at the visit location. Times are consistent with the
        /// `ShipmentModel`.
        #[prost(message, optional, tag = "4")]
        pub start_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Total visit load demand as the sum of the shipment and the visit request
        /// `load_demands`. The values are negative if the visit is a delivery.
        /// Demands are reported for the same types as the
        /// \[Transition.loads][google.cloud.optimization.v1.ShipmentRoute.Transition\]
        /// (see this field).
        #[prost(btree_map = "string, message", tag = "11")]
        pub load_demands: ::prost::alloc::collections::BTreeMap<
            ::prost::alloc::string::String,
            super::shipment::Load,
        >,
        /// Extra detour time due to the shipments visited on the route before the
        /// visit and to the potential waiting time induced by time windows.
        /// If the visit is a delivery, the detour is computed from the corresponding
        /// pickup visit and is equal to:
        /// ```
        /// start_time(delivery) - start_time(pickup)
        /// - (duration(pickup) + travel duration from the pickup location
        /// to the delivery location).
        /// ```
        /// Otherwise, it is computed from the vehicle `start_location` and is equal
        /// to:
        /// ```
        /// start_time - vehicle_start_time - travel duration from
        /// the vehicle's `start_location` to the visit.
        /// ```
        #[prost(message, optional, tag = "6")]
        pub detour: ::core::option::Option<::prost_types::Duration>,
        /// Copy of the corresponding `Shipment.label`, if specified in the
        /// `Shipment`.
        #[prost(string, tag = "7")]
        pub shipment_label: ::prost::alloc::string::String,
        /// Copy of the corresponding
        /// \[VisitRequest.label][google.cloud.optimization.v1.Shipment.VisitRequest.label\],
        /// if specified in the `VisitRequest`.
        #[prost(string, tag = "8")]
        pub visit_label: ::prost::alloc::string::String,
        /// Deprecated: Use
        /// \[Transition.vehicle_loads][google.cloud.optimization.v1.ShipmentRoute.Transition.vehicle_loads\]
        /// instead. Vehicle loads upon arrival at the visit location, for each type
        /// specified in
        /// \[Vehicle.capacities][google.cloud.optimization.v1.Vehicle.capacities\],
        /// `start_load_intervals`, `end_load_intervals` or `demands`.
        ///
        /// Exception: we omit loads for quantity types unconstrained by intervals
        /// and that don't have any non-zero demand on the route.
        #[deprecated]
        #[prost(message, repeated, tag = "9")]
        pub arrival_loads: ::prost::alloc::vec::Vec<super::CapacityQuantity>,
        /// Deprecated: Use
        /// \[ShipmentRoute.Transition.delay_duration][google.cloud.optimization.v1.ShipmentRoute.Transition.delay_duration\]
        /// instead. Delay occurring before the visit starts.
        #[deprecated]
        #[prost(message, optional, tag = "10")]
        pub delay_before_start: ::core::option::Option<Delay>,
        /// Deprecated: Use
        /// \[Visit.load_demands][google.cloud.optimization.v1.ShipmentRoute.Visit.load_demands\]
        /// instead.
        #[deprecated]
        #[prost(message, repeated, tag = "5")]
        pub demands: ::prost::alloc::vec::Vec<super::CapacityQuantity>,
    }
    /// Transition between two events on the route. See the description of
    /// \[ShipmentRoute][google.cloud.optimization.v1.ShipmentRoute\].
    ///
    /// If the vehicle does not have a `start_location` and/or `end_location`, the
    /// corresponding travel metrics are 0.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Transition {
        /// Travel duration during this transition.
        #[prost(message, optional, tag = "1")]
        pub travel_duration: ::core::option::Option<::prost_types::Duration>,
        /// Distance traveled during the transition.
        #[prost(double, tag = "2")]
        pub travel_distance_meters: f64,
        /// When traffic is requested via
        /// \[OptimizeToursRequest.consider_road_traffic\]
        /// \[google.cloud.optimization.v1.OptimizeToursRequest.consider_road_traffic\],
        /// and the traffic info couldn't be retrieved for a `Transition`, this
        /// boolean is set to true. This may be temporary (rare hiccup in the
        /// realtime traffic servers) or permanent (no data for this location).
        #[prost(bool, tag = "3")]
        pub traffic_info_unavailable: bool,
        /// Sum of the delay durations applied to this transition. If any, the delay
        /// starts exactly `delay_duration` seconds before the next event (visit or
        /// vehicle end). See
        /// \[TransitionAttributes.delay][google.cloud.optimization.v1.TransitionAttributes.delay\].
        #[prost(message, optional, tag = "4")]
        pub delay_duration: ::core::option::Option<::prost_types::Duration>,
        /// Sum of the duration of the breaks occurring during this transition, if
        /// any. Details about each break's start time and duration are stored in
        /// \[ShipmentRoute.breaks][google.cloud.optimization.v1.ShipmentRoute.breaks\].
        #[prost(message, optional, tag = "5")]
        pub break_duration: ::core::option::Option<::prost_types::Duration>,
        /// Time spent waiting during this transition. Wait duration corresponds to
        /// idle time and does not include break time. Also note that this wait time
        /// may be split into several non-contiguous intervals.
        #[prost(message, optional, tag = "6")]
        pub wait_duration: ::core::option::Option<::prost_types::Duration>,
        /// Total duration of the transition, provided for convenience. It is equal
        /// to:
        ///
        /// * next visit `start_time` (or `vehicle_end_time` if this is the last
        /// transition) - this transition's `start_time`;
        /// * if `ShipmentRoute.has_traffic_infeasibilities` is false, the following
        /// additionally holds: `total_duration = travel_duration + delay_duration
        /// + break_duration + wait_duration`.
        #[prost(message, optional, tag = "7")]
        pub total_duration: ::core::option::Option<::prost_types::Duration>,
        /// Start time of this transition.
        #[prost(message, optional, tag = "8")]
        pub start_time: ::core::option::Option<::prost_types::Timestamp>,
        /// The encoded polyline representation of the route followed during the
        /// transition.
        /// This field is only populated if \[populate_transition_polylines\]
        /// \[google.cloud.optimization.v1.OptimizeToursRequest.populate_transition_polylines\]
        /// is set to true.
        #[prost(message, optional, tag = "9")]
        pub route_polyline: ::core::option::Option<EncodedPolyline>,
        /// Vehicle loads during this transition, for each type that either appears
        /// in this vehicle's
        /// \[Vehicle.load_limits][google.cloud.optimization.v1.Vehicle.load_limits\],
        /// or that have non-zero
        /// \[Shipment.load_demands][google.cloud.optimization.v1.Shipment.load_demands\]
        /// on some shipment performed on this route.
        ///
        /// The loads during the first transition are the starting loads of the
        /// vehicle route. Then, after each visit, the visit's `load_demands` are
        /// either added or subtracted to get the next transition's loads, depending
        /// on whether the visit was a pickup or a delivery.
        #[prost(btree_map = "string, message", tag = "11")]
        pub vehicle_loads: ::prost::alloc::collections::BTreeMap<
            ::prost::alloc::string::String,
            VehicleLoad,
        >,
        /// Deprecated: Use
        /// \[Transition.vehicle_loads][google.cloud.optimization.v1.ShipmentRoute.Transition.vehicle_loads\]
        /// instead.
        #[deprecated]
        #[prost(message, repeated, tag = "10")]
        pub loads: ::prost::alloc::vec::Vec<super::CapacityQuantity>,
    }
    /// Reports the actual load of the vehicle at some point along the route,
    /// for a given type (see
    /// \[Transition.vehicle_loads][google.cloud.optimization.v1.ShipmentRoute.Transition.vehicle_loads\]).
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VehicleLoad {
        /// The amount of load on the vehicle, for the given type. The unit of load
        /// is usually indicated by the type. See
        /// \[Transition.vehicle_loads][google.cloud.optimization.v1.ShipmentRoute.Transition.vehicle_loads\].
        #[prost(int64, tag = "1")]
        pub amount: i64,
    }
    /// The encoded representation of a polyline. More information on polyline
    /// encoding can be found here:
    /// <https://developers.google.com/maps/documentation/utilities/polylinealgorithm>
    /// <https://developers.google.com/maps/documentation/javascript/reference/geometry#encoding.>
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EncodedPolyline {
        /// String representing encoded points of the polyline.
        #[prost(string, tag = "1")]
        pub points: ::prost::alloc::string::String,
    }
    /// Data representing the execution of a break.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Break {
        /// Start time of a break.
        #[prost(message, optional, tag = "1")]
        pub start_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Duration of a break.
        #[prost(message, optional, tag = "2")]
        pub duration: ::core::option::Option<::prost_types::Duration>,
    }
    /// Deprecated: Use
    /// \[ShipmentRoute.Transition][google.cloud.optimization.v1.ShipmentRoute.Transition\]
    /// instead. Travel between each visit along the route: from the vehicle's
    /// `start_location` to the first visit's `arrival_location`, then from the
    /// first visit's `departure_location` to the second visit's
    /// `arrival_location`, and so on until the vehicle's `end_location`. This
    /// accounts only for the actual travel between visits, not counting the
    /// waiting time, the time spent performing a visit, nor the distance covered
    /// during a visit.
    ///
    /// Invariant: `travel_steps_size() == visits_size() + 1`.
    ///
    /// If the vehicle does not have a start_ and/or end_location, the
    /// corresponding travel metrics are 0 and/or empty.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TravelStep {
        /// Duration of the travel step.
        #[prost(message, optional, tag = "1")]
        pub duration: ::core::option::Option<::prost_types::Duration>,
        /// Distance traveled during the step.
        #[prost(double, tag = "2")]
        pub distance_meters: f64,
        /// When traffic is requested via
        /// \[OptimizeToursRequest.consider_road_traffic][google.cloud.optimization.v1.OptimizeToursRequest.consider_road_traffic\],
        /// and the traffic info couldn't be retrieved for a TravelStep, this boolean
        /// is set to true. This may be temporary (rare hiccup in the realtime
        /// traffic servers) or permanent (no data for this location).
        #[prost(bool, tag = "3")]
        pub traffic_info_unavailable: bool,
        /// The encoded polyline representation of the route followed during the
        /// step.
        ///
        /// This field is only populated if
        /// \[OptimizeToursRequest.populate_travel_step_polylines][google.cloud.optimization.v1.OptimizeToursRequest.populate_travel_step_polylines\]
        /// is set to true.
        #[prost(message, optional, tag = "4")]
        pub route_polyline: ::core::option::Option<EncodedPolyline>,
    }
}
/// Specifies details of unperformed shipments in a solution. For trivial cases
/// and/or if we are able to identify the cause for skipping, we report the
/// reason here.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SkippedShipment {
    /// The index corresponds to the index of the shipment in the source
    /// `ShipmentModel`.
    #[prost(int32, tag = "1")]
    pub index: i32,
    /// Copy of the corresponding
    /// \[Shipment.label][google.cloud.optimization.v1.Shipment.label\], if specified
    /// in the `Shipment`.
    #[prost(string, tag = "2")]
    pub label: ::prost::alloc::string::String,
    /// A list of reasons that explain why the shipment was skipped. See comment
    /// above `Reason`.
    #[prost(message, repeated, tag = "3")]
    pub reasons: ::prost::alloc::vec::Vec<skipped_shipment::Reason>,
}
/// Nested message and enum types in `SkippedShipment`.
pub mod skipped_shipment {
    /// If we can explain why the shipment was skipped, reasons will be listed
    /// here. If the reason is not the same for all vehicles, `reason` will have
    /// more than 1 element. A skipped shipment cannot have duplicate reasons,
    /// i.e. where all fields are the same except for `example_vehicle_index`.
    /// Example:
    /// ```
    /// reasons {
    ///    code: DEMAND_EXCEEDS_VEHICLE_CAPACITY
    ///    example_vehicle_index: 1
    ///    example_exceeded_capacity_type: "Apples"
    /// }
    /// reasons {
    ///    code: DEMAND_EXCEEDS_VEHICLE_CAPACITY
    ///    example_vehicle_index: 3
    ///    example_exceeded_capacity_type: "Pears"
    /// }
    /// reasons {
    ///    code: CANNOT_BE_PERFORMED_WITHIN_VEHICLE_DISTANCE_LIMIT
    ///    example_vehicle_index: 1
    /// }
    /// ```
    /// The skipped shipment is incompatible with all vehicles. The reasons may
    /// be different for all vehicles but at least one vehicle's "Apples"
    /// capacity would be exceeded (including vehicle 1), at least one vehicle's
    /// "Pears" capacity would be exceeded (including vehicle 3) and at least one
    /// vehicle's distance limit would be exceeded (including vehicle 1).
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Reason {
        /// Refer to the comments of Code.
        #[prost(enumeration = "reason::Code", tag = "1")]
        pub code: i32,
        /// If the reason is related to a shipment-vehicle incompatibility, this
        /// field provides the index of one relevant vehicle.
        #[prost(int32, optional, tag = "2")]
        pub example_vehicle_index: ::core::option::Option<i32>,
        /// If the reason code is `DEMAND_EXCEEDS_VEHICLE_CAPACITY`, documents one
        /// capacity type that is exceeded.
        #[prost(string, tag = "3")]
        pub example_exceeded_capacity_type: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `Reason`.
    pub mod reason {
        /// Code identifying the reason type. The order here is meaningless. In
        /// particular, it gives no indication of whether a given reason will
        /// appear before another in the solution, if both apply.
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
        pub enum Code {
            /// This should never be used. If we are unable to understand why a
            /// shipment was skipped, we simply return an empty set of reasons.
            Unspecified = 0,
            /// There is no vehicle in the model making all shipments infeasible.
            NoVehicle = 1,
            /// The demand of the shipment exceeds a vehicle's capacity for some
            /// capacity types, one of which is `example_exceeded_capacity_type`.
            DemandExceedsVehicleCapacity = 2,
            /// The minimum distance necessary to perform this shipment, i.e. from
            /// the vehicle's `start_location` to the shipment's pickup and/or delivery
            /// locations and to the vehicle's end location exceeds the vehicle's
            /// `route_distance_limit`.
            ///
            /// Note that for this computation we use the geodesic distances.
            CannotBePerformedWithinVehicleDistanceLimit = 3,
            /// The minimum time necessary to perform this shipment, including travel
            /// time, wait time and service time exceeds the vehicle's
            /// `route_duration_limit`.
            ///
            /// Note: travel time is computed in the best-case scenario, namely as
            /// geodesic distance x 36 m/s (roughly 130 km/hour).
            CannotBePerformedWithinVehicleDurationLimit = 4,
            /// Same as above but we only compare minimum travel time and the
            /// vehicle's `travel_duration_limit`.
            CannotBePerformedWithinVehicleTravelDurationLimit = 5,
            /// The vehicle cannot perform this shipment in the best-case scenario
            /// (see `CANNOT_BE_PERFORMED_WITHIN_VEHICLE_DURATION_LIMIT` for time
            /// computation) if it starts at its earliest start time: the total time
            /// would make the vehicle end after its latest end time.
            CannotBePerformedWithinVehicleTimeWindows = 6,
            /// The `allowed_vehicle_indices` field of the shipment is not empty and
            /// this vehicle does not belong to it.
            VehicleNotAllowed = 7,
        }
        impl Code {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Code::Unspecified => "CODE_UNSPECIFIED",
                    Code::NoVehicle => "NO_VEHICLE",
                    Code::DemandExceedsVehicleCapacity => {
                        "DEMAND_EXCEEDS_VEHICLE_CAPACITY"
                    }
                    Code::CannotBePerformedWithinVehicleDistanceLimit => {
                        "CANNOT_BE_PERFORMED_WITHIN_VEHICLE_DISTANCE_LIMIT"
                    }
                    Code::CannotBePerformedWithinVehicleDurationLimit => {
                        "CANNOT_BE_PERFORMED_WITHIN_VEHICLE_DURATION_LIMIT"
                    }
                    Code::CannotBePerformedWithinVehicleTravelDurationLimit => {
                        "CANNOT_BE_PERFORMED_WITHIN_VEHICLE_TRAVEL_DURATION_LIMIT"
                    }
                    Code::CannotBePerformedWithinVehicleTimeWindows => {
                        "CANNOT_BE_PERFORMED_WITHIN_VEHICLE_TIME_WINDOWS"
                    }
                    Code::VehicleNotAllowed => "VEHICLE_NOT_ALLOWED",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "CODE_UNSPECIFIED" => Some(Self::Unspecified),
                    "NO_VEHICLE" => Some(Self::NoVehicle),
                    "DEMAND_EXCEEDS_VEHICLE_CAPACITY" => {
                        Some(Self::DemandExceedsVehicleCapacity)
                    }
                    "CANNOT_BE_PERFORMED_WITHIN_VEHICLE_DISTANCE_LIMIT" => {
                        Some(Self::CannotBePerformedWithinVehicleDistanceLimit)
                    }
                    "CANNOT_BE_PERFORMED_WITHIN_VEHICLE_DURATION_LIMIT" => {
                        Some(Self::CannotBePerformedWithinVehicleDurationLimit)
                    }
                    "CANNOT_BE_PERFORMED_WITHIN_VEHICLE_TRAVEL_DURATION_LIMIT" => {
                        Some(Self::CannotBePerformedWithinVehicleTravelDurationLimit)
                    }
                    "CANNOT_BE_PERFORMED_WITHIN_VEHICLE_TIME_WINDOWS" => {
                        Some(Self::CannotBePerformedWithinVehicleTimeWindows)
                    }
                    "VEHICLE_NOT_ALLOWED" => Some(Self::VehicleNotAllowed),
                    _ => None,
                }
            }
        }
    }
}
/// Aggregated metrics for
/// \[ShipmentRoute][google.cloud.optimization.v1.ShipmentRoute\] (resp. for
/// \[OptimizeToursResponse][google.cloud.optimization.v1.OptimizeToursResponse\]
/// over all \[Transition][google.cloud.optimization.v1.ShipmentRoute.Transition\]
/// and/or \[Visit][google.cloud.optimization.v1.ShipmentRoute.Visit\] (resp. over
/// all \[ShipmentRoute][google.cloud.optimization.v1.ShipmentRoute\]) elements.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregatedMetrics {
    /// Number of shipments performed. Note that a pickup and delivery pair only
    /// counts once.
    #[prost(int32, tag = "1")]
    pub performed_shipment_count: i32,
    /// Total travel duration for a route or a solution.
    #[prost(message, optional, tag = "2")]
    pub travel_duration: ::core::option::Option<::prost_types::Duration>,
    /// Total wait duration for a route or a solution.
    #[prost(message, optional, tag = "3")]
    pub wait_duration: ::core::option::Option<::prost_types::Duration>,
    /// Total delay duration for a route or a solution.
    #[prost(message, optional, tag = "4")]
    pub delay_duration: ::core::option::Option<::prost_types::Duration>,
    /// Total break duration for a route or a solution.
    #[prost(message, optional, tag = "5")]
    pub break_duration: ::core::option::Option<::prost_types::Duration>,
    /// Total visit duration for a route or a solution.
    #[prost(message, optional, tag = "6")]
    pub visit_duration: ::core::option::Option<::prost_types::Duration>,
    /// The total duration should be equal to the sum of all durations above.
    /// For routes, it also corresponds to
    /// \[ShipmentRoute.vehicle_end_time][google.cloud.optimization.v1.ShipmentRoute.vehicle_end_time\]
    /// -
    /// \[ShipmentRoute.vehicle_start_time][google.cloud.optimization.v1.ShipmentRoute.vehicle_start_time\].
    #[prost(message, optional, tag = "7")]
    pub total_duration: ::core::option::Option<::prost_types::Duration>,
    /// Total travel distance for a route or a solution.
    #[prost(double, tag = "8")]
    pub travel_distance_meters: f64,
    /// Maximum load achieved over the entire route (resp. solution), for each of
    /// the quantities on this route (resp. solution), computed as the maximum over
    /// all
    /// \[Transition.vehicle_loads][google.cloud.optimization.v1.ShipmentRoute.Transition.vehicle_loads\]
    /// (resp.
    /// \[ShipmentRoute.metrics.max_loads][google.cloud.optimization.v1.AggregatedMetrics.max_loads\].
    #[prost(btree_map = "string, message", tag = "9")]
    pub max_loads: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        shipment_route::VehicleLoad,
    >,
    /// Deprecated: Use
    /// \[ShipmentRoute.route_costs][google.cloud.optimization.v1.ShipmentRoute.route_costs\]
    /// and
    /// \[OptimizeToursResponse.Metrics.costs][google.cloud.optimization.v1.OptimizeToursResponse.Metrics.costs\]
    /// instead.
    #[prost(btree_map = "string, double", tag = "10")]
    pub costs: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        f64,
    >,
    /// Deprecated: Use
    /// \[ShipmentRoute.route_total_cost][google.cloud.optimization.v1.ShipmentRoute.route_total_cost\]
    /// and
    /// \[OptimizeToursResponse.Metrics.total_cost][google.cloud.optimization.v1.OptimizeToursResponse.Metrics.total_cost\]
    /// instead.
    #[deprecated]
    #[prost(double, tag = "11")]
    pub total_cost: f64,
}
/// Solution injected in the request including information about which visits
/// must be constrained and how they must be constrained.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InjectedSolutionConstraint {
    /// Routes of the solution to inject. Some routes may be omitted from the
    /// original solution. The routes and skipped shipments must satisfy the basic
    /// validity assumptions listed for `injected_first_solution_routes`.
    #[prost(message, repeated, tag = "1")]
    pub routes: ::prost::alloc::vec::Vec<ShipmentRoute>,
    /// Skipped shipments of the solution to inject. Some may be omitted from the
    /// original solution. See the `routes` field.
    #[prost(message, repeated, tag = "2")]
    pub skipped_shipments: ::prost::alloc::vec::Vec<SkippedShipment>,
    /// For zero or more groups of vehicles, specifies when and how much to relax
    /// constraints. If this field is empty, all non-empty vehicle routes are
    /// fully constrained.
    #[prost(message, repeated, tag = "3")]
    pub constraint_relaxations: ::prost::alloc::vec::Vec<
        injected_solution_constraint::ConstraintRelaxation,
    >,
}
/// Nested message and enum types in `InjectedSolutionConstraint`.
pub mod injected_solution_constraint {
    /// For a group of vehicles, specifies at what threshold(s) constraints on
    /// visits will be relaxed and to which level. Shipments listed in
    /// the `skipped_shipment` field are constrained to be skipped; i.e., they
    /// cannot be performed.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConstraintRelaxation {
        /// All the visit constraint relaxations that will apply to visits on
        /// routes with vehicles in `vehicle_indices`.
        #[prost(message, repeated, tag = "1")]
        pub relaxations: ::prost::alloc::vec::Vec<constraint_relaxation::Relaxation>,
        /// Specifies the vehicle indices to which the visit constraint
        /// `relaxations` apply. If empty, this is considered the default and the
        /// `relaxations` apply to all vehicles that are not specified in other
        /// `constraint_relaxations`. There can be at most one default, i.e., at
        /// most one constraint relaxation field is allowed empty
        /// `vehicle_indices`. A vehicle index can only be listed once, even within
        /// several `constraint_relaxations`.
        ///
        /// A vehicle index is mapped the same as
        /// \[ShipmentRoute.vehicle_index][google.cloud.optimization.v1.ShipmentRoute.vehicle_index\],
        /// if `interpret_injected_solutions_using_labels` is true (see `fields`
        /// comment).
        #[prost(int32, repeated, tag = "2")]
        pub vehicle_indices: ::prost::alloc::vec::Vec<i32>,
    }
    /// Nested message and enum types in `ConstraintRelaxation`.
    pub mod constraint_relaxation {
        /// If `relaxations` is empty, the start time and sequence of all visits
        /// on `routes` are fully constrained and no new visits may be inserted or
        /// added to those routes. Also, a vehicle's start and end time in
        /// `routes` is fully constrained, unless the vehicle is empty (i.e., has no
        /// visits and has `used_if_route_is_empty` set to false in the model).
        ///
        /// `relaxations(i).level` specifies the constraint relaxation level applied
        /// to a visit #j that satisfies:
        ///
        ///    * `route.visits(j).start_time >= relaxations(i).threshold_time` AND
        ///    * `j + 1 >= relaxations(i).threshold_visit_count`
        ///
        /// Similarly, the vehicle start is relaxed to `relaxations(i).level` if it
        /// satisfies:
        ///
        ///    * `vehicle_start_time >= relaxations(i).threshold_time` AND
        ///    * `relaxations(i).threshold_visit_count == 0`
        /// and the vehicle end is relaxed to `relaxations(i).level` if it satisfies:
        ///    * `vehicle_end_time >= relaxations(i).threshold_time` AND
        ///    * `route.visits_size() + 1 >= relaxations(i).threshold_visit_count`
        ///
        /// To apply a relaxation level if a visit meets the `threshold_visit_count`
        /// OR the `threshold_time` add two `relaxations` with the same `level`:
        /// one with only `threshold_visit_count` set and the other with only
        /// `threshold_time` set. If a visit satisfies the conditions of multiple
        /// `relaxations`, the most relaxed level applies. As a result, from the
        /// vehicle start through the route visits in order to the vehicle end, the
        /// relaxation level becomes more relaxed: i.e., the relaxation level is
        /// non-decreasing as the route progresses.
        ///
        /// The timing and sequence of route visits that do not satisfy the
        /// threshold conditions of any `relaxations` are fully constrained
        /// and no visits may be inserted into these sequences. Also, if a
        /// vehicle start or end does not satisfy the conditions of any
        /// relaxation the time is fixed, unless the vehicle is empty.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Relaxation {
            /// The constraint relaxation level that applies when the conditions
            /// at or after `threshold_time` AND at least `threshold_visit_count` are
            /// satisfied.
            #[prost(enumeration = "relaxation::Level", tag = "1")]
            pub level: i32,
            /// The time at or after which the relaxation `level` may be applied.
            #[prost(message, optional, tag = "2")]
            pub threshold_time: ::core::option::Option<::prost_types::Timestamp>,
            /// The number of visits at or after which the relaxation `level` may be
            /// applied. If `threshold_visit_count` is 0 (or unset), the `level` may be
            /// applied directly at the vehicle start.
            ///
            /// If it is `route.visits_size() + 1`, the `level` may only be applied to
            /// the vehicle end. If it is more than `route.visits_size() + 1`,
            /// `level` is not applied at all for that route.
            #[prost(int32, tag = "3")]
            pub threshold_visit_count: i32,
        }
        /// Nested message and enum types in `Relaxation`.
        pub mod relaxation {
            /// Expresses the different constraint relaxation levels, which are
            /// applied for a visit and those that follow when it satisfies the
            /// threshold conditions.
            ///
            /// The enumeration below is in order of increasing relaxation.
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
            pub enum Level {
                /// Implicit default relaxation level: no constraints are relaxed,
                /// i.e., all visits are fully constrained.
                ///
                /// This value must not be explicitly used in `level`.
                Unspecified = 0,
                /// Visit start times and vehicle start/end times will be relaxed, but
                /// each visit remains bound to the same vehicle and the visit sequence
                /// must be observed: no visit can be inserted between them or before
                /// them.
                RelaxVisitTimesAfterThreshold = 1,
                /// Same as `RELAX_VISIT_TIMES_AFTER_THRESHOLD`, but the visit sequence
                /// is also relaxed: visits remain simply bound to their vehicle.
                RelaxVisitTimesAndSequenceAfterThreshold = 2,
                /// Same as `RELAX_VISIT_TIMES_AND_SEQUENCE_AFTER_THRESHOLD`, but the
                /// vehicle is also relaxed: visits are completely free at or after the
                /// threshold time and can potentially become unperformed.
                RelaxAllAfterThreshold = 3,
            }
            impl Level {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        Level::Unspecified => "LEVEL_UNSPECIFIED",
                        Level::RelaxVisitTimesAfterThreshold => {
                            "RELAX_VISIT_TIMES_AFTER_THRESHOLD"
                        }
                        Level::RelaxVisitTimesAndSequenceAfterThreshold => {
                            "RELAX_VISIT_TIMES_AND_SEQUENCE_AFTER_THRESHOLD"
                        }
                        Level::RelaxAllAfterThreshold => "RELAX_ALL_AFTER_THRESHOLD",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "LEVEL_UNSPECIFIED" => Some(Self::Unspecified),
                        "RELAX_VISIT_TIMES_AFTER_THRESHOLD" => {
                            Some(Self::RelaxVisitTimesAfterThreshold)
                        }
                        "RELAX_VISIT_TIMES_AND_SEQUENCE_AFTER_THRESHOLD" => {
                            Some(Self::RelaxVisitTimesAndSequenceAfterThreshold)
                        }
                        "RELAX_ALL_AFTER_THRESHOLD" => Some(Self::RelaxAllAfterThreshold),
                        _ => None,
                    }
                }
            }
        }
    }
}
/// Describes an error encountered when validating an `OptimizeToursRequest`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptimizeToursValidationError {
    /// A validation error is defined by the pair (`code`, `display_name`) which
    /// are always present.
    ///
    /// Other fields (below) provide more context about the error.
    ///
    /// *MULTIPLE ERRORS*:
    /// When there are multiple errors, the validation process tries to output
    /// several of them. Much like a compiler, this is an imperfect process. Some
    /// validation errors will be "fatal", meaning that they stop the entire
    /// validation process. This is the case for `display_name="UNSPECIFIED"`
    /// errors, among others. Some may cause the validation process to skip other
    /// errors.
    ///
    /// *STABILITY*:
    /// `code` and `display_name` should be very stable. But new codes and
    /// display names may appear over time, which may cause a given (invalid)
    /// request to yield a different (`code`, `display_name`) pair because the new
    /// error hid the old one (see "MULTIPLE ERRORS").
    ///
    /// *REFERENCE*: A list of all (code, name) pairs:
    ///
    /// * UNSPECIFIED = 0;
    /// * VALIDATION_TIMEOUT_ERROR = 10; Validation couldn't be completed within
    /// the deadline.
    ///
    /// * REQUEST_OPTIONS_ERROR = 12;
    ///      * REQUEST_OPTIONS_INVALID_SOLVING_MODE = 1201;
    ///      * REQUEST_OPTIONS_INVALID_MAX_VALIDATION_ERRORS = 1203;
    ///      * REQUEST_OPTIONS_INVALID_GEODESIC_METERS_PER_SECOND = 1204;
    ///      * REQUEST_OPTIONS_GEODESIC_METERS_PER_SECOND_TOO_SMALL = 1205;
    ///      * REQUEST_OPTIONS_MISSING_GEODESIC_METERS_PER_SECOND = 1206;
    ///      * REQUEST_OPTIONS_POPULATE_PATHFINDER_TRIPS_AND_GEODESIC_DISTANCE
    ///        = 1207;
    ///      * REQUEST_OPTIONS_COST_MODEL_OPTIONS_AND_GEODESIC_DISTANCE = 1208;
    ///      * REQUEST_OPTIONS_TRAVEL_MODE_INCOMPATIBLE_WITH_TRAFFIC = 1211;
    ///      * REQUEST_OPTIONS_MULTIPLE_TRAFFIC_FLAVORS = 1212;
    ///      * REQUEST_OPTIONS_INVALID_TRAFFIC_FLAVOR = 1213;
    ///      * REQUEST_OPTIONS_TRAFFIC_ENABLED_WITHOUT_GLOBAL_START_TIME = 1214;
    ///      * REQUEST_OPTIONS_TRAFFIC_ENABLED_WITH_PRECEDENCES = 1215;
    ///      * REQUEST_OPTIONS_TRAFFIC_PREFILL_MODE_INVALID = 1216;
    ///      * REQUEST_OPTIONS_TRAFFIC_PREFILL_ENABLED_WITHOUT_TRAFFIC = 1217;
    /// * INJECTED_SOLUTION_ERROR = 20;
    ///      * INJECTED_SOLUTION_MISSING_LABEL = 2000;
    ///      * INJECTED_SOLUTION_DUPLICATE_LABEL = 2001;
    ///      * INJECTED_SOLUTION_AMBIGUOUS_INDEX = 2002;
    ///      * INJECTED_SOLUTION_INFEASIBLE_AFTER_GETTING_TRAVEL_TIMES = 2003;
    ///      * INJECTED_SOLUTION_TRANSITION_INCONSISTENT_WITH_ACTUAL_TRAVEL = 2004;
    ///      * INJECTED_SOLUTION_CONCURRENT_SOLUTION_TYPES = 2005;
    ///      * INJECTED_SOLUTION_MORE_THAN_ONE_PER_TYPE = 2006;
    ///      * INJECTED_SOLUTION_REFRESH_WITHOUT_POPULATE = 2008;
    /// * SHIPMENT_MODEL_ERROR = 22;
    ///      * SHIPMENT_MODEL_TOO_LARGE = 2200;
    ///      * SHIPMENT_MODEL_TOO_MANY_CAPACITY_TYPES = 2201;
    ///      * SHIPMENT_MODEL_GLOBAL_START_TIME_NEGATIVE_OR_NAN = 2202;
    ///      * SHIPMENT_MODEL_GLOBAL_END_TIME_TOO_LARGE_OR_NAN = 2203;
    ///      * SHIPMENT_MODEL_GLOBAL_START_TIME_AFTER_GLOBAL_END_TIME = 2204;
    ///      * SHIPMENT_MODEL_GLOBAL_DURATION_TOO_LONG = 2205;
    ///      * SHIPMENT_MODEL_MAX_ACTIVE_VEHICLES_NOT_POSITIVE = 2206;
    ///      * SHIPMENT_MODEL_DURATION_MATRIX_TOO_LARGE = 2207;
    /// * INDEX_ERROR = 24;
    /// * TAG_ERROR = 26;
    /// * TIME_WINDOW_ERROR = 28;
    ///      * TIME_WINDOW_INVALID_START_TIME = 2800;
    ///      * TIME_WINDOW_INVALID_END_TIME = 2801;
    ///      * TIME_WINDOW_INVALID_SOFT_START_TIME = 2802;
    ///      * TIME_WINDOW_INVALID_SOFT_END_TIME = 2803;
    ///      * TIME_WINDOW_OUTSIDE_GLOBAL_TIME_WINDOW = 2804;
    ///      * TIME_WINDOW_START_TIME_AFTER_END_TIME = 2805;
    ///      * TIME_WINDOW_INVALID_COST_PER_HOUR_BEFORE_SOFT_START_TIME = 2806;
    ///      * TIME_WINDOW_INVALID_COST_PER_HOUR_AFTER_SOFT_END_TIME = 2807;
    ///      * TIME_WINDOW_COST_BEFORE_SOFT_START_TIME_WITHOUT_SOFT_START_TIME
    ///        = 2808;
    ///      * TIME_WINDOW_COST_AFTER_SOFT_END_TIME_WITHOUT_SOFT_END_TIME = 2809;
    ///      * TIME_WINDOW_SOFT_START_TIME_WITHOUT_COST_BEFORE_SOFT_START_TIME
    ///        = 2810;
    ///      * TIME_WINDOW_SOFT_END_TIME_WITHOUT_COST_AFTER_SOFT_END_TIME = 2811;
    ///      * TIME_WINDOW_OVERLAPPING_ADJACENT_OR_EARLIER_THAN_PREVIOUS = 2812;
    ///      * TIME_WINDOW_START_TIME_AFTER_SOFT_START_TIME = 2813;
    ///      * TIME_WINDOW_SOFT_START_TIME_AFTER_END_TIME = 2814;
    ///      * TIME_WINDOW_START_TIME_AFTER_SOFT_END_TIME = 2815;
    ///      * TIME_WINDOW_SOFT_END_TIME_AFTER_END_TIME = 2816;
    ///      * TIME_WINDOW_COST_BEFORE_SOFT_START_TIME_SET_AND_MULTIPLE_WINDOWS
    ///        = 2817;
    ///      * TIME_WINDOW_COST_AFTER_SOFT_END_TIME_SET_AND_MULTIPLE_WINDOWS = 2818;
    ///      * TRANSITION_ATTRIBUTES_ERROR = 30;
    ///      * TRANSITION_ATTRIBUTES_INVALID_COST = 3000;
    ///      * TRANSITION_ATTRIBUTES_INVALID_COST_PER_KILOMETER = 3001;
    ///      * TRANSITION_ATTRIBUTES_DUPLICATE_TAG_PAIR = 3002;
    ///      * TRANSITION_ATTRIBUTES_DISTANCE_LIMIT_MAX_METERS_UNSUPPORTED = 3003;
    ///      * TRANSITION_ATTRIBUTES_UNSPECIFIED_SOURCE_TAGS = 3004;
    ///      * TRANSITION_ATTRIBUTES_CONFLICTING_SOURCE_TAGS_FIELDS = 3005;
    ///      * TRANSITION_ATTRIBUTES_UNSPECIFIED_DESTINATION_TAGS = 3006;
    ///      * TRANSITION_ATTRIBUTES_CONFLICTING_DESTINATION_TAGS_FIELDS = 3007;
    ///      * TRANSITION_ATTRIBUTES_DELAY_DURATION_NEGATIVE_OR_NAN = 3008;
    ///      * TRANSITION_ATTRIBUTES_DELAY_DURATION_EXCEEDS_GLOBAL_DURATION = 3009;
    /// * AMOUNT_ERROR = 31;
    ///      * AMOUNT_NEGATIVE_VALUE = 3100;
    /// * LOAD_LIMIT_ERROR = 33;
    ///      * LOAD_LIMIT_INVALID_COST_ABOVE_SOFT_MAX = 3303;
    ///      * LOAD_LIMIT_SOFT_MAX_WITHOUT_COST_ABOVE_SOFT_MAX = 3304;
    ///      * LOAD_LIMIT_COST_ABOVE_SOFT_MAX_WITHOUT_SOFT_MAX = 3305;
    ///      * LOAD_LIMIT_NEGATIVE_SOFT_MAX = 3306;
    ///      * LOAD_LIMIT_MIXED_DEMAND_TYPE = 3307;
    ///      * LOAD_LIMIT_MAX_LOAD_NEGATIVE_VALUE = 3308;
    ///      * LOAD_LIMIT_SOFT_MAX_ABOVE_MAX = 3309;
    /// * INTERVAL_ERROR = 34;
    ///      * INTERVAL_MIN_EXCEEDS_MAX = 3401;
    ///      * INTERVAL_NEGATIVE_MIN = 3402;
    ///      * INTERVAL_NEGATIVE_MAX = 3403;
    ///      * INTERVAL_MIN_EXCEEDS_CAPACITY = 3404;
    ///      * INTERVAL_MAX_EXCEEDS_CAPACITY = 3405;
    /// * DISTANCE_LIMIT_ERROR = 36;
    ///      * DISTANCE_LIMIT_INVALID_COST_AFTER_SOFT_MAX = 3601;
    ///      * DISTANCE_LIMIT_SOFT_MAX_WITHOUT_COST_AFTER_SOFT_MAX = 3602;
    ///      * DISTANCE_LIMIT_COST_AFTER_SOFT_MAX_WITHOUT_SOFT_MAX = 3603;
    ///      * DISTANCE_LIMIT_NEGATIVE_MAX = 3604;
    ///      * DISTANCE_LIMIT_NEGATIVE_SOFT_MAX = 3605;
    ///      * DISTANCE_LIMIT_SOFT_MAX_LARGER_THAN_MAX = 3606;
    /// * DURATION_LIMIT_ERROR = 38;
    ///      * DURATION_LIMIT_MAX_DURATION_NEGATIVE_OR_NAN = 3800;
    ///      * DURATION_LIMIT_SOFT_MAX_DURATION_NEGATIVE_OR_NAN = 3801;
    ///      * DURATION_LIMIT_INVALID_COST_PER_HOUR_AFTER_SOFT_MAX = 3802;
    ///      * DURATION_LIMIT_SOFT_MAX_WITHOUT_COST_AFTER_SOFT_MAX = 3803;
    ///      * DURATION_LIMIT_COST_AFTER_SOFT_MAX_WITHOUT_SOFT_MAX = 3804;
    ///      * DURATION_LIMIT_QUADRATIC_SOFT_MAX_DURATION_NEGATIVE_OR_NAN = 3805;
    ///      * DURATION_LIMIT_INVALID_COST_AFTER_QUADRATIC_SOFT_MAX = 3806;
    ///      * DURATION_LIMIT_QUADRATIC_SOFT_MAX_WITHOUT_COST_PER_SQUARE_HOUR
    ///        = 3807;
    ///      * DURATION_LIMIT_COST_PER_SQUARE_HOUR_WITHOUT_QUADRATIC_SOFT_MAX
    ///        = 3808;
    ///      * DURATION_LIMIT_QUADRATIC_SOFT_MAX_WITHOUT_MAX = 3809;
    ///      * DURATION_LIMIT_SOFT_MAX_LARGER_THAN_MAX = 3810;
    ///      * DURATION_LIMIT_QUADRATIC_SOFT_MAX_LARGER_THAN_MAX = 3811;
    ///      * DURATION_LIMIT_DIFF_BETWEEN_MAX_AND_QUADRATIC_SOFT_MAX_TOO_LARGE
    ///        = 3812;
    ///      * DURATION_LIMIT_MAX_DURATION_EXCEEDS_GLOBAL_DURATION = 3813;
    ///      * DURATION_LIMIT_SOFT_MAX_DURATION_EXCEEDS_GLOBAL_DURATION = 3814;
    ///      * DURATION_LIMIT_QUADRATIC_SOFT_MAX_DURATION_EXCEEDS_GLOBAL_DURATION
    ///        = 3815;
    /// * SHIPMENT_ERROR = 40;
    ///      * SHIPMENT_PD_LIMIT_WITHOUT_PICKUP_AND_DELIVERY = 4014;
    ///      * SHIPMENT_PD_ABSOLUTE_DETOUR_LIMIT_DURATION_NEGATIVE_OR_NAN = 4000;
    ///      * SHIPMENT_PD_ABSOLUTE_DETOUR_LIMIT_DURATION_EXCEEDS_GLOBAL_DURATION
    ///        = 4001;
    ///      * SHIPMENT_PD_RELATIVE_DETOUR_LIMIT_INVALID = 4015;
    ///      * SHIPMENT_PD_DETOUR_LIMIT_AND_EXTRA_VISIT_DURATION = 4016;
    ///      * SHIPMENT_PD_TIME_LIMIT_DURATION_NEGATIVE_OR_NAN = 4002;
    ///      * SHIPMENT_PD_TIME_LIMIT_DURATION_EXCEEDS_GLOBAL_DURATION = 4003;
    ///      * SHIPMENT_EMPTY_SHIPMENT_TYPE = 4004;
    ///      * SHIPMENT_NO_PICKUP_NO_DELIVERY = 4005;
    ///      * SHIPMENT_INVALID_PENALTY_COST = 4006;
    ///      * SHIPMENT_ALLOWED_VEHICLE_INDEX_OUT_OF_BOUNDS = 4007;
    ///      * SHIPMENT_DUPLICATE_ALLOWED_VEHICLE_INDEX = 4008;
    ///      * SHIPMENT_INCONSISTENT_COST_FOR_VEHICLE_SIZE_WITHOUT_INDEX = 4009;
    ///      * SHIPMENT_INCONSISTENT_COST_FOR_VEHICLE_SIZE_WITH_INDEX = 4010;
    ///      * SHIPMENT_INVALID_COST_FOR_VEHICLE = 4011;
    ///      * SHIPMENT_COST_FOR_VEHICLE_INDEX_OUT_OF_BOUNDS = 4012;
    ///      * SHIPMENT_DUPLICATE_COST_FOR_VEHICLE_INDEX = 4013;
    /// * VEHICLE_ERROR = 42;
    ///      * VEHICLE_EMPTY_REQUIRED_OPERATOR_TYPE = 4200;
    ///      * VEHICLE_DUPLICATE_REQUIRED_OPERATOR_TYPE = 4201;
    ///      * VEHICLE_NO_OPERATOR_WITH_REQUIRED_OPERATOR_TYPE = 4202;
    ///      * VEHICLE_EMPTY_START_TAG = 4203;
    ///      * VEHICLE_DUPLICATE_START_TAG = 4204;
    ///      * VEHICLE_EMPTY_END_TAG = 4205;
    ///      * VEHICLE_DUPLICATE_END_TAG = 4206;
    ///      * VEHICLE_EXTRA_VISIT_DURATION_NEGATIVE_OR_NAN = 4207;
    ///      * VEHICLE_EXTRA_VISIT_DURATION_EXCEEDS_GLOBAL_DURATION = 4208;
    ///      * VEHICLE_EXTRA_VISIT_DURATION_EMPTY_KEY = 4209;
    ///      * VEHICLE_FIRST_SHIPMENT_INDEX_OUT_OF_BOUNDS = 4210;
    ///      * VEHICLE_FIRST_SHIPMENT_IGNORED = 4211;
    ///      * VEHICLE_FIRST_SHIPMENT_NOT_BOUND = 4212;
    ///      * VEHICLE_LAST_SHIPMENT_INDEX_OUT_OF_BOUNDS = 4213;
    ///      * VEHICLE_LAST_SHIPMENT_IGNORED = 4214;
    ///      * VEHICLE_LAST_SHIPMENT_NOT_BOUND = 4215;
    ///      * VEHICLE_IGNORED_WITH_USED_IF_ROUTE_IS_EMPTY = 4216;
    ///      * VEHICLE_INVALID_COST_PER_KILOMETER = 4217;
    ///      * VEHICLE_INVALID_COST_PER_HOUR = 4218;
    ///      * VEHICLE_INVALID_COST_PER_TRAVELED_HOUR = 4219;
    ///      * VEHICLE_INVALID_FIXED_COST = 4220;
    ///      * VEHICLE_INVALID_TRAVEL_DURATION_MULTIPLE = 4221;
    ///      * VEHICLE_TRAVEL_DURATION_MULTIPLE_WITH_SHIPMENT_PD_DETOUR_LIMITS
    ///        = 4223;
    ///      * VEHICLE_MATRIX_INDEX_WITH_SHIPMENT_PD_DETOUR_LIMITS = 4224;
    ///      * VEHICLE_MINIMUM_DURATION_LONGER_THAN_DURATION_LIMIT = 4222;
    /// * VISIT_REQUEST_ERROR = 44;
    ///      * VISIT_REQUEST_EMPTY_TAG = 4400;
    ///      * VISIT_REQUEST_DUPLICATE_TAG = 4401;
    ///      * VISIT_REQUEST_DURATION_NEGATIVE_OR_NAN = 4404;
    ///      * VISIT_REQUEST_DURATION_EXCEEDS_GLOBAL_DURATION = 4405;
    /// * PRECEDENCE_ERROR = 46;
    /// * BREAK_ERROR = 48;
    ///      * BREAK_RULE_EMPTY = 4800;
    ///      * BREAK_REQUEST_UNSPECIFIED_DURATION = 4801;
    ///      * BREAK_REQUEST_UNSPECIFIED_EARLIEST_START_TIME = 4802;
    ///      * BREAK_REQUEST_UNSPECIFIED_LATEST_START_TIME = 4803;
    ///      * BREAK_REQUEST_DURATION_NEGATIVE_OR_NAN = 4804; = 4804;
    ///      * BREAK_REQUEST_LATEST_START_TIME_BEFORE_EARLIEST_START_TIME = 4805;
    ///      * BREAK_REQUEST_EARLIEST_START_TIME_BEFORE_GLOBAL_START_TIME = 4806;
    ///      * BREAK_REQUEST_LATEST_END_TIME_AFTER_GLOBAL_END_TIME = 4807;
    ///      * BREAK_REQUEST_NON_SCHEDULABLE = 4808;
    ///      * BREAK_FREQUENCY_MAX_INTER_BREAK_DURATION_NEGATIVE_OR_NAN = 4809;
    ///      * BREAK_FREQUENCY_MIN_BREAK_DURATION_NEGATIVE_OR_NAN = 4810;
    ///      * BREAK_FREQUENCY_MIN_BREAK_DURATION_EXCEEDS_GLOBAL_DURATION = 4811;
    ///      * BREAK_FREQUENCY_MAX_INTER_BREAK_DURATION_EXCEEDS_GLOBAL_DURATION
    ///        = 4812;
    ///      * BREAK_REQUEST_DURATION_EXCEEDS_GLOBAL_DURATION = 4813;
    ///      * BREAK_FREQUENCY_MISSING_MAX_INTER_BREAK_DURATION = 4814;
    ///      * BREAK_FREQUENCY_MISSING_MIN_BREAK_DURATION = 4815;
    /// * SHIPMENT_TYPE_INCOMPATIBILITY_ERROR = 50;
    ///      * SHIPMENT_TYPE_INCOMPATIBILITY_EMPTY_TYPE = 5001;
    ///      * SHIPMENT_TYPE_INCOMPATIBILITY_LESS_THAN_TWO_TYPES = 5002;
    ///      * SHIPMENT_TYPE_INCOMPATIBILITY_DUPLICATE_TYPE = 5003;
    ///      * SHIPMENT_TYPE_INCOMPATIBILITY_INVALID_INCOMPATIBILITY_MODE = 5004;
    ///      * SHIPMENT_TYPE_INCOMPATIBILITY_TOO_MANY_INCOMPATIBILITIES = 5005;
    /// * SHIPMENT_TYPE_REQUIREMENT_ERROR = 52;
    ///      * SHIPMENT_TYPE_REQUIREMENT_NO_REQUIRED_TYPE = 52001;
    ///      * SHIPMENT_TYPE_REQUIREMENT_NO_DEPENDENT_TYPE = 52002;
    ///      * SHIPMENT_TYPE_REQUIREMENT_INVALID_REQUIREMENT_MODE = 52003;
    ///      * SHIPMENT_TYPE_REQUIREMENT_TOO_MANY_REQUIREMENTS = 52004;
    ///      * SHIPMENT_TYPE_REQUIREMENT_EMPTY_REQUIRED_TYPE = 52005;
    ///      * SHIPMENT_TYPE_REQUIREMENT_DUPLICATE_REQUIRED_TYPE = 52006;
    ///      * SHIPMENT_TYPE_REQUIREMENT_NO_REQUIRED_TYPE_FOUND = 52007;
    ///      * SHIPMENT_TYPE_REQUIREMENT_EMPTY_DEPENDENT_TYPE = 52008;
    ///      * SHIPMENT_TYPE_REQUIREMENT_DUPLICATE_DEPENDENT_TYPE = 52009;
    ///      * SHIPMENT_TYPE_REQUIREMENT_SELF_DEPENDENT_TYPE = 52010;
    ///      * SHIPMENT_TYPE_REQUIREMENT_GRAPH_HAS_CYCLES = 52011;
    /// * VEHICLE_OPERATOR_ERROR = 54;
    ///      * VEHICLE_OPERATOR_EMPTY_TYPE = 5400;
    ///      * VEHICLE_OPERATOR_MULTIPLE_START_TIME_WINDOWS = 5401;
    ///      * VEHICLE_OPERATOR_SOFT_START_TIME_WINDOW = 5402;
    ///      * VEHICLE_OPERATOR_MULTIPLE_END_TIME_WINDOWS = 5403;
    ///      * VEHICLE_OPERATOR_SOFT_END_TIME_WINDOW = 5404;
    /// * DURATION_SECONDS_MATRIX_ERROR = 56;
    ///      * DURATION_SECONDS_MATRIX_DURATION_NEGATIVE_OR_NAN = 5600;
    ///      * DURATION_SECONDS_MATRIX_DURATION_EXCEEDS_GLOBAL_DURATION = 5601;
    #[prost(int32, tag = "1")]
    pub code: i32,
    /// The error display name.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// An error context may involve 0, 1 (most of the time) or more fields. For
    /// example, referring to vehicle #4 and shipment #2's first pickup can be
    /// done as follows:
    /// ```
    /// fields { name: "vehicles" index: 4}
    /// fields { name: "shipments" index: 2 sub_field {name: "pickups" index: 0} }
    /// ```
    /// Note, however, that the cardinality of `fields` should not change for a
    /// given error code.
    #[prost(message, repeated, tag = "3")]
    pub fields: ::prost::alloc::vec::Vec<
        optimize_tours_validation_error::FieldReference,
    >,
    /// Human-readable string describing the error. There is a 1:1 mapping
    /// between `code` and `error_message` (when code != "UNSPECIFIED").
    ///
    /// *STABILITY*: Not stable: the error message associated to a given `code` may
    /// change (hopefully to clarify it) over time. Please rely on the
    /// `display_name` and `code` instead.
    #[prost(string, tag = "4")]
    pub error_message: ::prost::alloc::string::String,
    /// May contain the value(s) of the field(s). This is not always available. You
    /// should absolutely not rely on it and use it only for manual model
    /// debugging.
    #[prost(string, tag = "5")]
    pub offending_values: ::prost::alloc::string::String,
}
/// Nested message and enum types in `OptimizeToursValidationError`.
pub mod optimize_tours_validation_error {
    /// Specifies a context for the validation error. A `FieldReference` always
    /// refers to a given field in this file and follows the same hierarchical
    /// structure. For example, we may specify element #2 of `start_time_windows`
    /// of vehicle #5 using:
    /// ```
    /// name: "vehicles" index: 5 sub_field { name: "end_time_windows" index: 2 }
    /// ```
    /// We however omit top-level entities such as `OptimizeToursRequest` or
    /// `ShipmentModel` to avoid crowding the message.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FieldReference {
        /// Name of the field, e.g., "vehicles".
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// Recursively nested sub-field, if needed.
        #[prost(message, optional, boxed, tag = "3")]
        pub sub_field: ::core::option::Option<
            ::prost::alloc::boxed::Box<FieldReference>,
        >,
        #[prost(oneof = "field_reference::IndexOrKey", tags = "2, 4")]
        pub index_or_key: ::core::option::Option<field_reference::IndexOrKey>,
    }
    /// Nested message and enum types in `FieldReference`.
    pub mod field_reference {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum IndexOrKey {
            /// Index of the field if repeated.
            #[prost(int32, tag = "2")]
            Index(i32),
            /// Key if the field is a map.
            #[prost(string, tag = "4")]
            Key(::prost::alloc::string::String),
        }
    }
}
/// Generated client implementations.
pub mod fleet_routing_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// A service for optimizing vehicle tours.
    ///
    /// Validity of certain types of fields:
    ///
    ///   * `google.protobuf.Timestamp`
    ///     * Times are in Unix time: seconds since 1970-01-01T00:00:00+00:00.
    ///     * seconds must be in [0, 253402300799],
    ///       i.e. in [1970-01-01T00:00:00+00:00, 9999-12-31T23:59:59+00:00].
    ///     * nanos must be unset or set to 0.
    ///   * `google.protobuf.Duration`
    ///     * seconds must be in [0, 253402300799],
    ///       i.e. in [1970-01-01T00:00:00+00:00, 9999-12-31T23:59:59+00:00].
    ///     * nanos must be unset or set to 0.
    ///   * `google.type.LatLng`
    ///     * latitude must be in [-90.0, 90.0].
    ///     * longitude must be in [-180.0, 180.0].
    ///     * at least one of latitude and longitude must be non-zero.
    #[derive(Debug, Clone)]
    pub struct FleetRoutingClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> FleetRoutingClient<T>
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
        ) -> FleetRoutingClient<InterceptedService<T, F>>
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
            FleetRoutingClient::new(InterceptedService::new(inner, interceptor))
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
        /// Sends an `OptimizeToursRequest` containing a `ShipmentModel` and returns an
        /// `OptimizeToursResponse` containing `ShipmentRoute`s, which are a set of
        /// routes to be performed by vehicles minimizing the overall cost.
        ///
        /// A `ShipmentModel` model consists mainly of `Shipment`s that need to be
        /// carried out and `Vehicle`s that can be used to transport the `Shipment`s.
        /// The `ShipmentRoute`s assign `Shipment`s to `Vehicle`s. More specifically,
        /// they assign a series of `Visit`s to each vehicle, where a `Visit`
        /// corresponds to a `VisitRequest`, which is a pickup or delivery for a
        /// `Shipment`.
        ///
        /// The goal is to provide an assignment of `ShipmentRoute`s to `Vehicle`s that
        /// minimizes the total cost where cost has many components defined in the
        /// `ShipmentModel`.
        pub async fn optimize_tours(
            &mut self,
            request: impl tonic::IntoRequest<super::OptimizeToursRequest>,
        ) -> Result<tonic::Response<super::OptimizeToursResponse>, tonic::Status> {
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
                "/google.cloud.optimization.v1.FleetRouting/OptimizeTours",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Optimizes vehicle tours for one or more `OptimizeToursRequest`
        /// messages as a batch.
        ///
        /// This method is a Long Running Operation (LRO). The inputs for optimization
        /// (`OptimizeToursRequest` messages) and outputs (`OptimizeToursResponse`
        /// messages) are read/written from/to Cloud Storage in user-specified
        /// format. Like the `OptimizeTours` method, each `OptimizeToursRequest`
        /// contains a `ShipmentModel` and returns an `OptimizeToursResponse`
        /// containing `ShipmentRoute`s, which are a set of routes to be performed by
        /// vehicles minimizing the overall cost.
        pub async fn batch_optimize_tours(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchOptimizeToursRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.optimization.v1.FleetRouting/BatchOptimizeTours",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
