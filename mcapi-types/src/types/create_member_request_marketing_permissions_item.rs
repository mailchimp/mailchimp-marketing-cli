pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A single marketing permission a subscriber has either opted-in to or opted-out of.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateMemberRequestMarketingPermissionsItem {
    /// If the subscriber has opted-in to the marketing permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The id for the marketing permission on the list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketing_permission_id: Option<String>,
}

impl CreateMemberRequestMarketingPermissionsItem {
    pub fn builder() -> CreateMemberRequestMarketingPermissionsItemBuilder {
        <CreateMemberRequestMarketingPermissionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateMemberRequestMarketingPermissionsItemBuilder {
    enabled: Option<bool>,
    marketing_permission_id: Option<String>,
}

impl CreateMemberRequestMarketingPermissionsItemBuilder {
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn marketing_permission_id(mut self, value: impl Into<String>) -> Self {
        self.marketing_permission_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateMemberRequestMarketingPermissionsItem`].
    pub fn build(self) -> Result<CreateMemberRequestMarketingPermissionsItem, BuildError> {
        Ok(CreateMemberRequestMarketingPermissionsItem {
            enabled: self.enabled,
            marketing_permission_id: self.marketing_permission_id,
        })
    }
}
