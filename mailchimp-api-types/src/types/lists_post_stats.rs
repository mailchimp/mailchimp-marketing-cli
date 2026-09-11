pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Open and click rates for this subscriber.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListsPostStats {
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

impl ListsPostStats {
    pub fn builder() -> ListsPostStatsBuilder {
        <ListsPostStatsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListsPostStatsBuilder {
    avg_click_rate: Option<f64>,
    avg_open_rate: Option<f64>,
}

impl ListsPostStatsBuilder {
    pub fn avg_click_rate(mut self, value: f64) -> Self {
        self.avg_click_rate = Some(value);
        self
    }

    pub fn avg_open_rate(mut self, value: f64) -> Self {
        self.avg_open_rate = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListsPostStats`].
    pub fn build(self) -> Result<ListsPostStats, BuildError> {
        Ok(ListsPostStats {
            avg_click_rate: self.avg_click_rate,
            avg_open_rate: self.avg_open_rate,
        })
    }
}
