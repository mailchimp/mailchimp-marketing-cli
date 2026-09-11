pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Information about a specific product variant.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ECommerceProductVariant {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ECommerceProductVariantLinksItem>>,
    /// The backorders of a product variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backorders: Option<String>,
    /// The date and time the product was created in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// A unique identifier for the product variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The image URL for a product variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// The inventory quantity of a product variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_quantity: Option<i64>,
    /// The price of a product variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub price: Option<f64>,
    /// The stock keeping unit (SKU) of a product variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    /// The title of a product variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The date and time the product was last updated in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
    /// The URL for a product variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The visibility of a product variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

impl ECommerceProductVariant {
    pub fn builder() -> ECommerceProductVariantBuilder {
        <ECommerceProductVariantBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ECommerceProductVariantBuilder {
    links: Option<Vec<ECommerceProductVariantLinksItem>>,
    backorders: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    image_url: Option<String>,
    inventory_quantity: Option<i64>,
    price: Option<f64>,
    sku: Option<String>,
    title: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
    url: Option<String>,
    visibility: Option<String>,
}

impl ECommerceProductVariantBuilder {
    pub fn links(mut self, value: Vec<ECommerceProductVariantLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn backorders(mut self, value: impl Into<String>) -> Self {
        self.backorders = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
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

    pub fn inventory_quantity(mut self, value: i64) -> Self {
        self.inventory_quantity = Some(value);
        self
    }

    pub fn price(mut self, value: f64) -> Self {
        self.price = Some(value);
        self
    }

    pub fn sku(mut self, value: impl Into<String>) -> Self {
        self.sku = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn visibility(mut self, value: impl Into<String>) -> Self {
        self.visibility = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ECommerceProductVariant`].
    pub fn build(self) -> Result<ECommerceProductVariant, BuildError> {
        Ok(ECommerceProductVariant {
            links: self.links,
            backorders: self.backorders,
            created_at: self.created_at,
            id: self.id,
            image_url: self.image_url,
            inventory_quantity: self.inventory_quantity,
            price: self.price,
            sku: self.sku,
            title: self.title,
            updated_at: self.updated_at,
            url: self.url,
            visibility: self.visibility,
        })
    }
}
