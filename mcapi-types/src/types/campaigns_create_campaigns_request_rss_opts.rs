pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// [RSS](https://mailchimp.com/help/share-your-blog-posts-with-mailchimp/) options, specific to an RSS campaign.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateCampaignsRequestRssOpts {
    /// Whether to add CSS to images in the RSS feed to constrain their width in campaigns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constrain_rss_img: Option<bool>,
    /// The URL for the RSS feed.
    #[serde(default)]
    pub feed_url: String,
    /// The frequency of the RSS Campaign.
    pub frequency: CreateCampaignsRequestRssOptsFrequency,
    /// The schedule for sending the RSS Campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<CreateCampaignsRequestRssOptsSchedule>,
}

impl CreateCampaignsRequestRssOpts {
    pub fn builder() -> CreateCampaignsRequestRssOptsBuilder {
        <CreateCampaignsRequestRssOptsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCampaignsRequestRssOptsBuilder {
    constrain_rss_img: Option<bool>,
    feed_url: Option<String>,
    frequency: Option<CreateCampaignsRequestRssOptsFrequency>,
    schedule: Option<CreateCampaignsRequestRssOptsSchedule>,
}

impl CreateCampaignsRequestRssOptsBuilder {
    pub fn constrain_rss_img(mut self, value: bool) -> Self {
        self.constrain_rss_img = Some(value);
        self
    }

    pub fn feed_url(mut self, value: impl Into<String>) -> Self {
        self.feed_url = Some(value.into());
        self
    }

    pub fn frequency(mut self, value: CreateCampaignsRequestRssOptsFrequency) -> Self {
        self.frequency = Some(value);
        self
    }

    pub fn schedule(mut self, value: CreateCampaignsRequestRssOptsSchedule) -> Self {
        self.schedule = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateCampaignsRequestRssOpts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`feed_url`](CreateCampaignsRequestRssOptsBuilder::feed_url)
    /// - [`frequency`](CreateCampaignsRequestRssOptsBuilder::frequency)
    pub fn build(self) -> Result<CreateCampaignsRequestRssOpts, BuildError> {
        Ok(CreateCampaignsRequestRssOpts {
            constrain_rss_img: self.constrain_rss_img,
            feed_url: self.feed_url.ok_or_else(|| BuildError::missing_field("feed_url"))?,
            frequency: self.frequency.ok_or_else(|| BuildError::missing_field("frequency"))?,
            schedule: self.schedule,
        })
    }
}
