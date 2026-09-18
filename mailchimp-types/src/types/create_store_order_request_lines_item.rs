pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Information about a specific order line.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateStoreOrderRequestLinesItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<CreateStoreOrderRequestLinesItemDiscount>,
    /// A unique identifier for the order line item.
    #[serde(default)]
    pub id: String,
    pub price: CreateStoreOrderRequestLinesItemPrice,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<EcommerceStoresOrdersPost>,
    /// A unique identifier for the product associated with the order line item.
    #[serde(default)]
    pub product_id: String,
    /// A unique identifier for the product variant associated with the order line item.
    #[serde(default)]
    pub product_variant_id: String,
    /// The quantity of an order line item.
    #[serde(default)]
    pub quantity: i64,
}

impl CreateStoreOrderRequestLinesItem {
    pub fn builder() -> CreateStoreOrderRequestLinesItemBuilder {
        <CreateStoreOrderRequestLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateStoreOrderRequestLinesItemBuilder {
    discount: Option<CreateStoreOrderRequestLinesItemDiscount>,
    id: Option<String>,
    price: Option<CreateStoreOrderRequestLinesItemPrice>,
    product: Option<EcommerceStoresOrdersPost>,
    product_id: Option<String>,
    product_variant_id: Option<String>,
    quantity: Option<i64>,
}

impl CreateStoreOrderRequestLinesItemBuilder {
    pub fn discount(mut self, value: CreateStoreOrderRequestLinesItemDiscount) -> Self {
        self.discount = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn price(mut self, value: CreateStoreOrderRequestLinesItemPrice) -> Self {
        self.price = Some(value);
        self
    }

    pub fn product(mut self, value: EcommerceStoresOrdersPost) -> Self {
        self.product = Some(value);
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

    /// Consumes the builder and constructs a [`CreateStoreOrderRequestLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateStoreOrderRequestLinesItemBuilder::id)
    /// - [`price`](CreateStoreOrderRequestLinesItemBuilder::price)
    /// - [`product_id`](CreateStoreOrderRequestLinesItemBuilder::product_id)
    /// - [`product_variant_id`](CreateStoreOrderRequestLinesItemBuilder::product_variant_id)
    /// - [`quantity`](CreateStoreOrderRequestLinesItemBuilder::quantity)
    pub fn build(self) -> Result<CreateStoreOrderRequestLinesItem, BuildError> {
        Ok(CreateStoreOrderRequestLinesItem {
            discount: self.discount,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            price: self.price.ok_or_else(|| BuildError::missing_field("price"))?,
            product: self.product,
            product_id: self.product_id.ok_or_else(|| BuildError::missing_field("product_id"))?,
            product_variant_id: self.product_variant_id.ok_or_else(|| BuildError::missing_field("product_variant_id"))?,
            quantity: self.quantity.ok_or_else(|| BuildError::missing_field("quantity"))?,
        })
    }
}
