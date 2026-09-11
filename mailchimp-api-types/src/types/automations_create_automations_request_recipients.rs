pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// List settings for the Automation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAutomationsRequestRecipients {
    /// The id of the List.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The id of the store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
}

impl CreateAutomationsRequestRecipients {
    pub fn builder() -> CreateAutomationsRequestRecipientsBuilder {
        <CreateAutomationsRequestRecipientsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAutomationsRequestRecipientsBuilder {
    list_id: Option<String>,
    store_id: Option<String>,
}

impl CreateAutomationsRequestRecipientsBuilder {
    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    pub fn store_id(mut self, value: impl Into<String>) -> Self {
        self.store_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAutomationsRequestRecipients`].
    pub fn build(self) -> Result<CreateAutomationsRequestRecipients, BuildError> {
        Ok(CreateAutomationsRequestRecipients {
            list_id: self.list_id,
            store_id: self.store_id,
        })
    }
}
