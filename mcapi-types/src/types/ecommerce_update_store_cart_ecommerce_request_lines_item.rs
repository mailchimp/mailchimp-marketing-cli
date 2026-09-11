pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Information about a specific cart line item.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateStoreCartEcommerceRequestLinesItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<UpdateStoreCartEcommerceRequestLinesItemPrice>,
    /// A unique identifier for the product associated with the cart line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// A unique identifier for the product variant associated with the cart line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_variant_id: Option<String>,
    /// The quantity of a cart line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
}

impl UpdateStoreCartEcommerceRequestLinesItem {
    pub fn builder() -> UpdateStoreCartEcommerceRequestLinesItemBuilder {
        <UpdateStoreCartEcommerceRequestLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateStoreCartEcommerceRequestLinesItemBuilder {
    price: Option<UpdateStoreCartEcommerceRequestLinesItemPrice>,
    product_id: Option<String>,
    product_variant_id: Option<String>,
    quantity: Option<i64>,
}

impl UpdateStoreCartEcommerceRequestLinesItemBuilder {
    pub fn price(mut self, value: UpdateStoreCartEcommerceRequestLinesItemPrice) -> Self {
        self.price = Some(value);
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn product_variant_id(mut self, value: impl Into<String>) -> Self {
        self.product_variant_id = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: i64) -> Self {
        self.quantity = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateStoreCartEcommerceRequestLinesItem`].
    pub fn build(self) -> Result<UpdateStoreCartEcommerceRequestLinesItem, BuildError> {
        Ok(UpdateStoreCartEcommerceRequestLinesItem {
            price: self.price,
            product_id: self.product_id,
            product_variant_id: self.product_variant_id,
            quantity: self.quantity,
        })
    }
}
