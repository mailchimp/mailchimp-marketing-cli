pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LandingPageReportTimeseriesWeeklyStatsClicksItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub val: Option<i64>,
}

impl LandingPageReportTimeseriesWeeklyStatsClicksItem {
    pub fn builder() -> LandingPageReportTimeseriesWeeklyStatsClicksItemBuilder {
        <LandingPageReportTimeseriesWeeklyStatsClicksItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LandingPageReportTimeseriesWeeklyStatsClicksItemBuilder {
    date: Option<String>,
    val: Option<i64>,
}

impl LandingPageReportTimeseriesWeeklyStatsClicksItemBuilder {
    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn val(mut self, value: i64) -> Self {
        self.val = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LandingPageReportTimeseriesWeeklyStatsClicksItem`].
    pub fn build(self) -> Result<LandingPageReportTimeseriesWeeklyStatsClicksItem, BuildError> {
        Ok(LandingPageReportTimeseriesWeeklyStatsClicksItem {
            date: self.date,
            val: self.val,
        })
    }
}
