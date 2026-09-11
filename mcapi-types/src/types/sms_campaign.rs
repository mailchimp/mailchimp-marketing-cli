pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A single SMS campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SmsCampaign {
    /// A string that uniquely identifies this campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The ID used in the Mailchimp web application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_id: Option<String>,
    /// The name of the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The current status of the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The channel for this campaign (sms or whatsapp).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    /// The numeric ID of the list associated with this campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<i64>,
    /// The number of recipients for this campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_count: Option<i64>,
    /// The date and time the campaign was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub create_time: Option<DateTime<FixedOffset>>,
    /// The date and time the campaign is scheduled to send.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub send_time: Option<DateTime<FixedOffset>>,
    /// The date and time the campaign was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
    /// The date and time the campaign will stop sending in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub expire_time: Option<DateTime<FixedOffset>>,
    /// Whether the campaign is configured to send immediately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_send_now: Option<bool>,
    /// The ID of the folder this campaign is in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    /// The segment IDs used to target recipients for this campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<i64>>,
    /// The segment IDs excluded from receiving this campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_segments: Option<Vec<i64>>,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<SmsCampaignLinksItem>>,
}

impl SmsCampaign {
    pub fn builder() -> SmsCampaignBuilder {
        <SmsCampaignBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SmsCampaignBuilder {
    id: Option<String>,
    web_id: Option<String>,
    name: Option<String>,
    status: Option<String>,
    channel: Option<String>,
    list_id: Option<i64>,
    recipient_count: Option<i64>,
    create_time: Option<DateTime<FixedOffset>>,
    send_time: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    expire_time: Option<DateTime<FixedOffset>>,
    is_send_now: Option<bool>,
    folder_id: Option<String>,
    segments: Option<Vec<i64>>,
    excluded_segments: Option<Vec<i64>>,
    links: Option<Vec<SmsCampaignLinksItem>>,
}

impl SmsCampaignBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn web_id(mut self, value: impl Into<String>) -> Self {
        self.web_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn channel(mut self, value: impl Into<String>) -> Self {
        self.channel = Some(value.into());
        self
    }

    pub fn list_id(mut self, value: i64) -> Self {
        self.list_id = Some(value);
        self
    }

    pub fn recipient_count(mut self, value: i64) -> Self {
        self.recipient_count = Some(value);
        self
    }

    pub fn create_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.create_time = Some(value);
        self
    }

    pub fn send_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.send_time = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn expire_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expire_time = Some(value);
        self
    }

    pub fn is_send_now(mut self, value: bool) -> Self {
        self.is_send_now = Some(value);
        self
    }

    pub fn folder_id(mut self, value: impl Into<String>) -> Self {
        self.folder_id = Some(value.into());
        self
    }

    pub fn segments(mut self, value: Vec<i64>) -> Self {
        self.segments = Some(value);
        self
    }

    pub fn excluded_segments(mut self, value: Vec<i64>) -> Self {
        self.excluded_segments = Some(value);
        self
    }

    pub fn links(mut self, value: Vec<SmsCampaignLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SmsCampaign`].
    pub fn build(self) -> Result<SmsCampaign, BuildError> {
        Ok(SmsCampaign {
            id: self.id,
            web_id: self.web_id,
            name: self.name,
            status: self.status,
            channel: self.channel,
            list_id: self.list_id,
            recipient_count: self.recipient_count,
            create_time: self.create_time,
            send_time: self.send_time,
            updated_at: self.updated_at,
            expire_time: self.expire_time,
            is_send_now: self.is_send_now,
            folder_id: self.folder_id,
            segments: self.segments,
            excluded_segments: self.excluded_segments,
            links: self.links,
        })
    }
}
