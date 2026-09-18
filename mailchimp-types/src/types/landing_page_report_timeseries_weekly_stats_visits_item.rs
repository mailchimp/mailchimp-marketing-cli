pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LandingPageReportTimeseriesWeeklyStatsVisitsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub val: Option<i64>,
}

impl LandingPageReportTimeseriesWeeklyStatsVisitsItem {
    pub fn builder() -> LandingPageReportTimeseriesWeeklyStatsVisitsItemBuilder {
        <LandingPageReportTimeseriesWeeklyStatsVisitsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LandingPageReportTimeseriesWeeklyStatsVisitsItemBuilder {
    date: Option<String>,
    val: Option<i64>,
}

impl LandingPageReportTimeseriesWeeklyStatsVisitsItemBuilder {
    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn val(mut self, value: i64) -> Self {
        self.val = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LandingPageReportTimeseriesWeeklyStatsVisitsItem`].
    pub fn build(self) -> Result<LandingPageReportTimeseriesWeeklyStatsVisitsItem, BuildError> {
        Ok(LandingPageReportTimeseriesWeeklyStatsVisitsItem {
            date: self.date,
            val: self.val,
        })
    }
}
