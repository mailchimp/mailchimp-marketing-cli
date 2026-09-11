pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListFacebookAdEcommerceProductActivityReportingResponseProductsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_purchased: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_total: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total_purchased: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total_revenue: Option<f64>,
}

impl ListFacebookAdEcommerceProductActivityReportingResponseProductsItem {
    pub fn builder() -> ListFacebookAdEcommerceProductActivityReportingResponseProductsItemBuilder {
        <ListFacebookAdEcommerceProductActivityReportingResponseProductsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListFacebookAdEcommerceProductActivityReportingResponseProductsItemBuilder {
    currency_code: Option<String>,
    image_url: Option<String>,
    recommendation_purchased: Option<i64>,
    recommendation_total: Option<i64>,
    sku: Option<String>,
    title: Option<String>,
    total_purchased: Option<f64>,
    total_revenue: Option<f64>,
}

impl ListFacebookAdEcommerceProductActivityReportingResponseProductsItemBuilder {
    pub fn currency_code(mut self, value: impl Into<String>) -> Self {
        self.currency_code = Some(value.into());
        self
    }

    pub fn image_url(mut self, value: impl Into<String>) -> Self {
        self.image_url = Some(value.into());
        self
    }

    pub fn recommendation_purchased(mut self, value: i64) -> Self {
        self.recommendation_purchased = Some(value);
        self
    }

    pub fn recommendation_total(mut self, value: i64) -> Self {
        self.recommendation_total = Some(value);
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

    pub fn total_purchased(mut self, value: f64) -> Self {
        self.total_purchased = Some(value);
        self
    }

    pub fn total_revenue(mut self, value: f64) -> Self {
        self.total_revenue = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListFacebookAdEcommerceProductActivityReportingResponseProductsItem`].
    pub fn build(self) -> Result<ListFacebookAdEcommerceProductActivityReportingResponseProductsItem, BuildError> {
        Ok(ListFacebookAdEcommerceProductActivityReportingResponseProductsItem {
            currency_code: self.currency_code,
            image_url: self.image_url,
            recommendation_purchased: self.recommendation_purchased,
            recommendation_total: self.recommendation_total,
            sku: self.sku,
            title: self.title,
            total_purchased: self.total_purchased,
            total_revenue: self.total_revenue,
        })
    }
}
