pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LandingPageReportTimeseriesDailyStatsUniqueVisitsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub val: Option<i64>,
}

impl LandingPageReportTimeseriesDailyStatsUniqueVisitsItem {
    pub fn builder() -> LandingPageReportTimeseriesDailyStatsUniqueVisitsItemBuilder {
        <LandingPageReportTimeseriesDailyStatsUniqueVisitsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LandingPageReportTimeseriesDailyStatsUniqueVisitsItemBuilder {
    date: Option<String>,
    val: Option<i64>,
}

impl LandingPageReportTimeseriesDailyStatsUniqueVisitsItemBuilder {
    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn val(mut self, value: i64) -> Self {
        self.val = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LandingPageReportTimeseriesDailyStatsUniqueVisitsItem`].
    pub fn build(self) -> Result<LandingPageReportTimeseriesDailyStatsUniqueVisitsItem, BuildError> {
        Ok(LandingPageReportTimeseriesDailyStatsUniqueVisitsItem {
            date: self.date,
            val: self.val,
        })
    }
}
