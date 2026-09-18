pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A summary of the interaction with the campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmailActivityActivityItem {
    /// One of the following actions: 'open', 'click', or 'bounce'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// The IP address recorded for the action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// The date and time recorded for the action in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub timestamp: Option<DateTime<FixedOffset>>,
    /// If the action is a 'bounce', the type of bounce received: 'hard', 'soft'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// If the action is a 'click', the URL on which the member clicked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl EmailActivityActivityItem {
    pub fn builder() -> EmailActivityActivityItemBuilder {
        <EmailActivityActivityItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmailActivityActivityItemBuilder {
    action: Option<String>,
    ip: Option<String>,
    timestamp: Option<DateTime<FixedOffset>>,
    r#type: Option<String>,
    url: Option<String>,
}

impl EmailActivityActivityItemBuilder {
    pub fn action(mut self, value: impl Into<String>) -> Self {
        self.action = Some(value.into());
        self
    }

    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EmailActivityActivityItem`].
    pub fn build(self) -> Result<EmailActivityActivityItem, BuildError> {
        Ok(EmailActivityActivityItem {
            action: self.action,
            ip: self.ip,
            timestamp: self.timestamp,
            r#type: self.r#type,
            url: self.url,
        })
    }
}
