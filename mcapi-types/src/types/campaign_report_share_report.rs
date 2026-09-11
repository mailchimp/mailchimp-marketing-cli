pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The url and password for the [VIP report](https://mailchimp.com/help/share-a-campaign-report/).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CampaignReportShareReport {
    /// If password protected, the password for the VIP report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_password: Option<String>,
    /// The URL for the VIP report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_url: Option<String>,
}

impl CampaignReportShareReport {
    pub fn builder() -> CampaignReportShareReportBuilder {
        <CampaignReportShareReportBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignReportShareReportBuilder {
    share_password: Option<String>,
    share_url: Option<String>,
}

impl CampaignReportShareReportBuilder {
    pub fn share_password(mut self, value: impl Into<String>) -> Self {
        self.share_password = Some(value.into());
        self
    }

    pub fn share_url(mut self, value: impl Into<String>) -> Self {
        self.share_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CampaignReportShareReport`].
    pub fn build(self) -> Result<CampaignReportShareReport, BuildError> {
        Ok(CampaignReportShareReport {
            share_password: self.share_password,
            share_url: self.share_url,
        })
    }
}
