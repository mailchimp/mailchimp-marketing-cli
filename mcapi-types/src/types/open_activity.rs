pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A list of a member's opens activity in a specific campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OpenActivity {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<OpenActivityLinksItem>>,
    /// The unique id for the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// The status of the member, namely if they are subscribed, unsubscribed, deleted, non-subscribed, transactional, pending, or need reconfirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_status: Option<String>,
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
    /// A dictionary of merge fields where the keys are the merge tags. See the [Merge Fields documentation](https://mailchimp.com/developer/marketing/docs/merge-fields/#structure) for more about the structure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_fields: Option<HashMap<String, OpenActivityMergeFieldsValue>>,
    /// An array of timestamps for each time a list member opened the campaign. If a list member opens an email multiple times, this will return a separate timestamp for each open event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opens: Option<Vec<OpenActivityOpensItem>>,
    /// The total number of times the this campaign was opened by the list member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opens_count: Option<i64>,
    /// The total number of times the this campaign was opened by the list member excluding opens from email clients that use proxies .
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_excluded_opens_count: Option<i64>,
    /// [VIP status](https://mailchimp.com/help/designate-and-send-to-vip-contacts/) for subscriber.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vip: Option<bool>,
}

impl OpenActivity {
    pub fn builder() -> OpenActivityBuilder {
        <OpenActivityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OpenActivityBuilder {
    links: Option<Vec<OpenActivityLinksItem>>,
    campaign_id: Option<String>,
    contact_status: Option<String>,
    email_address: Option<String>,
    email_id: Option<String>,
    list_id: Option<String>,
    list_is_active: Option<bool>,
    merge_fields: Option<HashMap<String, OpenActivityMergeFieldsValue>>,
    opens: Option<Vec<OpenActivityOpensItem>>,
    opens_count: Option<i64>,
    proxy_excluded_opens_count: Option<i64>,
    vip: Option<bool>,
}

impl OpenActivityBuilder {
    pub fn links(mut self, value: Vec<OpenActivityLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    pub fn contact_status(mut self, value: impl Into<String>) -> Self {
        self.contact_status = Some(value.into());
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

    pub fn merge_fields(mut self, value: HashMap<String, OpenActivityMergeFieldsValue>) -> Self {
        self.merge_fields = Some(value);
        self
    }

    pub fn opens(mut self, value: Vec<OpenActivityOpensItem>) -> Self {
        self.opens = Some(value);
        self
    }

    pub fn opens_count(mut self, value: i64) -> Self {
        self.opens_count = Some(value);
        self
    }

    pub fn proxy_excluded_opens_count(mut self, value: i64) -> Self {
        self.proxy_excluded_opens_count = Some(value);
        self
    }

    pub fn vip(mut self, value: bool) -> Self {
        self.vip = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OpenActivity`].
    pub fn build(self) -> Result<OpenActivity, BuildError> {
        Ok(OpenActivity {
            links: self.links,
            campaign_id: self.campaign_id,
            contact_status: self.contact_status,
            email_address: self.email_address,
            email_id: self.email_id,
            list_id: self.list_id,
            list_is_active: self.list_is_active,
            merge_fields: self.merge_fields,
            opens: self.opens,
            opens_count: self.opens_count,
            proxy_excluded_opens_count: self.proxy_excluded_opens_count,
            vip: self.vip,
        })
    }
}
