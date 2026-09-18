pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A summary of an individual campaign's settings and content.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Campaigns {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<CampaignsLinksItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ab_split_opts: Option<AbTestingOptions>,
    /// The link to the campaign's archive version in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_url: Option<String>,
    /// How the campaign's content is put together.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<CampaignsContentType>,
    /// The date and time the campaign was created in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub create_time: Option<DateTime<FixedOffset>>,
    /// Updates on campaigns in the process of sending.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_status: Option<CampaignsDeliveryStatus>,
    /// The total number of emails sent for this campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails_sent: Option<i64>,
    /// A string that uniquely identifies this campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The original link to the campaign's archive version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_archive_url: Option<String>,
    /// Determines if the campaign needs its blocks refreshed by opening the web-based campaign editor. Deprecated and will always return false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_block_refresh: Option<bool>,
    /// If this campaign is the child of another campaign, this identifies the parent campaign. For Example, for RSS or Automation children.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_campaign_id: Option<String>,
    /// List settings for the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<CampaignsRecipients>,
    /// For sent campaigns, a summary of opens, clicks, and e-commerce data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_summary: Option<CampaignsReportSummary>,
    /// Determines if the campaign qualifies for the Campaign Resend Shortcuts. Only included when query parameter `include_resend_shortcuts` is `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resend_shortcut_eligibility: Option<CampaignsResendShortcutEligibility>,
    /// Information about campaigns related through shortcuts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resend_shortcut_usage: Option<CampaignsResendShortcutUsage>,
    /// Determines if the campaign qualifies to be resent to non-openers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resendable: Option<bool>,
    /// [RSS](https://mailchimp.com/help/share-your-blog-posts-with-mailchimp/) options for a campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rss_opts: Option<CampaignsRssOpts>,
    /// The date and time a campaign was sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub send_time: Option<DateTime<FixedOffset>>,
    /// The settings for your campaign, including subject, from name, reply-to address, and more.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<CampaignsSettings>,
    /// The preview for the campaign, rendered by social networks like Facebook and Twitter. [Learn more](https://mailchimp.com/help/enable-and-customize-social-cards/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_card: Option<CampaignsSocialCard>,
    /// The current status of the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CampaignsStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking: Option<CampaignTrackingOptions>,
    /// There are four types of [campaigns](https://mailchimp.com/help/getting-started-with-campaigns/) you can create in Mailchimp. A/B Split campaigns have been deprecated and variate campaigns should be used instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<CampaignsType>,
    /// The settings specific to A/B test campaigns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variate_settings: Option<CampaignsVariateSettings>,
    /// The ID used in the Mailchimp web application. View this campaign in your Mailchimp account at `https://{dc}.admin.mailchimp.com/campaigns/show/?id={web_id}`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_id: Option<i64>,
}

