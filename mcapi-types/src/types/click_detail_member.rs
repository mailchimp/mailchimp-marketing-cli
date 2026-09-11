pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A subscriber who clicked a specific URL in a specific campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ClickDetailMember {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ClickDetailMemberLinksItem>>,
    /// The campaign id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// The total number of times the subscriber clicked on the link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicks: Option<i64>,
    /// The status of the member, namely if they are subscribed, unsubscribed, deleted, non-subscribed, transactional, pending, or need reconfirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_status: Option<String>,
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
    pub merge_fields: Option<HashMap<String, ClickDetailMemberMergeFieldsValue>>,
    /// The id for the tracked URL in the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_id: Option<String>,
    /// [VIP status](https://mailchimp.com/help/designate-and-send-to-vip-contacts/) for subscriber.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vip: Option<bool>,
}

impl ClickDetailMember {
    pub fn builder() -> ClickDetailMemberBuilder {
        <ClickDetailMemberBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClickDetailMemberBuilder {
    links: Option<Vec<ClickDetailMemberLinksItem>>,
    campaign_id: Option<String>,
    clicks: Option<i64>,
    contact_status: Option<String>,
    email_address: Option<String>,
    email_id: Option<String>,
    list_id: Option<String>,
    list_is_active: Option<bool>,
    merge_fields: Option<HashMap<String, ClickDetailMemberMergeFieldsValue>>,
    url_id: Option<String>,
    vip: Option<bool>,
}

impl ClickDetailMemberBuilder {
    pub fn links(mut self, value: Vec<ClickDetailMemberLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    pub fn clicks(mut self, value: i64) -> Self {
        self.clicks = Some(value);
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

    pub fn merge_fields(mut self, value: HashMap<String, ClickDetailMemberMergeFieldsValue>) -> Self {
        self.merge_fields = Some(value);
        self
    }

    pub fn url_id(mut self, value: impl Into<String>) -> Self {
        self.url_id = Some(value.into());
        self
    }

    pub fn vip(mut self, value: bool) -> Self {
        self.vip = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ClickDetailMember`].
    pub fn build(self) -> Result<ClickDetailMember, BuildError> {
        Ok(ClickDetailMember {
            links: self.links,
            campaign_id: self.campaign_id,
            clicks: self.clicks,
            contact_status: self.contact_status,
            email_address: self.email_address,
            email_id: self.email_id,
            list_id: self.list_id,
            list_is_active: self.list_is_active,
            merge_fields: self.merge_fields,
            url_id: self.url_id,
            vip: self.vip,
        })
    }
}
