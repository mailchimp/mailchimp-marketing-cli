pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Information about a specific customer. Orders for existing customers should include only the `id` parameter in the `customer` object body.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EcommerceStoresCartsPatch {
    /// A unique identifier for the customer. Limited to 50 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The customer's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<EcommerceStoresCartsPatchAddress>,
    /// The customer's company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// The customer's first name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// The customer's last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The customer's opt-in status. This value will never overwrite the opt-in status of a pre-existing Mailchimp list member, but will apply to list members that are added through the e-commerce API endpoints. Customers who don't opt in to your Mailchimp list [will be added as `Transactional` members](https://mailchimp.com/developer/marketing/docs/e-commerce/#customers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_in_status: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_spent: Option<EcommerceStoresCartsPatchTotalSpent>,
}

impl EcommerceStoresCartsPatch {
    pub fn builder() -> EcommerceStoresCartsPatchBuilder {
        <EcommerceStoresCartsPatchBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EcommerceStoresCartsPatchBuilder {
    id: Option<String>,
    address: Option<EcommerceStoresCartsPatchAddress>,
    company: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    opt_in_status: Option<bool>,
    total_spent: Option<EcommerceStoresCartsPatchTotalSpent>,
}

impl EcommerceStoresCartsPatchBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn address(mut self, value: EcommerceStoresCartsPatchAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn company(mut self, value: impl Into<String>) -> Self {
        self.company = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
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

    pub fn total_spent(mut self, value: EcommerceStoresCartsPatchTotalSpent) -> Self {
        self.total_spent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EcommerceStoresCartsPatch`].
    pub fn build(self) -> Result<EcommerceStoresCartsPatch, BuildError> {
        Ok(EcommerceStoresCartsPatch {
            id: self.id,
            address: self.address,
            company: self.company,
            first_name: self.first_name,
            last_name: self.last_name,
            opt_in_status: self.opt_in_status,
            total_spent: self.total_spent,
        })
    }
}
