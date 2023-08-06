/*
 * Planera Resa
 *
 * Sök och planera resor med Västtrafik
 *
 * The version of the OpenAPI document: v4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// JourneyDetailsPeriodTicketValidityApiModel : Information about ticket validity.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct JourneyDetailsPeriodTicketValidityApiModel {
    /// The ticket id.
    #[serde(rename = "id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<String>>,
    /// Indicates if the ticket is valid for the journey.
    #[serde(rename = "isValidForJourney", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_valid_for_journey: Option<Option<bool>>,
}

impl JourneyDetailsPeriodTicketValidityApiModel {
    /// Information about ticket validity.
    pub fn new() -> JourneyDetailsPeriodTicketValidityApiModel {
        JourneyDetailsPeriodTicketValidityApiModel {
            id: None,
            is_valid_for_journey: None,
        }
    }
}


