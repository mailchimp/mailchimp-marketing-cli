pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An object describing the bounce summary for the campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CampaignReportBounces {
    /// The total number of hard bounced email addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard_bounces: Option<i64>,
    /// The total number of soft bounced email addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soft_bounces: Option<i64>,
    /// The total number of addresses that were syntax-related bounces.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syntax_errors: Option<i64>,
}

impl CampaignReportBounces {
    pub fn builder() -> CampaignReportBouncesBuilder {
        <CampaignReportBouncesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignReportBouncesBuilder {
    hard_bounces: Option<i64>,
    soft_bounces: Option<i64>,
    syntax_errors: Option<i64>,
}

impl CampaignReportBouncesBuilder {
    pub fn hard_bounces(mut self, value: i64) -> Self {
        self.hard_bounces = Some(value);
        self
    }

    pub fn soft_bounces(mut self, value: i64) -> Self {
        self.soft_bounces = Some(value);
        self
    }

    pub fn syntax_errors(mut self, value: i64) -> Self {
        self.syntax_errors = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignReportBounces`].
    pub fn build(self) -> Result<CampaignReportBounces, BuildError> {
        Ok(CampaignReportBounces {
            hard_bounces: self.hard_bounces,
            soft_bounces: self.soft_bounces,
            syntax_errors: self.syntax_errors,
        })
    }
}
