pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateStoreProductImageEcommerceRequest {
    /// A unique identifier for the product image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The URL for a product image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The list of product variants using the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_ids: Option<Vec<UpdateStoreProductImageEcommerceRequestVariantIdsItem>>,
}

impl UpdateStoreProductImageEcommerceRequest {
    pub fn builder() -> UpdateStoreProductImageEcommerceRequestBuilder {
        <UpdateStoreProductImageEcommerceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateStoreProductImageEcommerceRequestBuilder {
    id: Option<String>,
    url: Option<String>,
    variant_ids: Option<Vec<UpdateStoreProductImageEcommerceRequestVariantIdsItem>>,
}

impl UpdateStoreProductImageEcommerceRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn variant_ids(mut self, value: Vec<UpdateStoreProductImageEcommerceRequestVariantIdsItem>) -> Self {
        self.variant_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateStoreProductImageEcommerceRequest`].
    pub fn build(self) -> Result<UpdateStoreProductImageEcommerceRequest, BuildError> {
        Ok(UpdateStoreProductImageEcommerceRequest {
            id: self.id,
            url: self.url,
            variant_ids: self.variant_ids,
        })
    }
}

