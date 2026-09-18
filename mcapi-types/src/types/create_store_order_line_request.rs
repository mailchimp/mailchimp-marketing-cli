pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateStoreOrderLineRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<CreateStoreOrderLineRequestDiscount>,
    /// A unique identifier for the order line item.
    #[serde(default)]
    pub id: String,
    pub price: CreateStoreOrderLineRequestPrice,
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

impl CreateStoreOrderLineRequest {
    pub fn builder() -> CreateStoreOrderLineRequestBuilder {
        <CreateStoreOrderLineRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateStoreOrderLineRequestBuilder {
    discount: Option<CreateStoreOrderLineRequestDiscount>,
    id: Option<String>,
    price: Option<CreateStoreOrderLineRequestPrice>,
    product: Option<EcommerceStoresOrdersPost>,
    product_id: Option<String>,
    product_variant_id: Option<String>,
    quantity: Option<i64>,
}

impl CreateStoreOrderLineRequestBuilder {
    pub fn discount(mut self, value: CreateStoreOrderLineRequestDiscount) -> Self {
        self.discount = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn price(mut self, value: CreateStoreOrderLineRequestPrice) -> Self {
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

    /// Consumes the builder and constructs a [`CreateStoreOrderLineRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateStoreOrderLineRequestBuilder::id)
    /// - [`price`](CreateStoreOrderLineRequestBuilder::price)
    /// - [`product_id`](CreateStoreOrderLineRequestBuilder::product_id)
    /// - [`product_variant_id`](CreateStoreOrderLineRequestBuilder::product_variant_id)
    /// - [`quantity`](CreateStoreOrderLineRequestBuilder::quantity)
    pub fn build(self) -> Result<CreateStoreOrderLineRequest, BuildError> {
        Ok(CreateStoreOrderLineRequest {
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

