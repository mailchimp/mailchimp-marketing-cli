pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LandingPageReportTimeseriesDailyStatsVisitsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub val: Option<i64>,
}

impl LandingPageReportTimeseriesDailyStatsVisitsItem {
    pub fn builder() -> LandingPageReportTimeseriesDailyStatsVisitsItemBuilder {
        <LandingPageReportTimeseriesDailyStatsVisitsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LandingPageReportTimeseriesDailyStatsVisitsItemBuilder {
    date: Option<String>,
    val: Option<i64>,
}

impl LandingPageReportTimeseriesDailyStatsVisitsItemBuilder {
    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn val(mut self, value: i64) -> Self {
        self.val = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LandingPageReportTimeseriesDailyStatsVisitsItem`].
    pub fn build(self) -> Result<LandingPageReportTimeseriesDailyStatsVisitsItem, BuildError> {
        Ok(LandingPageReportTimeseriesDailyStatsVisitsItem {
            date: self.date,
            val: self.val,
        })
    }
}
