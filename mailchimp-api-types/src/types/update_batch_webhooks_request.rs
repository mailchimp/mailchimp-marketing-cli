pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateBatchWebhooksRequest {
    /// Whether the webhook receives requests or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// A valid URL for the Webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl UpdateBatchWebhooksRequest {
    pub fn builder() -> UpdateBatchWebhooksRequestBuilder {
        <UpdateBatchWebhooksRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateBatchWebhooksRequestBuilder {
    enabled: Option<bool>,
    url: Option<String>,
}

impl UpdateBatchWebhooksRequestBuilder {
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateBatchWebhooksRequest`].
    pub fn build(self) -> Result<UpdateBatchWebhooksRequest, BuildError> {
        Ok(UpdateBatchWebhooksRequest {
            enabled: self.enabled,
            url: self.url,
        })
    }
}

