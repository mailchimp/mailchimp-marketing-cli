pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Information about a specific cart line item.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ECommerceCartLineItem {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ECommerceCartLineItemLinksItem>>,
    /// A unique identifier for the cart line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The price of a cart line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub price: Option<f64>,
    /// A unique identifier for the product associated with the cart line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// The name of the product for the cart line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_title: Option<String>,
    /// A unique identifier for the product variant associated with the cart line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_variant_id: Option<String>,
    /// The name of the product variant for the cart line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_variant_title: Option<String>,
    /// The quantity of a cart line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
}

impl ECommerceCartLineItem {
    pub fn builder() -> ECommerceCartLineItemBuilder {
        <ECommerceCartLineItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ECommerceCartLineItemBuilder {
    links: Option<Vec<ECommerceCartLineItemLinksItem>>,
    id: Option<String>,
    price: Option<f64>,
    product_id: Option<String>,
    product_title: Option<String>,
    product_variant_id: Option<String>,
    product_variant_title: Option<String>,
    quantity: Option<i64>,
}

impl ECommerceCartLineItemBuilder {
    pub fn links(mut self, value: Vec<ECommerceCartLineItemLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn price(mut self, value: f64) -> Self {
        self.price = Some(value);
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn product_title(mut self, value: impl Into<String>) -> Self {
        self.product_title = Some(value.into());
        self
    }

    pub fn product_variant_id(mut self, value: impl Into<String>) -> Self {
        self.product_variant_id = Some(value.into());
        self
    }

    pub fn product_variant_title(mut self, value: impl Into<String>) -> Self {
        self.product_variant_title = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: i64) -> Self {
        self.quantity = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ECommerceCartLineItem`].
    pub fn build(self) -> Result<ECommerceCartLineItem, BuildError> {
        Ok(ECommerceCartLineItem {
            links: self.links,
            id: self.id,
            price: self.price,
            product_id: self.product_id,
            product_title: self.product_title,
            product_variant_id: self.product_variant_id,
            product_variant_title: self.product_variant_title,
            quantity: self.quantity,
        })
    }
}
