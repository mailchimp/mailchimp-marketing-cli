pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Webhook configured for the given list.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListWebhooks {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListWebhooksLinksItem>>,
    /// The events that can trigger the webhook and whether they are enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<ListWebhooksEvents>,
    /// An string that uniquely identifies this webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The unique id for the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// Whether outbound deliveries are HMAC-signed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_enabled: Option<bool>,
    /// The HMAC signing secret. Returned exactly once at creation. This should be stored securely; if lost, delete and recreate the webhook to obtain a new secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_secret: Option<String>,
    /// The possible sources of any events that can trigger the webhook and whether they are enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<ListWebhooksSources>,
    /// A valid URL for the Webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ListWebhooks {
    pub fn builder() -> ListWebhooksBuilder {
        <ListWebhooksBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListWebhooksBuilder {
    links: Option<Vec<ListWebhooksLinksItem>>,
    events: Option<ListWebhooksEvents>,
    id: Option<String>,
    list_id: Option<String>,
    signing_enabled: Option<bool>,
    signing_secret: Option<String>,
    sources: Option<ListWebhooksSources>,
    url: Option<String>,
}

impl ListWebhooksBuilder {
    pub fn links(mut self, value: Vec<ListWebhooksLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn events(mut self, value: ListWebhooksEvents) -> Self {
        self.events = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    pub fn signing_enabled(mut self, value: bool) -> Self {
        self.signing_enabled = Some(value);
        self
    }

    pub fn signing_secret(mut self, value: impl Into<String>) -> Self {
        self.signing_secret = Some(value.into());
        self
    }

    pub fn sources(mut self, value: ListWebhooksSources) -> Self {
        self.sources = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListWebhooks`].
    pub fn build(self) -> Result<ListWebhooks, BuildError> {
        Ok(ListWebhooks {
            links: self.links,
            events: self.events,
            id: self.id,
            list_id: self.list_id,
            signing_enabled: self.signing_enabled,
            signing_secret: self.signing_secret,
            sources: self.sources,
            url: self.url,
        })
    }
}
