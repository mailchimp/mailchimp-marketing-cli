pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Open and click rates for this subscriber.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListMembersStats {
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
    /// Ecommerce stats for the list member if the list is attached to a store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecommerce_data: Option<ListMembersStatsEcommerceData>,
}

impl ListMembersStats {
    pub fn builder() -> ListMembersStatsBuilder {
        <ListMembersStatsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMembersStatsBuilder {
    avg_click_rate: Option<f64>,
    avg_open_rate: Option<f64>,
    ecommerce_data: Option<ListMembersStatsEcommerceData>,
}

impl ListMembersStatsBuilder {
    pub fn avg_click_rate(mut self, value: f64) -> Self {
        self.avg_click_rate = Some(value);
        self
    }

    pub fn avg_open_rate(mut self, value: f64) -> Self {
        self.avg_open_rate = Some(value);
        self
    }

    pub fn ecommerce_data(mut self, value: ListMembersStatsEcommerceData) -> Self {
        self.ecommerce_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListMembersStats`].
    pub fn build(self) -> Result<ListMembersStats, BuildError> {
        Ok(ListMembersStats {
            avg_click_rate: self.avg_click_rate,
            avg_open_rate: self.avg_open_rate,
            ecommerce_data: self.ecommerce_data,
        })
    }
}