impl Campaigns {
    pub fn builder() -> CampaignsBuilder {
        <CampaignsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignsBuilder {
    links: Option<Vec<CampaignsLinksItem>>,
    ab_split_opts: Option<AbTestingOptions>,
    archive_url: Option<String>,
    content_type: Option<CampaignsContentType>,
    create_time: Option<DateTime<FixedOffset>>,
    delivery_status: Option<CampaignsDeliveryStatus>,
    emails_sent: Option<i64>,
    id: Option<String>,
    long_archive_url: Option<String>,
    needs_block_refresh: Option<bool>,
    parent_campaign_id: Option<String>,
    recipients: Option<CampaignsRecipients>,
    report_summary: Option<CampaignsReportSummary>,
    resend_shortcut_eligibility: Option<CampaignsResendShortcutEligibility>,
    resend_shortcut_usage: Option<CampaignsResendShortcutUsage>,
    resendable: Option<bool>,
    rss_opts: Option<CampaignsRssOpts>,
    send_time: Option<DateTime<FixedOffset>>,
    settings: Option<CampaignsSettings>,
    social_card: Option<CampaignsSocialCard>,
    status: Option<CampaignsStatus>,
    tracking: Option<CampaignTrackingOptions>,
    r#type: Option<CampaignsType>,
    variate_settings: Option<CampaignsVariateSettings>,
    web_id: Option<i64>,
}

impl CampaignsBuilder {
    pub fn links(mut self, value: Vec<CampaignsLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn ab_split_opts(mut self, value: AbTestingOptions) -> Self {
        self.ab_split_opts = Some(value);
        self
    }

    pub fn archive_url(mut self, value: impl Into<String>) -> Self {
        self.archive_url = Some(value.into());
        self
    }

    pub fn content_type(mut self, value: CampaignsContentType) -> Self {
        self.content_type = Some(value);
        self
    }

    pub fn create_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.create_time = Some(value);
        self
    }

    pub fn delivery_status(mut self, value: CampaignsDeliveryStatus) -> Self {
        self.delivery_status = Some(value);
        self
    }

    pub fn emails_sent(mut self, value: i64) -> Self {
        self.emails_sent = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn long_archive_url(mut self, value: impl Into<String>) -> Self {
        self.long_archive_url = Some(value.into());
        self
    }

    pub fn needs_block_refresh(mut self, value: bool) -> Self {
        self.needs_block_refresh = Some(value);
        self
    }

    pub fn parent_campaign_id(mut self, value: impl Into<String>) -> Self {
        self.parent_campaign_id = Some(value.into());
        self
    }

    pub fn recipients(mut self, value: CampaignsRecipients) -> Self {
        self.recipients = Some(value);
        self
    }

    pub fn report_summary(mut self, value: CampaignsReportSummary) -> Self {
        self.report_summary = Some(value);
        self
    }

    pub fn resend_shortcut_eligibility(mut self, value: CampaignsResendShortcutEligibility) -> Self {
        self.resend_shortcut_eligibility = Some(value);
        self
    }

    pub fn resend_shortcut_usage(mut self, value: CampaignsResendShortcutUsage) -> Self {
        self.resend_shortcut_usage = Some(value);
        self
    }

    pub fn resendable(mut self, value: bool) -> Self {
        self.resendable = Some(value);
        self
    }

    pub fn rss_opts(mut self, value: CampaignsRssOpts) -> Self {
        self.rss_opts = Some(value);
        self
    }

    pub fn send_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.send_time = Some(value);
        self
    }

    pub fn settings(mut self, value: CampaignsSettings) -> Self {
        self.settings = Some(value);
        self
    }

    pub fn social_card(mut self, value: CampaignsSocialCard) -> Self {
        self.social_card = Some(value);
        self
    }

    pub fn status(mut self, value: CampaignsStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tracking(mut self, value: CampaignTrackingOptions) -> Self {
        self.tracking = Some(value);
        self
    }

    pub fn r#type(mut self, value: CampaignsType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn variate_settings(mut self, value: CampaignsVariateSettings) -> Self {
        self.variate_settings = Some(value);
        self
    }

    pub fn web_id(mut self, value: i64) -> Self {
        self.web_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Campaigns`].
    pub fn build(self) -> Result<Campaigns, BuildError> {
        Ok(Campaigns {
            links: self.links,
            ab_split_opts: self.ab_split_opts,
            archive_url: self.archive_url,
            content_type: self.content_type,
            create_time: self.create_time,
            delivery_status: self.delivery_status,
            emails_sent: self.emails_sent,
            id: self.id,
            long_archive_url: self.long_archive_url,
            needs_block_refresh: self.needs_block_refresh,
            parent_campaign_id: self.parent_campaign_id,
            recipients: self.recipients,
            report_summary: self.report_summary,
            resend_shortcut_eligibility: self.resend_shortcut_eligibility,
            resend_shortcut_usage: self.resend_shortcut_usage,
            resendable: self.resendable,
            rss_opts: self.rss_opts,
            send_time: self.send_time,
            settings: self.settings,
            social_card: self.social_card,
            status: self.status,
            tracking: self.tracking,
            r#type: self.r#type,
            variate_settings: self.variate_settings,
            web_id: self.web_id,
        })
    }
}
