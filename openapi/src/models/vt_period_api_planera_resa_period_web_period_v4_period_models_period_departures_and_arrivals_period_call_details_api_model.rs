/*
 * Planera Resa
 *
 * Sök och planera resor med Västtrafik
 *
 * The version of the OpenAPI document: v4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DeparturesAndArrivalsPeriodCallDetailsApiModel : Information about a call on the trip leg.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DeparturesAndArrivalsPeriodCallDetailsApiModel {
    #[serde(rename = "stopPoint")]
    pub stop_point: Box<crate::models::JourneyDetailsPeriodStopPointApiModel>,
    /// The planned arrival time for the call in RFC 3339 format.
    #[serde(rename = "plannedArrivalTime", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub planned_arrival_time: Option<Option<String>>,
    /// The planned departure time for the call in RFC 3339 format.
    #[serde(rename = "plannedDepartureTime", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub planned_departure_time: Option<Option<String>>,
    /// The estimated arrival time for the call in RFC 3339 format.
    #[serde(rename = "estimatedArrivalTime", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub estimated_arrival_time: Option<Option<String>>,
    /// The estimated departure time for the call in RFC 3339 format.
    #[serde(rename = "estimatedDepartureTime", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub estimated_departure_time: Option<Option<String>>,
    /// The best known time of the call in RFC 3339 format. Is EstimatedArrivalTime if exists, otherwise PlannedArrivalTime.
    #[serde(rename = "estimatedOtherwisePlannedArrivalTime", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub estimated_otherwise_planned_arrival_time: Option<Option<String>>,
    /// The best known time of the call in RFC 3339 format. Is EstimatedDepartureTime if exists, otherwise PlannedDepartureTime.
    #[serde(rename = "estimatedOtherwisePlannedDepartureTime", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub estimated_otherwise_planned_departure_time: Option<Option<String>>,
    /// The planned platform of the call.
    #[serde(rename = "plannedPlatform", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub planned_platform: Option<Option<String>>,
    /// The estimated platform of the call.
    #[serde(rename = "estimatedPlatform", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub estimated_platform: Option<Option<String>>,
    /// The latitude of the stop point of the call.
    #[serde(rename = "latitude", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub latitude: Option<Option<f64>>,
    /// The longitude of the stop point of the call.
    #[serde(rename = "longitude", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub longitude: Option<Option<f64>>,
    /// The index of the stop point of the call.
    #[serde(rename = "index", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub index: Option<Option<String>>,
    #[serde(rename = "occupancy", skip_serializing_if = "Option::is_none")]
    pub occupancy: Option<Box<crate::models::OccupancyInformationApiModel>>,
    /// Flag indicating if the call is cancelled.
    #[serde(rename = "isCancelled", skip_serializing_if = "Option::is_none")]
    pub is_cancelled: Option<bool>,
    /// Flag indicating if the departure is cancelled.
    #[serde(rename = "isDepartureCancelled", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_departure_cancelled: Option<Option<bool>>,
    /// Flag indicating if the arrival is cancelled.
    #[serde(rename = "isArrivalCancelled", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_arrival_cancelled: Option<Option<bool>>,
}

impl DeparturesAndArrivalsPeriodCallDetailsApiModel {
    /// Information about a call on the trip leg.
    pub fn new(stop_point: crate::models::JourneyDetailsPeriodStopPointApiModel) -> DeparturesAndArrivalsPeriodCallDetailsApiModel {
        DeparturesAndArrivalsPeriodCallDetailsApiModel {
            stop_point: Box::new(stop_point),
            planned_arrival_time: None,
            planned_departure_time: None,
            estimated_arrival_time: None,
            estimated_departure_time: None,
            estimated_otherwise_planned_arrival_time: None,
            estimated_otherwise_planned_departure_time: None,
            planned_platform: None,
            estimated_platform: None,
            latitude: None,
            longitude: None,
            index: None,
            occupancy: None,
            is_cancelled: None,
            is_departure_cancelled: None,
            is_arrival_cancelled: None,
        }
    }
}


