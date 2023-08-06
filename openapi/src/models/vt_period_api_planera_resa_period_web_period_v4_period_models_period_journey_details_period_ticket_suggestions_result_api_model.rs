/*
 * Planera Resa
 *
 * Sök och planera resor med Västtrafik
 *
 * The version of the OpenAPI document: v4
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct JourneyDetailsPeriodTicketSuggestionsResultApiModel {
    /// Flag indicating that an error occurred while getting ticket suggestions.
    #[serde(rename = "hasError", skip_serializing_if = "Option::is_none")]
    pub has_error: Option<bool>,
    /// Ticket suggestions for a journey.
    #[serde(rename = "ticketSuggestions", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ticket_suggestions: Option<Option<Vec<crate::models::JourneyDetailsPeriodTicketSuggestionApiModel>>>,
    /// An array with the tickets from the existingTickets-array in the post-body. Validity for the journey is indicated for each ticket in the array. Included if 'ticketsuggestions' is passed in the includes array in the request, otherwise null.
    #[serde(rename = "ticketValidities", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ticket_validities: Option<Option<Vec<crate::models::JourneyDetailsPeriodTicketValidityApiModel>>>,
}

impl JourneyDetailsPeriodTicketSuggestionsResultApiModel {
    pub fn new() -> JourneyDetailsPeriodTicketSuggestionsResultApiModel {
        JourneyDetailsPeriodTicketSuggestionsResultApiModel {
            has_error: None,
            ticket_suggestions: None,
            ticket_validities: None,
        }
    }
}


