pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateStoreOrderEcommerceRequest {
    /// The billing address for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<CreateStoreOrderEcommerceRequestBillingAddress>,
    /// A string that uniquely identifies the campaign for an order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// A cart id that the order was placed for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cart_id: Option<CreateStoreOrderEcommerceRequestCartId>,
    /// The date and time the order was cancelled in ISO 8601 format. Note: passing a value for this parameter will cancel the order being created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled_at_foreign: Option<String>,
    /// The three-letter ISO 4217 code for the currency that the store accepts.
    #[serde(default)]
    pub currency_code: String,
    #[serde(default)]
    pub customer: EcommerceStoresCartsPost,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_total: Option<CreateStoreOrderEcommerceRequestDiscountTotal>,
    /// The order status. Use this parameter to trigger [Order Notifications](https://mailchimp.com/developer/marketing/docs/e-commerce/#order-notifications).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_status: Option<String>,
    /// The fulfillment status for the order. Use this parameter to trigger [Order Notifications](https://mailchimp.com/developer/marketing/docs/e-commerce/#order-notifications).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_status: Option<String>,
    /// A unique identifier for the order.
    #[serde(default)]
    pub id: String,
    /// The URL for the page where the buyer landed when entering the shop.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landing_site: Option<String>,
    /// An array of the order's line items.
    #[serde(default)]
    pub lines: Vec<CreateStoreOrderEcommerceRequestLinesItem>,
    pub order_total: CreateStoreOrderEcommerceRequestOrderTotal,
    /// The URL for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_url: Option<String>,
    /// The outreach associated with this order. For example, an email campaign or Facebook ad.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outreach: Option<CreateStoreOrderEcommerceRequestOutreach>,
    /// The date and time the order was processed in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at_foreign: Option<String>,
    /// The promo codes applied on the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promos: Option<Vec<CreateStoreOrderEcommerceRequestPromosItem>>,
    /// The shipping address for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<CreateStoreOrderEcommerceRequestShippingAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_total: Option<CreateStoreOrderEcommerceRequestShippingTotal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total: Option<CreateStoreOrderEcommerceRequestTaxTotal>,
    /// The tracking carrier associated with the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_carrier: Option<String>,
    /// The Mailchimp tracking code for the order. Uses the 'mc_tc' parameter in E-Commerce tracking URLs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_code: Option<CreateStoreOrderEcommerceRequestTrackingCode>,
    /// The tracking number associated with the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
    /// The tracking URL associated with the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_url: Option<String>,
    /// The date and time the order was updated in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_foreign: Option<String>,
}

