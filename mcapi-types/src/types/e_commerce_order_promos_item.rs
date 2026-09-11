pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ECommerceOrderPromosItem {
    /// The amount of discount applied on the total price. For example if the total cost was $100 and the customer paid $95.5, amount_discounted will be 4.5 For free shipping set amount_discounted to 0
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub amount_discounted: Option<f64>,
    /// The Promo Code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Type of discount. For free shipping set type to fixed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ECommerceOrderPromosItemType>,
}

impl ECommerceOrderPromosItem {
    pub fn builder() -> ECommerceOrderPromosItemBuilder {
        <ECommerceOrderPromosItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ECommerceOrderPromosItemBuilder {
    amount_discounted: Option<f64>,
    code: Option<String>,
    r#type: Option<ECommerceOrderPromosItemType>,
}

impl ECommerceOrderPromosItemBuilder {
    pub fn amount_discounted(mut self, value: f64) -> Self {
        self.amount_discounted = Some(value);
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: ECommerceOrderPromosItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ECommerceOrderPromosItem`].
    pub fn build(self) -> Result<ECommerceOrderPromosItem, BuildError> {
        Ok(ECommerceOrderPromosItem {
            amount_discounted: self.amount_discounted,
            code: self.code,
            r#type: self.r#type,
        })
    }
}
