pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Stats for Group A.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ClickDetailReportAbSplitA {
    /// The percentage of total clicks for Group A.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub click_percentage_a: Option<f64>,
    /// The total number of clicks for Group A.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_clicks_a: Option<i64>,
    /// The percentage of unique clicks for Group A.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub unique_click_percentage_a: Option<f64>,
    /// The number of unique clicks for Group A.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_clicks_a: Option<i64>,
}

impl ClickDetailReportAbSplitA {
    pub fn builder() -> ClickDetailReportAbSplitABuilder {
        <ClickDetailReportAbSplitABuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClickDetailReportAbSplitABuilder {
    click_percentage_a: Option<f64>,
    total_clicks_a: Option<i64>,
    unique_click_percentage_a: Option<f64>,
    unique_clicks_a: Option<i64>,
}

impl ClickDetailReportAbSplitABuilder {
    pub fn click_percentage_a(mut self, value: f64) -> Self {
        self.click_percentage_a = Some(value);
        self
    }

    pub fn total_clicks_a(mut self, value: i64) -> Self {
        self.total_clicks_a = Some(value);
        self
    }

    pub fn unique_click_percentage_a(mut self, value: f64) -> Self {
        self.unique_click_percentage_a = Some(value);
        self
    }

    pub fn unique_clicks_a(mut self, value: i64) -> Self {
        self.unique_clicks_a = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ClickDetailReportAbSplitA`].
    pub fn build(self) -> Result<ClickDetailReportAbSplitA, BuildError> {
        Ok(ClickDetailReportAbSplitA {
            click_percentage_a: self.click_percentage_a,
            total_clicks_a: self.total_clicks_a,
            unique_click_percentage_a: self.unique_click_percentage_a,
            unique_clicks_a: self.unique_clicks_a,
        })
    }
}
