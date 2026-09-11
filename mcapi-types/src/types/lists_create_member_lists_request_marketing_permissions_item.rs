pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A single marketing permission a subscriber has either opted-in to or opted-out of.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateMemberListsRequestMarketingPermissionsItem {
    /// If the subscriber has opted-in to the marketing permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The id for the marketing permission on the list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketing_permission_id: Option<String>,
}

impl CreateMemberListsRequestMarketingPermissionsItem {
    pub fn builder() -> CreateMemberListsRequestMarketingPermissionsItemBuilder {
        <CreateMemberListsRequestMarketingPermissionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateMemberListsRequestMarketingPermissionsItemBuilder {
    enabled: Option<bool>,
    marketing_permission_id: Option<String>,
}

impl CreateMemberListsRequestMarketingPermissionsItemBuilder {
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn marketing_permission_id(mut self, value: impl Into<String>) -> Self {
        self.marketing_permission_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateMemberListsRequestMarketingPermissionsItem`].
    pub fn build(self) -> Result<CreateMemberListsRequestMarketingPermissionsItem, BuildError> {
        Ok(CreateMemberListsRequestMarketingPermissionsItem {
            enabled: self.enabled,
            marketing_permission_id: self.marketing_permission_id,
        })
    }
}
