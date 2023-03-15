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
    /// of due North, 90 specifies a heading of due East, etc. You can use this
    /// field only for `DRIVE` and `TWO_WHEELER` travel modes.
    #[prost(message, optional, tag = "2")]
    pub heading: ::core::option::Option<i32>,
}
/// Encapsulates a waypoint. Waypoints mark both the beginning and end of a
/// route, and include intermediate stops along the route.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Waypoint {
    /// Marks this waypoint as a milestone rather a stopping point. For
    /// each non-via waypoint in the request, the response appends an entry to the
    /// `legs` array to provide the details for stopovers on that leg of the
    /// trip. Set this value to true when you want the route to pass through this
    /// waypoint without stopping over. Via waypoints don't cause an entry to be
    /// added to the `legs` array, but they do route the journey through the
    /// waypoint. You can only set this value on waypoints that are intermediates.
    /// The request fails if you set this field on terminal waypoints.
    /// If `ComputeRoutesRequest.optimize_waypoint_order`
    /// is set to true then this field cannot be set to
    /// true; otherwise, the request fails.
    #[prost(bool, tag = "3")]
    pub via: bool,
    /// Indicates that the waypoint is meant for vehicles to stop at, where the
    /// intention is to either pickup or drop-off. When you set this value, the
    /// calculated route won't include non-`via` waypoints on roads that are
    /// unsuitable for pickup and drop-off. This option works only for `DRIVE` and
    /// `TWO_WHEELER` travel modes, and when the `location_type` is `location`.
    #[prost(bool, tag = "4")]
    pub vehicle_stopover: bool,
    /// Indicates that the location of this waypoint is meant to have a preference
    /// for the vehicle to stop at a particular side of road. When you set this
    /// value, the route will pass through the location so that the vehicle can
    /// stop at the side of road that the location is biased towards from the
    /// center of the road. This option works only for 'DRIVE' and 'TWO_WHEELER'
    /// travel modes.
    #[prost(bool, tag = "5")]
    pub side_of_road: bool,
    /// Different ways to represent a location.
    #[prost(oneof = "waypoint::LocationType", tags = "1, 2, 7")]
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
        /// Human readable address or a plus code.
        /// See <https://plus.codes> for details.
        #[prost(string, tag = "7")]
        Address(::prost::alloc::string::String),
    }
}
/// A set of values that specify the navigation action to take for the current
/// step (e.g., turn left, merge, straight, etc.).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Maneuver {
    /// Not used.
    Unspecified = 0,
    /// Turn slightly to the left.
    TurnSlightLeft = 1,
    /// Turn sharply to the left.
    TurnSharpLeft = 2,
    /// Make a left u-turn.
    UturnLeft = 3,
    /// Turn left.
    TurnLeft = 4,
    /// Turn slightly to the right.
    TurnSlightRight = 5,
    /// Turn sharply to the right.
    TurnSharpRight = 6,
    /// Make a right u-turn.
    UturnRight = 7,
    /// Turn right.
    TurnRight = 8,
    /// Go straight.
    Straight = 9,
    /// Take the left ramp.
    RampLeft = 10,
    /// Take the right ramp.
    RampRight = 11,
    /// Merge into traffic.
    Merge = 12,
    /// Take the left fork.
    ForkLeft = 13,
    /// Take the right fork.
    ForkRight = 14,
    /// Take the ferry.
    Ferry = 15,
    /// Take the train leading onto the ferry.
    FerryTrain = 16,
    /// Turn left at the roundabout.
    RoundaboutLeft = 17,
    /// Turn right at the roundabout.
    RoundaboutRight = 18,
}
impl Maneuver {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Maneuver::Unspecified => "MANEUVER_UNSPECIFIED",
            Maneuver::TurnSlightLeft => "TURN_SLIGHT_LEFT",
            Maneuver::TurnSharpLeft => "TURN_SHARP_LEFT",
            Maneuver::UturnLeft => "UTURN_LEFT",
            Maneuver::TurnLeft => "TURN_LEFT",
            Maneuver::TurnSlightRight => "TURN_SLIGHT_RIGHT",
            Maneuver::TurnSharpRight => "TURN_SHARP_RIGHT",
            Maneuver::UturnRight => "UTURN_RIGHT",
            Maneuver::TurnRight => "TURN_RIGHT",
            Maneuver::Straight => "STRAIGHT",
            Maneuver::RampLeft => "RAMP_LEFT",
            Maneuver::RampRight => "RAMP_RIGHT",
            Maneuver::Merge => "MERGE",
            Maneuver::ForkLeft => "FORK_LEFT",
            Maneuver::ForkRight => "FORK_RIGHT",
            Maneuver::Ferry => "FERRY",
            Maneuver::FerryTrain => "FERRY_TRAIN",
            Maneuver::RoundaboutLeft => "ROUNDABOUT_LEFT",
            Maneuver::RoundaboutRight => "ROUNDABOUT_RIGHT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MANEUVER_UNSPECIFIED" => Some(Self::Unspecified),
            "TURN_SLIGHT_LEFT" => Some(Self::TurnSlightLeft),
            "TURN_SHARP_LEFT" => Some(Self::TurnSharpLeft),
            "UTURN_LEFT" => Some(Self::UturnLeft),
            "TURN_LEFT" => Some(Self::TurnLeft),
            "TURN_SLIGHT_RIGHT" => Some(Self::TurnSlightRight),
            "TURN_SHARP_RIGHT" => Some(Self::TurnSharpRight),
            "UTURN_RIGHT" => Some(Self::UturnRight),
            "TURN_RIGHT" => Some(Self::TurnRight),
            "STRAIGHT" => Some(Self::Straight),
            "RAMP_LEFT" => Some(Self::RampLeft),
            "RAMP_RIGHT" => Some(Self::RampRight),
            "MERGE" => Some(Self::Merge),
            "FORK_LEFT" => Some(Self::ForkLeft),
            "FORK_RIGHT" => Some(Self::ForkRight),
            "FERRY" => Some(Self::Ferry),
            "FERRY_TRAIN" => Some(Self::FerryTrain),
            "ROUNDABOUT_LEFT" => Some(Self::RoundaboutLeft),
            "ROUNDABOUT_RIGHT" => Some(Self::RoundaboutRight),
            _ => None,
        }
    }
}
/// Encapsulates navigation instructions for a
/// \[RouteLegStep][google.maps.routing.v2.RouteLegStep\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NavigationInstruction {
    /// Encapsulates the navigation instructions for the current step (e.g., turn
    /// left, merge, straight, etc.). This field determines which icon to display.
    #[prost(enumeration = "Maneuver", tag = "1")]
    pub maneuver: i32,
    /// Instructions for navigating this step.
    #[prost(string, tag = "2")]
    pub instructions: ::prost::alloc::string::String,
}
/// A set of values used to specify the mode of travel.
/// NOTE: WALK, BICYCLE, and TWO_WHEELER routes are in beta and might sometimes
/// be missing clear sidewalks, pedestrian paths, or bicycling paths.
/// You must display this warning to the user for all walking, bicycling, and
/// two-wheel routes that you display in your app.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RouteTravelMode {
    /// No travel mode specified. Defaults to `DRIVE`.
    TravelModeUnspecified = 0,
    /// Travel by passenger car.
    Drive = 1,
    /// Travel by bicycle.
    Bicycle = 2,
    /// Travel by walking.
    Walk = 3,
    /// Two-wheeled, motorized vehicle. For example, motorcycle. Note that this
    /// differs from the `BICYCLE` travel mode which covers human-powered mode.
    TwoWheeler = 4,
}
impl RouteTravelMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RouteTravelMode::TravelModeUnspecified => "TRAVEL_MODE_UNSPECIFIED",
            RouteTravelMode::Drive => "DRIVE",
            RouteTravelMode::Bicycle => "BICYCLE",
            RouteTravelMode::Walk => "WALK",
            RouteTravelMode::TwoWheeler => "TWO_WHEELER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TRAVEL_MODE_UNSPECIFIED" => Some(Self::TravelModeUnspecified),
            "DRIVE" => Some(Self::Drive),
            "BICYCLE" => Some(Self::Bicycle),
            "WALK" => Some(Self::Walk),
            "TWO_WHEELER" => Some(Self::TwoWheeler),
            _ => None,
        }
    }
}
/// Information related to how and why a fallback result was used. If this field
/// is set, then it means the server used a different routing mode from your
/// preferred mode as fallback.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FallbackInfo {
    /// Routing mode used for the response. If fallback was triggered, the mode
    /// may be different from routing preference set in the original client
    /// request.
    #[prost(enumeration = "FallbackRoutingMode", tag = "1")]
    pub routing_mode: i32,
    /// The reason why fallback response was used instead of the original response.
    /// This field is only populated when the fallback mode is triggered and the
    /// fallback response is returned.
    #[prost(enumeration = "FallbackReason", tag = "2")]
    pub reason: i32,
}
/// Reasons for using fallback response.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FallbackReason {
    /// No fallback reason specified.
    Unspecified = 0,
    /// A server error happened while calculating routes with your preferred
    /// routing mode, but we were able to return a result calculated by an
    /// alternative mode.
    ServerError = 1,
    /// We were not able to finish the calculation with your preferred routing mode
    /// on time, but we were able to return a result calculated by an alternative
    /// mode.
    LatencyExceeded = 2,
}
impl FallbackReason {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FallbackReason::Unspecified => "FALLBACK_REASON_UNSPECIFIED",
            FallbackReason::ServerError => "SERVER_ERROR",
            FallbackReason::LatencyExceeded => "LATENCY_EXCEEDED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FALLBACK_REASON_UNSPECIFIED" => Some(Self::Unspecified),
            "SERVER_ERROR" => Some(Self::ServerError),
            "LATENCY_EXCEEDED" => Some(Self::LatencyExceeded),
            _ => None,
        }
    }
}
/// Actual routing mode used for returned fallback response.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FallbackRoutingMode {
    /// Not used.
    Unspecified = 0,
    /// Indicates the "TRAFFIC_UNAWARE" routing mode was used to compute the
    /// response.
    FallbackTrafficUnaware = 1,
    /// Indicates the "TRAFFIC_AWARE" routing mode was used to compute the
    /// response.
    FallbackTrafficAware = 2,
}
impl FallbackRoutingMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FallbackRoutingMode::Unspecified => "FALLBACK_ROUTING_MODE_UNSPECIFIED",
            FallbackRoutingMode::FallbackTrafficUnaware => "FALLBACK_TRAFFIC_UNAWARE",
            FallbackRoutingMode::FallbackTrafficAware => "FALLBACK_TRAFFIC_AWARE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FALLBACK_ROUTING_MODE_UNSPECIFIED" => Some(Self::Unspecified),
            "FALLBACK_TRAFFIC_UNAWARE" => Some(Self::FallbackTrafficUnaware),
            "FALLBACK_TRAFFIC_AWARE" => Some(Self::FallbackTrafficAware),
            _ => None,
        }
    }
}
/// A set of values that specify factors to take into consideration when
/// calculating the route.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RoutingPreference {
    /// No routing preference specified. Default to `TRAFFIC_UNAWARE`.
    Unspecified = 0,
    /// Computes routes without taking live traffic conditions into consideration.
    /// Suitable when traffic conditions don't matter or are not applicable.
    /// Using this value produces the lowest latency.
    /// Note: For `RouteTravelMode` DRIVE and TWO_WHEELER choice of route and
    /// duration are based on road network and average time-independent traffic
    /// conditions. Results for a given request may vary over time due to changes
    /// in the road network, updated average traffic conditions, and the
    /// distributed nature of the service. Results may also vary between
    /// nearly-equivalent routes at any time or frequency.
    TrafficUnaware = 1,
    /// Calculates routes taking live traffic conditions into consideration.
    /// In contrast to `TRAFFIC_AWARE_OPTIMAL`, some optimizations are applied to
    /// significantly reduce latency.
    TrafficAware = 2,
    /// Calculates the routes taking live traffic conditions into consideration,
    /// without applying most performance optimizations. Using this value produces
    /// the highest latency.
    TrafficAwareOptimal = 3,
}
impl RoutingPreference {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RoutingPreference::Unspecified => "ROUTING_PREFERENCE_UNSPECIFIED",
            RoutingPreference::TrafficUnaware => "TRAFFIC_UNAWARE",
            RoutingPreference::TrafficAware => "TRAFFIC_AWARE",
            RoutingPreference::TrafficAwareOptimal => "TRAFFIC_AWARE_OPTIMAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ROUTING_PREFERENCE_UNSPECIFIED" => Some(Self::Unspecified),
            "TRAFFIC_UNAWARE" => Some(Self::TrafficUnaware),
            "TRAFFIC_AWARE" => Some(Self::TrafficAware),
            "TRAFFIC_AWARE_OPTIMAL" => Some(Self::TrafficAwareOptimal),
            _ => None,
        }
    }
}
/// List of toll passes around the world that we support.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TollPass {
    /// Not used. If this value is used, then the request fails.
    Unspecified = 0,
    /// Sydney toll pass. See additional details at <https://www.myetoll.com.au.>
    AuEtollTag = 82,
    /// Sydney toll pass. See additional details at <https://www.tollpay.com.au.>
    AuEwayTag = 83,
    /// Australia-wide toll pass.
    /// See additional details at <https://www.linkt.com.au/.>
    AuLinkt = 2,
    /// Argentina toll pass. See additional details at <https://telepase.com.ar>
    ArTelepase = 3,
    /// Brazil toll pass. See additional details at <https://www.autoexpreso.com>
    BrAutoExpreso = 81,
    /// Brazil toll pass. See additional details at <https://conectcar.com.>
    BrConectcar = 7,
    /// Brazil toll pass. See additional details at <https://movemais.com.>
    BrMoveMais = 8,
    /// Brazil toll pass. See additional details at <https://pasorapido.gob.do/>
    BrPassaRapido = 88,
    /// Brazil toll pass. See additional details at <https://www.semparar.com.br.>
    BrSemParar = 9,
    /// Brazil toll pass. See additional details at <https://taggy.com.br.>
    BrTaggy = 10,
    /// Brazil toll pass. See additional details at
    /// <https://veloe.com.br/site/onde-usar.>
    BrVeloe = 11,
    /// Canada to United States border crossing.
    CaUsAkwasasneSeawayCorporateCard = 84,
    /// Canada to United States border crossing.
    CaUsAkwasasneSeawayTransitCard = 85,
    /// Ontario, Canada to Michigan, United States border crossing.
    CaUsBlueWaterEdgePass = 18,
    /// Ontario, Canada to Michigan, United States border crossing.
    CaUsConnexion = 19,
    /// Canada to United States border crossing.
    CaUsNexusCard = 20,
    /// Indonesia.
    /// E-card provided by multiple banks used to pay for tolls. All e-cards
    /// via banks are charged the same so only one enum value is needed. E.g.
    /// - Bank Mandiri <https://www.bankmandiri.co.id/e-money>
    /// - BCA <https://www.bca.co.id/flazz>
    /// - BNI <https://www.bni.co.id/id-id/ebanking/tapcash>
    IdEToll = 16,
    /// India.
    InFastag = 78,
    /// India, HP state plate exemption.
    InLocalHpPlateExempt = 79,
    /// Mexico toll pass.
    /// <https://iave.capufe.gob.mx/#/>
    MxIave = 90,
    /// Mexico
    /// <https://www.pase.com.mx>
    MxPase = 91,
    /// Mexico
    ///   <https://operadoravial.com/quick-pass/>
    MxQuickpass = 93,
    /// <http://appsh.chihuahua.gob.mx/transparencia/?doc=/ingresos/TelepeajeFormato4.pdf>
    MxSistemaTelepeajeChihuahua = 89,
    /// Mexico
    MxTagIave = 12,
    /// Mexico toll pass company. One of many operating in Mexico City. See
    /// additional details at <https://www.televia.com.mx.>
    MxTagTelevia = 13,
    /// Mexico toll pass company. One of many operating in Mexico City.
    /// <https://www.televia.com.mx>
    MxTelevia = 92,
    /// Mexico toll pass. See additional details at
    /// <https://www.viapass.com.mx/viapass/web_home.aspx.>
    MxViapass = 14,
    /// AL, USA.
    UsAlFreedomPass = 21,
    /// AK, USA.
    UsAkAntonAndersonTunnelBookOf10Tickets = 22,
    /// CA, USA.
    UsCaFastrak = 4,
    /// Indicates driver has any FasTrak pass in addition to the DMV issued Clean
    /// Air Vehicle (CAV) sticker.
    /// <https://www.bayareafastrak.org/en/guide/doINeedFlex.shtml>
    UsCaFastrakCavSticker = 86,
    /// CO, USA.
    UsCoExpresstoll = 23,
    /// CO, USA.
    UsCoGoPass = 24,
    /// DE, USA.
    UsDeEzpassde = 25,
    /// FL, USA.
    UsFlBobSikesTollBridgePass = 65,
    /// FL, USA.
    UsFlDunesCommunityDevelopmentDistrictExpresscard = 66,
    /// FL, USA.
    UsFlEpass = 67,
    /// FL, USA.
    UsFlGibaTollPass = 68,
    /// FL, USA.
    UsFlLeeway = 69,
    /// FL, USA.
    UsFlSunpass = 70,
    /// FL, USA.
    UsFlSunpassPro = 71,
    /// IL, USA.
    UsIlEzpassil = 73,
    /// IL, USA.
    UsIlIpass = 72,
    /// IN, USA.
    UsInEzpassin = 26,
    /// KS, USA.
    UsKsBestpassHorizon = 27,
    /// KS, USA.
    UsKsKtag = 28,
    /// KS, USA.
    UsKsNationalpass = 29,
    /// KS, USA.
    UsKsPrepassElitepass = 30,
    /// KY, USA.
    UsKyRiverlink = 31,
    /// LA, USA.
    UsLaGeauxpass = 32,
    /// LA, USA.
    UsLaTollTag = 33,
    /// MA, USA.
    UsMaEzpassma = 6,
    /// MD, USA.
    UsMdEzpassmd = 34,
    /// ME, USA.
    UsMeEzpassme = 35,
    /// MI, USA.
    UsMiAmbassadorBridgePremierCommuterCard = 36,
    /// MI, USA.
    UsMiGrosseIleTollBridgePassTag = 37,
    /// MI, USA.
    UsMiIqProxCard = 38,
    /// MI, USA.
    UsMiMackinacBridgeMacPass = 39,
    /// MI, USA.
    UsMiNexpressToll = 40,
    /// MN, USA.
    UsMnEzpassmn = 41,
    /// NC, USA.
    UsNcEzpassnc = 42,
    /// NC, USA.
    UsNcPeachPass = 87,
    /// NC, USA.
    UsNcQuickPass = 43,
    /// NH, USA.
    UsNhEzpassnh = 80,
    /// NJ, USA.
    UsNjDownbeachExpressPass = 75,
    /// NJ, USA.
    UsNjEzpassnj = 74,
    /// NY, USA.
    UsNyExpresspass = 76,
    /// NY, USA.
    UsNyEzpassny = 77,
    /// OH, USA.
    UsOhEzpassoh = 44,
    /// PA, USA.
    UsPaEzpasspa = 45,
    /// RI, USA.
    UsRiEzpassri = 46,
    /// SC, USA.
    UsScPalpass = 47,
    /// TX, USA.
    UsTxBancpass = 48,
    /// TX, USA.
    UsTxDelRioPass = 49,
    /// TX, USA.
    UsTxEfastPass = 50,
    /// TX, USA.
    UsTxEaglePassExpressCard = 51,
    /// TX, USA.
    UsTxEptoll = 52,
    /// TX, USA.
    UsTxEzCross = 53,
    /// TX, USA.
    UsTxEztag = 54,
    /// TX, USA.
    UsTxLaredoTradeTag = 55,
    /// TX, USA.
    UsTxPluspass = 56,
    /// TX, USA.
    UsTxTolltag = 57,
    /// TX, USA.
    UsTxTxtag = 58,
    /// TX, USA.
    UsTxXpressCard = 59,
    /// UT, USA.
    UsUtAdamsAveParkwayExpresscard = 60,
    /// VA, USA.
    UsVaEzpassva = 61,
    /// WA, USA.
    UsWaBreezeby = 17,
    /// WA, USA.
    UsWaGoodToGo = 1,
    /// WV, USA.
    UsWvEzpasswv = 62,
    /// WV, USA.
    UsWvMemorialBridgeTickets = 63,
    /// WV, USA.
    UsWvNewellTollBridgeTicket = 64,
}
impl TollPass {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TollPass::Unspecified => "TOLL_PASS_UNSPECIFIED",
            TollPass::AuEtollTag => "AU_ETOLL_TAG",
            TollPass::AuEwayTag => "AU_EWAY_TAG",
            TollPass::AuLinkt => "AU_LINKT",
            TollPass::ArTelepase => "AR_TELEPASE",
            TollPass::BrAutoExpreso => "BR_AUTO_EXPRESO",
            TollPass::BrConectcar => "BR_CONECTCAR",
            TollPass::BrMoveMais => "BR_MOVE_MAIS",
            TollPass::BrPassaRapido => "BR_PASSA_RAPIDO",
            TollPass::BrSemParar => "BR_SEM_PARAR",
            TollPass::BrTaggy => "BR_TAGGY",
            TollPass::BrVeloe => "BR_VELOE",
            TollPass::CaUsAkwasasneSeawayCorporateCard => {
                "CA_US_AKWASASNE_SEAWAY_CORPORATE_CARD"
            }
            TollPass::CaUsAkwasasneSeawayTransitCard => {
                "CA_US_AKWASASNE_SEAWAY_TRANSIT_CARD"
            }
            TollPass::CaUsBlueWaterEdgePass => "CA_US_BLUE_WATER_EDGE_PASS",
            TollPass::CaUsConnexion => "CA_US_CONNEXION",
            TollPass::CaUsNexusCard => "CA_US_NEXUS_CARD",
            TollPass::IdEToll => "ID_E_TOLL",
            TollPass::InFastag => "IN_FASTAG",
            TollPass::InLocalHpPlateExempt => "IN_LOCAL_HP_PLATE_EXEMPT",
            TollPass::MxIave => "MX_IAVE",
            TollPass::MxPase => "MX_PASE",
            TollPass::MxQuickpass => "MX_QUICKPASS",
            TollPass::MxSistemaTelepeajeChihuahua => "MX_SISTEMA_TELEPEAJE_CHIHUAHUA",
            TollPass::MxTagIave => "MX_TAG_IAVE",
            TollPass::MxTagTelevia => "MX_TAG_TELEVIA",
            TollPass::MxTelevia => "MX_TELEVIA",
            TollPass::MxViapass => "MX_VIAPASS",
            TollPass::UsAlFreedomPass => "US_AL_FREEDOM_PASS",
            TollPass::UsAkAntonAndersonTunnelBookOf10Tickets => {
                "US_AK_ANTON_ANDERSON_TUNNEL_BOOK_OF_10_TICKETS"
            }
            TollPass::UsCaFastrak => "US_CA_FASTRAK",
            TollPass::UsCaFastrakCavSticker => "US_CA_FASTRAK_CAV_STICKER",
            TollPass::UsCoExpresstoll => "US_CO_EXPRESSTOLL",
            TollPass::UsCoGoPass => "US_CO_GO_PASS",
            TollPass::UsDeEzpassde => "US_DE_EZPASSDE",
            TollPass::UsFlBobSikesTollBridgePass => "US_FL_BOB_SIKES_TOLL_BRIDGE_PASS",
            TollPass::UsFlDunesCommunityDevelopmentDistrictExpresscard => {
                "US_FL_DUNES_COMMUNITY_DEVELOPMENT_DISTRICT_EXPRESSCARD"
            }
            TollPass::UsFlEpass => "US_FL_EPASS",
            TollPass::UsFlGibaTollPass => "US_FL_GIBA_TOLL_PASS",
            TollPass::UsFlLeeway => "US_FL_LEEWAY",
            TollPass::UsFlSunpass => "US_FL_SUNPASS",
            TollPass::UsFlSunpassPro => "US_FL_SUNPASS_PRO",
            TollPass::UsIlEzpassil => "US_IL_EZPASSIL",
            TollPass::UsIlIpass => "US_IL_IPASS",
            TollPass::UsInEzpassin => "US_IN_EZPASSIN",
            TollPass::UsKsBestpassHorizon => "US_KS_BESTPASS_HORIZON",
            TollPass::UsKsKtag => "US_KS_KTAG",
            TollPass::UsKsNationalpass => "US_KS_NATIONALPASS",
            TollPass::UsKsPrepassElitepass => "US_KS_PREPASS_ELITEPASS",
            TollPass::UsKyRiverlink => "US_KY_RIVERLINK",
            TollPass::UsLaGeauxpass => "US_LA_GEAUXPASS",
            TollPass::UsLaTollTag => "US_LA_TOLL_TAG",
            TollPass::UsMaEzpassma => "US_MA_EZPASSMA",
            TollPass::UsMdEzpassmd => "US_MD_EZPASSMD",
            TollPass::UsMeEzpassme => "US_ME_EZPASSME",
            TollPass::UsMiAmbassadorBridgePremierCommuterCard => {
                "US_MI_AMBASSADOR_BRIDGE_PREMIER_COMMUTER_CARD"
            }
            TollPass::UsMiGrosseIleTollBridgePassTag => {
                "US_MI_GROSSE_ILE_TOLL_BRIDGE_PASS_TAG"
            }
            TollPass::UsMiIqProxCard => "US_MI_IQ_PROX_CARD",
            TollPass::UsMiMackinacBridgeMacPass => "US_MI_MACKINAC_BRIDGE_MAC_PASS",
            TollPass::UsMiNexpressToll => "US_MI_NEXPRESS_TOLL",
            TollPass::UsMnEzpassmn => "US_MN_EZPASSMN",
            TollPass::UsNcEzpassnc => "US_NC_EZPASSNC",
            TollPass::UsNcPeachPass => "US_NC_PEACH_PASS",
            TollPass::UsNcQuickPass => "US_NC_QUICK_PASS",
            TollPass::UsNhEzpassnh => "US_NH_EZPASSNH",
            TollPass::UsNjDownbeachExpressPass => "US_NJ_DOWNBEACH_EXPRESS_PASS",
            TollPass::UsNjEzpassnj => "US_NJ_EZPASSNJ",
            TollPass::UsNyExpresspass => "US_NY_EXPRESSPASS",
            TollPass::UsNyEzpassny => "US_NY_EZPASSNY",
            TollPass::UsOhEzpassoh => "US_OH_EZPASSOH",
            TollPass::UsPaEzpasspa => "US_PA_EZPASSPA",
            TollPass::UsRiEzpassri => "US_RI_EZPASSRI",
            TollPass::UsScPalpass => "US_SC_PALPASS",
            TollPass::UsTxBancpass => "US_TX_BANCPASS",
            TollPass::UsTxDelRioPass => "US_TX_DEL_RIO_PASS",
            TollPass::UsTxEfastPass => "US_TX_EFAST_PASS",
            TollPass::UsTxEaglePassExpressCard => "US_TX_EAGLE_PASS_EXPRESS_CARD",
            TollPass::UsTxEptoll => "US_TX_EPTOLL",
            TollPass::UsTxEzCross => "US_TX_EZ_CROSS",
            TollPass::UsTxEztag => "US_TX_EZTAG",
            TollPass::UsTxLaredoTradeTag => "US_TX_LAREDO_TRADE_TAG",
            TollPass::UsTxPluspass => "US_TX_PLUSPASS",
            TollPass::UsTxTolltag => "US_TX_TOLLTAG",
            TollPass::UsTxTxtag => "US_TX_TXTAG",
            TollPass::UsTxXpressCard => "US_TX_XPRESS_CARD",
            TollPass::UsUtAdamsAveParkwayExpresscard => {
                "US_UT_ADAMS_AVE_PARKWAY_EXPRESSCARD"
            }
            TollPass::UsVaEzpassva => "US_VA_EZPASSVA",
            TollPass::UsWaBreezeby => "US_WA_BREEZEBY",
            TollPass::UsWaGoodToGo => "US_WA_GOOD_TO_GO",
            TollPass::UsWvEzpasswv => "US_WV_EZPASSWV",
            TollPass::UsWvMemorialBridgeTickets => "US_WV_MEMORIAL_BRIDGE_TICKETS",
            TollPass::UsWvNewellTollBridgeTicket => "US_WV_NEWELL_TOLL_BRIDGE_TICKET",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TOLL_PASS_UNSPECIFIED" => Some(Self::Unspecified),
            "AU_ETOLL_TAG" => Some(Self::AuEtollTag),
            "AU_EWAY_TAG" => Some(Self::AuEwayTag),
            "AU_LINKT" => Some(Self::AuLinkt),
            "AR_TELEPASE" => Some(Self::ArTelepase),
            "BR_AUTO_EXPRESO" => Some(Self::BrAutoExpreso),
            "BR_CONECTCAR" => Some(Self::BrConectcar),
            "BR_MOVE_MAIS" => Some(Self::BrMoveMais),
            "BR_PASSA_RAPIDO" => Some(Self::BrPassaRapido),
            "BR_SEM_PARAR" => Some(Self::BrSemParar),
            "BR_TAGGY" => Some(Self::BrTaggy),
            "BR_VELOE" => Some(Self::BrVeloe),
            "CA_US_AKWASASNE_SEAWAY_CORPORATE_CARD" => {
                Some(Self::CaUsAkwasasneSeawayCorporateCard)
            }
            "CA_US_AKWASASNE_SEAWAY_TRANSIT_CARD" => {
                Some(Self::CaUsAkwasasneSeawayTransitCard)
            }
            "CA_US_BLUE_WATER_EDGE_PASS" => Some(Self::CaUsBlueWaterEdgePass),
            "CA_US_CONNEXION" => Some(Self::CaUsConnexion),
            "CA_US_NEXUS_CARD" => Some(Self::CaUsNexusCard),
            "ID_E_TOLL" => Some(Self::IdEToll),
            "IN_FASTAG" => Some(Self::InFastag),
            "IN_LOCAL_HP_PLATE_EXEMPT" => Some(Self::InLocalHpPlateExempt),
            "MX_IAVE" => Some(Self::MxIave),
            "MX_PASE" => Some(Self::MxPase),
            "MX_QUICKPASS" => Some(Self::MxQuickpass),
            "MX_SISTEMA_TELEPEAJE_CHIHUAHUA" => Some(Self::MxSistemaTelepeajeChihuahua),
            "MX_TAG_IAVE" => Some(Self::MxTagIave),
            "MX_TAG_TELEVIA" => Some(Self::MxTagTelevia),
            "MX_TELEVIA" => Some(Self::MxTelevia),
            "MX_VIAPASS" => Some(Self::MxViapass),
            "US_AL_FREEDOM_PASS" => Some(Self::UsAlFreedomPass),
            "US_AK_ANTON_ANDERSON_TUNNEL_BOOK_OF_10_TICKETS" => {
                Some(Self::UsAkAntonAndersonTunnelBookOf10Tickets)
            }
            "US_CA_FASTRAK" => Some(Self::UsCaFastrak),
            "US_CA_FASTRAK_CAV_STICKER" => Some(Self::UsCaFastrakCavSticker),
            "US_CO_EXPRESSTOLL" => Some(Self::UsCoExpresstoll),
            "US_CO_GO_PASS" => Some(Self::UsCoGoPass),
            "US_DE_EZPASSDE" => Some(Self::UsDeEzpassde),
            "US_FL_BOB_SIKES_TOLL_BRIDGE_PASS" => Some(Self::UsFlBobSikesTollBridgePass),
            "US_FL_DUNES_COMMUNITY_DEVELOPMENT_DISTRICT_EXPRESSCARD" => {
                Some(Self::UsFlDunesCommunityDevelopmentDistrictExpresscard)
            }
            "US_FL_EPASS" => Some(Self::UsFlEpass),
            "US_FL_GIBA_TOLL_PASS" => Some(Self::UsFlGibaTollPass),
            "US_FL_LEEWAY" => Some(Self::UsFlLeeway),
            "US_FL_SUNPASS" => Some(Self::UsFlSunpass),
            "US_FL_SUNPASS_PRO" => Some(Self::UsFlSunpassPro),
            "US_IL_EZPASSIL" => Some(Self::UsIlEzpassil),
            "US_IL_IPASS" => Some(Self::UsIlIpass),
            "US_IN_EZPASSIN" => Some(Self::UsInEzpassin),
            "US_KS_BESTPASS_HORIZON" => Some(Self::UsKsBestpassHorizon),
            "US_KS_KTAG" => Some(Self::UsKsKtag),
            "US_KS_NATIONALPASS" => Some(Self::UsKsNationalpass),
            "US_KS_PREPASS_ELITEPASS" => Some(Self::UsKsPrepassElitepass),
            "US_KY_RIVERLINK" => Some(Self::UsKyRiverlink),
            "US_LA_GEAUXPASS" => Some(Self::UsLaGeauxpass),
            "US_LA_TOLL_TAG" => Some(Self::UsLaTollTag),
            "US_MA_EZPASSMA" => Some(Self::UsMaEzpassma),
            "US_MD_EZPASSMD" => Some(Self::UsMdEzpassmd),
            "US_ME_EZPASSME" => Some(Self::UsMeEzpassme),
            "US_MI_AMBASSADOR_BRIDGE_PREMIER_COMMUTER_CARD" => {
                Some(Self::UsMiAmbassadorBridgePremierCommuterCard)
            }
            "US_MI_GROSSE_ILE_TOLL_BRIDGE_PASS_TAG" => {
                Some(Self::UsMiGrosseIleTollBridgePassTag)
            }
            "US_MI_IQ_PROX_CARD" => Some(Self::UsMiIqProxCard),
            "US_MI_MACKINAC_BRIDGE_MAC_PASS" => Some(Self::UsMiMackinacBridgeMacPass),
            "US_MI_NEXPRESS_TOLL" => Some(Self::UsMiNexpressToll),
            "US_MN_EZPASSMN" => Some(Self::UsMnEzpassmn),
            "US_NC_EZPASSNC" => Some(Self::UsNcEzpassnc),
            "US_NC_PEACH_PASS" => Some(Self::UsNcPeachPass),
            "US_NC_QUICK_PASS" => Some(Self::UsNcQuickPass),
            "US_NH_EZPASSNH" => Some(Self::UsNhEzpassnh),
            "US_NJ_DOWNBEACH_EXPRESS_PASS" => Some(Self::UsNjDownbeachExpressPass),
            "US_NJ_EZPASSNJ" => Some(Self::UsNjEzpassnj),
            "US_NY_EXPRESSPASS" => Some(Self::UsNyExpresspass),
            "US_NY_EZPASSNY" => Some(Self::UsNyEzpassny),
            "US_OH_EZPASSOH" => Some(Self::UsOhEzpassoh),
            "US_PA_EZPASSPA" => Some(Self::UsPaEzpasspa),
            "US_RI_EZPASSRI" => Some(Self::UsRiEzpassri),
            "US_SC_PALPASS" => Some(Self::UsScPalpass),
            "US_TX_BANCPASS" => Some(Self::UsTxBancpass),
            "US_TX_DEL_RIO_PASS" => Some(Self::UsTxDelRioPass),
            "US_TX_EFAST_PASS" => Some(Self::UsTxEfastPass),
            "US_TX_EAGLE_PASS_EXPRESS_CARD" => Some(Self::UsTxEaglePassExpressCard),
            "US_TX_EPTOLL" => Some(Self::UsTxEptoll),
            "US_TX_EZ_CROSS" => Some(Self::UsTxEzCross),
            "US_TX_EZTAG" => Some(Self::UsTxEztag),
            "US_TX_LAREDO_TRADE_TAG" => Some(Self::UsTxLaredoTradeTag),
            "US_TX_PLUSPASS" => Some(Self::UsTxPluspass),
            "US_TX_TOLLTAG" => Some(Self::UsTxTolltag),
            "US_TX_TXTAG" => Some(Self::UsTxTxtag),
            "US_TX_XPRESS_CARD" => Some(Self::UsTxXpressCard),
            "US_UT_ADAMS_AVE_PARKWAY_EXPRESSCARD" => {
                Some(Self::UsUtAdamsAveParkwayExpresscard)
            }
            "US_VA_EZPASSVA" => Some(Self::UsVaEzpassva),
            "US_WA_BREEZEBY" => Some(Self::UsWaBreezeby),
            "US_WA_GOOD_TO_GO" => Some(Self::UsWaGoodToGo),
            "US_WV_EZPASSWV" => Some(Self::UsWvEzpasswv),
            "US_WV_MEMORIAL_BRIDGE_TICKETS" => Some(Self::UsWvMemorialBridgeTickets),
            "US_WV_NEWELL_TOLL_BRIDGE_TICKET" => Some(Self::UsWvNewellTollBridgeTicket),
            _ => None,
        }
    }
}
/// Labels for the `Route` that are useful to identify specific properties
/// of the route to compare against others.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RouteLabel {
    /// Default - not used.
    Unspecified = 0,
    /// The default "best" route returned for the route computation.
    DefaultRoute = 1,
    /// An alternative to the default "best" route. Routes like this will be
    /// returned when `ComputeRoutesRequest.compute_alternative_routes` is
    /// specified.
    DefaultRouteAlternate = 2,
    /// Fuel efficient route. Routes labeled with this value are determined to be
    /// optimized for Eco parameters such as fuel consumption.
    FuelEfficient = 3,
}
impl RouteLabel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RouteLabel::Unspecified => "ROUTE_LABEL_UNSPECIFIED",
            RouteLabel::DefaultRoute => "DEFAULT_ROUTE",
            RouteLabel::DefaultRouteAlternate => "DEFAULT_ROUTE_ALTERNATE",
            RouteLabel::FuelEfficient => "FUEL_EFFICIENT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ROUTE_LABEL_UNSPECIFIED" => Some(Self::Unspecified),
            "DEFAULT_ROUTE" => Some(Self::DefaultRoute),
            "DEFAULT_ROUTE_ALTERNATE" => Some(Self::DefaultRouteAlternate),
            "FUEL_EFFICIENT" => Some(Self::FuelEfficient),
            _ => None,
        }
    }
}
/// A set of values describing the vehicle's emission type.
/// Applies only to the DRIVE travel mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VehicleEmissionType {
    /// No emission type specified. Default to GASOLINE.
    Unspecified = 0,
    /// Gasoline/petrol fueled vehicle.
    Gasoline = 1,
    /// Electricity powered vehicle.
    Electric = 2,
    /// Hybrid fuel (such as gasoline + electric) vehicle.
    Hybrid = 3,
    /// Diesel fueled vehicle.
    Diesel = 4,
}
impl VehicleEmissionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VehicleEmissionType::Unspecified => "VEHICLE_EMISSION_TYPE_UNSPECIFIED",
            VehicleEmissionType::Gasoline => "GASOLINE",
            VehicleEmissionType::Electric => "ELECTRIC",
            VehicleEmissionType::Hybrid => "HYBRID",
            VehicleEmissionType::Diesel => "DIESEL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VEHICLE_EMISSION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "GASOLINE" => Some(Self::Gasoline),
            "ELECTRIC" => Some(Self::Electric),
            "HYBRID" => Some(Self::Hybrid),
            "DIESEL" => Some(Self::Diesel),
            _ => None,
        }
    }
}
/// Encapsulates the vehicle information, such as the license plate last
/// character.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VehicleInfo {
    /// Describes the vehicle's emission type.
    /// Applies only to the DRIVE travel mode.
    #[prost(enumeration = "VehicleEmissionType", tag = "2")]
    pub emission_type: i32,
}
/// A set of values that specify the unit of measure used in the display.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Units {
    /// Units of measure not specified. Defaults to the unit of measure inferred
    /// from the request.
    Unspecified = 0,
    /// Metric units of measure.
    Metric = 1,
    /// Imperial (English) units of measure.
    Imperial = 2,
}
impl Units {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Units::Unspecified => "UNITS_UNSPECIFIED",
            Units::Metric => "METRIC",
            Units::Imperial => "IMPERIAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNITS_UNSPECIFIED" => Some(Self::Unspecified),
            "METRIC" => Some(Self::Metric),
            "IMPERIAL" => Some(Self::Imperial),
            _ => None,
        }
    }
}
/// Encapsulates toll information on a `Route` or on a `RouteLeg`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TollInfo {
    /// The monetary amount of tolls for the corresponding Route or RouteLeg.
    /// This list contains a money amount for each currency that is expected
    /// to be charged by the toll stations. Typically this list will contain only
    /// one item for routes with tolls in one currency. For international trips,
    /// this list may contain multiple items to reflect tolls in different
    /// currencies.
    #[prost(message, repeated, tag = "1")]
    pub estimated_price: ::prost::alloc::vec::Vec<super::super::super::r#type::Money>,
}
/// Traffic density indicator on a contiguous segment of a polyline or path.
/// Given a path with points P_0, P_1, ... , P_N (zero-based index), the
/// SpeedReadingInterval defines an interval and describes its traffic using the
/// following categories.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeedReadingInterval {
    /// The starting index of this interval in the polyline.
    #[prost(int32, optional, tag = "1")]
    pub start_polyline_point_index: ::core::option::Option<i32>,
    /// The ending index of this interval in the polyline.
    #[prost(int32, optional, tag = "2")]
    pub end_polyline_point_index: ::core::option::Option<i32>,
    /// Traffic speed in this interval.
    #[prost(enumeration = "speed_reading_interval::Speed", tag = "3")]
    pub speed: i32,
}
/// Nested message and enum types in `SpeedReadingInterval`.
pub mod speed_reading_interval {
    /// The classification of polyline speed based on traffic data.
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
    pub enum Speed {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// Normal speed, no slowdown is detected.
        Normal = 1,
        /// Slowdown detected, but no traffic jam formed.
        Slow = 2,
        /// Traffic jam detected.
        TrafficJam = 3,
    }
    impl Speed {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Speed::Unspecified => "SPEED_UNSPECIFIED",
                Speed::Normal => "NORMAL",
                Speed::Slow => "SLOW",
                Speed::TrafficJam => "TRAFFIC_JAM",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SPEED_UNSPECIFIED" => Some(Self::Unspecified),
                "NORMAL" => Some(Self::Normal),
                "SLOW" => Some(Self::Slow),
                "TRAFFIC_JAM" => Some(Self::TrafficJam),
                _ => None,
            }
        }
    }
}
/// Encapsulates an encoded polyline.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Polyline {
    /// Encapsulates the type of polyline. Defaults to encoded_polyline.
    #[prost(oneof = "polyline::PolylineType", tags = "1, 2")]
    pub polyline_type: ::core::option::Option<polyline::PolylineType>,
}
/// Nested message and enum types in `Polyline`.
pub mod polyline {
    /// Encapsulates the type of polyline. Defaults to encoded_polyline.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PolylineType {
        /// The string encoding of the polyline using the [polyline encoding
        /// algorithm](<https://developers.google.com/maps/documentation/utilities/polylinealgorithm>)
        #[prost(string, tag = "1")]
        EncodedPolyline(::prost::alloc::string::String),
        /// Specifies a polyline using the [GeoJSON LineString
        /// format](<https://tools.ietf.org/html/rfc7946#section-3.1.4>)
        #[prost(message, tag = "2")]
        GeoJsonLinestring(::prost_types::Struct),
    }
}
/// A set of values that specify the quality of the polyline.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PolylineQuality {
    /// No polyline quality preference specified. Defaults to `OVERVIEW`.
    Unspecified = 0,
    /// Specifies a high-quality polyline - which is composed using more points
    /// than `OVERVIEW`, at the cost of increased response size. Use this value
    /// when you need more precision.
    HighQuality = 1,
    /// Specifies an overview polyline - which is composed using a small number of
    /// points. Use this value when displaying an overview of the route. Using this
    /// option has a lower request latency compared to using the
    /// `HIGH_QUALITY` option.
    Overview = 2,
}
impl PolylineQuality {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PolylineQuality::Unspecified => "POLYLINE_QUALITY_UNSPECIFIED",
            PolylineQuality::HighQuality => "HIGH_QUALITY",
            PolylineQuality::Overview => "OVERVIEW",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "POLYLINE_QUALITY_UNSPECIFIED" => Some(Self::Unspecified),
            "HIGH_QUALITY" => Some(Self::HighQuality),
            "OVERVIEW" => Some(Self::Overview),
            _ => None,
        }
    }
}
/// Specifies the preferred type of polyline to be returned.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PolylineEncoding {
    /// No polyline type preference specified. Defaults to `ENCODED_POLYLINE`.
    Unspecified = 0,
    /// Specifies a polyline encoded using the [polyline encoding
    /// algorithm](<https://developers.google.com/maps/documentation/utilities/polylinealgorithm>).
    EncodedPolyline = 1,
    /// Specifies a polyline using the [GeoJSON LineString
    /// format](<https://tools.ietf.org/html/rfc7946#section-3.1.4>)
    GeoJsonLinestring = 2,
}
impl PolylineEncoding {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PolylineEncoding::Unspecified => "POLYLINE_ENCODING_UNSPECIFIED",
            PolylineEncoding::EncodedPolyline => "ENCODED_POLYLINE",
            PolylineEncoding::GeoJsonLinestring => "GEO_JSON_LINESTRING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "POLYLINE_ENCODING_UNSPECIFIED" => Some(Self::Unspecified),
            "ENCODED_POLYLINE" => Some(Self::EncodedPolyline),
            "GEO_JSON_LINESTRING" => Some(Self::GeoJsonLinestring),
            _ => None,
        }
    }
}
/// Encapsulates a route, which consists of a series of connected road segments
/// that join beginning, ending, and intermediate waypoints.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Route {
    /// Labels for the `Route` that are useful to identify specific properties
    /// of the route to compare against others.
    #[prost(enumeration = "RouteLabel", repeated, tag = "13")]
    pub route_labels: ::prost::alloc::vec::Vec<i32>,
    /// A collection of legs (path segments between waypoints) that make-up the
    /// route. Each leg corresponds to the trip between two non-`via` Waypoints.
    /// For example, a route with no intermediate waypoints has only one leg. A
    /// route that includes one non-`via` intermediate waypoint has two legs. A
    /// route that includes one `via` intermediate waypoint has one leg. The order
    /// of the legs matches the order of Waypoints from `origin` to `intermediates`
    /// to `destination`.
    #[prost(message, repeated, tag = "1")]
    pub legs: ::prost::alloc::vec::Vec<RouteLeg>,
    /// The travel distance of the route, in meters.
    #[prost(int32, tag = "2")]
    pub distance_meters: i32,
    /// The length of time needed to navigate the route. If you set the
    /// `routing_preference` to `TRAFFIC_UNAWARE`, then this value is the same as
    /// `static_duration`. If you set the `routing_preference` to either
    /// `TRAFFIC_AWARE` or `TRAFFIC_AWARE_OPTIMAL`, then this value is calculated
    /// taking traffic conditions into account.
    #[prost(message, optional, tag = "3")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// The duration of traveling through the route without taking traffic
    /// conditions into consideration.
    #[prost(message, optional, tag = "4")]
    pub static_duration: ::core::option::Option<::prost_types::Duration>,
    /// The overall route polyline. This polyline will be the combined polyline of
    /// all `legs`.
    #[prost(message, optional, tag = "5")]
    pub polyline: ::core::option::Option<Polyline>,
    /// A description of the route.
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
    /// An array of warnings to show when displaying the route.
    #[prost(string, repeated, tag = "7")]
    pub warnings: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The viewport bounding box of the polyline.
    #[prost(message, optional, tag = "8")]
    pub viewport: ::core::option::Option<super::super::super::geo::r#type::Viewport>,
    /// Additional information about the route.
    #[prost(message, optional, tag = "9")]
    pub travel_advisory: ::core::option::Option<RouteTravelAdvisory>,
    /// Web-safe base64 encoded route token that can be passed to NavigationSDK,
    /// which allows the Navigation SDK to reconstruct the route during navigation,
    /// and in the event of rerouting honor the original intention when Routes
    /// ComputeRoutes is called. Customers should treat this token as an
    /// opaque blob.
    /// NOTE: `Route.route_token` is only available for requests that have set
    /// `ComputeRoutesRequest.routing_preference` to `TRAFFIC_AWARE` or
    /// `TRAFFIC_AWARE_OPTIMAL`. `Route.route_token` is also not supported for
    /// requests that have Via waypoints.
    #[prost(string, tag = "12")]
    pub route_token: ::prost::alloc::string::String,
}
/// Encapsulates the additional information that the user should be informed
/// about, such as possible traffic zone restriction etc.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteTravelAdvisory {
    /// Encapsulates information about tolls on the Route.
    /// This field is only populated if we expect there are tolls on the Route.
    /// If this field is set but the estimated_price subfield is not populated,
    /// we expect that road contains tolls but we do not know an estimated price.
    /// If this field is not set, then we expect there is no toll on the Route.
    #[prost(message, optional, tag = "2")]
    pub toll_info: ::core::option::Option<TollInfo>,
    /// Speed reading intervals detailing traffic density. Applicable in case of
    /// `TRAFFIC_AWARE` and `TRAFFIC_AWARE_OPTIMAL` routing preferences.
    /// The intervals cover the entire polyline of the route without overlap.
    /// The start point of a specified interval is the same as the end point of the
    /// preceding interval.
    ///
    /// Example:
    ///
    ///      polyline: A ---- B ---- C ---- D ---- E ---- F ---- G
    ///      speed_reading_intervals: [A,C), [C,D), [D,G).
    #[prost(message, repeated, tag = "3")]
    pub speed_reading_intervals: ::prost::alloc::vec::Vec<SpeedReadingInterval>,
    /// The fuel consumption prediction in microliters.
    #[prost(int64, tag = "5")]
    pub fuel_consumption_microliters: i64,
}
/// Encapsulates the additional information that the user should be informed
/// about, such as possible traffic zone restriction etc. on a route leg.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteLegTravelAdvisory {
    /// Encapsulates information about tolls on the specific RouteLeg.
    /// This field is only populated if we expect there are tolls on the RouteLeg.
    /// If this field is set but the estimated_price subfield is not populated,
    /// we expect that road contains tolls but we do not know an estimated price.
    /// If this field does not exist, then there is no toll on the RouteLeg.
    #[prost(message, optional, tag = "1")]
    pub toll_info: ::core::option::Option<TollInfo>,
    /// Speed reading intervals detailing traffic density. Applicable in case of
    /// `TRAFFIC_AWARE` and `TRAFFIC_AWARE_OPTIMAL` routing preferences.
    /// The intervals cover the entire polyline of the RouteLg without overlap.
    /// The start point of a specified interval is the same as the end point of the
    /// preceding interval.
    ///
    /// Example:
    ///
    ///      polyline: A ---- B ---- C ---- D ---- E ---- F ---- G
    ///      speed_reading_intervals: [A,C), [C,D), [D,G).
    #[prost(message, repeated, tag = "2")]
    pub speed_reading_intervals: ::prost::alloc::vec::Vec<SpeedReadingInterval>,
}
/// Encapsulates the additional information that the user should be informed
/// about, such as possible traffic zone restriction on a leg step.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteLegStepTravelAdvisory {
    /// NOTE: This field is not currently populated.
    #[prost(message, repeated, tag = "1")]
    pub speed_reading_intervals: ::prost::alloc::vec::Vec<SpeedReadingInterval>,
}
/// Encapsulates a segment between non-`via` waypoints.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteLeg {
    /// The travel distance of the route leg, in meters.
    #[prost(int32, tag = "1")]
    pub distance_meters: i32,
    /// The length of time needed to navigate the leg. If the `route_preference`
    /// is set to `TRAFFIC_UNAWARE`, then this value is the same as
    /// `static_duration`. If the `route_preference` is either `TRAFFIC_AWARE` or
    /// `TRAFFIC_AWARE_OPTIMAL`, then this value is calculated taking traffic
    /// conditions into account.
    #[prost(message, optional, tag = "2")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// The duration of traveling through the leg, calculated without taking
    /// traffic conditions into consideration.
    #[prost(message, optional, tag = "3")]
    pub static_duration: ::core::option::Option<::prost_types::Duration>,
    /// The overall polyline for this leg. This includes that each `step`'s
    /// polyline.
    #[prost(message, optional, tag = "4")]
    pub polyline: ::core::option::Option<Polyline>,
    /// The start location of this leg. This might be different from the provided
    /// `origin`. For example, when the provided `origin` is not near a road, this
    /// is a point on the road.
    #[prost(message, optional, tag = "5")]
    pub start_location: ::core::option::Option<Location>,
    /// The end location of this leg. This might be different from the provided
    /// `destination`. For example, when the provided `destination` is not near a
    /// road, this is a point on the road.
    #[prost(message, optional, tag = "6")]
    pub end_location: ::core::option::Option<Location>,
    /// An array of steps denoting segments within this leg. Each step represents
    /// one navigation instruction.
    #[prost(message, repeated, tag = "7")]
    pub steps: ::prost::alloc::vec::Vec<RouteLegStep>,
    /// Encapsulates the additional information that the user should be informed
    /// about, such as possible traffic zone restriction etc. on a route leg.
    #[prost(message, optional, tag = "8")]
    pub travel_advisory: ::core::option::Option<RouteLegTravelAdvisory>,
}
/// Encapsulates a segment of a `RouteLeg`. A step corresponds to a single
/// navigation instruction. Route legs are made up of steps.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteLegStep {
    /// The travel distance of this step, in meters. In some circumstances, this
    /// field might not have a value.
    #[prost(int32, tag = "1")]
    pub distance_meters: i32,
    /// The duration of travel through this step without taking traffic conditions
    /// into consideration. In some circumstances, this field might not have a
    /// value.
    #[prost(message, optional, tag = "2")]
    pub static_duration: ::core::option::Option<::prost_types::Duration>,
    /// The polyline associated with this step.
    #[prost(message, optional, tag = "3")]
    pub polyline: ::core::option::Option<Polyline>,
    /// The start location of this step.
    #[prost(message, optional, tag = "4")]
    pub start_location: ::core::option::Option<Location>,
    /// The end location of this step.
    #[prost(message, optional, tag = "5")]
    pub end_location: ::core::option::Option<Location>,
    /// Navigation instructions.
    #[prost(message, optional, tag = "6")]
    pub navigation_instruction: ::core::option::Option<NavigationInstruction>,
    /// Encapsulates the additional information that the user should be informed
    /// about, such as possible traffic zone restriction on a leg step.
    #[prost(message, optional, tag = "7")]
    pub travel_advisory: ::core::option::Option<RouteLegStepTravelAdvisory>,
}
/// Contains GeocodedWaypoints for origin, destination and intermediate
/// waypoints. Only populated for address waypoints.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeocodingResults {
    /// Origin geocoded waypoint.
    #[prost(message, optional, tag = "1")]
    pub origin: ::core::option::Option<GeocodedWaypoint>,
    /// Destination geocoded waypoint.
    #[prost(message, optional, tag = "2")]
    pub destination: ::core::option::Option<GeocodedWaypoint>,
    /// A list of intermediate geocoded waypoints each containing an index field
    /// that corresponds to the zero-based position of the waypoint in the order
    /// they were specified in the request.
    #[prost(message, repeated, tag = "3")]
    pub intermediates: ::prost::alloc::vec::Vec<GeocodedWaypoint>,
}
/// Details about the locations used as waypoints. Only populated for address
/// waypoints. Includes details about the geocoding results for the purposes of
/// determining what the address was geocoded to.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeocodedWaypoint {
    /// Indicates the status code resulting from the geocoding operation.
    #[prost(message, optional, tag = "1")]
    pub geocoder_status: ::core::option::Option<super::super::super::rpc::Status>,
    /// The index of the corresponding intermediate waypoint in the request.
    /// Only populated if the corresponding waypoint is an intermediate
    /// waypoint.
    #[prost(int32, optional, tag = "2")]
    pub intermediate_waypoint_request_index: ::core::option::Option<i32>,
    /// The type(s) of the result, in the form of zero or more type tags.
    /// Supported types:
    /// <https://developers.google.com/maps/documentation/geocoding/requests-geocoding#Types>
    #[prost(string, repeated, tag = "3")]
    pub r#type: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Indicates that the geocoder did not return an exact match for the original
    /// request, though it was able to match part of the requested address. You may
    /// wish to examine the original request for misspellings and/or an incomplete
    /// address.
    #[prost(bool, tag = "4")]
    pub partial_match: bool,
    /// The place ID for this result.
    #[prost(string, tag = "5")]
    pub place_id: ::prost::alloc::string::String,
}
/// Encapsulates a set of optional conditions to satisfy when calculating the
/// routes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteModifiers {
    /// Specifies whether to avoid toll roads where reasonable. Preference will be
    /// given to routes not containing toll roads. Applies only to the `DRIVE` and
    /// `TWO_WHEELER` travel modes.
    #[prost(bool, tag = "1")]
    pub avoid_tolls: bool,
    /// Specifies whether to avoid highways where reasonable. Preference will be
    /// given to routes not containing highways. Applies only to the `DRIVE` and
    /// `TWO_WHEELER` travel modes.
    #[prost(bool, tag = "2")]
    pub avoid_highways: bool,
    /// Specifies whether to avoid ferries where reasonable. Preference will be
    /// given to routes not containing travel by ferries.
    /// Applies only to the `DRIVE` and`TWO_WHEELER` travel modes.
    #[prost(bool, tag = "3")]
    pub avoid_ferries: bool,
    /// Specifies whether to avoid navigating indoors where reasonable. Preference
    /// will be given to routes not containing indoor navigation.
    /// Applies only to the `WALK` travel mode.
    #[prost(bool, tag = "4")]
    pub avoid_indoor: bool,
    /// Specifies the vehicle information.
    #[prost(message, optional, tag = "5")]
    pub vehicle_info: ::core::option::Option<VehicleInfo>,
    /// Encapsulates information about toll passes.
    /// If toll passes are provided, the API tries to return the pass price. If
    /// toll passes are not provided, the API treats the toll pass as unknown and
    /// tries to return the cash price.
    /// Applies only to the DRIVE and TWO_WHEELER travel modes.
    #[prost(enumeration = "TollPass", repeated, tag = "6")]
    pub toll_passes: ::prost::alloc::vec::Vec<i32>,
}
/// ComputeRoutes request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeRoutesRequest {
    /// Required. Origin waypoint.
    #[prost(message, optional, tag = "1")]
    pub origin: ::core::option::Option<Waypoint>,
    /// Required. Destination waypoint.
    #[prost(message, optional, tag = "2")]
    pub destination: ::core::option::Option<Waypoint>,
    /// Optional. A set of waypoints along the route (excluding terminal points),
    /// for either stopping at or passing by. Up to 25 intermediate waypoints are
    /// supported.
    #[prost(message, repeated, tag = "3")]
    pub intermediates: ::prost::alloc::vec::Vec<Waypoint>,
    /// Optional. Specifies the mode of transportation.
    #[prost(enumeration = "RouteTravelMode", tag = "4")]
    pub travel_mode: i32,
    /// Optional. Specifies how to compute the route. The server
    /// attempts to use the selected routing preference to compute the route. If
    ///   the routing preference results in an error or an extra long latency, then
    /// an error is returned. You can specify this option only when the
    /// `travel_mode` is `DRIVE` or `TWO_WHEELER`, otherwise the request fails.
    #[prost(enumeration = "RoutingPreference", tag = "5")]
    pub routing_preference: i32,
    /// Optional. Specifies your preference for the quality of the polyline.
    #[prost(enumeration = "PolylineQuality", tag = "6")]
    pub polyline_quality: i32,
    /// Optional. Specifies the preferred encoding for the polyline.
    #[prost(enumeration = "PolylineEncoding", tag = "12")]
    pub polyline_encoding: i32,
    /// Optional. The departure time. If you don't set this value, then this value
    /// defaults to the time that you made the request. If you set this value to a
    /// time that has already occurred, then the request fails.
    #[prost(message, optional, tag = "7")]
    pub departure_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Specifies whether to calculate alternate routes in addition to
    /// the route. No alternative routes are returned for requests that have
    /// intermediate waypoints.
    #[prost(bool, tag = "8")]
    pub compute_alternative_routes: bool,
    /// Optional. A set of conditions to satisfy that affect the way routes are
    /// calculated.
    #[prost(message, optional, tag = "9")]
    pub route_modifiers: ::core::option::Option<RouteModifiers>,
    /// Optional. The BCP-47 language code, such as "en-US" or "sr-Latn". For more
    /// information, see
    /// <http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.> See
    /// [Language Support](<https://developers.google.com/maps/faq#languagesupport>)
    /// for the list of supported languages. When you don't provide this value, the
    /// display language is inferred from the location of the route request.
    #[prost(string, tag = "10")]
    pub language_code: ::prost::alloc::string::String,
    /// Optional. Specifies the units of measure for the display fields. This
    /// includes the `instruction` field in `NavigationInstruction`. The units of
    /// measure used for the route, leg, step distance, and duration are not
    /// affected by this value. If you don't provide this value, then the display
    /// units are inferred from the location of the request.
    #[prost(enumeration = "Units", tag = "11")]
    pub units: i32,
    /// Optional. Specifies what reference routes to calculate as part of the
    /// request in addition to the default route. A reference route is a route with
    /// a different route calculation objective than the default route. For example
    /// an FUEL_EFFICIENT reference route calculation takes into account various
    /// parameters that would generate an optimal fuel efficient route.
    #[prost(
        enumeration = "compute_routes_request::ReferenceRoute",
        repeated,
        packed = "false",
        tag = "14"
    )]
    pub requested_reference_routes: ::prost::alloc::vec::Vec<i32>,
    /// Optional. A list of extra computations which may be used to complete the
    /// request. Note: These extra computations may return extra fields on the
    /// response. These extra fields must also be specified in the field mask to be
    /// returned in the response.
    #[prost(
        enumeration = "compute_routes_request::ExtraComputation",
        repeated,
        packed = "false",
        tag = "15"
    )]
    pub extra_computations: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `ComputeRoutesRequest`.
