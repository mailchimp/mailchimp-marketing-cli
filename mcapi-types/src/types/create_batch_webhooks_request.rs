pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateBatchWebhooksRequest {
    /// Whether the webhook receives requests or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// A valid URL for the Webhook.
    #[serde(default)]
    pub url: String,
}

impl CreateBatchWebhooksRequest {
    pub fn builder() -> CreateBatchWebhooksRequestBuilder {
        <CreateBatchWebhooksRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBatchWebhooksRequestBuilder {
    enabled: Option<bool>,
    url: Option<String>,
}

impl CreateBatchWebhooksRequestBuilder {
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateBatchWebhooksRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](CreateBatchWebhooksRequestBuilder::url)
    pub fn build(self) -> Result<CreateBatchWebhooksRequest, BuildError> {
        Ok(CreateBatchWebhooksRequest {
            enabled: self.enabled,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}

