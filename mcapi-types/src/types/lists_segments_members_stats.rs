pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Open and click rates for this subscriber.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListsSegmentsMembersStats {
    /// A subscriber's average clickthrough rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub avg_click_rate: Option<f64>,
    /// A subscriber's average open rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub avg_open_rate: Option<f64>,
}

impl ListsSegmentsMembersStats {
    pub fn builder() -> ListsSegmentsMembersStatsBuilder {
        <ListsSegmentsMembersStatsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListsSegmentsMembersStatsBuilder {
    avg_click_rate: Option<f64>,
    avg_open_rate: Option<f64>,
}

impl ListsSegmentsMembersStatsBuilder {
    pub fn avg_click_rate(mut self, value: f64) -> Self {
        self.avg_click_rate = Some(value);
        self
    }

    pub fn avg_open_rate(mut self, value: f64) -> Self {
        self.avg_open_rate = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListsSegmentsMembersStats`].
    pub fn build(self) -> Result<ListsSegmentsMembersStats, BuildError> {
        Ok(ListsSegmentsMembersStats {
            avg_click_rate: self.avg_click_rate,
            avg_open_rate: self.avg_open_rate,
        })
    }
}