pub mod compute_routes_request {
    /// A supported reference route on the ComputeRoutesRequest.
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
    pub enum ReferenceRoute {
        /// Not used. Requests containing this value fail.
        Unspecified = 0,
        /// Fuel efficient route. Routes labeled with this value are determined to be
        /// optimized for parameters such as fuel consumption.
        FuelEfficient = 1,
    }
    impl ReferenceRoute {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ReferenceRoute::Unspecified => "REFERENCE_ROUTE_UNSPECIFIED",
                ReferenceRoute::FuelEfficient => "FUEL_EFFICIENT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REFERENCE_ROUTE_UNSPECIFIED" => Some(Self::Unspecified),
                "FUEL_EFFICIENT" => Some(Self::FuelEfficient),
                _ => None,
            }
        }
    }
    /// Extra computations to perform while completing the request.
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
    pub enum ExtraComputation {
        /// Not used. Requests containing this value will fail.
        Unspecified = 0,
        /// Toll information for the route(s).
        Tolls = 1,
        /// Estimated fuel consumption for the route(s).
        FuelConsumption = 2,
        /// Traffic aware polylines for the route(s).
        TrafficOnPolyline = 3,
    }
    impl ExtraComputation {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ExtraComputation::Unspecified => "EXTRA_COMPUTATION_UNSPECIFIED",
                ExtraComputation::Tolls => "TOLLS",
                ExtraComputation::FuelConsumption => "FUEL_CONSUMPTION",
                ExtraComputation::TrafficOnPolyline => "TRAFFIC_ON_POLYLINE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "EXTRA_COMPUTATION_UNSPECIFIED" => Some(Self::Unspecified),
                "TOLLS" => Some(Self::Tolls),
                "FUEL_CONSUMPTION" => Some(Self::FuelConsumption),
                "TRAFFIC_ON_POLYLINE" => Some(Self::TrafficOnPolyline),
                _ => None,
            }
        }
    }
}
/// ComputeRoutes the response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeRoutesResponse {
    /// Contains an array of computed routes (up to three) when you specify
    /// compute_alternatives_routes, and contains just one route when you don't.
    /// When this array contains multiple entries, the first one is the most
    /// recommended route. If the array is empty, then it means no route could be
    /// found.
    #[prost(message, repeated, tag = "1")]
    pub routes: ::prost::alloc::vec::Vec<Route>,
    /// In some cases when the server is not able to compute the route results with
    /// all of the input preferences, it may fallback to using a different way of
    /// computation. When fallback mode is used, this field contains detailed info
    /// about the fallback response. Otherwise this field is unset.
    #[prost(message, optional, tag = "2")]
    pub fallback_info: ::core::option::Option<FallbackInfo>,
    /// Contains geocoding response info for waypoints specified as addresses.
    #[prost(message, optional, tag = "3")]
    pub geocoding_results: ::core::option::Option<GeocodingResults>,
}
/// ComputeRouteMatrix request message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeRouteMatrixRequest {
    /// Required. Array of origins, which determines the rows of the response
    /// matrix. Several size restrictions apply to the cardinality of origins and
    /// destinations:
    ///
    /// * The number of elements (origins × destinations) must be no greater than
    /// 625 in any case.
    /// * The number of elements (origins × destinations) must be no greater than
    /// 100 if routing_preference is set to `TRAFFIC_AWARE_OPTIMAL`.
    /// * The number of waypoints (origins + destinations) specified as `place_id`
    /// must be no greater than 50.
    #[prost(message, repeated, tag = "1")]
    pub origins: ::prost::alloc::vec::Vec<RouteMatrixOrigin>,
    /// Required. Array of destinations, which determines the columns of the
    /// response matrix.
    #[prost(message, repeated, tag = "2")]
    pub destinations: ::prost::alloc::vec::Vec<RouteMatrixDestination>,
    /// Optional. Specifies the mode of transportation.
    #[prost(enumeration = "RouteTravelMode", tag = "3")]
    pub travel_mode: i32,
    /// Optional. Specifies how to compute the route. The server attempts to use
    /// the selected routing preference to compute the route. If the routing
    /// preference results in an error or an extra long latency, an error is
    /// returned. You can specify this option only when the `travel_mode` is
    /// `DRIVE` or `TWO_WHEELER`, otherwise the request fails.
    #[prost(enumeration = "RoutingPreference", tag = "4")]
    pub routing_preference: i32,
    /// Optional. The departure time. If you don't set this value, this defaults to
    /// the time that you made the request. If you set this value to a time that
    /// has already occurred, the request fails.
    #[prost(message, optional, tag = "5")]
    pub departure_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. A list of extra computations which may be used to complete the
    /// request. Note: These extra computations may return extra fields on the
    /// response. These extra fields must also be specified in the field mask to be
    /// returned in the response.
    #[prost(
        enumeration = "compute_route_matrix_request::ExtraComputation",
        repeated,
        packed = "false",
        tag = "8"
    )]
    pub extra_computations: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `ComputeRouteMatrixRequest`.
