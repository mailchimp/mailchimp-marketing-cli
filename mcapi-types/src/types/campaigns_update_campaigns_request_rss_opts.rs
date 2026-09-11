pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// [RSS](https://mailchimp.com/help/share-your-blog-posts-with-mailchimp/) options for a campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateCampaignsRequestRssOpts {
    /// Whether to add CSS to images in the RSS feed to constrain their width in campaigns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constrain_rss_img: Option<bool>,
    /// The URL for the RSS feed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed_url: Option<String>,
    /// The frequency of the RSS Campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<UpdateCampaignsRequestRssOptsFrequency>,
    /// The schedule for sending the RSS Campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<UpdateCampaignsRequestRssOptsSchedule>,
}

impl UpdateCampaignsRequestRssOpts {
    pub fn builder() -> UpdateCampaignsRequestRssOptsBuilder {
        <UpdateCampaignsRequestRssOptsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCampaignsRequestRssOptsBuilder {
    constrain_rss_img: Option<bool>,
    feed_url: Option<String>,
    frequency: Option<UpdateCampaignsRequestRssOptsFrequency>,
    schedule: Option<UpdateCampaignsRequestRssOptsSchedule>,
}

impl UpdateCampaignsRequestRssOptsBuilder {
    pub fn constrain_rss_img(mut self, value: bool) -> Self {
        self.constrain_rss_img = Some(value);
        self
    }

    pub fn feed_url(mut self, value: impl Into<String>) -> Self {
        self.feed_url = Some(value.into());
        self
    }

    pub fn frequency(mut self, value: UpdateCampaignsRequestRssOptsFrequency) -> Self {
        self.frequency = Some(value);
        self
    }

    pub fn schedule(mut self, value: UpdateCampaignsRequestRssOptsSchedule) -> Self {
        self.schedule = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateCampaignsRequestRssOpts`].
    pub fn build(self) -> Result<UpdateCampaignsRequestRssOpts, BuildError> {
        Ok(UpdateCampaignsRequestRssOpts {
            constrain_rss_img: self.constrain_rss_img,
            feed_url: self.feed_url,
            frequency: self.frequency,
            schedule: self.schedule,
        })
    }
}
