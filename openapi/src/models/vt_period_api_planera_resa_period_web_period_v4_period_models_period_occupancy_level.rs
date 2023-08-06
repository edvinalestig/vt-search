/*
 * Planera Resa
 *
 * Sök och planera resor med Västtrafik
 *
 * The version of the OpenAPI document: v4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// OccupancyLevel : Represents a level of occupancy.

/// Represents a level of occupancy.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OccupancyLevel {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "notpublictransport")]
    Notpublictransport,

}

impl ToString for OccupancyLevel {
    fn to_string(&self) -> String {
        match self {
            Self::Low => String::from("low"),
            Self::Medium => String::from("medium"),
            Self::High => String::from("high"),
            Self::Incomplete => String::from("incomplete"),
            Self::Missing => String::from("missing"),
            Self::Notpublictransport => String::from("notpublictransport"),
        }
    }
}

impl Default for OccupancyLevel {
    fn default() -> OccupancyLevel {
        Self::Low
    }
}




