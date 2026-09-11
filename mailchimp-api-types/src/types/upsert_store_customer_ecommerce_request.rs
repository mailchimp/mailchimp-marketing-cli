pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpsertStoreCustomerEcommerceRequest {
    /// The customer's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<UpsertStoreCustomerEcommerceRequestAddress>,
    /// The customer's company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// The customer's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// The customer's first name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// A unique identifier for the customer. Limited to 50 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The customer's last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The customer's opt-in status. This value will never overwrite the opt-in status of a pre-existing Mailchimp list member, but will apply to list members that are added through the e-commerce API endpoints. Customers who don't opt in to your Mailchimp list [will be added as `Transactional` members](https://mailchimp.com/developer/marketing/docs/e-commerce/#customers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_in_status: Option<bool>,
    /// A US phone number for SMS contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_spent: Option<UpsertStoreCustomerEcommerceRequestTotalSpent>,
}

impl UpsertStoreCustomerEcommerceRequest {
    pub fn builder() -> UpsertStoreCustomerEcommerceRequestBuilder {
        <UpsertStoreCustomerEcommerceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpsertStoreCustomerEcommerceRequestBuilder {
    address: Option<UpsertStoreCustomerEcommerceRequestAddress>,
    company: Option<String>,
    email_address: Option<String>,
    first_name: Option<String>,
    id: Option<String>,
    last_name: Option<String>,
    opt_in_status: Option<bool>,
    sms_phone_number: Option<String>,
    total_spent: Option<UpsertStoreCustomerEcommerceRequestTotalSpent>,
}

impl UpsertStoreCustomerEcommerceRequestBuilder {
    pub fn address(mut self, value: UpsertStoreCustomerEcommerceRequestAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn company(mut self, value: impl Into<String>) -> Self {
        self.company = Some(value.into());
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

    pub fn sms_phone_number(mut self, value: impl Into<String>) -> Self {
        self.sms_phone_number = Some(value.into());
        self
    }

    pub fn total_spent(mut self, value: UpsertStoreCustomerEcommerceRequestTotalSpent) -> Self {
        self.total_spent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpsertStoreCustomerEcommerceRequest`].
    pub fn build(self) -> Result<UpsertStoreCustomerEcommerceRequest, BuildError> {
        Ok(UpsertStoreCustomerEcommerceRequest {
            address: self.address,
            company: self.company,
            email_address: self.email_address,
            first_name: self.first_name,
            id: self.id,
            last_name: self.last_name,
            opt_in_status: self.opt_in_status,
            sms_phone_number: self.sms_phone_number,
            total_spent: self.total_spent,
        })
    }
}

