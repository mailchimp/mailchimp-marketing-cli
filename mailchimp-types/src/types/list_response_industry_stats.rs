pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The [average campaign statistics](https://mailchimp.com/resources/research/email-marketing-benchmarks/?utm_source=mc-api&utm_medium=docs&utm_campaign=apidocs) for all campaigns in the account's specified industry.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListResponseIndustryStats {
    /// The average bounce rate for all campaigns in the account's specified industry.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub bounce_rate: Option<f64>,
    /// The average unique click rate for all campaigns in the account's specified industry.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub click_rate: Option<f64>,
    /// The average unique open rate for all campaigns in the account's specified industry.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub open_rate: Option<f64>,
}

impl ListResponseIndustryStats {
    pub fn builder() -> ListResponseIndustryStatsBuilder {
        <ListResponseIndustryStatsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListResponseIndustryStatsBuilder {
    bounce_rate: Option<f64>,
    click_rate: Option<f64>,
    open_rate: Option<f64>,
}

impl ListResponseIndustryStatsBuilder {
    pub fn bounce_rate(mut self, value: f64) -> Self {
        self.bounce_rate = Some(value);
        self
    }

    pub fn click_rate(mut self, value: f64) -> Self {
        self.click_rate = Some(value);
        self
    }

    pub fn open_rate(mut self, value: f64) -> Self {
        self.open_rate = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListResponseIndustryStats`].
    pub fn build(self) -> Result<ListResponseIndustryStats, BuildError> {
        Ok(ListResponseIndustryStats {
            bounce_rate: self.bounce_rate,
            click_rate: self.click_rate,
            open_rate: self.open_rate,
        })
    }
}
