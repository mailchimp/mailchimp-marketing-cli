pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Information about a specific cart line item.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateStoreCartRequestLinesItem {
    /// A unique identifier for the cart line item.
    #[serde(default)]
    pub id: String,
    pub price: CreateStoreCartRequestLinesItemPrice,
    /// A unique identifier for the product associated with the cart line item.
    #[serde(default)]
    pub product_id: String,
    /// A unique identifier for the product variant associated with the cart line item.
    #[serde(default)]
    pub product_variant_id: String,
    /// The quantity of a cart line item.
    #[serde(default)]
    pub quantity: i64,
}

impl CreateStoreCartRequestLinesItem {
    pub fn builder() -> CreateStoreCartRequestLinesItemBuilder {
        <CreateStoreCartRequestLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateStoreCartRequestLinesItemBuilder {
    id: Option<String>,
    price: Option<CreateStoreCartRequestLinesItemPrice>,
    product_id: Option<String>,
    product_variant_id: Option<String>,
    quantity: Option<i64>,
}

impl CreateStoreCartRequestLinesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn price(mut self, value: CreateStoreCartRequestLinesItemPrice) -> Self {
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

    /// Consumes the builder and constructs a [`CreateStoreCartRequestLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateStoreCartRequestLinesItemBuilder::id)
    /// - [`price`](CreateStoreCartRequestLinesItemBuilder::price)
    /// - [`product_id`](CreateStoreCartRequestLinesItemBuilder::product_id)
    /// - [`product_variant_id`](CreateStoreCartRequestLinesItemBuilder::product_variant_id)
    /// - [`quantity`](CreateStoreCartRequestLinesItemBuilder::quantity)
    pub fn build(self) -> Result<CreateStoreCartRequestLinesItem, BuildError> {
        Ok(CreateStoreCartRequestLinesItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            price: self.price.ok_or_else(|| BuildError::missing_field("price"))?,
            product_id: self.product_id.ok_or_else(|| BuildError::missing_field("product_id"))?,
            product_variant_id: self.product_variant_id.ok_or_else(|| BuildError::missing_field("product_variant_id"))?,
            quantity: self.quantity.ok_or_else(|| BuildError::missing_field("quantity"))?,
        })
    }
}
