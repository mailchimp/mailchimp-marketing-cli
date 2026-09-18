pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// [RSS](https://mailchimp.com/help/share-your-blog-posts-with-mailchimp/) options, specific to an RSS campaign.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateRequestRssOpts {
    /// Whether to add CSS to images in the RSS feed to constrain their width in campaigns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constrain_rss_img: Option<bool>,
    /// The URL for the RSS feed.
    #[serde(default)]
    pub feed_url: String,
    /// The frequency of the RSS Campaign.
    pub frequency: CreateRequestRssOptsFrequency,
    /// The schedule for sending the RSS Campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<CreateRequestRssOptsSchedule>,
}

impl CreateRequestRssOpts {
    pub fn builder() -> CreateRequestRssOptsBuilder {
        <CreateRequestRssOptsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateRequestRssOptsBuilder {
    constrain_rss_img: Option<bool>,
    feed_url: Option<String>,
    frequency: Option<CreateRequestRssOptsFrequency>,
    schedule: Option<CreateRequestRssOptsSchedule>,
}

impl CreateRequestRssOptsBuilder {
    pub fn constrain_rss_img(mut self, value: bool) -> Self {
        self.constrain_rss_img = Some(value);
        self
    }

    pub fn feed_url(mut self, value: impl Into<String>) -> Self {
        self.feed_url = Some(value.into());
        self
    }

    pub fn frequency(mut self, value: CreateRequestRssOptsFrequency) -> Self {
        self.frequency = Some(value);
        self
    }

    pub fn schedule(mut self, value: CreateRequestRssOptsSchedule) -> Self {
        self.schedule = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateRequestRssOpts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`feed_url`](CreateRequestRssOptsBuilder::feed_url)
    /// - [`frequency`](CreateRequestRssOptsBuilder::frequency)
    pub fn build(self) -> Result<CreateRequestRssOpts, BuildError> {
        Ok(CreateRequestRssOpts {
            constrain_rss_img: self.constrain_rss_img,
            feed_url: self.feed_url.ok_or_else(|| BuildError::missing_field("feed_url"))?,
            frequency: self.frequency.ok_or_else(|| BuildError::missing_field("frequency"))?,
            schedule: self.schedule,
        })
    }
}
