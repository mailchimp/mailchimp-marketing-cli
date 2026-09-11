pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateStoreProductVariantEcommerceRequest {
    /// The backorders of a product variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backorders: Option<String>,
    /// The image URL for a product variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// The inventory quantity of a product variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_quantity: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<UpdateStoreProductVariantEcommerceRequestPrice>,
    /// The stock keeping unit (SKU) of a product variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    /// The title of a product variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The URL for a product variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The visibility of a product variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

impl UpdateStoreProductVariantEcommerceRequest {
    pub fn builder() -> UpdateStoreProductVariantEcommerceRequestBuilder {
        <UpdateStoreProductVariantEcommerceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateStoreProductVariantEcommerceRequestBuilder {
    backorders: Option<String>,
    image_url: Option<String>,
    inventory_quantity: Option<i64>,
    price: Option<UpdateStoreProductVariantEcommerceRequestPrice>,
    sku: Option<String>,
    title: Option<String>,
    url: Option<String>,
    visibility: Option<String>,
}

impl UpdateStoreProductVariantEcommerceRequestBuilder {
    pub fn backorders(mut self, value: impl Into<String>) -> Self {
        self.backorders = Some(value.into());
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

    pub fn price(mut self, value: UpdateStoreProductVariantEcommerceRequestPrice) -> Self {
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

    /// Consumes the builder and constructs a [`UpdateStoreProductVariantEcommerceRequest`].
    pub fn build(self) -> Result<UpdateStoreProductVariantEcommerceRequest, BuildError> {
        Ok(UpdateStoreProductVariantEcommerceRequest {
            backorders: self.backorders,
            image_url: self.image_url,
            inventory_quantity: self.inventory_quantity,
            price: self.price,
            sku: self.sku,
            title: self.title,
            url: self.url,
            visibility: self.visibility,
        })
    }
}

