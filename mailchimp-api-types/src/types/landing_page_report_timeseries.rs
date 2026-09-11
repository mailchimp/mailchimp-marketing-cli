pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LandingPageReportTimeseries {
    /// The clicks and visits data from the last seven days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_stats: Option<LandingPageReportTimeseriesDailyStats>,
    /// The clicks and visits data from the last five weeks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_stats: Option<LandingPageReportTimeseriesWeeklyStats>,
}

impl LandingPageReportTimeseries {
    pub fn builder() -> LandingPageReportTimeseriesBuilder {
        <LandingPageReportTimeseriesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LandingPageReportTimeseriesBuilder {
    daily_stats: Option<LandingPageReportTimeseriesDailyStats>,
    weekly_stats: Option<LandingPageReportTimeseriesWeeklyStats>,
}

impl LandingPageReportTimeseriesBuilder {
    pub fn daily_stats(mut self, value: LandingPageReportTimeseriesDailyStats) -> Self {
        self.daily_stats = Some(value);
        self
    }

    pub fn weekly_stats(mut self, value: LandingPageReportTimeseriesWeeklyStats) -> Self {
        self.weekly_stats = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LandingPageReportTimeseries`].
    pub fn build(self) -> Result<LandingPageReportTimeseries, BuildError> {
        Ok(LandingPageReportTimeseries {
            daily_stats: self.daily_stats,
            weekly_stats: self.weekly_stats,
        })
    }
}
