pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A single marketing permission a subscriber has either opted-in to or opted-out of.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListMembersMarketingPermissionsItem {
    /// If the subscriber has opted-in to the marketing permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The id for the marketing permission on the list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketing_permission_id: Option<String>,
    /// The text of the marketing permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl ListMembersMarketingPermissionsItem {
    pub fn builder() -> ListMembersMarketingPermissionsItemBuilder {
        <ListMembersMarketingPermissionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMembersMarketingPermissionsItemBuilder {
    enabled: Option<bool>,
    marketing_permission_id: Option<String>,
    text: Option<String>,
}

impl ListMembersMarketingPermissionsItemBuilder {
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn marketing_permission_id(mut self, value: impl Into<String>) -> Self {
        self.marketing_permission_id = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListMembersMarketingPermissionsItem`].
    pub fn build(self) -> Result<ListMembersMarketingPermissionsItem, BuildError> {
        Ok(ListMembersMarketingPermissionsItem {
            enabled: self.enabled,
            marketing_permission_id: self.marketing_permission_id,
            text: self.text,
        })
    }
}
