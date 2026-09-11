pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// List settings for the Automation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AutomationWorkflowRecipients {
    /// The unique list id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The status of the list used, namely if it's deleted or disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_is_active: Option<bool>,
    /// List Name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_opts: Option<SegmentType>,
    /// The id of the store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
}

impl AutomationWorkflowRecipients {
    pub fn builder() -> AutomationWorkflowRecipientsBuilder {
        <AutomationWorkflowRecipientsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationWorkflowRecipientsBuilder {
    list_id: Option<String>,
    list_is_active: Option<bool>,
    list_name: Option<String>,
    segment_opts: Option<SegmentType>,
    store_id: Option<String>,
}

impl AutomationWorkflowRecipientsBuilder {
    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    pub fn list_is_active(mut self, value: bool) -> Self {
        self.list_is_active = Some(value);
        self
    }

    pub fn list_name(mut self, value: impl Into<String>) -> Self {
        self.list_name = Some(value.into());
        self
    }

    pub fn segment_opts(mut self, value: SegmentType) -> Self {
        self.segment_opts = Some(value);
        self
    }

    pub fn store_id(mut self, value: impl Into<String>) -> Self {
        self.store_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AutomationWorkflowRecipients`].
    pub fn build(self) -> Result<AutomationWorkflowRecipients, BuildError> {
        Ok(AutomationWorkflowRecipients {
            list_id: self.list_id,
            list_is_active: self.list_is_active,
            list_name: self.list_name,
            segment_opts: self.segment_opts,
            store_id: self.store_id,
        })
    }
}
