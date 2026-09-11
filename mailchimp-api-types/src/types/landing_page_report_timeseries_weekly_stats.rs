pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The clicks and visits data from the last five weeks.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LandingPageReportTimeseriesWeeklyStats {
    /// The total number of clicks in a week.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicks: Option<Vec<LandingPageReportTimeseriesWeeklyStatsClicksItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_visits: Option<Vec<LandingPageReportTimeseriesWeeklyStatsUniqueVisitsItem>>,
    /// The total number of visits in a week.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visits: Option<Vec<LandingPageReportTimeseriesWeeklyStatsVisitsItem>>,
}

impl LandingPageReportTimeseriesWeeklyStats {
    pub fn builder() -> LandingPageReportTimeseriesWeeklyStatsBuilder {
        <LandingPageReportTimeseriesWeeklyStatsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LandingPageReportTimeseriesWeeklyStatsBuilder {
    clicks: Option<Vec<LandingPageReportTimeseriesWeeklyStatsClicksItem>>,
    unique_visits: Option<Vec<LandingPageReportTimeseriesWeeklyStatsUniqueVisitsItem>>,
    visits: Option<Vec<LandingPageReportTimeseriesWeeklyStatsVisitsItem>>,
}

impl LandingPageReportTimeseriesWeeklyStatsBuilder {
    pub fn clicks(mut self, value: Vec<LandingPageReportTimeseriesWeeklyStatsClicksItem>) -> Self {
        self.clicks = Some(value);
        self
    }

    pub fn unique_visits(mut self, value: Vec<LandingPageReportTimeseriesWeeklyStatsUniqueVisitsItem>) -> Self {
        self.unique_visits = Some(value);
        self
    }

    pub fn visits(mut self, value: Vec<LandingPageReportTimeseriesWeeklyStatsVisitsItem>) -> Self {
        self.visits = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LandingPageReportTimeseriesWeeklyStats`].
    pub fn build(self) -> Result<LandingPageReportTimeseriesWeeklyStats, BuildError> {
        Ok(LandingPageReportTimeseriesWeeklyStats {
            clicks: self.clicks,
            unique_visits: self.unique_visits,
            visits: self.visits,
        })
    }
}
