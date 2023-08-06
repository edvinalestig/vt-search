/*
 * Planera Resa
 *
 * Sök och planera resor med Västtrafik
 *
 * The version of the OpenAPI document: v4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// JourneyDetailsIncludeType : The different kinds of detailed information that could be included in a get journey details request.

/// The different kinds of detailed information that could be included in a get journey details request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum JourneyDetailsIncludeType {
    #[serde(rename = "ticketsuggestions")]
    Ticketsuggestions,
    #[serde(rename = "triplegcoordinates")]
    Triplegcoordinates,
    #[serde(rename = "validzones")]
    Validzones,
    #[serde(rename = "servicejourneycalls")]
    Servicejourneycalls,
    #[serde(rename = "servicejourneycoordinates")]
    Servicejourneycoordinates,
    #[serde(rename = "links")]
    Links,
    #[serde(rename = "occupancy")]
    Occupancy,

}

impl ToString for JourneyDetailsIncludeType {
    fn to_string(&self) -> String {
        match self {
            Self::Ticketsuggestions => String::from("ticketsuggestions"),
            Self::Triplegcoordinates => String::from("triplegcoordinates"),
            Self::Validzones => String::from("validzones"),
            Self::Servicejourneycalls => String::from("servicejourneycalls"),
            Self::Servicejourneycoordinates => String::from("servicejourneycoordinates"),
            Self::Links => String::from("links"),
            Self::Occupancy => String::from("occupancy"),
        }
    }
}

impl Default for JourneyDetailsIncludeType {
    fn default() -> JourneyDetailsIncludeType {
        Self::Ticketsuggestions
    }
}




