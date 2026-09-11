pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateStoreProductVariantEcommerceRequest {
    /// The backorders of a product variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backorders: Option<String>,
    /// A unique identifier for the product variant.
    pub id: CreateStoreProductVariantEcommerceRequestId,
    /// The image URL for a product variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// The inventory quantity of a product variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_quantity: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<CreateStoreProductVariantEcommerceRequestPrice>,
    /// The stock keeping unit (SKU) of a product variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    /// The title of a product variant.
    #[serde(default)]
    pub title: String,
    /// The URL for a product variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The visibility of a product variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

impl CreateStoreProductVariantEcommerceRequest {
    pub fn builder() -> CreateStoreProductVariantEcommerceRequestBuilder {
        <CreateStoreProductVariantEcommerceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateStoreProductVariantEcommerceRequestBuilder {
    backorders: Option<String>,
    id: Option<CreateStoreProductVariantEcommerceRequestId>,
    image_url: Option<String>,
    inventory_quantity: Option<i64>,
    price: Option<CreateStoreProductVariantEcommerceRequestPrice>,
    sku: Option<String>,
    title: Option<String>,
    url: Option<String>,
    visibility: Option<String>,
}

impl CreateStoreProductVariantEcommerceRequestBuilder {
    pub fn backorders(mut self, value: impl Into<String>) -> Self {
        self.backorders = Some(value.into());
        self
    }

    pub fn id(mut self, value: CreateStoreProductVariantEcommerceRequestId) -> Self {
        self.id = Some(value);
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

    pub fn price(mut self, value: CreateStoreProductVariantEcommerceRequestPrice) -> Self {
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

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn visibility(mut self, value: impl Into<String>) -> Self {
        self.visibility = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateStoreProductVariantEcommerceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateStoreProductVariantEcommerceRequestBuilder::id)
    /// - [`title`](CreateStoreProductVariantEcommerceRequestBuilder::title)
    pub fn build(self) -> Result<CreateStoreProductVariantEcommerceRequest, BuildError> {
        Ok(CreateStoreProductVariantEcommerceRequest {
            backorders: self.backorders,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            image_url: self.image_url,
            inventory_quantity: self.inventory_quantity,
            price: self.price,
            sku: self.sku,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            url: self.url,
            visibility: self.visibility,
        })
    }
}

