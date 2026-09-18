pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum CreateAudienceContactRequestTagsItem {
        String(String),

        CreateAudienceContactRequestTagsItemName(CreateAudienceContactRequestTagsItemName),
}

impl CreateAudienceContactRequestTagsItem {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_create_audience_contact_request_tags_item_name(&self) -> bool {
        matches!(self, Self::CreateAudienceContactRequestTagsItemName(_))
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

    pub fn as_create_audience_contact_request_tags_item_name(&self) -> Option<&CreateAudienceContactRequestTagsItemName> {
        match self {
                    Self::CreateAudienceContactRequestTagsItemName(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_create_audience_contact_request_tags_item_name(self) -> Option<CreateAudienceContactRequestTagsItemName> {
        match self {
                    Self::CreateAudienceContactRequestTagsItemName(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for CreateAudienceContactRequestTagsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::CreateAudienceContactRequestTagsItemName(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
