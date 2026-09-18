pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A webhook configured for batch status updates.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BatchWebhook {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Vec<BatchWebhookLinksItemItem>>>,
    /// Whether the webhook receives requests or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// A string that uniquely identifies this Batch Webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether outbound deliveries are HMAC-signed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_enabled: Option<bool>,
    /// A valid URL for the Webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl BatchWebhook {
    pub fn builder() -> BatchWebhookBuilder {
        <BatchWebhookBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchWebhookBuilder {
    links: Option<Vec<Vec<BatchWebhookLinksItemItem>>>,
    enabled: Option<bool>,
    id: Option<String>,
    signing_enabled: Option<bool>,
    url: Option<String>,
}

impl BatchWebhookBuilder {
    pub fn links(mut self, value: Vec<Vec<BatchWebhookLinksItemItem>>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn signing_enabled(mut self, value: bool) -> Self {
        self.signing_enabled = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BatchWebhook`].
    pub fn build(self) -> Result<BatchWebhook, BuildError> {
        Ok(BatchWebhook {
            links: self.links,
            enabled: self.enabled,
            id: self.id,
            signing_enabled: self.signing_enabled,
            url: self.url,
        })
    }
}
