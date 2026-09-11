pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Information about a specific order line.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateStoreOrderEcommerceRequestLinesItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<CreateStoreOrderEcommerceRequestLinesItemDiscount>,
    /// A unique identifier for the order line item.
    #[serde(default)]
    pub id: String,
    pub price: CreateStoreOrderEcommerceRequestLinesItemPrice,
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

impl CreateStoreOrderEcommerceRequestLinesItem {
    pub fn builder() -> CreateStoreOrderEcommerceRequestLinesItemBuilder {
        <CreateStoreOrderEcommerceRequestLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateStoreOrderEcommerceRequestLinesItemBuilder {
    discount: Option<CreateStoreOrderEcommerceRequestLinesItemDiscount>,
    id: Option<String>,
    price: Option<CreateStoreOrderEcommerceRequestLinesItemPrice>,
    product: Option<EcommerceStoresOrdersPost>,
    product_id: Option<String>,
    product_variant_id: Option<String>,
    quantity: Option<i64>,
}

impl CreateStoreOrderEcommerceRequestLinesItemBuilder {
    pub fn discount(mut self, value: CreateStoreOrderEcommerceRequestLinesItemDiscount) -> Self {
        self.discount = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn price(mut self, value: CreateStoreOrderEcommerceRequestLinesItemPrice) -> Self {
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

    /// Consumes the builder and constructs a [`CreateStoreOrderEcommerceRequestLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateStoreOrderEcommerceRequestLinesItemBuilder::id)
    /// - [`price`](CreateStoreOrderEcommerceRequestLinesItemBuilder::price)
    /// - [`product_id`](CreateStoreOrderEcommerceRequestLinesItemBuilder::product_id)
    /// - [`product_variant_id`](CreateStoreOrderEcommerceRequestLinesItemBuilder::product_variant_id)
    /// - [`quantity`](CreateStoreOrderEcommerceRequestLinesItemBuilder::quantity)
    pub fn build(self) -> Result<CreateStoreOrderEcommerceRequestLinesItem, BuildError> {
        Ok(CreateStoreOrderEcommerceRequestLinesItem {
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
