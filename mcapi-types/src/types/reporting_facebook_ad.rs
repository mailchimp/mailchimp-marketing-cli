pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReportingFacebookAd {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_source_name: Option<String>,
    /// The date and time the ad was ended in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub end_time: Option<DateTime<FixedOffset>>,
    /// If the ad has a problem and needs attention.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_attention: Option<bool>,
    /// The date and time the ad was paused in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub paused_at: Option<DateTime<FixedOffset>>,
    /// The URL of the thumbnail for this outreach.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_canceled_by_facebook: Option<bool>,
    /// Audience settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<ReportingFacebookAdAudience>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience_activity: Option<ReportingFacebookAdAudienceActivity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget: Option<ReportingFacebookAdBudget>,
    /// Channel settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<ReportingFacebookAdChannel>,
    /// Report summary of facebook ad
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_summary: Option<ReportingFacebookAdReportSummary>,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ReportingFacebookAdLinksItem>>,
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

impl ReportingFacebookAd {
    pub fn builder() -> ReportingFacebookAdBuilder {
        <ReportingFacebookAdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportingFacebookAdBuilder {
    email_source_name: Option<String>,
    end_time: Option<DateTime<FixedOffset>>,
    needs_attention: Option<bool>,
    paused_at: Option<DateTime<FixedOffset>>,
    thumbnail: Option<String>,
    was_canceled_by_facebook: Option<bool>,
    audience: Option<ReportingFacebookAdAudience>,
    audience_activity: Option<ReportingFacebookAdAudienceActivity>,
    budget: Option<ReportingFacebookAdBudget>,
    channel: Option<ReportingFacebookAdChannel>,
    report_summary: Option<ReportingFacebookAdReportSummary>,
    links: Option<Vec<ReportingFacebookAdLinksItem>>,
    canceled_at: Option<DateTime<FixedOffset>>,
    create_time: Option<DateTime<FixedOffset>>,
    has_segment: Option<bool>,
    id: Option<String>,
    name: Option<String>,
    published_time: Option<DateTime<FixedOffset>>,
    recipients: Option<FacebookAdRecipients>,
    show_report: Option<bool>,
    start_time: Option<DateTime<FixedOffset>>,
    status: Option<FacebookAdStatus>,
    r#type: Option<FacebookAdType>,
    updated_at: Option<DateTime<FixedOffset>>,
    web_id: Option<i64>,
}

impl ReportingFacebookAdBuilder {
    pub fn email_source_name(mut self, value: impl Into<String>) -> Self {
        self.email_source_name = Some(value.into());
        self
    }

    pub fn end_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.end_time = Some(value);
        self
    }

    pub fn needs_attention(mut self, value: bool) -> Self {
        self.needs_attention = Some(value);
        self
    }

    pub fn paused_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.paused_at = Some(value);
        self
    }

    pub fn thumbnail(mut self, value: impl Into<String>) -> Self {
        self.thumbnail = Some(value.into());
        self
    }

    pub fn was_canceled_by_facebook(mut self, value: bool) -> Self {
        self.was_canceled_by_facebook = Some(value);
        self
    }

    pub fn audience(mut self, value: ReportingFacebookAdAudience) -> Self {
        self.audience = Some(value);
        self
    }

    pub fn audience_activity(mut self, value: ReportingFacebookAdAudienceActivity) -> Self {
        self.audience_activity = Some(value);
        self
    }

    pub fn budget(mut self, value: ReportingFacebookAdBudget) -> Self {
        self.budget = Some(value);
        self
    }

    pub fn channel(mut self, value: ReportingFacebookAdChannel) -> Self {
        self.channel = Some(value);
        self
    }

    pub fn report_summary(mut self, value: ReportingFacebookAdReportSummary) -> Self {
        self.report_summary = Some(value);
        self
    }

    pub fn links(mut self, value: Vec<ReportingFacebookAdLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

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

    /// Consumes the builder and constructs a [`ReportingFacebookAd`].
    pub fn build(self) -> Result<ReportingFacebookAd, BuildError> {
        Ok(ReportingFacebookAd {
            email_source_name: self.email_source_name,
            end_time: self.end_time,
            needs_attention: self.needs_attention,
            paused_at: self.paused_at,
            thumbnail: self.thumbnail,
            was_canceled_by_facebook: self.was_canceled_by_facebook,
            audience: self.audience,
            audience_activity: self.audience_activity,
            budget: self.budget,
            channel: self.channel,
            report_summary: self.report_summary,
            links: self.links,
            canceled_at: self.canceled_at,
            create_time: self.create_time,
            has_segment: self.has_segment,
            id: self.id,
            name: self.name,
            published_time: self.published_time,
            recipients: self.recipients,
            show_report: self.show_report,
            start_time: self.start_time,
            status: self.status,
            r#type: self.r#type,
            updated_at: self.updated_at,
            web_id: self.web_id,
        })
    }
}
