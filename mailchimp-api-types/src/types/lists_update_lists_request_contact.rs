pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// [Contact information displayed in campaign footers](https://mailchimp.com/help/about-campaign-footers/) to comply with international spam laws.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateListsRequestContact {
    /// The street address for the list contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address1: Option<String>,
    /// The street address for the list contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,
    /// The city for the list contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The company name for the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// A two-character ISO3166 country code. Defaults to US if invalid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The phone number for the list contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// The state for the list contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The postal or zip code for the list contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
}

impl UpdateListsRequestContact {
    pub fn builder() -> UpdateListsRequestContactBuilder {
        <UpdateListsRequestContactBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateListsRequestContactBuilder {
    address1: Option<String>,
    address2: Option<String>,
    city: Option<String>,
    company: Option<String>,
    country: Option<String>,
    phone: Option<String>,
    state: Option<String>,
    zip: Option<String>,
}

impl UpdateListsRequestContactBuilder {
    pub fn address1(mut self, value: impl Into<String>) -> Self {
        self.address1 = Some(value.into());
        self
    }

    pub fn address2(mut self, value: impl Into<String>) -> Self {
        self.address2 = Some(value.into());
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

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
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

    /// Consumes the builder and constructs a [`UpdateListsRequestContact`].
    pub fn build(self) -> Result<UpdateListsRequestContact, BuildError> {
        Ok(UpdateListsRequestContact {
            address1: self.address1,
            address2: self.address2,
            city: self.city,
            company: self.company,
            country: self.country,
            phone: self.phone,
            state: self.state,
            zip: self.zip,
        })
    }
}
