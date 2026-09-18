pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Information about a specific order line.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ECommerceOrderLineItem {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ECommerceOrderLineItemLinksItem>>,
    /// The total discount amount applied to a line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub discount: Option<f64>,
    /// A unique identifier for an order line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The image URL for a product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<ECommerceOrderLineItemPrice>,
    /// A unique identifier for the product associated with an order line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// The name of the product for an order line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_title: Option<String>,
    /// A unique identifier for the product variant associated with an order line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_variant_id: Option<String>,
    /// The name of the product variant for an order line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_variant_title: Option<String>,
    /// The order line item quantity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
}

impl ECommerceOrderLineItem {
    pub fn builder() -> ECommerceOrderLineItemBuilder {
        <ECommerceOrderLineItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ECommerceOrderLineItemBuilder {
    links: Option<Vec<ECommerceOrderLineItemLinksItem>>,
    discount: Option<f64>,
    id: Option<String>,
    image_url: Option<String>,
    price: Option<ECommerceOrderLineItemPrice>,
    product_id: Option<String>,
    product_title: Option<String>,
    product_variant_id: Option<String>,
    product_variant_title: Option<String>,
    quantity: Option<i64>,
}

impl ECommerceOrderLineItemBuilder {
    pub fn links(mut self, value: Vec<ECommerceOrderLineItemLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn discount(mut self, value: f64) -> Self {
        self.discount = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn image_url(mut self, value: impl Into<String>) -> Self {
        self.image_url = Some(value.into());
        self
    }

    pub fn price(mut self, value: ECommerceOrderLineItemPrice) -> Self {
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

    /// Consumes the builder and constructs a [`ECommerceOrderLineItem`].
    pub fn build(self) -> Result<ECommerceOrderLineItem, BuildError> {
        Ok(ECommerceOrderLineItem {
            links: self.links,
            discount: self.discount,
            id: self.id,
            image_url: self.image_url,
            price: self.price,
            product_id: self.product_id,
            product_title: self.product_title,
            product_variant_id: self.product_variant_id,
            product_variant_title: self.product_variant_title,
            quantity: self.quantity,
        })
    }
}
