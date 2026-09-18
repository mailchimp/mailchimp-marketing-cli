pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Report details about a sent campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CampaignReport {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<CampaignReportLinksItem>>,
    /// General stats about different groups of an A/B Split campaign. Does not return information about Multivariate Campaigns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ab_split: Option<CampaignReportAbSplit>,
    /// The number of abuse reports generated for this campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abuse_reports: Option<i64>,
    /// An object describing the bounce summary for the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounces: Option<CampaignReportBounces>,
    /// The title of the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_title: Option<String>,
    /// An object describing the click activity for the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicks: Option<CampaignReportClicks>,
    /// Updates on campaigns in the process of sending.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_status: Option<CampaignReportDeliveryStatus>,
    /// E-Commerce stats for a campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecommerce: Option<CampaignReportEcommerce>,
    /// The total number of emails sent for this campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails_sent: Option<i64>,
    /// An object describing campaign engagement on Facebook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facebook_likes: Option<CampaignReportFacebookLikes>,
    /// An object describing the forwards and forward activity for the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forwards: Option<CampaignReportForwards>,
    /// A string that uniquely identifies this campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The average campaign statistics for your industry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub industry_stats: Option<CampaignReportIndustryStats>,
    /// The unique list id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The status of the list used, namely if it's deleted or disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_is_active: Option<bool>,
    /// The name of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_name: Option<String>,
    /// The average campaign statistics for your list. This won't be present if we haven't calculated it yet for this list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_stats: Option<CampaignReportListStats>,
    /// An object describing the open activity for the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opens: Option<CampaignReportOpens>,
    /// The preview text for the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_text: Option<String>,
    /// For RSS campaigns, the date and time of the last send in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub rss_last_send: Option<DateTime<FixedOffset>>,
    /// The date and time a campaign was sent in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub send_time: Option<DateTime<FixedOffset>>,
    /// The url and password for the [VIP report](https://mailchimp.com/help/share-a-campaign-report/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_report: Option<CampaignReportShareReport>,
    /// The subject line for the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_line: Option<String>,
    /// An hourly breakdown of the performance of the campaign over the first 24 hours.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeseries: Option<Vec<CampaignReportTimeseriesItem>>,
    /// An hourly breakdown of sends, opens, and clicks if a campaign is sent using timewarp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timewarp: Option<Vec<CampaignReportTimewarpItem>>,
    /// The type of campaign (regular, plain-text, ab_split, rss, automation, variate, or auto).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The total number of unsubscribed members for this campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribed: Option<i64>,
}

