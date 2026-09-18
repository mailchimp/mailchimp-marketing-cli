pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum PatchAudienceContactRequestTagsItem {
        String(String),

        PatchAudienceContactRequestTagsItemName(PatchAudienceContactRequestTagsItemName),
}

impl PatchAudienceContactRequestTagsItem {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_patch_audience_contact_request_tags_item_name(&self) -> bool {
        matches!(self, Self::PatchAudienceContactRequestTagsItemName(_))
    }


    pub fn as_string(&self) -> Option<&str> {
        match self {
                    Self::String(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
                    Self::String(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_patch_audience_contact_request_tags_item_name(&self) -> Option<&PatchAudienceContactRequestTagsItemName> {
        match self {
                    Self::PatchAudienceContactRequestTagsItemName(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_patch_audience_contact_request_tags_item_name(self) -> Option<PatchAudienceContactRequestTagsItemName> {
        match self {
                    Self::PatchAudienceContactRequestTagsItemName(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for PatchAudienceContactRequestTagsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::PatchAudienceContactRequestTagsItemName(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
