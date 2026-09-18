pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A member who unsubscribed from a specific campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Unsubscribes {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<UnsubscribesLinksItem>>,
    /// The campaign id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// Email address for a subscriber.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// The MD5 hash of the lowercase version of the list member's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_id: Option<String>,
    /// The list id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The status of the list used, namely if it's deleted or disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_is_active: Option<bool>,
    /// A dictionary of merge fields where the keys are the merge tags. See the [Merge Fields documentation](https://mailchimp.com/developer/marketing/docs/merge-fields/#structure) for more about the structure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_fields: Option<HashMap<String, UnsubscribesMergeFieldsValue>>,
    /// If available, the reason listed by the member for unsubscribing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// The date and time the member opted-out in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub timestamp: Option<DateTime<FixedOffset>>,
    /// [VIP status](https://mailchimp.com/help/designate-and-send-to-vip-contacts/) for subscriber.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vip: Option<bool>,
}

impl Unsubscribes {
    pub fn builder() -> UnsubscribesBuilder {
        <UnsubscribesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnsubscribesBuilder {
    links: Option<Vec<UnsubscribesLinksItem>>,
    campaign_id: Option<String>,
    email_address: Option<String>,
    email_id: Option<String>,
    list_id: Option<String>,
    list_is_active: Option<bool>,
    merge_fields: Option<HashMap<String, UnsubscribesMergeFieldsValue>>,
    reason: Option<String>,
    timestamp: Option<DateTime<FixedOffset>>,
    vip: Option<bool>,
}

impl UnsubscribesBuilder {
    pub fn links(mut self, value: Vec<UnsubscribesLinksItem>) -> Self {
        self.links = Some(value);
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

    pub fn merge_fields(mut self, value: HashMap<String, UnsubscribesMergeFieldsValue>) -> Self {
        self.merge_fields = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn vip(mut self, value: bool) -> Self {
        self.vip = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Unsubscribes`].
    pub fn build(self) -> Result<Unsubscribes, BuildError> {
        Ok(Unsubscribes {
            links: self.links,
            campaign_id: self.campaign_id,
            email_address: self.email_address,
            email_id: self.email_id,
            list_id: self.list_id,
            list_is_active: self.list_is_active,
            merge_fields: self.merge_fields,
            reason: self.reason,
            timestamp: self.timestamp,
            vip: self.vip,
        })
    }
}
