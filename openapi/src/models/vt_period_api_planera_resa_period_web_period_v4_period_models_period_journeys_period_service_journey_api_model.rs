/*
 * Planera Resa
 *
 * Sök och planera resor med Västtrafik
 *
 * The version of the OpenAPI document: v4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// JourneysPeriodServiceJourneyApiModel : Information about a service journey of a departure or arrival.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct JourneysPeriodServiceJourneyApiModel {
    /// 16-digit Västtrafik service journey gid.
    #[serde(rename = "gid")]
    pub gid: String,
    /// A description of the direction.
    #[serde(rename = "direction", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub direction: Option<Option<String>>,
    /// Västtrafik service journey number that the trip leg is a part of.
    #[serde(rename = "number", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub number: Option<Option<String>>,
    #[serde(rename = "line", skip_serializing_if = "Option::is_none")]
    pub line: Option<Box<crate::models::JourneysPeriodLineApiModel>>,
}

impl JourneysPeriodServiceJourneyApiModel {
    /// Information about a service journey of a departure or arrival.
    pub fn new(gid: String) -> JourneysPeriodServiceJourneyApiModel {
        JourneysPeriodServiceJourneyApiModel {
            gid,
            direction: None,
            number: None,
            line: None,
        }
    }
}


