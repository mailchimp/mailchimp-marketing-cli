pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateCampaignsRequest {
    /// How the campaign's content is put together. The old drag and drop editor uses 'template' while the new editor uses 'multichannel'. Defaults to template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<CreateCampaignsRequestContentType>,
    /// List settings for the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<CreateCampaignsRequestRecipients>,
    /// [RSS](https://mailchimp.com/help/share-your-blog-posts-with-mailchimp/) options, specific to an RSS campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rss_opts: Option<CreateCampaignsRequestRssOpts>,
    /// The settings for your campaign, including subject, from name, reply-to address, and more.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<CreateCampaignsRequestSettings>,
    /// The preview for the campaign, rendered by social networks like Facebook and Twitter. [Learn more](https://mailchimp.com/help/enable-and-customize-social-cards/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_card: Option<CreateCampaignsRequestSocialCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking: Option<CampaignTrackingOptions>,
    /// There are four types of [campaigns](https://mailchimp.com/help/getting-started-with-campaigns/) you can create in Mailchimp. A/B Split campaigns have been deprecated and variate campaigns should be used instead.
    pub r#type: CreateCampaignsRequestType,
    /// The settings specific to A/B test campaigns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variate_settings: Option<CreateCampaignsRequestVariateSettings>,
}

impl CreateCampaignsRequest {
    pub fn builder() -> CreateCampaignsRequestBuilder {
        <CreateCampaignsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCampaignsRequestBuilder {
    content_type: Option<CreateCampaignsRequestContentType>,
    recipients: Option<CreateCampaignsRequestRecipients>,
    rss_opts: Option<CreateCampaignsRequestRssOpts>,
    settings: Option<CreateCampaignsRequestSettings>,
    social_card: Option<CreateCampaignsRequestSocialCard>,
    tracking: Option<CampaignTrackingOptions>,
    r#type: Option<CreateCampaignsRequestType>,
    variate_settings: Option<CreateCampaignsRequestVariateSettings>,
}

impl CreateCampaignsRequestBuilder {
    pub fn content_type(mut self, value: CreateCampaignsRequestContentType) -> Self {
        self.content_type = Some(value);
        self
    }

    pub fn recipients(mut self, value: CreateCampaignsRequestRecipients) -> Self {
        self.recipients = Some(value);
        self
    }

    pub fn rss_opts(mut self, value: CreateCampaignsRequestRssOpts) -> Self {
        self.rss_opts = Some(value);
        self
    }

    pub fn settings(mut self, value: CreateCampaignsRequestSettings) -> Self {
        self.settings = Some(value);
        self
    }

    pub fn social_card(mut self, value: CreateCampaignsRequestSocialCard) -> Self {
        self.social_card = Some(value);
        self
    }

    pub fn tracking(mut self, value: CampaignTrackingOptions) -> Self {
        self.tracking = Some(value);
        self
    }

    pub fn r#type(mut self, value: CreateCampaignsRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn variate_settings(mut self, value: CreateCampaignsRequestVariateSettings) -> Self {
        self.variate_settings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateCampaignsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](CreateCampaignsRequestBuilder::r#type)
    pub fn build(self) -> Result<CreateCampaignsRequest, BuildError> {
        Ok(CreateCampaignsRequest {
            content_type: self.content_type,
            recipients: self.recipients,
            rss_opts: self.rss_opts,
            settings: self.settings,
            social_card: self.social_card,
            tracking: self.tracking,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            variate_settings: self.variate_settings,
        })
    }
}

