pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateStoreCartLineEcommerceRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<UpdateStoreCartLineEcommerceRequestPrice>,
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

impl UpdateStoreCartLineEcommerceRequest {
    pub fn builder() -> UpdateStoreCartLineEcommerceRequestBuilder {
        <UpdateStoreCartLineEcommerceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateStoreCartLineEcommerceRequestBuilder {
    price: Option<UpdateStoreCartLineEcommerceRequestPrice>,
    product_id: Option<String>,
    product_variant_id: Option<String>,
    quantity: Option<i64>,
}

impl UpdateStoreCartLineEcommerceRequestBuilder {
    pub fn price(mut self, value: UpdateStoreCartLineEcommerceRequestPrice) -> Self {
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

    /// Consumes the builder and constructs a [`UpdateStoreCartLineEcommerceRequest`].
    pub fn build(self) -> Result<UpdateStoreCartLineEcommerceRequest, BuildError> {
        Ok(UpdateStoreCartLineEcommerceRequest {
            price: self.price,
            product_id: self.product_id,
            product_variant_id: self.product_variant_id,
            quantity: self.quantity,
        })
    }
}