impl CreateStoreOrderEcommerceRequest {
    pub fn builder() -> CreateStoreOrderEcommerceRequestBuilder {
        <CreateStoreOrderEcommerceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateStoreOrderEcommerceRequestBuilder {
    billing_address: Option<CreateStoreOrderEcommerceRequestBillingAddress>,
    campaign_id: Option<String>,
    cart_id: Option<CreateStoreOrderEcommerceRequestCartId>,
    cancelled_at_foreign: Option<String>,
    currency_code: Option<String>,
    customer: Option<EcommerceStoresCartsPost>,
    discount_total: Option<CreateStoreOrderEcommerceRequestDiscountTotal>,
    financial_status: Option<String>,
    fulfillment_status: Option<String>,
    id: Option<String>,
    landing_site: Option<String>,
    lines: Option<Vec<CreateStoreOrderEcommerceRequestLinesItem>>,
    order_total: Option<CreateStoreOrderEcommerceRequestOrderTotal>,
    order_url: Option<String>,
    outreach: Option<CreateStoreOrderEcommerceRequestOutreach>,
    processed_at_foreign: Option<String>,
    promos: Option<Vec<CreateStoreOrderEcommerceRequestPromosItem>>,
    shipping_address: Option<CreateStoreOrderEcommerceRequestShippingAddress>,
    shipping_total: Option<CreateStoreOrderEcommerceRequestShippingTotal>,
    tax_total: Option<CreateStoreOrderEcommerceRequestTaxTotal>,
    tracking_carrier: Option<String>,
    tracking_code: Option<CreateStoreOrderEcommerceRequestTrackingCode>,
    tracking_number: Option<String>,
    tracking_url: Option<String>,
    updated_at_foreign: Option<String>,
}

impl CreateStoreOrderEcommerceRequestBuilder {
    pub fn billing_address(mut self, value: CreateStoreOrderEcommerceRequestBillingAddress) -> Self {
        self.billing_address = Some(value);
        self
    }

    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    pub fn cart_id(mut self, value: CreateStoreOrderEcommerceRequestCartId) -> Self {
        self.cart_id = Some(value);
        self
    }

    pub fn cancelled_at_foreign(mut self, value: impl Into<String>) -> Self {
        self.cancelled_at_foreign = Some(value.into());
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

    pub fn discount_total(mut self, value: CreateStoreOrderEcommerceRequestDiscountTotal) -> Self {
        self.discount_total = Some(value);
        self
    }

    pub fn financial_status(mut self, value: impl Into<String>) -> Self {
        self.financial_status = Some(value.into());
        self
    }

    pub fn fulfillment_status(mut self, value: impl Into<String>) -> Self {
        self.fulfillment_status = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn landing_site(mut self, value: impl Into<String>) -> Self {
        self.landing_site = Some(value.into());
        self
    }

    pub fn lines(mut self, value: Vec<CreateStoreOrderEcommerceRequestLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    pub fn order_total(mut self, value: CreateStoreOrderEcommerceRequestOrderTotal) -> Self {
        self.order_total = Some(value);
        self
    }

    pub fn order_url(mut self, value: impl Into<String>) -> Self {
        self.order_url = Some(value.into());
        self
    }

    pub fn outreach(mut self, value: CreateStoreOrderEcommerceRequestOutreach) -> Self {
        self.outreach = Some(value);
        self
    }

    pub fn processed_at_foreign(mut self, value: impl Into<String>) -> Self {
        self.processed_at_foreign = Some(value.into());
        self
    }

    pub fn promos(mut self, value: Vec<CreateStoreOrderEcommerceRequestPromosItem>) -> Self {
        self.promos = Some(value);
        self
    }

    pub fn shipping_address(mut self, value: CreateStoreOrderEcommerceRequestShippingAddress) -> Self {
        self.shipping_address = Some(value);
        self
    }

    pub fn shipping_total(mut self, value: CreateStoreOrderEcommerceRequestShippingTotal) -> Self {
        self.shipping_total = Some(value);
        self
    }

    pub fn tax_total(mut self, value: CreateStoreOrderEcommerceRequestTaxTotal) -> Self {
        self.tax_total = Some(value);
        self
    }

    pub fn tracking_carrier(mut self, value: impl Into<String>) -> Self {
        self.tracking_carrier = Some(value.into());
        self
    }

    pub fn tracking_code(mut self, value: CreateStoreOrderEcommerceRequestTrackingCode) -> Self {
        self.tracking_code = Some(value);
        self
    }

    pub fn tracking_number(mut self, value: impl Into<String>) -> Self {
        self.tracking_number = Some(value.into());
        self
    }

    pub fn tracking_url(mut self, value: impl Into<String>) -> Self {
        self.tracking_url = Some(value.into());
        self
    }

    pub fn updated_at_foreign(mut self, value: impl Into<String>) -> Self {
        self.updated_at_foreign = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateStoreOrderEcommerceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency_code`](CreateStoreOrderEcommerceRequestBuilder::currency_code)
    /// - [`customer`](CreateStoreOrderEcommerceRequestBuilder::customer)
    /// - [`id`](CreateStoreOrderEcommerceRequestBuilder::id)
    /// - [`lines`](CreateStoreOrderEcommerceRequestBuilder::lines)
    /// - [`order_total`](CreateStoreOrderEcommerceRequestBuilder::order_total)
    pub fn build(self) -> Result<CreateStoreOrderEcommerceRequest, BuildError> {
        Ok(CreateStoreOrderEcommerceRequest {
            billing_address: self.billing_address,
            campaign_id: self.campaign_id,
            cart_id: self.cart_id,
            cancelled_at_foreign: self.cancelled_at_foreign,
            currency_code: self.currency_code.ok_or_else(|| BuildError::missing_field("currency_code"))?,
            customer: self.customer.ok_or_else(|| BuildError::missing_field("customer"))?,
            discount_total: self.discount_total,
            financial_status: self.financial_status,
            fulfillment_status: self.fulfillment_status,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            landing_site: self.landing_site,
            lines: self.lines.ok_or_else(|| BuildError::missing_field("lines"))?,
            order_total: self.order_total.ok_or_else(|| BuildError::missing_field("order_total"))?,
            order_url: self.order_url,
            outreach: self.outreach,
            processed_at_foreign: self.processed_at_foreign,
            promos: self.promos,
            shipping_address: self.shipping_address,
            shipping_total: self.shipping_total,
            tax_total: self.tax_total,
            tracking_carrier: self.tracking_carrier,
            tracking_code: self.tracking_code,
            tracking_number: self.tracking_number,
            tracking_url: self.tracking_url,
            updated_at_foreign: self.updated_at_foreign,
        })
    }
}

