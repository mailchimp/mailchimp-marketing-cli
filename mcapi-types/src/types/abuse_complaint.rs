pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Details of abuse complaints for a specific list. An abuse complaint occurs when your recipient clicks to 'report spam' in their email program.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AbuseComplaint {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<AbuseComplaintLinksItem>>,
    /// The campaign id for the abuse report
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// Date for the abuse report
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub date: Option<DateTime<FixedOffset>>,
    /// Email address for a subscriber.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// The MD5 hash of the lowercase version of the list member's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_id: Option<String>,
    /// The id for the abuse report
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The unique id of the list for the abuse report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The status of the list used, namely if it's deleted or disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_is_active: Option<bool>,
    /// A dictionary of merge fields where the keys are the merge tags. See the [Merge Fields documentation](https://mailchimp.com/developer/marketing/docs/merge-fields/#structure) for more about the structure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_fields: Option<HashMap<String, AbuseComplaintMergeFieldsValue>>,
    /// [VIP status](https://mailchimp.com/help/designate-and-send-to-vip-contacts/) for subscriber.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vip: Option<bool>,
}

impl AbuseComplaint {
    pub fn builder() -> AbuseComplaintBuilder {
        <AbuseComplaintBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AbuseComplaintBuilder {
    links: Option<Vec<AbuseComplaintLinksItem>>,
    campaign_id: Option<String>,
    date: Option<DateTime<FixedOffset>>,
    email_address: Option<String>,
    email_id: Option<String>,
    id: Option<i64>,
    list_id: Option<String>,
    list_is_active: Option<bool>,
    merge_fields: Option<HashMap<String, AbuseComplaintMergeFieldsValue>>,
    vip: Option<bool>,
}

impl AbuseComplaintBuilder {
    pub fn links(mut self, value: Vec<AbuseComplaintLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    pub fn date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.date = Some(value);
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

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
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

    pub fn merge_fields(mut self, value: HashMap<String, AbuseComplaintMergeFieldsValue>) -> Self {
        self.merge_fields = Some(value);
        self
    }

    pub fn vip(mut self, value: bool) -> Self {
        self.vip = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AbuseComplaint`].
    pub fn build(self) -> Result<AbuseComplaint, BuildError> {
        Ok(AbuseComplaint {
            links: self.links,
            campaign_id: self.campaign_id,
            date: self.date,
            email_address: self.email_address,
            email_id: self.email_id,
            id: self.id,
            list_id: self.list_id,
            list_is_active: self.list_is_active,
            merge_fields: self.merge_fields,
            vip: self.vip,
        })
    }
}
