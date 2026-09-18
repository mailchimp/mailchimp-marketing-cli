pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateStoreProductRequest {
    /// The description of a product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The handle of a product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    /// A unique identifier for the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<UpdateStoreProductRequestId>,
    /// The image URL for a product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// An array of the product's images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<UpdateStoreProductRequestImagesItem>>,
    /// The date and time the product was published in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at_foreign: Option<String>,
    /// The title of a product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The type of product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The URL for a product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// An array of the product's variants. At least one variant is required for each product. A variant can use the same `id` and `title` as the parent product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<UpdateStoreProductRequestVariantsItem>>,
    /// The vendor for a product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
}

impl UpdateStoreProductRequest {
    pub fn builder() -> UpdateStoreProductRequestBuilder {
        <UpdateStoreProductRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateStoreProductRequestBuilder {
    description: Option<String>,
    handle: Option<String>,
    id: Option<UpdateStoreProductRequestId>,
    image_url: Option<String>,
    images: Option<Vec<UpdateStoreProductRequestImagesItem>>,
    published_at_foreign: Option<String>,
    title: Option<String>,
    r#type: Option<String>,
    url: Option<String>,
    variants: Option<Vec<UpdateStoreProductRequestVariantsItem>>,
    vendor: Option<String>,
}

impl UpdateStoreProductRequestBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn handle(mut self, value: impl Into<String>) -> Self {
        self.handle = Some(value.into());
        self
    }

    pub fn id(mut self, value: UpdateStoreProductRequestId) -> Self {
        self.id = Some(value);
        self
    }

    pub fn image_url(mut self, value: impl Into<String>) -> Self {
        self.image_url = Some(value.into());
        self
    }

    pub fn images(mut self, value: Vec<UpdateStoreProductRequestImagesItem>) -> Self {
        self.images = Some(value);
        self
    }

    pub fn published_at_foreign(mut self, value: impl Into<String>) -> Self {
        self.published_at_foreign = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn variants(mut self, value: Vec<UpdateStoreProductRequestVariantsItem>) -> Self {
        self.variants = Some(value);
        self
    }

    pub fn vendor(mut self, value: impl Into<String>) -> Self {
        self.vendor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateStoreProductRequest`].
    pub fn build(self) -> Result<UpdateStoreProductRequest, BuildError> {
        Ok(UpdateStoreProductRequest {
            description: self.description,
            handle: self.handle,
            id: self.id,
            image_url: self.image_url,
            images: self.images,
            published_at_foreign: self.published_at_foreign,
            title: self.title,
            r#type: self.r#type,
            url: self.url,
            variants: self.variants,
            vendor: self.vendor,
        })
    }
}

