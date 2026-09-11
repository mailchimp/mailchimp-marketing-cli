pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// [RSS](https://mailchimp.com/help/share-your-blog-posts-with-mailchimp/) options for a campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CampaignsRssOpts {
    /// Whether to add CSS to images in the RSS feed to constrain their width in campaigns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constrain_rss_img: Option<bool>,
    /// The URL for the RSS feed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed_url: Option<String>,
    /// The frequency of the RSS Campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<CampaignsRssOptsFrequency>,
    /// The date the campaign was last sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_sent: Option<DateTime<FixedOffset>>,
    /// The schedule for sending the RSS Campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<CampaignsRssOptsSchedule>,
}

impl CampaignsRssOpts {
    pub fn builder() -> CampaignsRssOptsBuilder {
        <CampaignsRssOptsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignsRssOptsBuilder {
    constrain_rss_img: Option<bool>,
    feed_url: Option<String>,
    frequency: Option<CampaignsRssOptsFrequency>,
    last_sent: Option<DateTime<FixedOffset>>,
    schedule: Option<CampaignsRssOptsSchedule>,
}

impl CampaignsRssOptsBuilder {
    pub fn constrain_rss_img(mut self, value: bool) -> Self {
        self.constrain_rss_img = Some(value);
        self
    }

    pub fn feed_url(mut self, value: impl Into<String>) -> Self {
        self.feed_url = Some(value.into());
        self
    }

    pub fn frequency(mut self, value: CampaignsRssOptsFrequency) -> Self {
        self.frequency = Some(value);
        self
    }

    pub fn last_sent(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_sent = Some(value);
        self
    }

    pub fn schedule(mut self, value: CampaignsRssOptsSchedule) -> Self {
        self.schedule = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignsRssOpts`].
    pub fn build(self) -> Result<CampaignsRssOpts, BuildError> {
        Ok(CampaignsRssOpts {
            constrain_rss_img: self.constrain_rss_img,
            feed_url: self.feed_url,
            frequency: self.frequency,
            last_sent: self.last_sent,
            schedule: self.schedule,
        })
    }
}
