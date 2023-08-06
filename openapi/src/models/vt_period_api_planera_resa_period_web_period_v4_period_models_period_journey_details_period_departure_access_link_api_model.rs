/*
 * Planera Resa
 *
 * Sök och planera resor med Västtrafik
 *
 * The version of the OpenAPI document: v4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// JourneyDetailsPeriodDepartureAccessLinkApiModel : Information about a walk, bike or car link from origin to first public transport trip leg.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct JourneyDetailsPeriodDepartureAccessLinkApiModel {
    #[serde(rename = "transportMode", skip_serializing_if = "Option::is_none")]
    pub transport_mode: Option<crate::models::TransportMode>,
    #[serde(rename = "transportSubMode", skip_serializing_if = "Option::is_none")]
    pub transport_sub_mode: Option<crate::models::TransportSubMode>,
    #[serde(rename = "origin", skip_serializing_if = "Option::is_none")]
    pub origin: Option<Box<crate::models::JourneyDetailsPeriodLinkEndpointApiModel>>,
    #[serde(rename = "destination", skip_serializing_if = "Option::is_none")]
    pub destination: Option<Box<crate::models::JourneyDetailsPeriodCallApiModel>>,
    /// An ordered list (most important first) of notes related to the access link.
    #[serde(rename = "notes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub notes: Option<Option<Vec<crate::models::Note>>>,
    /// Distance in meters.
    #[serde(rename = "distanceInMeters", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub distance_in_meters: Option<Option<i32>>,
    /// The planned departure time in RFC 3339 format.
    #[serde(rename = "plannedDepartureTime", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub planned_departure_time: Option<Option<String>>,
    /// The planned arrival time in RFC 3339 format.
    #[serde(rename = "plannedArrivalTime", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub planned_arrival_time: Option<Option<String>>,
    /// The planned duration in minutes.
    #[serde(rename = "plannedDurationInMinutes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub planned_duration_in_minutes: Option<Option<i32>>,
    /// The estimated departure time in RFC 3339 format, if available.
    #[serde(rename = "estimatedDepartureTime", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub estimated_departure_time: Option<Option<String>>,
    /// The estimated arrival time in RFC 3339 format, if available.
    #[serde(rename = "estimatedArrivalTime", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub estimated_arrival_time: Option<Option<String>>,
    /// The estimated duration in minutes, if available.
    #[serde(rename = "estimatedDurationInMinutes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub estimated_duration_in_minutes: Option<Option<i32>>,
    /// Number of steps based on the distance and an estimated step length of 0.65 meters.
    #[serde(rename = "estimatedNumberOfSteps", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub estimated_number_of_steps: Option<Option<i32>>,
    /// The coordinates for the link.
    #[serde(rename = "linkCoordinates", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub link_coordinates: Option<Option<Vec<crate::models::CoordinateApiModel>>>,
    /// The segments that make up this link.
    #[serde(rename = "segments", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub segments: Option<Option<Vec<crate::models::JourneyDetailsPeriodLinkSegmentApiModel>>>,
}

impl JourneyDetailsPeriodDepartureAccessLinkApiModel {
    /// Information about a walk, bike or car link from origin to first public transport trip leg.
    pub fn new() -> JourneyDetailsPeriodDepartureAccessLinkApiModel {
        JourneyDetailsPeriodDepartureAccessLinkApiModel {
            transport_mode: None,
            transport_sub_mode: None,
            origin: None,
            destination: None,
            notes: None,
            distance_in_meters: None,
            planned_departure_time: None,
            planned_arrival_time: None,
            planned_duration_in_minutes: None,
            estimated_departure_time: None,
            estimated_arrival_time: None,
            estimated_duration_in_minutes: None,
            estimated_number_of_steps: None,
            link_coordinates: None,
            segments: None,
        }
    }
}


