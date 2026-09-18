pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateStoreProductImageRequest {
    /// A unique identifier for the product image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The URL for a product image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The list of product variants using the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_ids: Option<Vec<UpdateStoreProductImageRequestVariantIdsItem>>,
}

impl UpdateStoreProductImageRequest {
    pub fn builder() -> UpdateStoreProductImageRequestBuilder {
        <UpdateStoreProductImageRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateStoreProductImageRequestBuilder {
    id: Option<String>,
    url: Option<String>,
    variant_ids: Option<Vec<UpdateStoreProductImageRequestVariantIdsItem>>,
}

impl UpdateStoreProductImageRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn variant_ids(mut self, value: Vec<UpdateStoreProductImageRequestVariantIdsItem>) -> Self {
        self.variant_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateStoreProductImageRequest`].
    pub fn build(self) -> Result<UpdateStoreProductImageRequest, BuildError> {
        Ok(UpdateStoreProductImageRequest {
            id: self.id,
            url: self.url,
            variant_ids: self.variant_ids,
        })
    }
}

