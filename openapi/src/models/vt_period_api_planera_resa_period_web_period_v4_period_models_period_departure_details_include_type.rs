/*
 * Planera Resa
 *
 * Sök och planera resor med Västtrafik
 *
 * The version of the OpenAPI document: v4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DepartureDetailsIncludeType : The different kinds of detailed information that could be included in a get departure details request.

/// The different kinds of detailed information that could be included in a get departure details request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DepartureDetailsIncludeType {
    #[serde(rename = "servicejourneycalls")]
    Servicejourneycalls,
    #[serde(rename = "servicejourneycoordinates")]
    Servicejourneycoordinates,
    #[serde(rename = "occupancy")]
    Occupancy,

}

impl ToString for DepartureDetailsIncludeType {
    fn to_string(&self) -> String {
        match self {
            Self::Servicejourneycalls => String::from("servicejourneycalls"),
            Self::Servicejourneycoordinates => String::from("servicejourneycoordinates"),
            Self::Occupancy => String::from("occupancy"),
        }
    }
}

impl Default for DepartureDetailsIncludeType {
    fn default() -> DepartureDetailsIncludeType {
        Self::Servicejourneycalls
    }
}




