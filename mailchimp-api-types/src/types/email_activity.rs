pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A list of a member's subscriber activity in a specific campaign, including opens, clicks, and bounces.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmailActivity {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<EmailActivityLinksItem>>,
    /// An array of objects, each showing an interaction with the email. Member activity limited to 1,000 open activities and 1,000 click activities per member per campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity: Option<Vec<EmailActivityActivityItem>>,
    /// The unique id for the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// Email address for a subscriber.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// The MD5 hash of the lowercase version of the list member's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_id: Option<String>,
    /// The unique id for the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The status of the list used, namely if it's deleted or disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_is_active: Option<bool>,
}

impl EmailActivity {
    pub fn builder() -> EmailActivityBuilder {
        <EmailActivityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmailActivityBuilder {
    links: Option<Vec<EmailActivityLinksItem>>,
    activity: Option<Vec<EmailActivityActivityItem>>,
    campaign_id: Option<String>,
    email_address: Option<String>,
    email_id: Option<String>,
    list_id: Option<String>,
    list_is_active: Option<bool>,
}

impl EmailActivityBuilder {
    pub fn links(mut self, value: Vec<EmailActivityLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn activity(mut self, value: Vec<EmailActivityActivityItem>) -> Self {
        self.activity = Some(value);
        self
    }

    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    pub fn email_address(mut self, value: impl Into<String>) -> Self {
        self.email_address = Some(value.into());
        self
    }

    pub fn email_id(mut self, value: impl Into<String>) -> Self {
        self.email_id = Some(value.into());
        self
    }

    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    pub fn list_is_active(mut self, value: bool) -> Self {
        self.list_is_active = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EmailActivity`].
    pub fn build(self) -> Result<EmailActivity, BuildError> {
        Ok(EmailActivity {
            links: self.links,
            activity: self.activity,
            campaign_id: self.campaign_id,
            email_address: self.email_address,
            email_id: self.email_id,
            list_id: self.list_id,
            list_is_active: self.list_is_active,
        })
    }
}
