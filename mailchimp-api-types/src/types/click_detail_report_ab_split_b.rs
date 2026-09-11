pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Stats for Group B.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ClickDetailReportAbSplitB {
    /// The percentage of total clicks for Group B.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub click_percentage_b: Option<f64>,
    /// The total number of clicks for Group B.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_clicks_b: Option<i64>,
    /// The percentage of unique clicks for Group B.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub unique_click_percentage_b: Option<f64>,
    /// The number of unique clicks for Group B.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_clicks_b: Option<i64>,
}

impl ClickDetailReportAbSplitB {
    pub fn builder() -> ClickDetailReportAbSplitBBuilder {
        <ClickDetailReportAbSplitBBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClickDetailReportAbSplitBBuilder {
    click_percentage_b: Option<f64>,
    total_clicks_b: Option<i64>,
    unique_click_percentage_b: Option<f64>,
    unique_clicks_b: Option<i64>,
}

impl ClickDetailReportAbSplitBBuilder {
    pub fn click_percentage_b(mut self, value: f64) -> Self {
        self.click_percentage_b = Some(value);
        self
    }

    pub fn total_clicks_b(mut self, value: i64) -> Self {
        self.total_clicks_b = Some(value);
        self
    }

    pub fn unique_click_percentage_b(mut self, value: f64) -> Self {
        self.unique_click_percentage_b = Some(value);
        self
    }

    pub fn unique_clicks_b(mut self, value: i64) -> Self {
        self.unique_clicks_b = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ClickDetailReportAbSplitB`].
    pub fn build(self) -> Result<ClickDetailReportAbSplitB, BuildError> {
        Ok(ClickDetailReportAbSplitB {
            click_percentage_b: self.click_percentage_b,
            total_clicks_b: self.total_clicks_b,
            unique_click_percentage_b: self.unique_click_percentage_b,
            unique_clicks_b: self.unique_clicks_b,
        })
    }
}
