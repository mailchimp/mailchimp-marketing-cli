pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateStoreCartRequest {
    /// A string that uniquely identifies the campaign for a cart.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// The URL for the cart. This parameter is required for [Abandoned Cart](https://mailchimp.com/help/create-a-classic-abandoned-cart-email/) automations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_url: Option<String>,
    /// The three-letter ISO 4217 code for the currency that the cart uses.
    #[serde(default)]
    pub currency_code: String,
    #[serde(default)]
    pub customer: EcommerceStoresCartsPost,
    /// A unique identifier for the cart.
    pub id: CreateStoreCartRequestId,
    /// An array of the cart's line items.
    #[serde(default)]
    pub lines: Vec<CreateStoreCartRequestLinesItem>,
    pub order_total: CreateStoreCartRequestOrderTotal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total: Option<CreateStoreCartRequestTaxTotal>,
}

impl CreateStoreCartRequest {
    pub fn builder() -> CreateStoreCartRequestBuilder {
        <CreateStoreCartRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateStoreCartRequestBuilder {
    campaign_id: Option<String>,
    checkout_url: Option<String>,
    currency_code: Option<String>,
    customer: Option<EcommerceStoresCartsPost>,
    id: Option<CreateStoreCartRequestId>,
    lines: Option<Vec<CreateStoreCartRequestLinesItem>>,
    order_total: Option<CreateStoreCartRequestOrderTotal>,
    tax_total: Option<CreateStoreCartRequestTaxTotal>,
}

impl CreateStoreCartRequestBuilder {
    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    pub fn checkout_url(mut self, value: impl Into<String>) -> Self {
        self.checkout_url = Some(value.into());
        self
    }

    pub fn currency_code(mut self, value: impl Into<String>) -> Self {
        self.currency_code = Some(value.into());
        self
    }

    pub fn customer(mut self, value: EcommerceStoresCartsPost) -> Self {
        self.customer = Some(value);
        self
    }

    pub fn id(mut self, value: CreateStoreCartRequestId) -> Self {
        self.id = Some(value);
        self
    }

    pub fn lines(mut self, value: Vec<CreateStoreCartRequestLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    pub fn order_total(mut self, value: CreateStoreCartRequestOrderTotal) -> Self {
        self.order_total = Some(value);
        self
    }

    pub fn tax_total(mut self, value: CreateStoreCartRequestTaxTotal) -> Self {
        self.tax_total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateStoreCartRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency_code`](CreateStoreCartRequestBuilder::currency_code)
    /// - [`customer`](CreateStoreCartRequestBuilder::customer)
    /// - [`id`](CreateStoreCartRequestBuilder::id)
    /// - [`lines`](CreateStoreCartRequestBuilder::lines)
    /// - [`order_total`](CreateStoreCartRequestBuilder::order_total)
    pub fn build(self) -> Result<CreateStoreCartRequest, BuildError> {
        Ok(CreateStoreCartRequest {
            campaign_id: self.campaign_id,
            checkout_url: self.checkout_url,
            currency_code: self.currency_code.ok_or_else(|| BuildError::missing_field("currency_code"))?,
            customer: self.customer.ok_or_else(|| BuildError::missing_field("customer"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            lines: self.lines.ok_or_else(|| BuildError::missing_field("lines"))?,
            order_total: self.order_total.ok_or_else(|| BuildError::missing_field("order_total"))?,
            tax_total: self.tax_total,
        })
    }
}

