pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Channel settings
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReportingFacebookAdChannel {
    /// Is this for facebook audience
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fb_placement_audience: Option<bool>,
    /// Is this for facebook feed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fb_placement_feed: Option<bool>,
    /// Is this for instagram feed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ig_placement_feed: Option<bool>,
}

impl ReportingFacebookAdChannel {
    pub fn builder() -> ReportingFacebookAdChannelBuilder {
        <ReportingFacebookAdChannelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportingFacebookAdChannelBuilder {
    fb_placement_audience: Option<bool>,
    fb_placement_feed: Option<bool>,
    ig_placement_feed: Option<bool>,
}

impl ReportingFacebookAdChannelBuilder {
    pub fn fb_placement_audience(mut self, value: bool) -> Self {
        self.fb_placement_audience = Some(value);
        self
    }

    pub fn fb_placement_feed(mut self, value: bool) -> Self {
        self.fb_placement_feed = Some(value);
        self
    }

    pub fn ig_placement_feed(mut self, value: bool) -> Self {
        self.ig_placement_feed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReportingFacebookAdChannel`].
    pub fn build(self) -> Result<ReportingFacebookAdChannel, BuildError> {
        Ok(ReportingFacebookAdChannel {
            fb_placement_audience: self.fb_placement_audience,
            fb_placement_feed: self.fb_placement_feed,
            ig_placement_feed: self.ig_placement_feed,
        })
    }
}
