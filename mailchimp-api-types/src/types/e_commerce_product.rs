pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Information about a specific product.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ECommerceProduct {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ECommerceProductLinksItem>>,
    /// The currency code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// The description of a product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The handle of a product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    /// A unique identifier for the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The image URL for a product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// An array of the product's images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<ECommerceProductImagesItem>>,
    /// The date and time the product was published in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub published_at_foreign: Option<DateTime<FixedOffset>>,
    /// The title of a product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The type of product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The URL for a product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Returns up to 50 of the product's variants. To retrieve all variants use [Product Variants](https://mailchimp.com/developer/marketing/api/ecommerce-product-variants/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<ECommerceProductVariant>>,
    /// The vendor for a product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
}

impl ECommerceProduct {
    pub fn builder() -> ECommerceProductBuilder {
        <ECommerceProductBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ECommerceProductBuilder {
    links: Option<Vec<ECommerceProductLinksItem>>,
    currency_code: Option<String>,
    description: Option<String>,
    handle: Option<String>,
    id: Option<String>,
    image_url: Option<String>,
    images: Option<Vec<ECommerceProductImagesItem>>,
    published_at_foreign: Option<DateTime<FixedOffset>>,
    title: Option<String>,
    r#type: Option<String>,
    url: Option<String>,
    variants: Option<Vec<ECommerceProductVariant>>,
    vendor: Option<String>,
}

impl ECommerceProductBuilder {
    pub fn links(mut self, value: Vec<ECommerceProductLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn currency_code(mut self, value: impl Into<String>) -> Self {
        self.currency_code = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn handle(mut self, value: impl Into<String>) -> Self {
        self.handle = Some(value.into());
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

    pub fn images(mut self, value: Vec<ECommerceProductImagesItem>) -> Self {
        self.images = Some(value);
        self
    }

    pub fn published_at_foreign(mut self, value: DateTime<FixedOffset>) -> Self {
        self.published_at_foreign = Some(value);
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

    pub fn variants(mut self, value: Vec<ECommerceProductVariant>) -> Self {
        self.variants = Some(value);
        self
    }

    pub fn vendor(mut self, value: impl Into<String>) -> Self {
        self.vendor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ECommerceProduct`].
    pub fn build(self) -> Result<ECommerceProduct, BuildError> {
        Ok(ECommerceProduct {
            links: self.links,
            currency_code: self.currency_code,
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
