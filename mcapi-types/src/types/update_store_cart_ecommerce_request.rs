pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateStoreCartEcommerceRequest {
    /// A string that uniquely identifies the campaign associated with a cart.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// The URL for the cart. This parameter is required for [Abandoned Cart](https://mailchimp.com/help/create-a-classic-abandoned-cart-email/) automations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_url: Option<String>,
    /// The three-letter ISO 4217 code for the currency that the cart uses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<EcommerceStoresCartsPatch>,
    /// A unique identifier for the cart.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<UpdateStoreCartEcommerceRequestId>,
    /// An array of the cart's line items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<Vec<UpdateStoreCartEcommerceRequestLinesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_total: Option<UpdateStoreCartEcommerceRequestOrderTotal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total: Option<UpdateStoreCartEcommerceRequestTaxTotal>,
}

impl UpdateStoreCartEcommerceRequest {
    pub fn builder() -> UpdateStoreCartEcommerceRequestBuilder {
        <UpdateStoreCartEcommerceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateStoreCartEcommerceRequestBuilder {
    campaign_id: Option<String>,
    checkout_url: Option<String>,
    currency_code: Option<String>,
    customer: Option<EcommerceStoresCartsPatch>,
    id: Option<UpdateStoreCartEcommerceRequestId>,
    lines: Option<Vec<UpdateStoreCartEcommerceRequestLinesItem>>,
    order_total: Option<UpdateStoreCartEcommerceRequestOrderTotal>,
    tax_total: Option<UpdateStoreCartEcommerceRequestTaxTotal>,
}

impl UpdateStoreCartEcommerceRequestBuilder {
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

    pub fn customer(mut self, value: EcommerceStoresCartsPatch) -> Self {
        self.customer = Some(value);
        self
    }

    pub fn id(mut self, value: UpdateStoreCartEcommerceRequestId) -> Self {
        self.id = Some(value);
        self
    }

    pub fn lines(mut self, value: Vec<UpdateStoreCartEcommerceRequestLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    pub fn order_total(mut self, value: UpdateStoreCartEcommerceRequestOrderTotal) -> Self {
        self.order_total = Some(value);
        self
    }

    pub fn tax_total(mut self, value: UpdateStoreCartEcommerceRequestTaxTotal) -> Self {
        self.tax_total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateStoreCartEcommerceRequest`].
    pub fn build(self) -> Result<UpdateStoreCartEcommerceRequest, BuildError> {
        Ok(UpdateStoreCartEcommerceRequest {
            campaign_id: self.campaign_id,
            checkout_url: self.checkout_url,
            currency_code: self.currency_code,
            customer: self.customer,
            id: self.id,
            lines: self.lines,
            order_total: self.order_total,
            tax_total: self.tax_total,
        })
    }
}

