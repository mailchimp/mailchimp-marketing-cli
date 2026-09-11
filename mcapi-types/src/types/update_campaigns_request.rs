pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateCampaignsRequest {
    /// List settings for the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<UpdateCampaignsRequestRecipients>,
    /// [RSS](https://mailchimp.com/help/share-your-blog-posts-with-mailchimp/) options for a campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rss_opts: Option<UpdateCampaignsRequestRssOpts>,
    /// The settings for your campaign, including subject, from name, reply-to address, and more.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<UpdateCampaignsRequestSettings>,
    /// The preview for the campaign, rendered by social networks like Facebook and Twitter. [Learn more](https://mailchimp.com/help/enable-and-customize-social-cards/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_card: Option<UpdateCampaignsRequestSocialCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking: Option<CampaignTrackingOptions>,
    /// The settings specific to A/B test campaigns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variate_settings: Option<UpdateCampaignsRequestVariateSettings>,
}

impl UpdateCampaignsRequest {
    pub fn builder() -> UpdateCampaignsRequestBuilder {
        <UpdateCampaignsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCampaignsRequestBuilder {
    recipients: Option<UpdateCampaignsRequestRecipients>,
    rss_opts: Option<UpdateCampaignsRequestRssOpts>,
    settings: Option<UpdateCampaignsRequestSettings>,
    social_card: Option<UpdateCampaignsRequestSocialCard>,
    tracking: Option<CampaignTrackingOptions>,
    variate_settings: Option<UpdateCampaignsRequestVariateSettings>,
}

impl UpdateCampaignsRequestBuilder {
    pub fn recipients(mut self, value: UpdateCampaignsRequestRecipients) -> Self {
        self.recipients = Some(value);
        self
    }

    pub fn rss_opts(mut self, value: UpdateCampaignsRequestRssOpts) -> Self {
        self.rss_opts = Some(value);
        self
    }

    pub fn settings(mut self, value: UpdateCampaignsRequestSettings) -> Self {
        self.settings = Some(value);
        self
    }

    pub fn social_card(mut self, value: UpdateCampaignsRequestSocialCard) -> Self {
        self.social_card = Some(value);
        self
    }

    pub fn tracking(mut self, value: CampaignTrackingOptions) -> Self {
        self.tracking = Some(value);
        self
    }

    pub fn variate_settings(mut self, value: UpdateCampaignsRequestVariateSettings) -> Self {
        self.variate_settings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateCampaignsRequest`].
    pub fn build(self) -> Result<UpdateCampaignsRequest, BuildError> {
        Ok(UpdateCampaignsRequest {
            recipients: self.recipients,
            rss_opts: self.rss_opts,
            settings: self.settings,
            social_card: self.social_card,
            tracking: self.tracking,
            variate_settings: self.variate_settings,
        })
    }
}

