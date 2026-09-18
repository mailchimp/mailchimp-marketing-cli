pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateStoreProductImageRequest {
    /// A unique identifier for the product image.
    #[serde(default)]
    pub id: String,
    /// The URL for a product image.
    #[serde(default)]
    pub url: String,
    /// The list of product variants using the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_ids: Option<Vec<CreateStoreProductImageRequestVariantIdsItem>>,
}

impl CreateStoreProductImageRequest {
    pub fn builder() -> CreateStoreProductImageRequestBuilder {
        <CreateStoreProductImageRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateStoreProductImageRequestBuilder {
    id: Option<String>,
    url: Option<String>,
    variant_ids: Option<Vec<CreateStoreProductImageRequestVariantIdsItem>>,
}

impl CreateStoreProductImageRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn variant_ids(mut self, value: Vec<CreateStoreProductImageRequestVariantIdsItem>) -> Self {
        self.variant_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateStoreProductImageRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateStoreProductImageRequestBuilder::id)
    /// - [`url`](CreateStoreProductImageRequestBuilder::url)
    pub fn build(self) -> Result<CreateStoreProductImageRequest, BuildError> {
        Ok(CreateStoreProductImageRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            variant_ids: self.variant_ids,
        })
    }
}

