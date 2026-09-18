pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Ecommerce stats for the list member if the list is attached to a store.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListMembersStatsEcommerceData {
    /// The three-letter ISO 4217 code for the currency that the store accepts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// The total number of orders placed by the list member.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub number_of_orders: Option<f64>,
    /// The total revenue the list member has brought in.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total_revenue: Option<f64>,
}

impl ListMembersStatsEcommerceData {
    pub fn builder() -> ListMembersStatsEcommerceDataBuilder {
        <ListMembersStatsEcommerceDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMembersStatsEcommerceDataBuilder {
    currency_code: Option<String>,
    number_of_orders: Option<f64>,
    total_revenue: Option<f64>,
}

impl ListMembersStatsEcommerceDataBuilder {
    pub fn currency_code(mut self, value: impl Into<String>) -> Self {
        self.currency_code = Some(value.into());
        self
    }

    pub fn number_of_orders(mut self, value: f64) -> Self {
        self.number_of_orders = Some(value);
        self
    }

    pub fn total_revenue(mut self, value: f64) -> Self {
        self.total_revenue = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListMembersStatsEcommerceData`].
    pub fn build(self) -> Result<ListMembersStatsEcommerceData, BuildError> {
        Ok(ListMembersStatsEcommerceData {
            currency_code: self.currency_code,
            number_of_orders: self.number_of_orders,
            total_revenue: self.total_revenue,
        })
    }
}
