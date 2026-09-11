pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateStoreOrderLineEcommerceRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<UpdateStoreOrderLineEcommerceRequestDiscount>,
    /// A unique identifier for the order line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<UpdateStoreOrderLineEcommerceRequestPrice>,
    /// A unique identifier for the product associated with the order line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// A unique identifier for the product variant associated with the order line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_variant_id: Option<String>,
    /// The quantity of an order line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
}

impl UpdateStoreOrderLineEcommerceRequest {
    pub fn builder() -> UpdateStoreOrderLineEcommerceRequestBuilder {
        <UpdateStoreOrderLineEcommerceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateStoreOrderLineEcommerceRequestBuilder {
    discount: Option<UpdateStoreOrderLineEcommerceRequestDiscount>,
    id: Option<String>,
    price: Option<UpdateStoreOrderLineEcommerceRequestPrice>,
    product_id: Option<String>,
    product_variant_id: Option<String>,
    quantity: Option<i64>,
}

impl UpdateStoreOrderLineEcommerceRequestBuilder {
    pub fn discount(mut self, value: UpdateStoreOrderLineEcommerceRequestDiscount) -> Self {
        self.discount = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn price(mut self, value: UpdateStoreOrderLineEcommerceRequestPrice) -> Self {
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

    /// Consumes the builder and constructs a [`UpdateStoreOrderLineEcommerceRequest`].
    pub fn build(self) -> Result<UpdateStoreOrderLineEcommerceRequest, BuildError> {
        Ok(UpdateStoreOrderLineEcommerceRequest {
            discount: self.discount,
            id: self.id,
            price: self.price,
            product_id: self.product_id,
            product_variant_id: self.product_variant_id,
            quantity: self.quantity,
        })
    }
}

