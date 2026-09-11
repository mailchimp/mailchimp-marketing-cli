pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Information about a specific cart.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ECommerceCart {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ECommerceCartLinksItem>>,
    /// A string that uniquely identifies the campaign associated with a cart.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// The URL for the cart. This parameter is required for [Abandoned Cart](https://mailchimp.com/help/create-a-classic-abandoned-cart-email/) automations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_url: Option<String>,
    /// The date and time the cart was created in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// The three-letter ISO 4217 code for the currency that the cart uses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<ECommerceCustomer>,
    /// A unique identifier for the cart.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// An array of the cart's line items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<Vec<ECommerceCartLineItem>>,
    /// The order total for the cart.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub order_total: Option<f64>,
    /// The total tax for the cart.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub tax_total: Option<f64>,
    /// The date and time the cart was last updated in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl ECommerceCart {
    pub fn builder() -> ECommerceCartBuilder {
        <ECommerceCartBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ECommerceCartBuilder {
    links: Option<Vec<ECommerceCartLinksItem>>,
    campaign_id: Option<String>,
    checkout_url: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    currency_code: Option<String>,
    customer: Option<ECommerceCustomer>,
    id: Option<String>,
    lines: Option<Vec<ECommerceCartLineItem>>,
    order_total: Option<f64>,
    tax_total: Option<f64>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl ECommerceCartBuilder {
    pub fn links(mut self, value: Vec<ECommerceCartLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    pub fn checkout_url(mut self, value: impl Into<String>) -> Self {
        self.checkout_url = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn currency_code(mut self, value: impl Into<String>) -> Self {
        self.currency_code = Some(value.into());
        self
    }

    pub fn customer(mut self, value: ECommerceCustomer) -> Self {
        self.customer = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn lines(mut self, value: Vec<ECommerceCartLineItem>) -> Self {
        self.lines = Some(value);
        self
    }

    pub fn order_total(mut self, value: f64) -> Self {
        self.order_total = Some(value);
        self
    }

    pub fn tax_total(mut self, value: f64) -> Self {
        self.tax_total = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ECommerceCart`].
    pub fn build(self) -> Result<ECommerceCart, BuildError> {
        Ok(ECommerceCart {
            links: self.links,
            campaign_id: self.campaign_id,
            checkout_url: self.checkout_url,
            created_at: self.created_at,
            currency_code: self.currency_code,
            customer: self.customer,
            id: self.id,
            lines: self.lines,
            order_total: self.order_total,
            tax_total: self.tax_total,
            updated_at: self.updated_at,
        })
    }
}
