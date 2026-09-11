pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// [RSS](https://mailchimp.com/help/share-your-blog-posts-with-mailchimp/) options for a campaign.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CampaignRssOpts {
    /// Whether to add CSS to images in the RSS feed to constrain their width in campaigns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constrain_rss_img: Option<bool>,
    /// The URL for the RSS feed.
    #[serde(default)]
    pub feed_url: String,
    /// The frequency of the RSS Campaign.
    pub frequency: CampaignRssOptsFrequency,
    /// The date the campaign was last sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_sent: Option<DateTime<FixedOffset>>,
    /// The schedule for sending the RSS Campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<CampaignRssOptsSchedule>,
}

impl CampaignRssOpts {
    pub fn builder() -> CampaignRssOptsBuilder {
        <CampaignRssOptsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignRssOptsBuilder {
    constrain_rss_img: Option<bool>,
    feed_url: Option<String>,
    frequency: Option<CampaignRssOptsFrequency>,
    last_sent: Option<DateTime<FixedOffset>>,
    schedule: Option<CampaignRssOptsSchedule>,
}

impl CampaignRssOptsBuilder {
    pub fn constrain_rss_img(mut self, value: bool) -> Self {
        self.constrain_rss_img = Some(value);
        self
    }

    pub fn feed_url(mut self, value: impl Into<String>) -> Self {
        self.feed_url = Some(value.into());
        self
    }

    pub fn frequency(mut self, value: CampaignRssOptsFrequency) -> Self {
        self.frequency = Some(value);
        self
    }

    pub fn last_sent(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_sent = Some(value);
        self
    }

    pub fn schedule(mut self, value: CampaignRssOptsSchedule) -> Self {
        self.schedule = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignRssOpts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`feed_url`](CampaignRssOptsBuilder::feed_url)
    /// - [`frequency`](CampaignRssOptsBuilder::frequency)
    pub fn build(self) -> Result<CampaignRssOpts, BuildError> {
        Ok(CampaignRssOpts {
            constrain_rss_img: self.constrain_rss_img,
            feed_url: self.feed_url.ok_or_else(|| BuildError::missing_field("feed_url"))?,
            frequency: self.frequency.ok_or_else(|| BuildError::missing_field("frequency"))?,
            last_sent: self.last_sent,
            schedule: self.schedule,
        })
    }
}