pub mod compute_route_matrix_request {
    /// Extra computations to perform while completing the request.
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
    pub enum ExtraComputation {
        /// Not used. Requests containing this value will fail.
        Unspecified = 0,
        /// Toll information for the matrix element(s).
        Tolls = 1,
    }
    impl ExtraComputation {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ExtraComputation::Unspecified => "EXTRA_COMPUTATION_UNSPECIFIED",
                ExtraComputation::Tolls => "TOLLS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "EXTRA_COMPUTATION_UNSPECIFIED" => Some(Self::Unspecified),
                "TOLLS" => Some(Self::Tolls),
                _ => None,
            }
        }
    }
}
/// A single origin for ComputeRouteMatrixRequest
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteMatrixOrigin {
    /// Required. Origin waypoint
    #[prost(message, optional, tag = "1")]
    pub waypoint: ::core::option::Option<Waypoint>,
    /// Optional. Modifiers for every route that takes this as the origin
    #[prost(message, optional, tag = "2")]
    pub route_modifiers: ::core::option::Option<RouteModifiers>,
}
/// A single destination for ComputeRouteMatrixRequest
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteMatrixDestination {
    /// Required. Destination waypoint
    #[prost(message, optional, tag = "1")]
    pub waypoint: ::core::option::Option<Waypoint>,
}
/// Encapsulates route information computed for an origin/destination pair in the
/// ComputeRouteMatrix API. This proto can be streamed to the client.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteMatrixElement {
    /// Zero-based index of the origin in the request.
    #[prost(int32, optional, tag = "1")]
    pub origin_index: ::core::option::Option<i32>,
    /// Zero-based index of the destination in the request.
    #[prost(int32, optional, tag = "2")]
    pub destination_index: ::core::option::Option<i32>,
    /// Error status code for this element.
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<super::super::super::rpc::Status>,
    /// Indicates whether the route was found or not. Independent of status.
    #[prost(enumeration = "RouteMatrixElementCondition", tag = "9")]
    pub condition: i32,
    /// The travel distance of the route, in meters.
    #[prost(int32, tag = "4")]
    pub distance_meters: i32,
    /// The length of time needed to navigate the route. If you set the
    /// `routing_preference` to `TRAFFIC_UNAWARE`, then this value is the same as
    /// `static_duration`. If you set the `routing_preference` to either
    /// `TRAFFIC_AWARE` or `TRAFFIC_AWARE_OPTIMAL`, then this value is calculated
    /// taking traffic conditions into account.
    #[prost(message, optional, tag = "5")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// The duration of traveling through the route without taking traffic
    /// conditions into consideration.
    #[prost(message, optional, tag = "6")]
    pub static_duration: ::core::option::Option<::prost_types::Duration>,
    /// Additional information about the route. For example: restriction
    /// information and toll information
    #[prost(message, optional, tag = "7")]
    pub travel_advisory: ::core::option::Option<RouteTravelAdvisory>,
    /// In some cases when the server is not able to compute the route with the
    /// given preferences for this particular origin/destination pair, it may
    /// fall back to using a different mode of computation. When fallback mode is
    /// used, this field contains detailed information about the fallback response.
    /// Otherwise this field is unset.
    #[prost(message, optional, tag = "8")]
    pub fallback_info: ::core::option::Option<FallbackInfo>,
}
/// The condition of the route being returned.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RouteMatrixElementCondition {
    /// Only used when the `status` of the element is not OK.
    Unspecified = 0,
    /// A route was found, and the corresponding information was filled out for the
    /// element.
    RouteExists = 1,
    /// No route could be found. Fields containing route information, such as
    /// `distance_meters` or `duration`, will not be filled out in the element.
    RouteNotFound = 2,
}
impl RouteMatrixElementCondition {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RouteMatrixElementCondition::Unspecified => {
                "ROUTE_MATRIX_ELEMENT_CONDITION_UNSPECIFIED"
            }
            RouteMatrixElementCondition::RouteExists => "ROUTE_EXISTS",
            RouteMatrixElementCondition::RouteNotFound => "ROUTE_NOT_FOUND",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ROUTE_MATRIX_ELEMENT_CONDITION_UNSPECIFIED" => Some(Self::Unspecified),
            "ROUTE_EXISTS" => Some(Self::RouteExists),
            "ROUTE_NOT_FOUND" => Some(Self::RouteNotFound),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod routes_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The Routes API.
    #[derive(Debug, Clone)]
    pub struct RoutesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> RoutesClient<T>
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
        ) -> RoutesClient<InterceptedService<T, F>>
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
            RoutesClient::new(InterceptedService::new(inner, interceptor))
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
        /// Returns the primary route along with optional alternate routes, given a set
        /// of terminal and intermediate waypoints.
        ///
        /// **NOTE:** This method requires that you specify a response field mask in
        /// the input. You can provide the response field mask by using URL parameter
        /// `$fields` or `fields`, or by using an HTTP/gRPC header `X-Goog-FieldMask`
        /// (see the [available URL parameters and
        /// headers](https://cloud.google.com/apis/docs/system-parameters). The value
        /// is a comma separated list of field paths. See detailed documentation about
        /// [how to construct the field
        /// paths](https://github.com/protocolbuffers/protobuf/blob/master/src/google/protobuf/field_mask.proto).
        ///
        /// For example, in this method:
        ///
        /// * Field mask of all available fields (for manual inspection):
        ///   `X-Goog-FieldMask: *`
        /// * Field mask of Route-level duration, distance, and polyline (an example
        /// production setup):
        ///   `X-Goog-FieldMask:
        ///   routes.duration,routes.distanceMeters,routes.polyline.encodedPolyline`
        ///
        /// Google discourage the use of the wildcard (`*`) response field mask, or
        /// specifying the field mask at the top level (`routes`), because:
        ///
        /// * Selecting only the fields that you need helps our server save computation
        /// cycles, allowing us to return the result to you with a lower latency.
        /// * Selecting only the fields that you need
        /// in your production job ensures stable latency performance. We might add
        /// more response fields in the future, and those new fields might require
        /// extra computation time. If you select all fields, or if you select all
        /// fields at the top level, then you might experience performance degradation
        /// because any new field we add will be automatically included in the
        /// response.
        /// * Selecting only the fields that you need results in a smaller response
        /// size, and thus higher network throughput.
        pub async fn compute_routes(
            &mut self,
            request: impl tonic::IntoRequest<super::ComputeRoutesRequest>,
        ) -> Result<tonic::Response<super::ComputeRoutesResponse>, tonic::Status> {
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
                "/google.maps.routing.v2.Routes/ComputeRoutes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Takes in a list of origins and destinations and returns a stream containing
        /// route information for each combination of origin and destination.
        ///
        /// **NOTE:** This method requires that you specify a response field mask in
        /// the input. You can provide the response field mask by using the URL
        /// parameter `$fields` or `fields`, or by using the HTTP/gRPC header
        /// `X-Goog-FieldMask` (see the [available URL parameters and
        /// headers](https://cloud.google.com/apis/docs/system-parameters). The value
        /// is a comma separated list of field paths. See this detailed documentation
        /// about [how to construct the field
        /// paths](https://github.com/protocolbuffers/protobuf/blob/master/src/google/protobuf/field_mask.proto).
        ///
        /// For example, in this method:
        ///
        /// * Field mask of all available fields (for manual inspection):
        ///   `X-Goog-FieldMask: *`
        /// * Field mask of route durations, distances, element status, condition, and
        ///   element indices (an example production setup):
        ///   `X-Goog-FieldMask:
        ///   originIndex,destinationIndex,status,condition,distanceMeters,duration`
        ///
        /// It is critical that you include `status` in your field mask as otherwise
        /// all messages will appear to be OK. Google discourages the use of the
        /// wildcard (`*`) response field mask, because:
        ///
        /// * Selecting only the fields that you need helps our server save computation
        /// cycles, allowing us to return the result to you with a lower latency.
        /// * Selecting only the fields that you need in your production job ensures
        /// stable latency performance. We might add more response fields in the
        /// future, and those new fields might require extra computation time. If you
        /// select all fields, or if you select all fields at the top level, then you
        /// might experience performance degradation because any new field we add will
        /// be automatically included in the response.
        /// * Selecting only the fields that you need results in a smaller response
        /// size, and thus higher network throughput.
        pub async fn compute_route_matrix(
            &mut self,
            request: impl tonic::IntoRequest<super::ComputeRouteMatrixRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::RouteMatrixElement>>,
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
                "/google.maps.routing.v2.Routes/ComputeRouteMatrix",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}
