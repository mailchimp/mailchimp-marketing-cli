pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateStoreCartLineRequest {
    /// A unique identifier for the cart line item.
    #[serde(default)]
    pub id: String,
    pub price: CreateStoreCartLineRequestPrice,
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

impl CreateStoreCartLineRequest {
    pub fn builder() -> CreateStoreCartLineRequestBuilder {
        <CreateStoreCartLineRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateStoreCartLineRequestBuilder {
    id: Option<String>,
    price: Option<CreateStoreCartLineRequestPrice>,
    product_id: Option<String>,
    product_variant_id: Option<String>,
    quantity: Option<i64>,
}

impl CreateStoreCartLineRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn price(mut self, value: CreateStoreCartLineRequestPrice) -> Self {
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

    /// Consumes the builder and constructs a [`CreateStoreCartLineRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateStoreCartLineRequestBuilder::id)
    /// - [`price`](CreateStoreCartLineRequestBuilder::price)
    /// - [`product_id`](CreateStoreCartLineRequestBuilder::product_id)
    /// - [`product_variant_id`](CreateStoreCartLineRequestBuilder::product_variant_id)
    /// - [`quantity`](CreateStoreCartLineRequestBuilder::quantity)
    pub fn build(self) -> Result<CreateStoreCartLineRequest, BuildError> {
        Ok(CreateStoreCartLineRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            price: self.price.ok_or_else(|| BuildError::missing_field("price"))?,
            product_id: self.product_id.ok_or_else(|| BuildError::missing_field("product_id"))?,
            product_variant_id: self.product_variant_id.ok_or_else(|| BuildError::missing_field("product_variant_id"))?,
            quantity: self.quantity.ok_or_else(|| BuildError::missing_field("quantity"))?,
        })
    }
}

