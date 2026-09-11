pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Information about the account contact.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListRootResponseContact {
    /// The street address for the account contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr1: Option<String>,
    /// The street address for the account contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr2: Option<String>,
    /// The city for the account contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The company name for the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// The country for the account contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The state for the account contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The zip code for the account contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
}

impl ListRootResponseContact {
    pub fn builder() -> ListRootResponseContactBuilder {
        <ListRootResponseContactBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListRootResponseContactBuilder {
    addr1: Option<String>,
    addr2: Option<String>,
    city: Option<String>,
    company: Option<String>,
    country: Option<String>,
    state: Option<String>,
    zip: Option<String>,
}

impl ListRootResponseContactBuilder {
    pub fn addr1(mut self, value: impl Into<String>) -> Self {
        self.addr1 = Some(value.into());
        self
    }

    pub fn addr2(mut self, value: impl Into<String>) -> Self {
        self.addr2 = Some(value.into());
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn company(mut self, value: impl Into<String>) -> Self {
        self.company = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn zip(mut self, value: impl Into<String>) -> Self {
        self.zip = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListRootResponseContact`].
    pub fn build(self) -> Result<ListRootResponseContact, BuildError> {
        Ok(ListRootResponseContact {
            addr1: self.addr1,
            addr2: self.addr2,
            city: self.city,
            company: self.company,
            country: self.country,
            state: self.state,
            zip: self.zip,
        })
    }
}
