pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReportingFacebookAdAudienceActivityImpressionsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impressions: Option<i64>,
}

impl ReportingFacebookAdAudienceActivityImpressionsItem {
    pub fn builder() -> ReportingFacebookAdAudienceActivityImpressionsItemBuilder {
        <ReportingFacebookAdAudienceActivityImpressionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportingFacebookAdAudienceActivityImpressionsItemBuilder {
    date: Option<String>,
    impressions: Option<i64>,
}

impl ReportingFacebookAdAudienceActivityImpressionsItemBuilder {
    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn impressions(mut self, value: i64) -> Self {
        self.impressions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReportingFacebookAdAudienceActivityImpressionsItem`].
    pub fn build(self) -> Result<ReportingFacebookAdAudienceActivityImpressionsItem, BuildError> {
        Ok(ReportingFacebookAdAudienceActivityImpressionsItem {
            date: self.date,
            impressions: self.impressions,
        })
    }
}
