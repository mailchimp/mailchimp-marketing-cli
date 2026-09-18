pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Information about a specific customer. For existing customers include only the `id` parameter in the `customer` object body.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EcommerceStoresCartsPost {
    /// The customer's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<EcommerceStoresCartsPostAddress>,
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
    #[serde(default)]
    pub id: String,
    /// The customer's last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The customer's opt-in status. This value will never overwrite the opt-in status of a pre-existing Mailchimp list member, but will apply to list members that are added through the e-commerce API endpoints. Customers who don't opt in to your Mailchimp list [will be added as `Transactional` members](https://mailchimp.com/developer/marketing/docs/e-commerce/#customers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_in_status: Option<bool>,
}

impl EcommerceStoresCartsPost {
    pub fn builder() -> EcommerceStoresCartsPostBuilder {
        <EcommerceStoresCartsPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EcommerceStoresCartsPostBuilder {
    address: Option<EcommerceStoresCartsPostAddress>,
    company: Option<String>,
    email_address: Option<String>,
    first_name: Option<String>,
    id: Option<String>,
    last_name: Option<String>,
    opt_in_status: Option<bool>,
}

impl EcommerceStoresCartsPostBuilder {
    pub fn address(mut self, value: EcommerceStoresCartsPostAddress) -> Self {
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

    /// Consumes the builder and constructs a [`EcommerceStoresCartsPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](EcommerceStoresCartsPostBuilder::id)
    pub fn build(self) -> Result<EcommerceStoresCartsPost, BuildError> {
        Ok(EcommerceStoresCartsPost {
            address: self.address,
            company: self.company,
            email_address: self.email_address,
            first_name: self.first_name,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_name: self.last_name,
            opt_in_status: self.opt_in_status,
        })
    }
}
