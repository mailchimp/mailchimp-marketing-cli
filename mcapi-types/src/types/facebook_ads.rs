pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FacebookAds {
    #[serde(flatten)]
    pub facebook_ad_fields: FacebookAd,
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
    pub audience: Option<FacebookAdsAudience>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget: Option<FacebookAdsBudget>,
    /// Channel settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<FacebookAdsChannel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<FacebookAdsContent>,
    /// Check if this ad is connected to a facebook page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<FacebookAdsFeedback>,
    /// Check if this ad has audience setup
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_audience: Option<bool>,
    /// Check if this ad has content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_content: Option<bool>,
    /// Check if this ad is connected to a facebook page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_connected: Option<bool>,
    /// Connected Site
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<FacebookAdsSite>,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<FacebookAdsLinksItem>>,
}

impl FacebookAds {
    pub fn builder() -> FacebookAdsBuilder {
        <FacebookAdsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FacebookAdsBuilder {
    facebook_ad_fields: Option<FacebookAd>,
    email_source_name: Option<String>,
    end_time: Option<DateTime<FixedOffset>>,
    needs_attention: Option<bool>,
    paused_at: Option<DateTime<FixedOffset>>,
    thumbnail: Option<String>,
    was_canceled_by_facebook: Option<bool>,
    audience: Option<FacebookAdsAudience>,
    budget: Option<FacebookAdsBudget>,
    channel: Option<FacebookAdsChannel>,
    content: Option<FacebookAdsContent>,
    feedback: Option<FacebookAdsFeedback>,
    has_audience: Option<bool>,
    has_content: Option<bool>,
    is_connected: Option<bool>,
    site: Option<FacebookAdsSite>,
    links: Option<Vec<FacebookAdsLinksItem>>,
}

impl FacebookAdsBuilder {
    pub fn facebook_ad_fields(mut self, value: FacebookAd) -> Self {
        self.facebook_ad_fields = Some(value);
        self
    }

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

    pub fn audience(mut self, value: FacebookAdsAudience) -> Self {
        self.audience = Some(value);
        self
    }

    pub fn budget(mut self, value: FacebookAdsBudget) -> Self {
        self.budget = Some(value);
        self
    }

    pub fn channel(mut self, value: FacebookAdsChannel) -> Self {
        self.channel = Some(value);
        self
    }

    pub fn content(mut self, value: FacebookAdsContent) -> Self {
        self.content = Some(value);
        self
    }

    pub fn feedback(mut self, value: FacebookAdsFeedback) -> Self {
        self.feedback = Some(value);
        self
    }

    pub fn has_audience(mut self, value: bool) -> Self {
        self.has_audience = Some(value);
        self
    }

    pub fn has_content(mut self, value: bool) -> Self {
        self.has_content = Some(value);
        self
    }

    pub fn is_connected(mut self, value: bool) -> Self {
        self.is_connected = Some(value);
        self
    }

    pub fn site(mut self, value: FacebookAdsSite) -> Self {
        self.site = Some(value);
        self
    }

    pub fn links(mut self, value: Vec<FacebookAdsLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FacebookAds`].
    /// This method will fail if any of the following fields are not set:
    /// - [`facebook_ad_fields`](FacebookAdsBuilder::facebook_ad_fields)
    pub fn build(self) -> Result<FacebookAds, BuildError> {
        Ok(FacebookAds {
            facebook_ad_fields: self.facebook_ad_fields.ok_or_else(|| BuildError::missing_field("facebook_ad_fields"))?,
            email_source_name: self.email_source_name,
            end_time: self.end_time,
            needs_attention: self.needs_attention,
            paused_at: self.paused_at,
            thumbnail: self.thumbnail,
            was_canceled_by_facebook: self.was_canceled_by_facebook,
            audience: self.audience,
            budget: self.budget,
            channel: self.channel,
            content: self.content,
            feedback: self.feedback,
            has_audience: self.has_audience,
            has_content: self.has_content,
            is_connected: self.is_connected,
            site: self.site,
            links: self.links,
        })
    }
}
