pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Information about a specific customer.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ECommerceCustomer {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ECommerceCustomerLinksItem>>,
    /// The customer's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<ECommerceCustomerAddress>,
    /// The customer's company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// The date and time the customer was created in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// The customer's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// The customer's first name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// A unique identifier for the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The customer's last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The customer's opt-in status. This value will never overwrite the opt-in status of a pre-existing Mailchimp list member, but will apply to list members that are added through the e-commerce API endpoints. Customers who don't opt in to your Mailchimp list [will be added as `Transactional` members](https://mailchimp.com/developer/marketing/docs/e-commerce/#customers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_in_status: Option<bool>,
    /// The customer's total order count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orders_count: Option<i64>,
    /// A US phone number for SMS contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_spent: Option<ECommerceCustomerTotalSpent>,
    /// The date and time the customer was last updated in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl ECommerceCustomer {
    pub fn builder() -> ECommerceCustomerBuilder {
        <ECommerceCustomerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ECommerceCustomerBuilder {
    links: Option<Vec<ECommerceCustomerLinksItem>>,
    address: Option<ECommerceCustomerAddress>,
    company: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    email_address: Option<String>,
    first_name: Option<String>,
    id: Option<String>,
    last_name: Option<String>,
    opt_in_status: Option<bool>,
    orders_count: Option<i64>,
    sms_phone_number: Option<String>,
    total_spent: Option<ECommerceCustomerTotalSpent>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl ECommerceCustomerBuilder {
    pub fn links(mut self, value: Vec<ECommerceCustomerLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn address(mut self, value: ECommerceCustomerAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn company(mut self, value: impl Into<String>) -> Self {
        self.company = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn email_address(mut self, value: impl Into<String>) -> Self {
        self.email_address = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn opt_in_status(mut self, value: bool) -> Self {
        self.opt_in_status = Some(value);
        self
    }

    pub fn orders_count(mut self, value: i64) -> Self {
        self.orders_count = Some(value);
        self
    }

    pub fn sms_phone_number(mut self, value: impl Into<String>) -> Self {
        self.sms_phone_number = Some(value.into());
        self
    }

    pub fn total_spent(mut self, value: ECommerceCustomerTotalSpent) -> Self {
        self.total_spent = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ECommerceCustomer`].
    pub fn build(self) -> Result<ECommerceCustomer, BuildError> {
        Ok(ECommerceCustomer {
            links: self.links,
            address: self.address,
            company: self.company,
            created_at: self.created_at,
            email_address: self.email_address,
            first_name: self.first_name,
            id: self.id,
            last_name: self.last_name,
            opt_in_status: self.opt_in_status,
            orders_count: self.orders_count,
            sms_phone_number: self.sms_phone_number,
            total_spent: self.total_spent,
            updated_at: self.updated_at,
        })
    }
}
