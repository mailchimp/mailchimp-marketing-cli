pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A subscriber's status for a specific campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SentTo {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<SentToLinksItem>>,
    /// For A/B Split Campaigns, the group the member was apart of.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absplit_group: Option<SentToAbsplitGroup>,
    /// The campaign id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// Email address for a subscriber.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// The MD5 hash of the lowercase version of the list member's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_id: Option<String>,
    /// For campaigns sent with timewarp, the time zone group the member is apart of.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmt_offset: Option<i64>,
    /// The date and time of the last open for this member in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_open: Option<DateTime<FixedOffset>>,
    /// The unique list id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The status of the list used, namely if it's deleted or disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_is_active: Option<bool>,
    /// A dictionary of merge fields where the keys are the merge tags. See the [Merge Fields documentation](https://mailchimp.com/developer/marketing/docs/merge-fields/#structure) for more about the structure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_fields: Option<HashMap<String, SentToMergeFieldsValue>>,
    /// The number of times a campaign was opened by this member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_count: Option<i64>,
    /// The status of the email delivered to this subscriber. `hard` and `soft` refer to different [bounce types](https://mailchimp.com/help/soft-vs-hard-bounces/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SentToStatus>,
    /// [VIP status](https://mailchimp.com/help/designate-and-send-to-vip-contacts/) for subscriber.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vip: Option<bool>,
}

impl SentTo {
    pub fn builder() -> SentToBuilder {
        <SentToBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SentToBuilder {
    links: Option<Vec<SentToLinksItem>>,
    absplit_group: Option<SentToAbsplitGroup>,
    campaign_id: Option<String>,
    email_address: Option<String>,
    email_id: Option<String>,
    gmt_offset: Option<i64>,
    last_open: Option<DateTime<FixedOffset>>,
    list_id: Option<String>,
    list_is_active: Option<bool>,
    merge_fields: Option<HashMap<String, SentToMergeFieldsValue>>,
    open_count: Option<i64>,
    status: Option<SentToStatus>,
    vip: Option<bool>,
}

impl SentToBuilder {
    pub fn links(mut self, value: Vec<SentToLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn absplit_group(mut self, value: SentToAbsplitGroup) -> Self {
        self.absplit_group = Some(value);
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

    pub fn gmt_offset(mut self, value: i64) -> Self {
        self.gmt_offset = Some(value);
        self
    }

    pub fn last_open(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_open = Some(value);
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

    pub fn merge_fields(mut self, value: HashMap<String, SentToMergeFieldsValue>) -> Self {
        self.merge_fields = Some(value);
        self
    }

    pub fn open_count(mut self, value: i64) -> Self {
        self.open_count = Some(value);
        self
    }

    pub fn status(mut self, value: SentToStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn vip(mut self, value: bool) -> Self {
        self.vip = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SentTo`].
    pub fn build(self) -> Result<SentTo, BuildError> {
        Ok(SentTo {
            links: self.links,
            absplit_group: self.absplit_group,
            campaign_id: self.campaign_id,
            email_address: self.email_address,
            email_id: self.email_id,
            gmt_offset: self.gmt_offset,
            last_open: self.last_open,
            list_id: self.list_id,
            list_is_active: self.list_is_active,
            merge_fields: self.merge_fields,
            open_count: self.open_count,
            status: self.status,
            vip: self.vip,
        })
    }
}
