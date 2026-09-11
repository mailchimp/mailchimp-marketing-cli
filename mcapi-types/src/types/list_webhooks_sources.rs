pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The possible sources of any events that can trigger the webhook and whether they are enabled.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListWebhooksSources {
    /// Whether the webhook is triggered by admin-initiated actions in the web interface.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin: Option<bool>,
    /// Whether the webhook is triggered by actions initiated via the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<bool>,
    /// Whether the webhook is triggered by subscriber-initiated actions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<bool>,
}

impl ListWebhooksSources {
    pub fn builder() -> ListWebhooksSourcesBuilder {
        <ListWebhooksSourcesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListWebhooksSourcesBuilder {
    admin: Option<bool>,
    api: Option<bool>,
    user: Option<bool>,
}

impl ListWebhooksSourcesBuilder {
    pub fn admin(mut self, value: bool) -> Self {
        self.admin = Some(value);
        self
    }

    pub fn api(mut self, value: bool) -> Self {
        self.api = Some(value);
        self
    }

    pub fn user(mut self, value: bool) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListWebhooksSources`].
    pub fn build(self) -> Result<ListWebhooksSources, BuildError> {
        Ok(ListWebhooksSources {
            admin: self.admin,
            api: self.api,
            user: self.user,
        })
    }
}
