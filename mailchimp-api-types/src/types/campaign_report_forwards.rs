pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An object describing the forwards and forward activity for the campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CampaignReportForwards {
    /// How many times the campaign has been forwarded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forwards_count: Option<i64>,
    /// How many times the forwarded campaign has been opened.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forwards_opens: Option<i64>,
}

impl CampaignReportForwards {
    pub fn builder() -> CampaignReportForwardsBuilder {
        <CampaignReportForwardsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignReportForwardsBuilder {
    forwards_count: Option<i64>,
    forwards_opens: Option<i64>,
}

impl CampaignReportForwardsBuilder {
    pub fn forwards_count(mut self, value: i64) -> Self {
        self.forwards_count = Some(value);
        self
    }

    pub fn forwards_opens(mut self, value: i64) -> Self {
        self.forwards_opens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignReportForwards`].
    pub fn build(self) -> Result<CampaignReportForwards, BuildError> {
        Ok(CampaignReportForwards {
            forwards_count: self.forwards_count,
            forwards_opens: self.forwards_opens,
        })
    }
}
