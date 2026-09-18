pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReportingFacebookAdAudienceActivityClicksItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicks: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

impl ReportingFacebookAdAudienceActivityClicksItem {
    pub fn builder() -> ReportingFacebookAdAudienceActivityClicksItemBuilder {
        <ReportingFacebookAdAudienceActivityClicksItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportingFacebookAdAudienceActivityClicksItemBuilder {
    clicks: Option<i64>,
    date: Option<String>,
}

impl ReportingFacebookAdAudienceActivityClicksItemBuilder {
    pub fn clicks(mut self, value: i64) -> Self {
        self.clicks = Some(value);
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReportingFacebookAdAudienceActivityClicksItem`].
    pub fn build(self) -> Result<ReportingFacebookAdAudienceActivityClicksItem, BuildError> {
        Ok(ReportingFacebookAdAudienceActivityClicksItem {
            clicks: self.clicks,
            date: self.date,
        })
    }
}
