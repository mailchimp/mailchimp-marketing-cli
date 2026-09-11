pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Configure a webhook for the given list.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddWebhook {
    /// The events that can trigger the webhook and whether they are enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<AddWebhookEvents>,
    /// The possible sources of any events that can trigger the webhook and whether they are enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<AddWebhookSources>,
    /// A valid URL for the Webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl AddWebhook {
    pub fn builder() -> AddWebhookBuilder {
        <AddWebhookBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddWebhookBuilder {
    events: Option<AddWebhookEvents>,
    sources: Option<AddWebhookSources>,
    url: Option<String>,
}

impl AddWebhookBuilder {
    pub fn events(mut self, value: AddWebhookEvents) -> Self {
        self.events = Some(value);
        self
    }

    pub fn sources(mut self, value: AddWebhookSources) -> Self {
        self.sources = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AddWebhook`].
    pub fn build(self) -> Result<AddWebhook, BuildError> {
        Ok(AddWebhook {
            events: self.events,
            sources: self.sources,
            url: self.url,
        })
    }
}
