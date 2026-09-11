pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An object describing campaign engagement on Facebook.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CampaignReportFacebookLikes {
    /// The number of Facebook likes for the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facebook_likes: Option<i64>,
    /// The number of recipients who liked the campaign on Facebook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_likes: Option<i64>,
    /// The number of unique likes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_likes: Option<i64>,
}

impl CampaignReportFacebookLikes {
    pub fn builder() -> CampaignReportFacebookLikesBuilder {
        <CampaignReportFacebookLikesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignReportFacebookLikesBuilder {
    facebook_likes: Option<i64>,
    recipient_likes: Option<i64>,
    unique_likes: Option<i64>,
}

impl CampaignReportFacebookLikesBuilder {
    pub fn facebook_likes(mut self, value: i64) -> Self {
        self.facebook_likes = Some(value);
        self
    }

    pub fn recipient_likes(mut self, value: i64) -> Self {
        self.recipient_likes = Some(value);
        self
    }

    pub fn unique_likes(mut self, value: i64) -> Self {
        self.unique_likes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignReportFacebookLikes`].
    pub fn build(self) -> Result<CampaignReportFacebookLikes, BuildError> {
        Ok(CampaignReportFacebookLikes {
            facebook_likes: self.facebook_likes,
            recipient_likes: self.recipient_likes,
            unique_likes: self.unique_likes,
        })
    }
}
