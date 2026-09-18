pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateWebhookResponse {
    #[serde(flatten)]
    pub list_webhooks_fields: ListWebhooks,
    /// The HMAC signing secret. Returned exactly once at creation. This should be stored securely; if lost, delete and recreate the webhook to obtain a new secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_secret: Option<String>,
}

impl CreateWebhookResponse {
    pub fn builder() -> CreateWebhookResponseBuilder {
        <CreateWebhookResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateWebhookResponseBuilder {
    list_webhooks_fields: Option<ListWebhooks>,
    signing_secret: Option<String>,
}

impl CreateWebhookResponseBuilder {
    pub fn list_webhooks_fields(mut self, value: ListWebhooks) -> Self {
        self.list_webhooks_fields = Some(value);
        self
    }

    pub fn signing_secret(mut self, value: impl Into<String>) -> Self {
        self.signing_secret = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateWebhookResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`list_webhooks_fields`](CreateWebhookResponseBuilder::list_webhooks_fields)
    pub fn build(self) -> Result<CreateWebhookResponse, BuildError> {
        Ok(CreateWebhookResponse {
            list_webhooks_fields: self.list_webhooks_fields.ok_or_else(|| BuildError::missing_field("list_webhooks_fields"))?,
            signing_secret: self.signing_secret,
        })
    }
}
