/*
 * Planera Resa
 *
 * Sök och planera resor med Västtrafik
 *
 * The version of the OpenAPI document: v4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LocationsPeriodGetLocationsResponse : The response to a get locations request, includes the results and pagination information.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LocationsPeriodGetLocationsResponse {
    /// The results.
    #[serde(rename = "results", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub results: Option<Option<Vec<crate::models::LocationsPeriodLocationApiModel>>>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::PaginationProperties>>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<crate::models::PaginationLinks>>,
}

impl LocationsPeriodGetLocationsResponse {
    /// The response to a get locations request, includes the results and pagination information.
    pub fn new() -> LocationsPeriodGetLocationsResponse {
        LocationsPeriodGetLocationsResponse {
            results: None,
            pagination: None,
            links: None,
        }
    }
}


