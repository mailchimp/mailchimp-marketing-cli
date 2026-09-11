pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The clicks and visits data from the last seven days.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LandingPageReportTimeseriesDailyStats {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicks: Option<Vec<LandingPageReportTimeseriesDailyStatsClicksItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_visits: Option<Vec<LandingPageReportTimeseriesDailyStatsUniqueVisitsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visits: Option<Vec<LandingPageReportTimeseriesDailyStatsVisitsItem>>,
}

impl LandingPageReportTimeseriesDailyStats {
    pub fn builder() -> LandingPageReportTimeseriesDailyStatsBuilder {
        <LandingPageReportTimeseriesDailyStatsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LandingPageReportTimeseriesDailyStatsBuilder {
    clicks: Option<Vec<LandingPageReportTimeseriesDailyStatsClicksItem>>,
    unique_visits: Option<Vec<LandingPageReportTimeseriesDailyStatsUniqueVisitsItem>>,
    visits: Option<Vec<LandingPageReportTimeseriesDailyStatsVisitsItem>>,
}

impl LandingPageReportTimeseriesDailyStatsBuilder {
    pub fn clicks(mut self, value: Vec<LandingPageReportTimeseriesDailyStatsClicksItem>) -> Self {
        self.clicks = Some(value);
        self
    }

    pub fn unique_visits(mut self, value: Vec<LandingPageReportTimeseriesDailyStatsUniqueVisitsItem>) -> Self {
        self.unique_visits = Some(value);
        self
    }

    pub fn visits(mut self, value: Vec<LandingPageReportTimeseriesDailyStatsVisitsItem>) -> Self {
        self.visits = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LandingPageReportTimeseriesDailyStats`].
    pub fn build(self) -> Result<LandingPageReportTimeseriesDailyStats, BuildError> {
        Ok(LandingPageReportTimeseriesDailyStats {
            clicks: self.clicks,
            unique_visits: self.unique_visits,
            visits: self.visits,
        })
    }
}
