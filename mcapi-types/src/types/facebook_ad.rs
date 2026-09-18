pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FacebookAd {
    /// The date and time the outreach was canceled in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub canceled_at: Option<DateTime<FixedOffset>>,
    /// The date and time the outreach was created in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub create_time: Option<DateTime<FixedOffset>>,
    /// If this outreach targets a segment of your audience.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_segment: Option<bool>,
    /// Unique ID of an Outreach.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Title or name of an Outreach.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The date and time the outreach was (or will be) published in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub published_time: Option<DateTime<FixedOffset>>,
    /// High level audience information for who the outreach targets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<FacebookAdRecipients>,
    /// High level reporting stats for an outreach.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_summary: Option<FacebookAdReportSummary>,
    /// Outreach report availability. Note: This property is hotly debated in what it _should_ convey. See [MCP-1371](https://jira.mailchimp.com/browse/MCP-1371) for more context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_report: Option<bool>,
    /// The date and time the outreach was started in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub start_time: Option<DateTime<FixedOffset>>,
    /// The status of this outreach.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<FacebookAdStatus>,
    /// The type of outreach this object is.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<FacebookAdType>,
    /// The date and time the outreach was last updated in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
    /// The ID used in the Mailchimp web application. For example, for a `regular` outreach, you can view this campaign in your Mailchimp account at `https://{dc}.admin.mailchimp.com/campaigns/show/?id={web_id}`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_id: Option<i64>,
}

impl FacebookAd {
    pub fn builder() -> FacebookAdBuilder {
        <FacebookAdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FacebookAdBuilder {
    canceled_at: Option<DateTime<FixedOffset>>,
    create_time: Option<DateTime<FixedOffset>>,
    has_segment: Option<bool>,
    id: Option<String>,
    name: Option<String>,
    published_time: Option<DateTime<FixedOffset>>,
    recipients: Option<FacebookAdRecipients>,
    report_summary: Option<FacebookAdReportSummary>,
    show_report: Option<bool>,
    start_time: Option<DateTime<FixedOffset>>,
    status: Option<FacebookAdStatus>,
    r#type: Option<FacebookAdType>,
    updated_at: Option<DateTime<FixedOffset>>,
    web_id: Option<i64>,
}

impl FacebookAdBuilder {
    pub fn canceled_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.canceled_at = Some(value);
        self
    }

    pub fn create_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.create_time = Some(value);
        self
    }

    pub fn has_segment(mut self, value: bool) -> Self {
        self.has_segment = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn published_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.published_time = Some(value);
        self
    }

    pub fn recipients(mut self, value: FacebookAdRecipients) -> Self {
        self.recipients = Some(value);
        self
    }

    pub fn report_summary(mut self, value: FacebookAdReportSummary) -> Self {
        self.report_summary = Some(value);
        self
    }

    pub fn show_report(mut self, value: bool) -> Self {
        self.show_report = Some(value);
        self
    }

    pub fn start_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn status(mut self, value: FacebookAdStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn r#type(mut self, value: FacebookAdType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn web_id(mut self, value: i64) -> Self {
        self.web_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FacebookAd`].
    pub fn build(self) -> Result<FacebookAd, BuildError> {
        Ok(FacebookAd {
            canceled_at: self.canceled_at,
            create_time: self.create_time,
            has_segment: self.has_segment,
            id: self.id,
            name: self.name,
            published_time: self.published_time,
            recipients: self.recipients,
            report_summary: self.report_summary,
            show_report: self.show_report,
            start_time: self.start_time,
            status: self.status,
            r#type: self.r#type,
            updated_at: self.updated_at,
            web_id: self.web_id,
        })
    }
}
