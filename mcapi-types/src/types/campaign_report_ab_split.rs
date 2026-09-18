pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// General stats about different groups of an A/B Split campaign. Does not return information about Multivariate Campaigns.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CampaignReportAbSplit {
    /// Stats for Campaign A.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a: Option<CampaignReportAbSplitA>,
    /// Stats for Campaign B.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b: Option<CampaignReportAbSplitB>,
}

impl CampaignReportAbSplit {
    pub fn builder() -> CampaignReportAbSplitBuilder {
        <CampaignReportAbSplitBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignReportAbSplitBuilder {
    a: Option<CampaignReportAbSplitA>,
    b: Option<CampaignReportAbSplitB>,
}

impl CampaignReportAbSplitBuilder {
    pub fn a(mut self, value: CampaignReportAbSplitA) -> Self {
        self.a = Some(value);
        self
    }

    pub fn b(mut self, value: CampaignReportAbSplitB) -> Self {
        self.b = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignReportAbSplit`].
    pub fn build(self) -> Result<CampaignReportAbSplit, BuildError> {
        Ok(CampaignReportAbSplit {
            a: self.a,
            b: self.b,
        })
    }
}