impl CampaignReport {
    pub fn builder() -> CampaignReportBuilder {
        <CampaignReportBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignReportBuilder {
    links: Option<Vec<CampaignReportLinksItem>>,
    ab_split: Option<CampaignReportAbSplit>,
    abuse_reports: Option<i64>,
    bounces: Option<CampaignReportBounces>,
    campaign_title: Option<String>,
    clicks: Option<CampaignReportClicks>,
    delivery_status: Option<CampaignReportDeliveryStatus>,
    ecommerce: Option<CampaignReportEcommerce>,
    emails_sent: Option<i64>,
    facebook_likes: Option<CampaignReportFacebookLikes>,
    forwards: Option<CampaignReportForwards>,
    id: Option<String>,
    industry_stats: Option<CampaignReportIndustryStats>,
    list_id: Option<String>,
    list_is_active: Option<bool>,
    list_name: Option<String>,
    list_stats: Option<CampaignReportListStats>,
    opens: Option<CampaignReportOpens>,
    preview_text: Option<String>,
    rss_last_send: Option<DateTime<FixedOffset>>,
    send_time: Option<DateTime<FixedOffset>>,
    share_report: Option<CampaignReportShareReport>,
    subject_line: Option<String>,
    timeseries: Option<Vec<CampaignReportTimeseriesItem>>,
    timewarp: Option<Vec<CampaignReportTimewarpItem>>,
    r#type: Option<String>,
    unsubscribed: Option<i64>,
}

impl CampaignReportBuilder {
    pub fn links(mut self, value: Vec<CampaignReportLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn ab_split(mut self, value: CampaignReportAbSplit) -> Self {
        self.ab_split = Some(value);
        self
    }

    pub fn abuse_reports(mut self, value: i64) -> Self {
        self.abuse_reports = Some(value);
        self
    }

    pub fn bounces(mut self, value: CampaignReportBounces) -> Self {
        self.bounces = Some(value);
        self
    }

    pub fn campaign_title(mut self, value: impl Into<String>) -> Self {
        self.campaign_title = Some(value.into());
        self
    }

    pub fn clicks(mut self, value: CampaignReportClicks) -> Self {
        self.clicks = Some(value);
        self
    }

    pub fn delivery_status(mut self, value: CampaignReportDeliveryStatus) -> Self {
        self.delivery_status = Some(value);
        self
    }

    pub fn ecommerce(mut self, value: CampaignReportEcommerce) -> Self {
        self.ecommerce = Some(value);
        self
    }

    pub fn emails_sent(mut self, value: i64) -> Self {
        self.emails_sent = Some(value);
        self
    }

    pub fn facebook_likes(mut self, value: CampaignReportFacebookLikes) -> Self {
        self.facebook_likes = Some(value);
        self
    }

    pub fn forwards(mut self, value: CampaignReportForwards) -> Self {
        self.forwards = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn industry_stats(mut self, value: CampaignReportIndustryStats) -> Self {
        self.industry_stats = Some(value);
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

    pub fn list_name(mut self, value: impl Into<String>) -> Self {
        self.list_name = Some(value.into());
        self
    }

    pub fn list_stats(mut self, value: CampaignReportListStats) -> Self {
        self.list_stats = Some(value);
        self
    }

    pub fn opens(mut self, value: CampaignReportOpens) -> Self {
        self.opens = Some(value);
        self
    }

    pub fn preview_text(mut self, value: impl Into<String>) -> Self {
        self.preview_text = Some(value.into());
        self
    }

    pub fn rss_last_send(mut self, value: DateTime<FixedOffset>) -> Self {
        self.rss_last_send = Some(value);
        self
    }

    pub fn send_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.send_time = Some(value);
        self
    }

    pub fn share_report(mut self, value: CampaignReportShareReport) -> Self {
        self.share_report = Some(value);
        self
    }

    pub fn subject_line(mut self, value: impl Into<String>) -> Self {
        self.subject_line = Some(value.into());
        self
    }

    pub fn timeseries(mut self, value: Vec<CampaignReportTimeseriesItem>) -> Self {
        self.timeseries = Some(value);
        self
    }

    pub fn timewarp(mut self, value: Vec<CampaignReportTimewarpItem>) -> Self {
        self.timewarp = Some(value);
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn unsubscribed(mut self, value: i64) -> Self {
        self.unsubscribed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignReport`].
    pub fn build(self) -> Result<CampaignReport, BuildError> {
        Ok(CampaignReport {
            links: self.links,
            ab_split: self.ab_split,
            abuse_reports: self.abuse_reports,
            bounces: self.bounces,
            campaign_title: self.campaign_title,
            clicks: self.clicks,
            delivery_status: self.delivery_status,
            ecommerce: self.ecommerce,
            emails_sent: self.emails_sent,
            facebook_likes: self.facebook_likes,
            forwards: self.forwards,
            id: self.id,
            industry_stats: self.industry_stats,
            list_id: self.list_id,
            list_is_active: self.list_is_active,
            list_name: self.list_name,
            list_stats: self.list_stats,
            opens: self.opens,
            preview_text: self.preview_text,
            rss_last_send: self.rss_last_send,
            send_time: self.send_time,
            share_report: self.share_report,
            subject_line: self.subject_line,
            timeseries: self.timeseries,
            timewarp: self.timewarp,
            r#type: self.r#type,
            unsubscribed: self.unsubscribed,
        })
    }
}
