pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateStoreCartLineEcommerceRequest {
    /// A unique identifier for the cart line item.
    #[serde(default)]
    pub id: String,
    pub price: CreateStoreCartLineEcommerceRequestPrice,
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

impl CreateStoreCartLineEcommerceRequest {
    pub fn builder() -> CreateStoreCartLineEcommerceRequestBuilder {
        <CreateStoreCartLineEcommerceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateStoreCartLineEcommerceRequestBuilder {
    id: Option<String>,
    price: Option<CreateStoreCartLineEcommerceRequestPrice>,
    product_id: Option<String>,
    product_variant_id: Option<String>,
    quantity: Option<i64>,
}

impl CreateStoreCartLineEcommerceRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn price(mut self, value: CreateStoreCartLineEcommerceRequestPrice) -> Self {
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

    /// Consumes the builder and constructs a [`CreateStoreCartLineEcommerceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateStoreCartLineEcommerceRequestBuilder::id)
    /// - [`price`](CreateStoreCartLineEcommerceRequestBuilder::price)
    /// - [`product_id`](CreateStoreCartLineEcommerceRequestBuilder::product_id)
    /// - [`product_variant_id`](CreateStoreCartLineEcommerceRequestBuilder::product_variant_id)
    /// - [`quantity`](CreateStoreCartLineEcommerceRequestBuilder::quantity)
    pub fn build(self) -> Result<CreateStoreCartLineEcommerceRequest, BuildError> {
        Ok(CreateStoreCartLineEcommerceRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            price: self.price.ok_or_else(|| BuildError::missing_field("price"))?,
            product_id: self.product_id.ok_or_else(|| BuildError::missing_field("product_id"))?,
            product_variant_id: self.product_variant_id.ok_or_else(|| BuildError::missing_field("product_variant_id"))?,
            quantity: self.quantity.ok_or_else(|| BuildError::missing_field("quantity"))?,
        })
    }
}

