pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Information about a specific order.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ECommerceOrder {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ECommerceOrderLinksItem>>,
    /// The billing address for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<ECommerceOrderBillingAddress>,
    /// A string that uniquely identifies the campaign associated with an order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// A cart id that the order was placed for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cart_id: Option<String>,
    /// The date and time the order was cancelled in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub cancelled_at_foreign: Option<DateTime<FixedOffset>>,
    /// The three-letter ISO 4217 code for the currency that the store accepts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<ECommerceCustomer>,
    /// The total amount of the discounts to be applied to the price of the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub discount_total: Option<f64>,
    /// The order status. Use this parameter to trigger [Order Notifications](https://mailchimp.com/developer/marketing/docs/e-commerce/#order-notifications).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_status: Option<String>,
    /// The fulfillment status for the order. Use this parameter to trigger [Order Notifications](https://mailchimp.com/developer/marketing/docs/e-commerce/#order-notifications).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_status: Option<String>,
    /// A unique identifier for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The URL for the page where the buyer landed when entering the shop.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landing_site: Option<String>,
    /// An array of the order's line items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<Vec<ECommerceOrderLineItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_total: Option<ECommerceOrderOrderTotal>,
    /// The URL for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_url: Option<String>,
    /// The outreach associated with this order. For example, an email campaign or Facebook ad.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outreach: Option<ECommerceOrderOutreach>,
    /// The date and time the order was processed in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub processed_at_foreign: Option<DateTime<FixedOffset>>,
    /// The promo codes applied on the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promos: Option<Vec<ECommerceOrderPromosItem>>,
    /// The shipping address for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ECommerceOrderShippingAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_total: Option<ECommerceOrderShippingTotal>,
    /// The unique identifier for the store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total: Option<ECommerceOrderTaxTotal>,
    /// The tracking carrier associated with the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_carrier: Option<String>,
    /// The Mailchimp tracking code for the order. Uses the 'mc_tc' parameter in E-Commerce tracking URLs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_code: Option<ECommerceOrderTrackingCode>,
    /// The tracking number associated with the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
    /// The tracking URL associated with the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_url: Option<String>,
    /// The date and time the order was updated in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at_foreign: Option<DateTime<FixedOffset>>,
}

impl ECommerceOrder {
    pub fn builder() -> ECommerceOrderBuilder {
        <ECommerceOrderBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ECommerceOrderBuilder {
    links: Option<Vec<ECommerceOrderLinksItem>>,
    billing_address: Option<ECommerceOrderBillingAddress>,
    campaign_id: Option<String>,
    cart_id: Option<String>,
    cancelled_at_foreign: Option<DateTime<FixedOffset>>,
    currency_code: Option<String>,
    customer: Option<ECommerceCustomer>,
    discount_total: Option<f64>,
    financial_status: Option<String>,
    fulfillment_status: Option<String>,
    id: Option<String>,
    landing_site: Option<String>,
    lines: Option<Vec<ECommerceOrderLineItem>>,
    order_total: Option<ECommerceOrderOrderTotal>,
    order_url: Option<String>,
    outreach: Option<ECommerceOrderOutreach>,
    processed_at_foreign: Option<DateTime<FixedOffset>>,
    promos: Option<Vec<ECommerceOrderPromosItem>>,
    shipping_address: Option<ECommerceOrderShippingAddress>,
    shipping_total: Option<ECommerceOrderShippingTotal>,
    store_id: Option<String>,
    tax_total: Option<ECommerceOrderTaxTotal>,
    tracking_carrier: Option<String>,
    tracking_code: Option<ECommerceOrderTrackingCode>,
    tracking_number: Option<String>,
    tracking_url: Option<String>,
    updated_at_foreign: Option<DateTime<FixedOffset>>,
}

impl ECommerceOrderBuilder {
    pub fn links(mut self, value: Vec<ECommerceOrderLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn billing_address(mut self, value: ECommerceOrderBillingAddress) -> Self {
        self.billing_address = Some(value);
        self
    }

    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    pub fn cart_id(mut self, value: impl Into<String>) -> Self {
        self.cart_id = Some(value.into());
        self
    }

    pub fn cancelled_at_foreign(mut self, value: DateTime<FixedOffset>) -> Self {
        self.cancelled_at_foreign = Some(value);
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

    pub fn discount_total(mut self, value: f64) -> Self {
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

    pub fn lines(mut self, value: Vec<ECommerceOrderLineItem>) -> Self {
        self.lines = Some(value);
        self
    }

    pub fn order_total(mut self, value: ECommerceOrderOrderTotal) -> Self {
        self.order_total = Some(value);
        self
    }

    pub fn order_url(mut self, value: impl Into<String>) -> Self {
        self.order_url = Some(value.into());
        self
    }

    pub fn outreach(mut self, value: ECommerceOrderOutreach) -> Self {
        self.outreach = Some(value);
        self
    }

    pub fn processed_at_foreign(mut self, value: DateTime<FixedOffset>) -> Self {
        self.processed_at_foreign = Some(value);
        self
    }

    pub fn promos(mut self, value: Vec<ECommerceOrderPromosItem>) -> Self {
        self.promos = Some(value);
        self
    }

    pub fn shipping_address(mut self, value: ECommerceOrderShippingAddress) -> Self {
        self.shipping_address = Some(value);
        self
    }

    pub fn shipping_total(mut self, value: ECommerceOrderShippingTotal) -> Self {
        self.shipping_total = Some(value);
        self
    }

    pub fn store_id(mut self, value: impl Into<String>) -> Self {
        self.store_id = Some(value.into());
        self
    }

    pub fn tax_total(mut self, value: ECommerceOrderTaxTotal) -> Self {
        self.tax_total = Some(value);
        self
    }

    pub fn tracking_carrier(mut self, value: impl Into<String>) -> Self {
        self.tracking_carrier = Some(value.into());
        self
    }

    pub fn tracking_code(mut self, value: ECommerceOrderTrackingCode) -> Self {
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

    pub fn updated_at_foreign(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at_foreign = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ECommerceOrder`].
    pub fn build(self) -> Result<ECommerceOrder, BuildError> {
        Ok(ECommerceOrder {
            links: self.links,
            billing_address: self.billing_address,
            campaign_id: self.campaign_id,
            cart_id: self.cart_id,
            cancelled_at_foreign: self.cancelled_at_foreign,
            currency_code: self.currency_code,
            customer: self.customer,
            discount_total: self.discount_total,
            financial_status: self.financial_status,
            fulfillment_status: self.fulfillment_status,
            id: self.id,
            landing_site: self.landing_site,
            lines: self.lines,
            order_total: self.order_total,
            order_url: self.order_url,
            outreach: self.outreach,
            processed_at_foreign: self.processed_at_foreign,
            promos: self.promos,
            shipping_address: self.shipping_address,
            shipping_total: self.shipping_total,
            store_id: self.store_id,
            tax_total: self.tax_total,
            tracking_carrier: self.tracking_carrier,
            tracking_code: self.tracking_code,
            tracking_number: self.tracking_number,
            tracking_url: self.tracking_url,
            updated_at_foreign: self.updated_at_foreign,
        })
    }
}
