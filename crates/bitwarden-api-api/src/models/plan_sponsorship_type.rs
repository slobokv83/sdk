/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

///
#[repr(i64)]
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize_repr, Deserialize_repr,
)]
pub enum PlanSponsorshipType {
    _0 = 0,
}

impl ToString for PlanSponsorshipType {
    fn to_string(&self) -> String {
        match self {
            Self::_0 => String::from("0"),
        }
    }
}

impl Default for PlanSponsorshipType {
    fn default() -> PlanSponsorshipType {
        Self::_0
    }
}
